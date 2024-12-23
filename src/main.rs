use svg_util::pie_chart::{PieChart, StartStyle};

fn main() {
    let mut z = PieChart::new();
    // z.set_start(-1.57 / 2.0, StartStyle::Centered);
    use svg::Document;
    use svg::node::element::Path;
    use svg::node::element::path::Data;

    z.set_segments(&[0.15, 0.3, 0.4, 0.1]);

    let data = Data::new()
        .move_to((-1000, 1000))
        .line_to((1000, 1000))
        .line_to((1000, -1000))
        .line_to((-1000, -1000))
        .close();

    let path = Path::new()
        .set("fill", "black")
        .set("d", data);

    let document = Document::new()
        .set("viewBox", (-200, -200, 400, 400)) // from -200,-200, width and height of 400.
        .set("width", "2000px")
        .set("height", "2000px")
        .add(path);

    let document = document.add(z.svg());

    svg::save("/tmp/test_chart.svg", &document).expect("failed to write svg");
}
