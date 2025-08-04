use dancepredictor::{DanceStage, StepGraph};
use rgc_chart::models::hitobjects::HitObjectView;
use std::io::Write;
use petgraph::dot::Dot;

fn main() {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);

    let chart = rgc_chart::parse::from_sm(include_str!("../basic.sm"))
        .expect("Failed to parse StepMania chart");

    for HitObjectView {
        time, row, ..
    } in chart.hitobjects.iter_views()
    {
        graph.append(*time as f32 / 1000.0, row);
    }

    write!(
        std::fs::File::create("out.txt").unwrap(),
        "{}",
        Dot::with_config(&graph.graph, &[])
    )
    .unwrap();

    let path = graph.compute_path();
    for row in path {
        println!("{}", row);
    }
}
