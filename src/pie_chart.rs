
use svg::node::element::{Group, Path, path::Data};
use std::f64::consts::PI;


// https://davidmathlogic.com/colorblind/
const FALLBACK_COLORS: [&str; 4] = [
    "#D81B60",
    "#1E88E5",
    "#FFC107",
    "#004D40",
];


#[derive(Debug, Clone, Default)]
pub struct PieSegment {
    /// The ratio this segment depicts, between 0.0 and 1.0.
    pub ratio: f64,
    /// The color to use for this segment, if empty fallback colors are used.
    pub color: String,
}

impl From<f64> for PieSegment {
    fn from(ratio: f64) -> Self {
        PieSegment{
            ratio,
            color: String::new(),
        }
    }
}


/// Style for positioning of the start segment.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StartStyle {
    /// The edge of the first segment is aligned with the start offset.
    Edge,
    /// The center of the first segment is aligned with the start offset.
    Centered,
}

#[derive(Debug, Clone)]
pub struct PieChart {
    segments: Vec<PieSegment>,
    radius: f64,
    start_offset: f64,
    start_style: StartStyle,
}
impl Default for PieChart {
    fn default() -> Self {
        Self {
            segments: vec![],
            radius: 1.0,
            start_offset: 0.0,
            start_style: StartStyle::Edge,
        }
    }
}

impl PieChart {
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the radius.
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    /// Normally, start is from 3 o clock clockwise.
    pub fn set_start(&mut self, offset: f64, style: StartStyle) {
        self.start_offset = offset;
        self.start_style = style;
    }

    /// Set the segments in this chart.
    pub fn set_segments<T: Into<PieSegment> + Clone>(&mut self, segments: &[T])  {
        self.segments = segments.iter().map(|z| Into::<PieSegment>::into(z.clone())).collect();
    }

    /// Mutable retrieval of a segment.
    pub fn segment_mut(&mut self, index: usize) -> Option<&mut PieSegment> {
        self.segments.get_mut(index)
    }

    /// Render the piechart to svg.
    pub fn svg(&self) -> Group {
        let mut group = Group::new();

        let mut current_pos: f64 = self.start_offset;

        // If we have a centered style, subtract by half of the first ratio.
        if self.start_style == StartStyle::Centered {
            if let Some(s) = self.segments.first() {
                current_pos -= (s.ratio / 2.0) * 2.0 * PI;
            }
        }
        for (si, s) in self.segments.iter().enumerate() {
            let angle = s.ratio * 2.0 * PI;
            let arc_sx = (current_pos).cos() * self.radius;
            let arc_sy = (current_pos).sin() * self.radius;
            let arc_ex = (angle + current_pos).cos() * self.radius;
            let arc_ey = (angle + current_pos).sin() * self.radius;
            let data = Data::new()
                .move_to((0.0, 0.0)) // all circles start in 0,0
                .line_to((arc_sx, arc_sy))
                .elliptical_arc_by((
                    self.radius, self.radius,
                    0.0,  // x axis rotation of the ellipse
                    0, 1, // large flag arc, sweep flag
                    arc_ex - arc_sx, arc_ey - arc_sy))
                .line_to((arc_ex, arc_ey))
                .close();
            
            current_pos += angle;
            let color = if s.color.is_empty() {
                FALLBACK_COLORS[si % FALLBACK_COLORS.len()].to_owned()
            } else {
                s.color.clone()
            };
            let path = Path::new()
                .set("fill", color)
                // .set("stroke", "none")
                // .set("stroke-width", 3)
                .set("d", data);
            group = group.add(path);
        }
        group
    }

}

    