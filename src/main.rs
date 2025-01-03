use std::f64::consts::PI;
use svg::node::element::Group;
use svg::Document;
use svg_util::pie_chart::{PieChart, StartStyle};
use svg_util::tab::{Tab, TabEdge};
use svg_util::transform::Transformed;

fn piechart_canvas() -> Document {
    use svg::node::element::path::Data;
    use svg::node::element::Path;

    let data = Data::new()
        .move_to((-1000, 1000))
        .line_to((1000, 1000))
        .line_to((1000, -1000))
        .line_to((-1000, -1000))
        .close();

    let path = Path::new().set("fill", "black").set("d", data);

    Document::new()
        .set("viewBox", (-200, -200, 400, 400)) // from -200,-200, width and height of 400.
        .set("width", "2000px")
        .set("height", "2000px")
        .add(path)
}

fn make_piechart() {
    let mut pie_chart = PieChart::new();

    pie_chart.set_radius(100.0);

    // Center first slice at 45 degrees.
    pie_chart.set_start(-PI / 4.0, StartStyle::Center);

    // 12 o clock
    pie_chart.set_start(-PI / 2.0, StartStyle::Edge);

    pie_chart.set_segments(&[0.15, 0.3, 0.4, 0.15]);

    let document = piechart_canvas();
    let document = document.add(pie_chart);

    svg::save("/tmp/test_pie_chart.svg", &document).expect("failed to write svg");
}
fn make_piechart_align_largest() {
    let mut pie_chart = PieChart::new();

    pie_chart.set_radius(100.0);

    // Center the largest to its expected location if all parts were equal.
    pie_chart.set_start(PI / 4.0, StartStyle::CenterLargest);

    // Max segment is the fourth, it should align to top right, because we started with PI/4, which
    // offsets such that we end up with pieces centered starting at half past four.
    pie_chart.set_segments(&[0.21, 0.19, 0.19, 0.41]);

    let document = piechart_canvas();
    let document = document.add(pie_chart.svg());

    svg::save("/tmp/test_pie_chart_align_largest.svg", &document).expect("failed to write svg");
}

fn make_tab() {
    // let mut tab = Tab::new();

    let document = piechart_canvas();

    // Canvas is [-200,200] x [-200, 200]
    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(10.0)
            .tab(15.0, 25.0)
            .tab_position(15.0)
            .tab_edge(TabEdge::Left)
            .svg()
            .set("stroke", "red")
            .set("fill", "none"),
    );
    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(5.0)
            .tab(15.0, 25.0)
            .tab_position(15.0)
            .tab_edge(TabEdge::Left)
            .svg()
            .set("stroke", "green")
            .set("fill", "none"),
    );
    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(5.0)
            .tab(15.0, 25.0)
            .tab_position(15.0)
            .tab_edge(TabEdge::Left)
            .svg()
            .set("stroke", "white")
            .set("fill", "none")
            .translated_xy(80.0, 0.0),
    );
    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(2.0)
            .tab(5.0, 25.0)
            .tab_position(15.0)
            .tab_edge(TabEdge::Left)
            .svg()
            .set("stroke", "blue")
            .set("fill", "none")
            .translated_xy(80.0, 0.0),
    );

    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(2.0)
            .tab(5.0, 25.0)
            .tab_position(0.0)
            .tab_edge(TabEdge::Left)
            .svg()
            .set("stroke", "red")
            .set("fill", "none")
            .translated_xy(-80.0, 0.0),
    );
    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(2.0)
            .tab(5.0, 25.0)
            .tab_position(4.0)
            .tab_edge(TabEdge::Left)
            .svg()
            .set("stroke", "magenta")
            .set("fill", "none")
            .translated_xy(-80.0, 0.0),
    );

    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(5.0)
            .tab(25.0, 10.0)
            .tab_position(30.0)
            .tab_edge(TabEdge::Right)
            .svg()
            .set("stroke", "orange")
            .set("fill", "none")
            .translated_xy(-80.0, -90.0),
    );

    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(3.0)
            .tab(10.0, 10.0)
            .tab_position(0.0)
            .tab_edge(TabEdge::Bottom)
            .svg()
            .set("stroke", "orange")
            .set("fill", "none")
            .translated_xy(-80.0, -190.0),
    );

    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(5.0)
            .tab(10.0, 20.0)
            .tab_position(30.0)
            .tab_edge(TabEdge::Left)
            .svg()
            .set("stroke", "green")
            .set("fill", "none")
            .translated_xy(80.0, -90.0),
    );
    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(5.0)
            .tab(10.0, 20.0)
            .tab_position(60.0)
            .tab_edge(TabEdge::Left)
            .svg()
            .set("stroke", "purple")
            .set("fill", "none")
            .translated_xy(80.0, -90.0),
    );
    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(5.0)
            .tab(10.0, 20.0)
            .tab_position(0.0)
            .tab_edge(TabEdge::Left)
            .svg()
            .set("stroke", "yellow")
            .set("fill", "none")
            .translated_xy(80.0, -90.0),
    );

    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(2.0)
            .tab(10.0, 20.0)
            .tab_position(10.0)
            .tab_edge(TabEdge::Top)
            .svg()
            .set("stroke", "purple")
            .set("fill", "none")
            .translated_xy(00.0, -90.0),
    );
    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(5.0)
            .tab(10.0, 20.0)
            .tab_position(10.0)
            .tab_edge(TabEdge::Top)
            .svg()
            .set("stroke", "teal")
            .set("fill", "none")
            .translated_xy(00.0, -90.0),
    );

    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(5.0)
            .tab(10.0, 20.0)
            .tab_position(0.0)
            .tab_edge(TabEdge::Top)
            .svg()
            .set("stroke", "teal")
            .set("fill", "none")
            .translated_xy(0.0, 110.0),
    );
    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(5.0)
            .tab(10.0, 20.0)
            .tab_position(40.0)
            .tab_edge(TabEdge::Top)
            .svg()
            .set("stroke", "yellow")
            .set("fill", "none")
            .translated_xy(0.0, 110.0),
    );
    let document = document.add(
        Group::new()
            .add(
                Tab::new()
                    .sized(50.0, 80.0)
                    .radius(5.0)
                    .tab(10.0, 20.0)
                    .tab_position(40.0)
                    .tab_edge(TabEdge::Top),
            )
            .translated_xy(80.0, 110.0),
    );
    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(5.0)
            .tab(0.0, 0.0)
            .tab_position(40.0)
            .tab_edge(TabEdge::Top)
            .svg()
            .set("stroke", "yellow")
            .set("fill", "none")
            .translated_xy(80.0, 110.0),
    );

    let document = document.add(
        Tab::new()
            .sized(50.0, 80.0)
            .radius(5.0)
            .tab(10.0, 20.0)
            .tab_position(40.0)
            .tab_edge(TabEdge::None)
            .svg()
            .set("stroke", "yellow")
            .set("fill", "none")
            .translated_xy(-80.0, 110.0),
    );

    svg::save("/tmp/test_tab.svg", &document).expect("failed to write svg");
}

fn main() {
    make_piechart();
    make_piechart_align_largest();
    make_tab();
}
