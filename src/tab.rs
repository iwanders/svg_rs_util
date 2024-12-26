use svg::node::element::Path;

use svg::node::element::path::Data;

/*
https://yqnn.github.io/svg-path-editor/#P=M_25_0_L_260_0_A_20_20_0_0_1_280_20_V_160_A_20_20_0_0_1_260_180_L_20_180_A_20_20_0_0_1_0_160_L_0_140_A_20_20_0_0_0_-20_120_L_-50_120_A_20_20_0_0_1_-70_100_L_-70_80_A_20_20_0_0_1_-50_60_L_-20_60_A_20_20_0_0_0_0_40_L_0_20_A_20_20_0_0_1_20_0_Z

                         width
                     <--------------->
                     O---------------+ ^  ^
                     |               | :  : tab position
               ^ +---+               | :  v
         tab   : |                   | :
         height: |                   | :
               : |                   | :
               : |                   | :
               : |                   | :
               : |                   | :
               v +---+               | :height
                     |               | :
                     |               | :
                     |               | :
                     |               | :
                     |               | :
                     +---------------+ v
                <--->
                 tab width
*/

#[derive(Clone, Copy, Default)]
pub enum TabEdge {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Copy, Default)]
pub struct Tab {
    radius: f64,
    width: f64,
    height: f64,
    tab_edge: TabEdge,
    tab_width: f64,
    tab_height: f64,
    tab_position: f64,
}

impl Tab {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn sized(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }
    pub fn tab(mut self, tab_width: f64, tab_height: f64) -> Self {
        self.tab_width = tab_width;
        self.tab_height = tab_height;
        self
    }
    pub fn tab_edge(mut self, tab_edge: TabEdge) -> Self {
        self.tab_edge = tab_edge;
        self
    }

    pub fn tab_position(mut self, tab_position: f64) -> Self {
        self.tab_position = tab_position;
        self
    }

    pub fn svg(&self) -> Path {
        let r = self.radius;

        /*


                     O-A------------B--+
                     P                 |
                     |                 |
                     O                 C
               ^ +M-N+                 |
         tab   : L                     |
         height: |                     |
               : |                     |
               : |                     |
               : K                     |
               : |                     |
               v +-J-I-+               |
                       |               |
                       H               |
                       |               |
                       G               D
                       |               |
                       +--F---------E--+

                 tab width
        */

        #[derive(Default)]
        struct TabData {
            a: (f64, f64),
            b: (f64, f64),
            c: (f64, f64),
            d: (f64, f64),
            e: (f64, f64),
            f: (f64, f64),
            g: (f64, f64),
            h: (f64, f64),
            i: (f64, f64),
            j: (f64, f64),
            k: (f64, f64),
            l: (f64, f64),
            m: (f64, f64),
            n: (f64, f64),
            o: (f64, f64),
            p: (f64, f64),
        }
        let d = match self.tab_edge {
            TabEdge::Left => TabData {
                a: (r, 0.0),
                b: (self.width - r, 0.0),
                c: (self.width, r),
                d: (self.width, self.height - r),
                e: (self.width - r, self.height),
                f: (r, self.height),
                g: (0.0, self.height - r),
                h: (0.0, self.tab_position + 1.0 * r + self.tab_height),
                i: (-r, self.tab_position + 0.0 * r + self.tab_height),
                j: (
                    -self.tab_width + r,
                    self.tab_position + 0.0 * r + self.tab_height,
                ),
                k: (
                    -self.tab_width,
                    self.tab_position - 1.0 * r + self.tab_height,
                ),
                l: (-self.tab_width, self.tab_position + 1.0 * r),
                m: (-self.tab_width + r, self.tab_position),
                n: (-r, self.tab_position),
                o: (0.0, self.tab_position - r),
                p: ((0.0, r)),
                ..Default::default()
            },
            TabEdge::Top => {
                // Previously, we had the tab on the left, and start with the top.
                // Now the tab is on the top, so we start with the right.
                // rotate all points?
                TabData {
                    a: (self.width, r),
                    b: (self.width, self.height - r),
                    c: (self.width - r, self.height),
                    d: (r, self.height),
                    e: (0.0, self.height - r), // bottom left corner done.
                    f: (0.0, r),               // to start of top left corner.
                    g: (r, 0.0),
                    h: (self.tab_position - 1.0 * r, 0.0),
                    i: (self.tab_position, -r),
                    j: (self.tab_position, -self.tab_height + r),
                    k: (self.tab_position + r, -self.tab_height), // arc to
                    l: (self.tab_position + self.tab_width - r, -self.tab_height), // tab straight.
                    m: (self.tab_position + self.tab_width, -self.tab_height + r),
                    n: (self.tab_position + self.tab_width, -r),
                    o: (self.tab_position + self.tab_width + r, 0.0),
                    p: (self.width - r, 0.0),
                    ..Default::default()
                }
            }
            _ => todo!(),
        };

        let mut data = Data::new()
            .move_to(d.a) // start at radius offset
            .line_to(d.b) // straight line at top to first curve.
            .elliptical_arc_to((
                r, r,
                0.0, // x axis rotation of the ellipse
                0, 1, // large flag arc, sweep flag
                d.c.0,
                d.c.1,
            ))
            .line_to(d.d) // straight line to second curve
            .elliptical_arc_to((
                r, r,
                0.0, // x axis rotation of the ellipse
                0,
                1, // large flag arc, sweep flag
                d.e.0,
                d.e.1,
            ))
            .line_to(d.f) // straight line to third curve
            ;

        if (self.tab_position + 1.0 * r + self.tab_height) < self.height {
            data = data
                .elliptical_arc_to((
                    r, r, 0.0, // x axis rotation of the ellipse
                    0, 1, // large flag arc, sweep flag
                    d.g.0, d.g.1,
                ))
                .line_to(d.h)
                .elliptical_arc_to((
                    // first arc towards tab.
                    r, r, 0.0, // x axis rotation of the ellipse
                    0, 0, // large flag arc, sweep flag
                    d.i.0, d.i.1,
                ));
        }

        data = data
            .line_to(d.j)
            .elliptical_arc_to((
                r, r, 0.0, // x axis rotation of the ellipse
                0, 1, // large flag arc, sweep flag
                d.k.0, d.k.1,
            ))
            .line_to(d.l)
            .elliptical_arc_to((
                r, r, 0.0, // x axis rotation of the ellipse
                0, 1, // large flag arc, sweep flag
                d.m.0, d.m.1,
            ))
            .line_to(d.n);

        // Prevent the uglyness if the tab position is at the lower edge (0.0).
        if self.tab_position > r {
            data = data
                .elliptical_arc_to((
                    r, r, 0.0, // x axis rotation of the ellipse
                    0, 0, // large flag arc, sweep flag
                    d.o.0, d.o.1,
                ))
                .line_to(d.p) //Line to tab start.
                .elliptical_arc_to((
                    r, r, 0.0, // x axis rotation of the ellipse
                    0, 1, // large flag arc, sweep flag
                    d.a.0, d.a.1,
                ));
        };
        data = data.close();

        let path = Path::new().set("d", data);
        path
    }
}
