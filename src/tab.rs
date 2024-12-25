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
    Left,
    #[default]
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
        // let h_straight = self.height - r * 2.0;
        // let w_straight = self.width - r * 2.0;
        // let tab_h_straight = self.tab_height - r * 2.0;
        // let tab_w_straight = self.tab_width - r * 2.0;

        // M 25 0 L 260 0 A 20 20 0 0 1 280 20 V 160 A 20 20 0 0 1 260 180 L 20 180 A 20 20 0 0 1 0 160 L 0 140 A 20 20 0 0 0 -20 120 L -50 120 A 20 20 0 0 1 -70 100 L -70 80 A 20 20 0 0 1 -50 60 L -20 60 A 20 20 0 0 0 0 40 L 0 20 A 20 20 0 0 1 20 0 Z
        let data = Data::new()
            .move_to((r, 0.0)) // start at radius offset
            .line_to((self.width -  r, 0.0)) // straight line at top to first curve.
            .elliptical_arc_to((
                r, r,
                0.0, // x axis rotation of the ellipse
                0, 1, // large flag arc, sweep flag
                self.width,
                r,
            ))
            .line_to((self.width, self.height - r)) // straight line to second curve
            .elliptical_arc_to((
                r, r,
                0.0, // x axis rotation of the ellipse
                0,
                1, // large flag arc, sweep flag
                self.width - r,
                self.height,
            ))
            .line_to((r, self.height)) // straight line to third curve
            .elliptical_arc_to((
                r, r,
                0.0, // x axis rotation of the ellipse
                0,
                1, // large flag arc, sweep flag
                0.0,
                self.height - r,
            ))
            .line_to((0.0, self.tab_position + 3.0 * r + self.tab_height)) //Line to tab start.
            .elliptical_arc_to(( // first arc towards tab.
                r, r,
                0.0, // x axis rotation of the ellipse
                0,
                0, // large flag arc, sweep flag
                -r,
                self.tab_position + 2.0 * r + self.tab_height,
            ))
            .line_to((-self.tab_width + r, self.tab_position + 2.0 * r + self.tab_height)) //Line to tab start.
            .elliptical_arc_to(( // first arc towards tab.
                r, r,
                0.0, // x axis rotation of the ellipse
                0,
                1, // large flag arc, sweep flag
                -self.tab_width,
                self.tab_position + 1.0 * r + self.tab_height,
            ))
            .line_to((-self.tab_width, self.tab_position + 2.0 * r)) //Line to tab start.
            .elliptical_arc_to(( // first arc towards tab.
                r, r,
                0.0, // x axis rotation of the ellipse
                0,
                1, // large flag arc, sweep flag
                -self.tab_width + r,
                self.tab_position + 1.0 * r,
            ))
            .line_to((-r, self.tab_position + 1.0 * r)) //Line to tab start.
            .elliptical_arc_to(( // first arc towards tab.
                r, r,
                0.0, // x axis rotation of the ellipse
                0,
                0, // large flag arc, sweep flag
                0.0,
                self.tab_position,
            ))
            .line_to((0.0, r)) //Line to tab start.
            .elliptical_arc_to(( // first arc towards tab.
                r, r,
                0.0, // x axis rotation of the ellipse
                0,
                1, // large flag arc, sweep flag
                r,
                0.0,
            ))
            // .close();
            ;

        let path = Path::new().set("d", data);
        path
    }
}
