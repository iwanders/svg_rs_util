pub trait Transformed: svg::Node {
    /// Translate this element in x direction by the provided value.
    fn translate_x<T: Into<svg::node::Value>>(&mut self, x: T)
    where
        Self: Sized,
    {
        self.assign("transform", format!("translate({},0.0)", x.into()));
    }
    /// Translate this element in x direction by the provided value.
    fn translated_x<T: Into<svg::node::Value>>(self, x: T) -> Self
    where
        Self: Sized,
    {
        let mut z = self;
        z.translate_x(x);
        z
    }

    /// Translate this element in y direction by the provided value.
    fn translate_y<T: Into<svg::node::Value>>(&mut self, y: T)
    where
        Self: Sized,
    {
        self.assign("transform", format!("translate(0.0,{})", y.into()));
    }
    /// Translate this element in y direction by the provided value.
    fn translated_y<T: Into<svg::node::Value>>(self, y: T) -> Self
    where
        Self: Sized,
    {
        let mut z = self;
        z.translate_y(y);
        z
    }

    /// Translate this element in x and y direction by the provided value.
    fn translate_xy<T: Into<svg::node::Value>, U: Into<svg::node::Value>>(&mut self, x: T, y: U)
    where
        Self: Sized,
    {
        self.assign("transform", format!("translate({},{})", x.into(), y.into()));
    }
    /// Translate this element in x and y direction by the provided value.
    fn translated_xy<T: Into<svg::node::Value>, U: Into<svg::node::Value>>(self, x: T, y: U) -> Self
    where
        Self: Sized,
    {
        let mut z = self;
        z.translate_xy(x, y);
        z
    }

    /// A rotated transform, rotates by x degrees.
    fn rotated<T: Into<svg::node::Value>>(self, x: T) -> Self
    where
        Self: Sized,
    {
        let mut z = self;
        z.assign("transform", format!("rotate({})", x.into()));
        z
    }
}

// Blanket implementation for all nodes, since transform is applicable to anything.
impl<T: svg::node::Node> Transformed for T {}
