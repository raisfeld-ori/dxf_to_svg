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
                let start_angle = arc.start_angle.to_radians();
                let end_angle = arc.end_angle.to_radians();
                let center_x = arc.center.x;
                let center_y = arc.center.y;
                let radius = arc.radius;
                let start_x = center_x + radius * start_angle.cos();
                let start_y = center_y + radius * start_angle.sin();
                let end_x = center_x + radius * end_angle.cos();
                let end_y = center_y + radius * end_angle.sin();
                let large_arc_flag = if (end_angle - start_angle).abs() > std::f64::consts::PI { "1" } else { "0" };
            
                svg.push_str(&format!(
                    r#"<path d="M {} {} A {} {} {} {} {} stroke="{}" fill="none" />"#,
                    start_x, start_y, radius, radius, large_arc_flag, end_x, end_y, color
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
            EntityType::Attribute(_attribute) => {
                println!("Unsupported entity type: {:?}. Continuing without this entity...", entity.specific);
                continue;
            }
            EntityType::Ellipse(_ellipse) => {
                let center = &_ellipse.center;
                let radius = &_ellipse.normal;
                svg.push_str(&format!(
                    r#"<ellipse cx="{}" cy="{}" rx="{}" ry="{}" stroke="{}" fill="none" />"#,
                    center.x,
                    center.y,
                    radius.x,
                    radius.y,
                    color
                ));
            }
            EntityType::Image(_image) => {
                println!("Unsupported entity type: {:?}. Continuing without this entity...", entity.specific);
                continue;
            }
            EntityType::Light(_light) => {
                println!("Unsupported entity type: {:?}. Continuing without this entity...", entity.specific);
                continue;
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
