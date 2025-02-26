use crate::extensions::{FlowPara, FlowRegion, FlowRoot};
use svg::node::element::{Rectangle, Text};
use svg::node::{Attributes, Value};
use svg::Node;
/// A wrapper for FlowRoot, FlowPara and FlowRegion.
#[derive(Debug, Clone)]
pub struct FlowText {
    root: FlowRoot,
    region: FlowRegion,
    paragraphs: Vec<FlowPara>,
    attributes: Attributes,
}

impl FlowText {
    pub fn rectangle<T>(width: f64, height: f64, text: T) -> Self
    where
        T: Into<String>,
    {
        let text: String = text.into();

        let root = FlowRoot::new();
        let area = Rectangle::new().set("width", width).set("height", height);

        let mut paragraphs = vec![];
        let z = text.split("\n");
        for p in z {
            let t = Text::new(p);
            let para = FlowPara::new().add(t);
            paragraphs.push(para);
        }

        let region = FlowRegion::new().add(area);

        FlowText {
            root,
            region,
            paragraphs,
            attributes: Default::default(),
        }
    }

    fn assemble(&self) -> Box<(dyn svg::Node + 'static)> {
        let mut root = self.root.clone();

        let mut region = self.region.clone();

        for (k, v) in self.attributes.iter() {
            root.assign(k.clone(), v.clone());
        }

        root.append(region);
        for p in self.paragraphs.iter() {
            root.append(p.clone());
        }
        root.into()
    }
    pub fn set<T, U>(self, name: T, value: U) -> Self
    where
        T: Into<String>,
        U: Into<Value>,
    {
        let mut z = self;
        z.attributes.insert(name.into(), value.into());
        z
    }
    fn assign<T, U>(&mut self, name: T, value: U)
    where
        Self: Sized,
        T: Into<String>,
        U: Into<Value>,
    {
        self.attributes.insert(name.into(), value.into());
    }
}

impl From<FlowText> for Box<(dyn svg::Node + 'static)> {
    fn from(val: FlowText) -> Self {
        val.assemble()
    }
}
