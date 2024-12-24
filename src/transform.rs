
pub trait Transformed : svg::Node{
    fn translated_x<T: Into<svg::node::Value>>(self, x: T) -> Self where Self: Sized {
        let mut z = self;
        z.assign("transform", format!("translate({},0.0)", x.into()));
        z
    }
    fn translated_y<T: Into<svg::node::Value>>(self, y: T) -> Self where Self: Sized {
        let mut z = self;
        z.assign("transform", format!("translate(0.0,{})", y.into()));
        z
    }
    fn translated_xy<T: Into<svg::node::Value>, U: Into<svg::node::Value>>(self, x: T, y: U) -> Self where Self: Sized {
        let mut z = self;
        z.assign("transform", format!("translate({},{})", x.into(), y.into()));
        z
    }
}
impl<T: svg::node::Node> Transformed for T {}
