use std::rc::Rc;
use svg::node::element::{Group, Polyline};
use svg::node::Node;

#[derive(Debug, Copy, Clone, Default)]
struct Range {
    min: f64,
    max: f64,
}

#[derive(Debug, Copy, Clone, Default)]
enum AxisOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Axis {
    canvas_length: f64,
    range: Range,
    orientation: AxisOrientation,
}

impl Axis {
    fn horizontal(canvas_length: f64) -> Self {
        Self {
            canvas_length,
            range: Range {
                min: 0.0,
                max: canvas_length,
            },
            orientation: AxisOrientation::Horizontal,
            ..Default::default()
        }
    }

    fn vertical(canvas_length: f64) -> Self {
        Self {
            canvas_length,
            range: Range {
                min: 0.0,
                max: canvas_length,
            },
            orientation: AxisOrientation::Vertical,
            ..Default::default()
        }
    }

    fn project(&self, v: f64) -> f64 {
        todo!("what semantics do we want here if range spans zero?");
        let shifted = v - self.range.min;
        let ratio = shifted / (self.range.max - self.range.min);
        let canvas_pos = ratio * self.canvas_length;
        canvas_pos
    }

    fn svg(&self) -> Group {
        let mut group = Group::new();

        group
    }

    pub fn set_range(&mut self, min: f64, max: f64) {
        self.range.min = min;
        self.range.max = max;
    }
}

impl From<Axis> for Box<(dyn Node + 'static)> {
    fn from(val: Axis) -> Self {
        Box::new(val.svg())
    }
}

use std::cell::RefCell;
#[derive(Debug, Clone, Default)]
pub struct AxisHorizontal(Rc<RefCell<Axis>>);
impl AxisHorizontal {
    pub fn new(canvas_length: f64) -> Self {
        Self(Rc::new(Axis::horizontal(canvas_length).into()))
    }
    pub fn combine(&self, vertical: &AxisVertical) -> Frame {
        Frame {
            vertical: vertical.clone(),
            horizontal: self.clone(),
        }
    }
    fn project(&self, v: f64) -> f64 {
        let z = self.0.borrow_mut();
        z.project(v)
    }

    pub fn set_range(&self, min: f64, max: f64) {
        let mut z = self.0.borrow_mut();
        z.set_range(min, max)
    }
}

#[derive(Debug, Clone, Default)]
pub struct AxisVertical(Rc<RefCell<Axis>>);
impl AxisVertical {
    pub fn new(canvas_length: f64) -> Self {
        Self(Rc::new(Axis::vertical(canvas_length).into()))
    }
    pub fn combine(&self, horizontal: &AxisHorizontal) -> Frame {
        Frame {
            horizontal: horizontal.clone(),
            vertical: self.clone(),
        }
    }
    fn project(&self, v: f64) -> f64 {
        let z = self.0.borrow_mut();
        z.project(v)
    }

    pub fn set_range(&self, min: f64, max: f64) {
        let mut z = self.0.borrow_mut();
        z.set_range(min, max)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Frame {
    pub horizontal: AxisHorizontal,
    pub vertical: AxisVertical,
}

#[derive(Debug, Clone, Default)]
pub struct DrawElement {
    frame: Frame,
    data: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, Default)]
pub struct DrawElementHandle(Rc<DrawElement>);

impl DrawElementHandle {
    pub fn new(frame: &Frame, data: &[(f64, f64)]) -> DrawElementHandle {
        Self(
            DrawElement {
                frame: frame.clone(),
                data: data.to_vec(),
            }
            .into(),
        )
    }
    pub fn svg(&self) -> Group {
        let mut group = Group::new();
        let mut points = String::new();
        for (x, y) in self.data.iter() {
            let px = self.frame.horizontal.project(*x);
            let py = self.frame.vertical.project(*y);
            points += &format!("{},{} ", px, py);
        }
        let path = Polyline::new().set("points", points);
        group.append(path);
        group
    }
}

impl std::ops::Deref for DrawElementHandle {
    type Target = DrawElement;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<&DrawElementHandle> for Box<(dyn Node + 'static)> {
    fn from(val: &DrawElementHandle) -> Self {
        Box::new(val.svg())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Plot {
    frames: Vec<Frame>,
    elements: Vec<DrawElementHandle>,
}

impl Plot {
    pub fn new(frame: &Frame) -> Self {
        Self {
            frames: vec![frame.clone()],
            elements: vec![],
        }
    }

    pub fn line_xy(&mut self, data: &[(f64, f64)]) -> DrawElementHandle {
        let f = self.frames.first().unwrap();
        let el = DrawElementHandle::new(f, data);
        self.elements.push(el.clone());
        el
    }

    pub fn svg(&self) -> Group {
        let mut group = Group::new();

        for el in self.elements.iter() {
            group.append(el)
        }
        group
    }
}

impl From<Plot> for Box<(dyn Node + 'static)> {
    fn from(val: Plot) -> Self {
        Box::new(val.svg())
    }
}
