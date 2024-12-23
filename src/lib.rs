use dxf::entities::{Entity, EntityType};

pub fn dxf_to_svg(entities: Vec<&Entity>) -> String {
    let mut svg = String::new();
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\">");
    for entity in entities {
        let mut color = entity.common.color_name.as_str();
        if color.trim().is_empty(){color = "Black";}
        match &entity.specific {
            EntityType::Line(line) => {
                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" />"#,
                    line.p1.x, line.p1.y, line.p2.x, line.p2.y, color
                ));
            }
            EntityType::Circle(circle) => {
                svg.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="{}" stroke="{}" fill="none" />"#,
                    circle.center.x, circle.center.y, circle.radius, color
                ));
            }
            EntityType::Arc(arc) => {
                // Convert DXF arc to SVG path
                let start_angle = arc.start_angle.to_radians();
                let end_angle = arc.end_angle.to_radians();
                let start_x = arc.center.x + arc.radius * start_angle.cos();
                let start_y = arc.center.y + arc.radius * start_angle.sin();
                let end_x = arc.center.x + arc.radius * end_angle.cos();
                let end_y = arc.center.y + arc.radius * end_angle.sin();
                let large_arc_flag = if (arc.end_angle - arc.start_angle).abs() > 180.0 { 1 } else { 0 };
                
                svg.push_str(&format!(
                    r#"<path d="M {} {} A {} {} 0 {} 1 {} {}" stroke="{}" fill="none" />"#,
                    start_x, start_y, arc.radius, arc.radius, large_arc_flag, end_x, end_y, color
                ));
            }
            // Handle other entity types (e.g., Polyline, Ellipse) similarly
            _ => {}
        }
    }
    svg.push_str("</svg>");
    svg
}

#[cfg(test)]
mod tests {
    use dxf::{entities::Line, Point};

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>", dxf_to_svg(vec![]));
        let mut entities = vec![];
        let entity = Entity::new(EntityType::Line(Line::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0))));
        entities.push(&entity);
        assert_eq!("<svg xmlns=\"http://www.w3.org/2000/svg\"><line x1=\"0\" y1=\"0\" x2=\"1\" y2=\"1\" stroke=\"Black\" /></svg>", dxf_to_svg(entities));
    }
}
