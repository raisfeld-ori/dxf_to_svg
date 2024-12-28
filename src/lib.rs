use dxf::entities::{Entity, EntityType};

/**
Takes in a vector of entities and displays them as an SVG string.
If an entity is not supported, it will be printed to the console and skipped.

 * entities - the list of entities you wish to turn into a string.
 * returns a string SVG representation of the entities.
 */
pub fn dxf_to_svg(entities: Vec<&Entity>) -> String {
    let mut svg = String::new();
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\">");
    for entity in entities {
        let mut color = entity.common.color_name.as_str();
        if color.trim().is_empty() { color = "Black"; }
        match &entity.specific {
            EntityType::Line(line) => {
                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" />"#,
                    line.p1.x, line.p1.y, line.p2.x, line.p2.y, color
                ));
            }

            EntityType::Insert(insert) => {
                svg.push_str(&format!(
                    r#"<use xlink:href="{}" x="{}" y="{}" width="{}" height="{}" />"#,
                    insert.name,
                    insert.location.x, insert.location.y,
                    insert.x_scale_factor, insert.y_scale_factor
                ));
            }

            EntityType::LwPolyline(lwpolyline) => {
                svg.push_str(&format!(
                    r#"<polyline points="{}" stroke="{}" fill="none" />"#,
                    lwpolyline.vertices.iter()
                        .map(|p| format!("{},{} ", p.x, p.y))
                        .collect::<String>(),
                    color
                ));
            }

            EntityType::Polyline(polyline) => {
                svg.push_str(&format!(
                    r#"<polyline points="{}" stroke="{}" fill="none" />"#,
                    polyline.vertices()
                        .map(|p| format!("{},{} ", p.location.x, p.location.y))
                        .collect::<String>(),
                    color
                ));
            }

            EntityType::Circle(circle) => {
                svg.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="{}" stroke="{}" fill="none" />"#,
                    circle.center.x, circle.center.y, circle.radius, color
                ));
            }

            EntityType::Arc(arc) => {
                // Calculate start and end points
                let start_angle = arc.start_angle.to_radians();
                let end_angle = arc.end_angle.to_radians();
                let start_x = arc.center.x + arc.radius * start_angle.cos();
                let start_y = arc.center.y + arc.radius * start_angle.sin();
                let end_x = arc.center.x + arc.radius * end_angle.cos();
                let end_y = arc.center.y + arc.radius * end_angle.sin();
                
                // Determine if arc should be drawn clockwise or counterclockwise
                let large_arc = (end_angle - start_angle).abs() > std::f64::consts::PI;
                let sweep = if end_angle > start_angle { 1 } else { 0 };
                
                svg.push_str(&format!(
                    r#"<path d="M {},{} A {},{} 0 {} {} {},{}" stroke="{}" fill="none" />"#,
                    start_x, start_y,
                    arc.radius, arc.radius,
                    if large_arc { 1 } else { 0 },
                    sweep,
                    end_x, end_y,
                    color
                ));
            }

            EntityType::Ellipse(ellipse) => {
                // Calculate major axis length
                let major_axis_length = (
                    ellipse.major_axis.x.powi(2) + 
                    ellipse.major_axis.y.powi(2)
                ).sqrt();
                
                // Calculate rotation angle in degrees
                let rotation = ellipse.major_axis.y.atan2(ellipse.major_axis.x).to_degrees();
                
                svg.push_str(&format!(
                    r#"<ellipse cx="{}" cy="{}" rx="{}" ry="{}" transform="rotate({} {} {})" stroke="{}" fill="none" />"#,
                    ellipse.center.x, ellipse.center.y,
                    major_axis_length,
                    major_axis_length * ellipse.minor_axis_ratio,
                    rotation,
                    ellipse.center.x, ellipse.center.y,
                    color
                ));
            }

            EntityType::Spline(spline) => {
                // Convert spline control points to SVG path
                let mut path = String::from("M ");
                if let Some(first_point) = spline.control_points.first() {
                    path.push_str(&format!("{},{} ", first_point.x, first_point.y));
                    path.push_str("C ");
                    
                    for window in spline.control_points.windows(3) {
                        path.push_str(&format!("{},{} {},{} {},{} ",
                            window[0].x, window[0].y,
                            window[1].x, window[1].y,
                            window[2].x, window[2].y
                        ));
                    }
                }
                
                svg.push_str(&format!(
                    r#"<path d="{}" stroke="{}" fill="none" />"#,
                    path.trim(),
                    color
                ));
            }

            EntityType::AngularThreePointDimension(_angular_three_point_dimension) => {
                let point1 = &_angular_three_point_dimension.definition_point_2;
                let point2 = &_angular_three_point_dimension.definition_point_3;
                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" />"#,
                    point1.x, point1.y, point2.x, point2.y, color
                ));
            }

            EntityType::ArcAlignedText(_angular_dimension) => {
                let center = &_angular_dimension.center_point;
                let text = &_angular_dimension.text; 
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" fill="{}">{}</text>"#,
                    center.x,
                    center.y,
                    color,
                    text
                ));
            }

            EntityType::Text(text) => {
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" fill="{}">{}</text>"#,
                    text.location.x,
                    text.location.y,
                    color,
                    text.value
                ));
            }

            EntityType::ModelPoint(point) => {
                svg.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="{}" stroke="{}" fill="none" />"#,
                    point.location.x, point.location.y, 1.0, color
                ));
            }

            _ => {
                println!("Unsupported entity type: {:?}. Continuing without this entity...", entity.specific);
                continue;
            }
        }
    }
    svg.push_str("</svg>");
    svg
}

#[cfg(test)]
mod tests {
    use dxf::{entities::Line, Point};
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_basic_entities() {
        // Test empty vector
        assert_eq!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>",
            dxf_to_svg(vec![])
        );

        let mut entities = vec![];
        let line = Entity::new(EntityType::Line(
            Line::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0)
            )
        ));
        entities.push(&line);
        assert_eq!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\"><line x1=\"0\" y1=\"0\" x2=\"1\" y2=\"1\" stroke=\"Black\" /></svg>",
            dxf_to_svg(entities)
        );
    }

    #[test]
    fn test_arc() {
        let mut entities = vec![];
        let arc = Entity::new(EntityType::Arc(
            dxf::entities::Arc::new(
                Point::new(0.0, 0.0, 0.0),
                1.0,
                0.0,
                PI / 2.0
            )
        ));
        entities.push(&arc);
        // We don't assert the exact string here since floating point formatting might vary
        let result = dxf_to_svg(entities);
        assert!(result.contains("<path"));
        assert!(result.contains("stroke=\"Black\""));
    }
}