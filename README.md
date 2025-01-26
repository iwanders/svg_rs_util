# svg_util

- I wanted to make a piechart with the [svg](https://crates.io/crates/svg) crate, this contains helpers to make a piechart.
- I needed to transform groups, so there's a trait to help with that such that I don't have to worry about writing a `transform` attribute each time.
- There's code to make a 'tab' shape with smooth corners. The tab 'label' protrusion can be located on all edges.
- The `FlowText` struct implements Inkscape's `flowRoot`, `flowPara` and `flowRegion` elements to facility wrapping text.

Repo name does not match crate name, crate name is `svg_util`, but I found that too generic for a repository name.
