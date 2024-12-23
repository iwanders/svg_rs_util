use svg_util::pie_chart::PieChart;

fn main() {
    let mut z = PieChart::new();
    use svg::Document;
    use svg::node::element::Path;
    use svg::node::element::path::Data;

    z.set_segments(&[0.2, 0.3, 0.4, 0.1]);

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
        .set("viewBox", (0, 0, 70, 70))
        .add(path);

    let document = document.add(z.svg());

    svg::save("/tmp/test_chart.svg", &document).expect("failed to write svg");
}
