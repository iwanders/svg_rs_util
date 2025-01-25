// https://www.w3.org/TR/2004/WD-SVG12-20041027/flow.html
// Supported by Inkscape
use std::collections::hash_map::DefaultHasher;
use svg::node::element::Element;
use svg::node::{Attributes, Children, Node, NodeDefaultHash, Value};

mod tag {

    macro_rules! implement {
        ($($const_name:ident: $tag_name:expr,)*) => ($(
            #[doc = $tag_name]
            pub const $const_name: &'static str = $tag_name;
        )*);
    }

    implement! {
        FlowRoot: "flowRoot",
        FlowRegion: "flowRegion",
        FlowPara: "flowPara",
    }
}

macro_rules! implement_nested(
    ($struct_name:ident::$field_name:ident) => (
        implement_nested!($struct_name::$field_name []);
    );
    ($struct_name:ident::$field_name:ident [$($indicator_name:ident),*]) => (
        impl $struct_name {
            /// Append a node.
            pub fn add<T>(mut self, node: T) -> Self
            where
                T: Into<Box<dyn Node>>,
            {
                Node::append(&mut self, node);
                self
            }

            /// Assign an attribute.
            #[inline]
            pub fn set<T, U>(mut self, name: T, value: U) -> Self
            where
                T: Into<String>,
                U: Into<Value>,
            {
                Node::assign(&mut self, name, value);
                self
            }
        }

        impl Node for $struct_name {
            #[inline]
            fn append<T>(&mut self, node: T)
            where
                T: Into<Box<dyn Node>>,
            {
                self.$field_name.append(node);
            }

            #[inline]
            fn assign<T, U>(&mut self, name: T, value: U)
            where
                T: Into<String>,
                U: Into<Value>,
            {
                self.$field_name.assign(name, value);
            }

            #[inline]
            fn get_name(&self) -> &str {
                self.$field_name.get_name()
            }

            #[inline]
            fn get_attributes(&self) -> Option<&Attributes> {
                self.$field_name.get_attributes().into()
            }

            #[inline]
            fn get_attributes_mut(&mut self) -> Option<&mut Attributes> {
                self.$field_name.get_attributes_mut().into()
            }

            #[inline]
            fn get_children(&self) -> Option<&Children> {
                self.$field_name.get_children().into()
            }

            #[inline]
            fn get_children_mut(&mut self) -> Option<&mut Children> {
                self.$field_name.get_children_mut().into()
            }

            $(
                #[inline]
                fn $indicator_name(&self) -> bool {
                    true
                }
            )*
        }

        impl std::ops::Deref for $struct_name {
            type Target = Element;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field_name
            }
        }

        impl std::ops::DerefMut for $struct_name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field_name
            }
        }

        impl std::fmt::Display for $struct_name {
            #[inline]
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                if self.is_bareable() {
                    write!(formatter, "{:#}", self.$field_name)
                } else {
                    self.$field_name.fmt(formatter)
                }
            }
        }

        impl From<$struct_name> for Element {
            #[inline]
            fn from(value: $struct_name) -> Self {
                value.$field_name
            }
        }
    );
);

macro_rules! implement {
    ($(#[$doc:meta] struct $struct_name:ident)*) => ($(
        #[$doc]
        #[derive(Clone, Debug)]
        pub struct $struct_name {
            inner: Element,
        }

        impl $struct_name {
            /// Create a node.
            #[inline]
            pub fn new() -> Self {
                $struct_name {
                    inner: Element::new(tag::$struct_name),
                }
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl NodeDefaultHash for $struct_name {
            #[inline]
            fn default_hash(&self, state: &mut DefaultHasher) {
                self.inner.default_hash(state);
            }
        }

        implement_nested! { $struct_name::inner }
    )*);
}

implement! {
    #[doc = "An [`a`](https://www.w3.org/TR/SVG/linking.html#AElement) element."]
    struct FlowRoot

    #[doc = "An [`animate`](https://www.w3.org/TR/SVG/animate.html#AnimateElement) element."]
    struct FlowRegion

    #[doc = "An [`animateColor`](https://www.w3.org/TR/SVG/animate.html#AnimateColorElement) element."]
    struct FlowPara
}
