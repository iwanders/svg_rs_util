
use svg::node::element::Group;

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
            radius: 1.0,
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
        for f in self.segments.iter() {
        }
        todo!()
    }
}

    