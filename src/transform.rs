
pub trait Transformed : svg::Node{
    /// Translate this element in x direction by the provided value.
    fn translated_x<T: Into<svg::node::Value>>(self, x: T) -> Self where Self: Sized {
        let mut z = self;
        z.assign("transform", format!("translate({},0.0)", x.into()));
        z
    }
    /// Translate this element in y direction by the provided value.
    fn translated_y<T: Into<svg::node::Value>>(self, y: T) -> Self where Self: Sized {
        let mut z = self;
        z.assign("transform", format!("translate(0.0,{})", y.into()));
        z
    }
    /// Translate this element in x and y direction by the provided value.
    fn translated_xy<T: Into<svg::node::Value>, U: Into<svg::node::Value>>(self, x: T, y: U) -> Self where Self: Sized {
        let mut z = self;
        z.assign("transform", format!("translate({},{})", x.into(), y.into()));
        z
    }
}

// Blanket implementation for all nodes, since transform is applicable to anything.
impl<T: svg::node::Node> Transformed for T {}
