
use svg::node::element::{Group, Path, path::Data};
use std::f64::consts::PI;

const FULL_CIRCLE_DEGREES: f64 = 360.0; // Why is this all in degrees!?
const RAD_TO_DEG:f64 = 2.0 * PI / FULL_CIRCLE_DEGREES;

// https://davidmathlogic.com/colorblind/
const FALLBACK_COLORS: [&'static str; 4] = [
    "#D81B60",
    "#1E88E5",
    "#FFC107",
    "#004D40",
];


#[derive(Debug, Clone, Default)]
pub struct PieSegment {
    ratio: f64,
    color: String,
}

impl PieSegment {
    pub fn set_color(&mut self, color: &str) {
        self.color = color.to_owned();
    }
    pub fn set_ratio(&mut self, ratio: f64) {
        self.ratio = ratio;
    }
}

impl From<f64> for PieSegment {
    fn from(v: f64) -> Self {
        let mut s = PieSegment::default();
        s.set_ratio(v);
        s
    }
}


#[derive(Debug, Clone)]
pub struct PieChart {
    segments: Vec<PieSegment>,
    radius: f64,
}
impl Default for PieChart {
    fn default() -> Self {
        Self {
            segments: vec![],
            radius: 100.0,
        }
    }
}

impl PieChart {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_segments<T: Into<PieSegment> + Clone>(&mut self, segments: &[T])  {
        self.segments = segments.into_iter().map(|z| Into::<PieSegment>::into(z.clone())).collect();
    }

    pub fn segment_mut(&mut self, index: usize) -> Option<&mut PieSegment> {
        self.segments.get_mut(index)
    }
    pub fn svg(&self) -> Group {
        let mut group = Group::new();

        let mut current_pos: f64 = 0.0;
        for (si, s) in self.segments.iter().enumerate() {
            let angle = s.ratio * 2.0 * PI;
            let arc_sx = (current_pos).cos() * self.radius;
            let arc_sy = (current_pos).sin() * self.radius;
            let arc_ex = (angle + current_pos).cos() * self.radius;
            let arc_ey = (angle + current_pos).sin() * self.radius;
            let data = Data::new()
                .move_to((0.0, 0.0)) // all circles start in 0,0
                .line_to((arc_sx, arc_sy))
                .line_to(((angle*0.2 + current_pos).cos() * self.radius, (angle*0.2 + current_pos).sin() * self.radius))
                .line_to(((angle*0.4 + current_pos).cos() * self.radius, (angle*0.4 + current_pos).sin() * self.radius))
                .line_to(((angle*0.6 + current_pos).cos() * self.radius, (angle*0.6 + current_pos).sin() * self.radius))
                .line_to(((angle*0.8 + current_pos).cos() * self.radius, (angle*0.8 + current_pos).sin() * self.radius))
                .line_to(((angle*1.0 + current_pos).cos() * self.radius, (angle*1.0 + current_pos).sin() * self.radius))
                // .line_to((x_end, y_high))
                // .elliptical_arc_by((
                    // self.radius, self.radius,
                    // current_pos * RAD_TO_DEG,  // current x axis rotation
                    // 1, 1, // large flag arc, sweep flag
                    // arc_sx - arc_sx, arc_ey - arc_sy))
                .line_to((arc_ex, arc_ey))
                .line_to((0.0, 0.0))
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

    