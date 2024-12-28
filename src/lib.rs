use dxf::entities::{Entity, EntityType};
use std::f64::consts::PI;

/**
Takes in a vector of entities and displays them as an SVG string.
If an entity is not supported, it will be printed to the console and skipped.

* `entities` - the list of entities you wish to turn into a string.
* Returns a string SVG representation of the entities.
*/
pub fn dxf_to_svg(entities: Vec<&Entity>) -> String {
    let mut svg = String::new();
    svg.push_str(
        r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" height="100%" width="100%" viewBox="0 0 100 100">"#,
    );

    for entity in entities {
        let color = if entity.common.color_name.trim().is_empty() {
            "black"
        } else {
            entity.common.color_name.as_str()
        };

        match &entity.specific {
            EntityType::Line(line) => {
                svg.push_str(&format!(
                    r#"<line x1="{:.3}" y1="{:.3}" x2="{:.3}" y2="{:.3}" stroke="{}" />"#,
                    line.p1.x, line.p1.y, line.p2.x, line.p2.y, color
                ));
            }

            EntityType::Insert(insert) => {
                svg.push_str(&format!(
                    r#"<use href="{}{}" x="{:.3}" y="{:.3}" width="{:.3}" height="{:.3}" />"#,
                    "#",
                    insert.name,
                    insert.location.x, insert.location.y,
                    insert.x_scale_factor, insert.y_scale_factor
                ));
            }

            EntityType::LwPolyline(lwpolyline) => {
                if lwpolyline.vertices.is_empty() {
                    continue;
                }
                svg.push_str(&format!(
                    r#"<polyline points="{}" stroke="{}" fill="none" />"#,
                    lwpolyline.vertices.iter()
                        .map(|p| format!("{:.3},{:.3}", p.x, p.y))
                        .collect::<Vec<_>>()
                        .join(" "),
                    color
                ));
            }

            EntityType::Polyline(polyline) => {
                let vertices: Vec<_> = polyline.vertices().collect();
                if vertices.is_empty() {
                    continue;
                }
                svg.push_str(&format!(
                    r#"<polyline points="{}" stroke="{}" fill="none" />"#,
                    vertices.iter()
                        .map(|p| format!("{:.3},{:.3}", p.location.x, p.location.y))
                        .collect::<Vec<_>>()
                        .join(" "),
                    color
                ));
            }

            EntityType::Circle(circle) => {
                svg.push_str(&format!(
                    r#"<circle cx="{:.3}" cy="{:.3}" r="{:.3}" stroke="{}" fill="none" />"#,
                    circle.center.x, circle.center.y, circle.radius, color
                ));
            }

            EntityType::Arc(arc) => {
                let start_angle = arc.start_angle.to_radians();
                let end_angle = arc.end_angle.to_radians();
                let start_x = arc.center.x + arc.radius * start_angle.cos();
                let start_y = arc.center.y + arc.radius * start_angle.sin();
                let end_x = arc.center.x + arc.radius * end_angle.cos();
                let end_y = arc.center.y + arc.radius * end_angle.sin();
                
                let sweep = if end_angle > start_angle { 1 } else { 0 };
                let large_arc = if (end_angle - start_angle).abs() % (2.0 * PI) > PI { 1 } else { 0 };
                
                svg.push_str(&format!(
                    r#"<path d="M {:.3},{:.3} A {:.3},{:.3} 0 {} {} {:.3},{:.3}" stroke="{}" fill="none" />"#,
                    start_x, start_y,
                    arc.radius, arc.radius,
                    large_arc, sweep,
                    end_x, end_y,
                    color
                ));
            }

            EntityType::Ellipse(ellipse) => {
                let major_axis_length = (
                    ellipse.major_axis.x.powi(2) + 
                    ellipse.major_axis.y.powi(2)
                ).sqrt();
                
                let rotation = ellipse.major_axis.y.atan2(ellipse.major_axis.x).to_degrees();
                
                svg.push_str(&format!(
                    r#"<ellipse cx="{:.3}" cy="{:.3}" rx="{:.3}" ry="{:.3}" transform="rotate({:.3} {} {})" stroke="{}" fill="none" />"#,
                    ellipse.center.x, ellipse.center.y,
                    major_axis_length,
                    major_axis_length * ellipse.minor_axis_ratio,
                    rotation,
                    ellipse.center.x, ellipse.center.y,  // Rotate around the center point
                    color
                ));
            }

            EntityType::Spline(spline) => {
                if spline.control_points.len() < 2 {
                    continue;
                }

                let mut path = String::new();
                let points = &spline.control_points;
                
                // Start path at first point
                path.push_str(&format!("M {:.3},{:.3} ", points[0].x, points[0].y));
                
                // Use cubic Bézier curves between points
                let mut i = 1;
                while i < points.len() - 2 {
                    path.push_str(&format!("C {:.3},{:.3} {:.3},{:.3} {:.3},{:.3} ",
                        points[i].x, points[i].y,
                        points[i + 1].x, points[i + 1].y,
                        points[i + 2].x, points[i + 2].y
                    ));
                    i += 3;
                }
                
                svg.push_str(&format!(
                    r#"<path d="{}" stroke="{}" fill="none" />"#,
                    path.trim(),
                    color
                ));
            }

            EntityType::Text(text) => {
                // Escape special characters in text
                let escaped_text = escape_xml_text(&text.value);
                svg.push_str(&format!(
                    r#"<text x="{:.3}" y="{:.3}" fill="{}">{}</text>"#,
                    text.location.x,
                    text.location.y,
                    color,
                    escaped_text
                ));
            }

            EntityType::ModelPoint(point) => {
                svg.push_str(&format!(
                    r#"<circle cx="{:.3}" cy="{:.3}" r="1" stroke="{}" fill="none" />"#,
                    point.location.x, point.location.y, color
                ));
            }
            EntityType::RotatedDimension(dimension) => {
                let start_point = &dimension.definition_point_2; // Start of dimension line
                let end_point = &dimension.definition_point_3;   // End of dimension line
                let text_position = &dimension.insertion_point;    // Midpoint for text
                let measurement = &dimension.dimension_base.text; // Measurement text
            
                svg.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
                svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-100 -100 200 200" xmlns:xlink="http://www.w3.org/1999/xlink">"#);
            
                // Add the dimension line
                svg.push_str(&format!(
                    r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="black" />"#,
                    x1 = start_point.x,
                    y1 = start_point.y,
                    x2 = end_point.x,
                    y2 = end_point.y
                ));
            
                // Add the dimension text
                svg.push_str(&format!(
                    r#"<text x="{x}" y="{y}" fill="black" font-size="12" text-anchor="middle">{text}</text>"#,
                    x = text_position.x,
                    y = text_position.y,
                    text = measurement
                ));
            }
            _ => {
                println!("Unsupported entity type: {:?}", entity.common.layer);
                continue;
            }
        }
    }
    svg.push_str("</svg>");
    svg
}

/// Escape special characters in XML text content
fn escape_xml_text(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use dxf::entities::Line;
    use dxf::Point;

    #[test]
    fn test_basic_entities() {
        // Test empty vector
        let empty_svg = dxf_to_svg(vec![]);
        assert!(empty_svg.contains("viewBox=\"0 0 100 100\""));
        assert!(empty_svg.starts_with("<svg"));
        assert!(empty_svg.ends_with("</svg>"));

        let mut entities = vec![];
        let line = Entity::new(EntityType::Line(
            Line::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0)
            )
        ));
        entities.push(&line);
        let result = dxf_to_svg(entities);
        assert!(result.contains("viewBox"));
        assert!(result.contains("<line"));
        assert!(result.contains("stroke=\"black\""));
    }

    #[test]
    fn test_text_escaping() {
        assert_eq!(
            escape_xml_text("Test & <example>"),
            "Test &amp; &lt;example&gt;"
        );
    }
}