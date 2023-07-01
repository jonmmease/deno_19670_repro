mod converter;
mod module_loader;

#[macro_use]
extern crate lazy_static;

use crate::converter::VlConverter;

#[tokio::main]
async fn main() {
    let vl_spec = serde_json::json!(
        {
          "$schema": "https://vega.github.io/schema/vega-lite/v5.8.0.json",
          "width": 300,
          "height": 175,
          "config": {"view": {"continuousWidth": 300, "continuousHeight": 300}},
          "data": {
            "url": "https://cdn.jsdelivr.net/npm/vega-datasets@v1.29.0/data/income.json"
          },
          "mark": {"type": "geoshape"},
          "encoding": {
            "color": {"field": "pct", "type": "quantitative"},
            "shape": {"field": "geo", "type": "geojson"},
            "tooltip": [
              {"field": "name", "type": "nominal"},
              {"field": "pct", "type": "quantitative"}
            ]
          },
          "projection": {"type": "albersUsa"},
          "transform": [
            {
              "lookup": "id",
              "as": "geo",
              "from": {
                "data": {
                  "url": "https://cdn.jsdelivr.net/npm/vega-datasets@v1.29.0/data/us-10m.json",
                  "format": {"feature": "states", "type": "topojson"}
                },
                "key": "id"
              }
            }
          ]
        }
    );

    // Create Vega-Lite Converter and perform conversion
    let mut converter = VlConverter::new();
    let mut svg_data: String = String::new();
    for _ in 0..100 {
        svg_data = converter.vegalite_to_svg(vl_spec.clone()).await.unwrap();
    }
    println!("Success!:  {}", &svg_data[..30]);
}
