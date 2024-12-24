use std::f64::consts::PI;
use svg::Document;
use svg_util::pie_chart::{PieChart, StartStyle};

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

    let document = Document::new()
        .set("viewBox", (-200, -200, 400, 400)) // from -200,-200, width and height of 400.
        .set("width", "2000px")
        .set("height", "2000px")
        .add(path);
    document
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
    let document = document.add(pie_chart.svg());

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

fn main() {
    make_piechart();
    make_piechart_align_largest();
}
