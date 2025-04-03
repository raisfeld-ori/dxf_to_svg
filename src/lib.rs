use dxf::entities::{Entity, EntityType};
use std::f64::consts::PI;

#[derive(Debug)]
struct Bounds {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Bounds {
    fn new() -> Self {
        Bounds {
            min_x: f64::INFINITY,
            min_y: f64::INFINITY,
            max_x: f64::NEG_INFINITY,
            max_y: f64::NEG_INFINITY,
        }
    }

    fn update(&mut self, x: f64, y: f64) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }

    // Add padding to the bounds
    fn with_padding(&self, padding_percent: f64) -> Self {
        let width = self.max_x - self.min_x;
        let height = self.max_y - self.min_y;
        let padding_x = width * padding_percent;
        let padding_y = height * padding_percent;

        Bounds {
            min_x: self.min_x - padding_x,
            min_y: self.min_y - padding_y,
            max_x: self.max_x + padding_x,
            max_y: self.max_y + padding_y,
        }
    }
}

fn is_angle_in_arc(angle: f64, start: f64, end: f64) -> bool {
    let mut angle = angle % (2.0 * PI);
    let start = start % (2.0 * PI);
    let mut end = end % (2.0 * PI);
    
    if start > end {
        end += 2.0 * PI;
        if angle < start {
            angle += 2.0 * PI;
        }
    }
    
    angle >= start && angle <= end
}

fn calculate_bounds(entities: &[&Entity]) -> Bounds {
    let mut bounds = Bounds::new();

    for entity in entities {
        match &entity.specific {
            EntityType::Line(line) => {
                bounds.update(line.p1.x, line.p1.y);
                bounds.update(line.p2.x, line.p2.y);
            }
            EntityType::Circle(circle) => {
                bounds.update(circle.center.x - circle.radius, circle.center.y - circle.radius);
                bounds.update(circle.center.x + circle.radius, circle.center.y + circle.radius);
            }
            EntityType::Arc(arc) => {
                // For arcs, we need to check start, end, and potential extreme points
                let start_angle = arc.start_angle.to_radians();
                let end_angle = arc.end_angle.to_radians();
                
                // Check start and end points
                bounds.update(
                    arc.center.x + arc.radius * start_angle.cos(),
                    arc.center.y + arc.radius * start_angle.sin()
                );
                bounds.update(
                    arc.center.x + arc.radius * end_angle.cos(),
                    arc.center.y + arc.radius * end_angle.sin()
                );
                
                // Check extreme points if they fall within the arc
                let angles = [0.0, PI/2.0, PI, 3.0*PI/2.0];
                for &angle in &angles {
                    if is_angle_in_arc(angle, start_angle, end_angle) {
                        bounds.update(
                            arc.center.x + arc.radius * angle.cos(),
                            arc.center.y + arc.radius * angle.sin()
                        );
                    }
                }
            }
            EntityType::LwPolyline(lwpolyline) => {
                for vertex in &lwpolyline.vertices {
                    bounds.update(vertex.x, vertex.y);
                }
            }
            EntityType::Polyline(polyline) => {
                for vertex in polyline.vertices() {
                    bounds.update(vertex.location.x, vertex.location.y);
                }
            }
            EntityType::Ellipse(ellipse) => {
                // Calculate the bounding box of the ellipse
                let major_axis_length = (
                    ellipse.major_axis.x.powi(2) + 
                    ellipse.major_axis.y.powi(2)
                ).sqrt();
                let minor_axis_length = major_axis_length * ellipse.minor_axis_ratio;
                
                bounds.update(ellipse.center.x - major_axis_length, ellipse.center.y - minor_axis_length);
                bounds.update(ellipse.center.x + major_axis_length, ellipse.center.y + minor_axis_length);
            }
            EntityType::Text(text) => {
                // For text, just use the insertion point
                // Note: This is a simplification as it doesn't account for text size
                bounds.update(text.location.x, text.location.y);
            }
            EntityType::ModelPoint(point) => {
                bounds.update(point.location.x, point.location.y);
            }
            EntityType::Face3D(face) => {
                bounds.update(face.first_corner.x, face.first_corner.y);
                bounds.update(face.second_corner.x, face.second_corner.y);
                bounds.update(face.third_corner.x, face.third_corner.y);
                bounds.update(face.fourth_corner.x, face.fourth_corner.y);
            }
            EntityType::Solid(solid) => {
                bounds.update(solid.first_corner.x, solid.first_corner.y);
                bounds.update(solid.second_corner.x, solid.second_corner.y);
                bounds.update(solid.third_corner.x, solid.third_corner.y);
                bounds.update(solid.fourth_corner.x, solid.fourth_corner.y);
            }
            EntityType::Leader(leader) => {
                for vertex in &leader.vertices {
                    bounds.update(vertex.x, vertex.y);
                }
            }
            EntityType::Helix(helix) => {
                bounds.update(helix.axis_base_point.x, helix.axis_base_point.y);
                bounds.update(helix.start_point.x, helix.start_point.y);
                // Add some padding for the helix radius
                bounds.update(helix.axis_base_point.x + helix.radius, helix.axis_base_point.y + helix.radius);
                bounds.update(helix.axis_base_point.x - helix.radius, helix.axis_base_point.y - helix.radius);
            }
            EntityType::Trace(trace) => {
                bounds.update(trace.first_corner.x, trace.first_corner.y);
                bounds.update(trace.second_corner.x, trace.second_corner.y);
                bounds.update(trace.third_corner.x, trace.third_corner.y);
                bounds.update(trace.fourth_corner.x, trace.fourth_corner.y);
            }
            EntityType::Shape(shape) => {
                bounds.update(shape.location.x, shape.location.y);
                // Add some padding based on shape size
                bounds.update(shape.location.x + shape.size, shape.location.y + shape.size);
                bounds.update(shape.location.x - shape.size, shape.location.y - shape.size);
            }
            _ => {
                continue;
            }
        }
    }

    bounds
}

/**
a struct containing a bunch of options around the svg.
Fill each of these or use None for default when using dxf_to_svg.

* `use_bounds` - if true, will add a viewBox to the svg at the size of the bounding box
* `padding` - the amount of padding to add to the viewBox
 */
pub struct SvgOptions {
    /// If true, will add a viewBox to the svg at the size of the bounding box
    pub use_bounds: bool,
    /// The amount of padding to add to the viewBox as a percentage (1.0 = 100%)
    pub padding: f64,
    /// The background color of the SVG. Set to "none" for transparent background.
    pub background_color: String,
    /// The default stroke width for entities
    pub stroke_width: f64,
    /// The default color for entities without a specific color
    pub default_color: String,
}

impl Default for SvgOptions {
    fn default() -> Self {
        Self {
            use_bounds: true,
            padding: 0.1, // 10% padding
            background_color: "white".to_string(),
            stroke_width: 1.0,
            default_color: "black".to_string(),
        }
    }
}

/**
Takes in a vector of entities and displays them as an SVG string.
If an entity is not supported, it will be printed to the console and skipped.

* `entities` - the list of entities you wish to turn into a string.
* Returns a string SVG representation of the entities.
*/
pub fn dxf_to_svg(entities: Vec<&Entity>, options: Option<SvgOptions>) -> String {
    let options = options.unwrap_or_default();
    let bounds = calculate_bounds(&entities).with_padding(options.padding);
    
    // Calculate scale and translation to normalize coordinates
    let width = bounds.max_x - bounds.min_x;
    let height = bounds.max_y - bounds.min_y;
    
    // Calculate the aspect ratio to maintain proportions
    let aspect_ratio = width / height;
    
    let mut svg = String::new();
    
    if options.use_bounds {
        // Add a viewBox that ensures the content is visible and properly scaled
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" 
            viewBox="{} {} {} {}" width="100%" height="100%" 
            preserveAspectRatio="xMidYMid meet">"#,
            0, // Start at 0 for normalized coordinates
            0,
            1000.0, // Use fixed width for consistent scaling
            1000.0 / aspect_ratio // Height adjusted by aspect ratio
        ));
        
        // Add a transform group to flip the Y axis and scale to normalized coordinates
        svg.push_str(&format!(
            r#"<g transform="scale({}, {}) translate({}, {})">"#,
            1000.0 / width, // Scale X to normalize to 1000 units width
            -1000.0 / width, // Scale Y (negative for flip) using same scale as X
            -bounds.min_x, // Translate X to start at 0
            -bounds.max_y  // Translate Y (after flip) to start at 0
        ));
    } else {
        svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" xmlns:xlink="http://www.w3.org/1999/xlink">"#);
    }

    // Add a white background rectangle (in normalized coordinates)
    if options.background_color != "none" {
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#,
            bounds.min_x,
            -bounds.max_y,
            width,
            height,
            options.background_color
        ));
    }

    for entity in entities {
        let color = if entity.common.color_name.trim().is_empty() {
            &options.default_color
        } else {
            entity.common.color_name.as_str()
        };

        let stroke_attr = format!("stroke=\"{}\" stroke-width=\"{}\"", color, options.stroke_width);

        match &entity.specific {
            EntityType::Line(line) => {
                svg.push_str(&format!(
                    r#"<line x1="{:.3}" y1="{:.3}" x2="{:.3}" y2="{:.3}" {} fill="none" />"#,
                    line.p1.x, line.p1.y, line.p2.x, line.p2.y, stroke_attr
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
                    r#"<polyline points="{}" {} />"#,
                    lwpolyline.vertices.iter()
                        .map(|p| format!("{:.3},{:.3}", p.x, p.y))
                        .collect::<Vec<_>>()
                        .join(" "),
                    stroke_attr
                ));
            }

            EntityType::Polyline(polyline) => {
                let vertices: Vec<_> = polyline.vertices().collect();
                if vertices.is_empty() {
                    continue;
                }
                svg.push_str(&format!(
                    r#"<polyline points="{}" {} />"#,
                    vertices.iter()
                        .map(|p| format!("{:.3},{:.3}", p.location.x, p.location.y))
                        .collect::<Vec<_>>()
                        .join(" "),
                    stroke_attr
                ));
            }

            EntityType::Circle(circle) => {
                svg.push_str(&format!(
                    r#"<circle cx="{:.3}" cy="{:.3}" r="{:.3}" {} />"#,
                    circle.center.x, circle.center.y, circle.radius, stroke_attr
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
                    r#"<path d="M {:.3},{:.3} A {:.3},{:.3} 0 {} {} {:.3},{:.3}" {} />"#,
                    start_x, start_y,
                    arc.radius, arc.radius,
                    large_arc, sweep,
                    end_x, end_y,
                    stroke_attr
                ));
            }

            EntityType::Ellipse(ellipse) => {
                let major_axis_length = (
                    ellipse.major_axis.x.powi(2) + 
                    ellipse.major_axis.y.powi(2)
                ).sqrt();
                
                let rotation = ellipse.major_axis.y.atan2(ellipse.major_axis.x).to_degrees();
                
                svg.push_str(&format!(
                    r#"<ellipse cx="{:.3}" cy="{:.3}" rx="{:.3}" ry="{:.3}" transform="rotate({:.3} {} {})" {} />"#,
                    ellipse.center.x, ellipse.center.y,
                    major_axis_length,
                    major_axis_length * ellipse.minor_axis_ratio,
                    rotation,
                    ellipse.center.x, ellipse.center.y,  // Rotate around the center point
                    stroke_attr
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
                    r#"<path d="{}" {} />"#,
                    path.trim(),
                    stroke_attr
                ));
            }

            EntityType::Text(text) => {
                // Escape special characters in text
                let escaped_text = escape_xml_text(&text.value);
                svg.push_str(&format!(
                    r#"<text x="{:.3}" y="{:.3}" {}>{}</text>"#,
                    text.location.x,
                    text.location.y,
                    stroke_attr,
                    escaped_text
                ));
            }

            EntityType::ModelPoint(point) => {
                svg.push_str(&format!(
                    r#"<circle cx="{:.3}" cy="{:.3}" r="1" {} />"#,
                    point.location.x, point.location.y, stroke_attr
                ));
            }
            EntityType::Face3D(face) => {
                svg.push_str(&format!(
                    r#"<polygon points="{:.3},{:.3} {:.3},{:.3} {:.3},{:.3} {:.3},{:.3}" {} />"#,
                    face.first_corner.x, face.first_corner.y,
                    face.second_corner.x, face.second_corner.y,
                    face.third_corner.x, face.third_corner.y,
                    face.fourth_corner.x, face.fourth_corner.y,
                    stroke_attr
                ));
            }
            EntityType::Solid(solid) => {
                svg.push_str(&format!(
                    r#"<polygon points="{:.3},{:.3} {:.3},{:.3} {:.3},{:.3} {:.3},{:.3}" {} />"#,
                    solid.first_corner.x, solid.first_corner.y,
                    solid.second_corner.x, solid.second_corner.y,
                    solid.third_corner.x, solid.third_corner.y,
                    solid.fourth_corner.x, solid.fourth_corner.y,
                    stroke_attr
                ));
            }
            EntityType::Leader(leader) => {
                if leader.vertices.is_empty() {
                    continue;
                }
                // Draw the leader line
                svg.push_str(&format!(
                    r#"<polyline points="{}" {} marker-end="url(#arrowhead)" />"#,
                    leader.vertices.iter()
                        .map(|p| format!("{:.3},{:.3}", p.x, p.y))
                        .collect::<Vec<_>>()
                        .join(" "),
                    stroke_attr
                ));
                // Add arrowhead marker if not already added
                if !svg.contains("def id=\"arrowhead\"") {
                    svg.push_str(
                        r#"<defs>
                            <marker id="arrowhead" markerWidth="10" markerHeight="7" 
                            refX="9" refY="3.5" orient="auto">
                                <polygon points="0 0, 10 3.5, 0 7" fill="black"/>
                            </marker>
                        </defs>"#
                    );
                }
            }
            EntityType::Helix(helix) => {
                // Approximate helix as a spiral path in 2D
                let mut path = format!("M {:.3},{:.3} ", helix.start_point.x, helix.start_point.y);
                let turns = helix.number_of_turns as i32;
                let points_per_turn = 16;
                let total_points = turns * points_per_turn;
                
                for i in 1..=total_points {
                    let angle = (i as f64) * 2.0 * PI / (points_per_turn as f64);
                    let radius = helix.radius * (i as f64) / (total_points as f64);
                    let x = helix.axis_base_point.x + radius * angle.cos();
                    let y = helix.axis_base_point.y + radius * angle.sin();
                    path.push_str(&format!("L {:.3},{:.3} ", x, y));
                }
                
                svg.push_str(&format!(
                    r#"<path d="{}" {} />"#,
                    path.trim(),
                    stroke_attr
                ));
            }
            EntityType::Trace(trace) => {
                svg.push_str(&format!(
                    r#"<polygon points="{:.3},{:.3} {:.3},{:.3} {:.3},{:.3} {:.3},{:.3}" {} />"#,
                    trace.first_corner.x, trace.first_corner.y,
                    trace.second_corner.x, trace.second_corner.y,
                    trace.third_corner.x, trace.third_corner.y,
                    trace.fourth_corner.x, trace.fourth_corner.y,
                    stroke_attr
                ));
            }
            EntityType::Shape(shape) => {
                // Render shape as a rectangle with the given size
                let half_size = shape.size / 2.0;
                svg.push_str(&format!(
                    r#"<rect x="{:.3}" y="{:.3}" width="{:.3}" height="{:.3}" 
                    transform="rotate({:.3} {} {})" {} />"#,
                    shape.location.x - half_size,
                    shape.location.y - half_size,
                    shape.size,
                    shape.size,
                    shape.rotation_angle,
                    shape.location.x,
                    shape.location.y,
                    stroke_attr
                ));
            }
            EntityType::RotatedDimension(dimension) => {
                let start_point = &dimension.definition_point_2; // Start of dimension line
                let end_point = &dimension.definition_point_3;   // End of dimension line
                let text_position = &dimension.insertion_point;    // Midpoint for text
                let measurement = &dimension.dimension_base.text; // Measurement text
                // Add the dimension line
                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" {stroke_attr} />"#,
                    start_point.x,
                    start_point.y,
                    end_point.x,
                    end_point.y,
                    stroke_attr = stroke_attr
                ));
            
                // Add the dimension text
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" {stroke_attr} font-size="12" text-anchor="middle">{text}</text>"#,
                    text_position.x,
                    text_position.y,
                    stroke_attr = stroke_attr,
                    text = measurement
                ));
            }
            _ => {
                println!("Unsupported entity type: {:?}", entity.common.layer);
                continue;
            }
        }
    }

    if options.use_bounds {
        svg.push_str("</g>");
    }
    svg.push_str("</svg>");
    svg
}

pub fn dxf_file_to_svg(file_path: &str, options: Option<SvgOptions>) -> String {
    let entities = dxf::Drawing::load_file(file_path).unwrap();
    dxf_to_svg(entities.entities().collect(), options)
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
    use std::fs;

    use super::*;
    use dxf::entities::Line;
    use dxf::Point;

    #[test]
    fn test_basic_entities() {
        // Test empty vector
        let empty_svg = dxf_to_svg(vec![], Some(SvgOptions {
            use_bounds: false,
            padding: 1.0,
            background_color: "white".to_string(),
            stroke_width: 1.0,
            default_color: "black".to_string(),
        }));
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
        let result = dxf_to_svg(entities, Some(SvgOptions::default()));
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

    #[test]
    fn test_file_to_svg() {
        let svg = dxf_file_to_svg("tests/test.dxf", Some(SvgOptions::default()));
        fs::write("tests/test.svg", svg).unwrap();
    }
}