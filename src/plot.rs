use crate::transform::*;
use std::rc::Rc;
use svg::node::element::{Group, Polyline};
use svg::node::{Attributes, Node, Value};

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

#[derive(Debug, Clone, Default)]
pub struct Axis {
    canvas_range: Range,
    plot_range: Range,
    orientation: AxisOrientation,
    attributes: Attributes,
}

impl Axis {
    fn horizontal(canvas_length: f64) -> Self {
        Self {
            canvas_range: Range {
                min: 0.0,
                max: canvas_length,
            },
            plot_range: Range {
                min: 0.0,
                max: canvas_length,
            },
            orientation: AxisOrientation::Horizontal,
            ..Default::default()
        }
    }

    fn vertical(canvas_length: f64) -> Self {
        Self {
            canvas_range: Range {
                min: 0.0,
                max: canvas_length,
            },
            plot_range: Range {
                min: 0.0,
                max: canvas_length,
            },
            orientation: AxisOrientation::Vertical,
            ..Default::default()
        }
    }

    fn project(&self, v: f64) -> f64 {
        let shifted = v - self.plot_range.min;
        let ratio = shifted / (self.plot_range.max - self.plot_range.min);
        let canvas_pos = ratio * (self.canvas_range.max - self.canvas_range.min);
        canvas_pos + self.canvas_range.min
    }

    fn svg(&self) -> Group {
        let mut group = Group::new();
        let mut points = String::new();
        let coords = match self.orientation {
            AxisOrientation::Horizontal => {
                [(self.canvas_range.min, 0.0), (self.canvas_range.max, 0.0)]
            }
            AxisOrientation::Vertical => {
                [(0.0, self.canvas_range.min), (0.0, self.canvas_range.max)]
            }
        };
        for (x, y) in coords {
            // let px = self.project(x);
            // let py = self.project(y);
            points += &format!("{},{} ", x, y);
        }
        let mut path = Polyline::new().set("points", points);
        let attr = path.get_attributes_mut().unwrap();
        for (k, v) in self.attributes.iter() {
            attr.insert(k.clone(), v.clone());
        }
        group.append(path);
        group
    }

    pub fn set_plot_range(&mut self, min: f64, max: f64) {
        self.plot_range.min = min;
        self.plot_range.max = max;
    }
    pub fn set_canvas_range(&mut self, min: f64, max: f64) {
        self.canvas_range.min = min;
        self.canvas_range.max = max;
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

    pub fn set_plot_range(&self, min: f64, max: f64) {
        let mut z = self.0.borrow_mut();
        z.set_plot_range(min, max)
    }
    pub fn set_canvas_range(&self, min: f64, max: f64) {
        let mut z = self.0.borrow_mut();
        z.set_canvas_range(min, max)
    }

    pub fn set<T, U>(self, name: T, value: U) -> Self
    where
        T: Into<String>,
        U: Into<Value>,
    {
        {
            let mut z = self.0.borrow_mut();
            z.attributes.insert(name.into(), value.into());
        }
        self
    }
    fn svg(&self) -> Group {
        let z = self.0.borrow();
        z.svg()
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

    pub fn set_plot_range(&self, min: f64, max: f64) {
        let mut z = self.0.borrow_mut();
        z.set_plot_range(min, max)
    }
    pub fn set_canvas_range(&self, min: f64, max: f64) {
        let mut z = self.0.borrow_mut();
        z.set_canvas_range(min, max)
    }
    pub fn set<T, U>(self, name: T, value: U) -> Self
    where
        T: Into<String>,
        U: Into<Value>,
    {
        {
            let mut z = self.0.borrow_mut();
            z.attributes.insert(name.into(), value.into());
        }
        self
    }
    fn svg(&self) -> Group {
        let z = self.0.borrow();
        z.svg()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Frame {
    pub horizontal: AxisHorizontal,
    pub vertical: AxisVertical,
}
impl Frame {
    fn svg(&self) -> Group {
        let mut group = Group::new();
        let hsvg = self.horizontal.svg();
        let vsvg = self.vertical.svg();
        // hsvg.translate_xy(0.0, -self.vertical.project(0.0));
        // vsvg.translate_xy(-self.horizontal.project(0.0), 0.0);
        group.append(hsvg);
        group.append(vsvg);
        group
    }
}

impl From<&Frame> for Box<(dyn Node + 'static)> {
    fn from(val: &Frame) -> Self {
        Box::new(val.svg())
    }
}

#[derive(Debug, Clone, Default)]
pub struct DrawElement {
    frame: Frame,
    data: Vec<(f64, f64)>,
    attributes: Attributes,
}

#[derive(Debug, Clone, Default)]
pub struct DrawElementHandle(Rc<RefCell<DrawElement>>);

impl DrawElementHandle {
    pub fn new(frame: &Frame, data: &[(f64, f64)]) -> DrawElementHandle {
        Self(Rc::new(RefCell::new(DrawElement {
            frame: frame.clone(),
            data: data.to_vec(),
            attributes: Default::default(),
        })))
    }
    pub fn svg(&self) -> Group {
        let z = self.0.borrow();
        let mut group = Group::new();
        let mut points = String::new();
        for (x, y) in z.data.iter() {
            let px = z.frame.horizontal.project(*x);
            let py = z.frame.vertical.project(*y);
            points += &format!("{},{} ", px, py);
        }
        let mut path = Polyline::new().set("points", points);
        let attr = path.get_attributes_mut().unwrap();
        for (k, v) in z.attributes.iter() {
            attr.insert(k.clone(), v.clone());
        }
        group.append(path);
        group
    }

    pub fn set<T, U>(self, name: T, value: U) -> Self
    where
        T: Into<String>,
        U: Into<Value>,
    {
        {
            let mut z = self.0.borrow_mut();
            z.attributes.insert(name.into(), value.into());
        }
        self
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
        let _ = frame;
        // Needs coordinate frame flip.
        Self {
            frames: vec![frame.clone()],
            elements: vec![],
        };
        todo!("not ready for production yet")
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
        for f in self.frames.iter() {
            group.append(f)
        }
        group
    }
}

impl From<Plot> for Box<(dyn Node + 'static)> {
    fn from(val: Plot) -> Self {
        Box::new(val.svg())
    }
}
