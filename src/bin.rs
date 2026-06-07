use danceparser::{
    SMChart,
    view::{NoteView, NoteViewer},
};
use dancepredictor::{DanceStage, StepGraph};
use petgraph::dot::Dot;
use std::io::{Cursor, Write};

fn main() {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);

    let chart = SMChart::from_sm(Cursor::new(include_str!("../basic.sm")))
        .expect("Failed to parse StepMania chart");
    let notes_data = chart.notes.first().unwrap();

    for NoteView { time, row, .. } in NoteViewer::new(&chart, notes_data) {
        graph.append(time, row);
    }

    write!(
        std::fs::File::create("out.txt").unwrap(),
        "{}",
        Dot::with_config(&graph.graph, &[])
    )
    .unwrap();

    let steps = graph.compute_steps();
    for step in steps {
        println!("{}", step.columns);
    }
}
