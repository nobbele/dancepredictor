use dancepredictor::{DanceStage, StepGraph};
use rgc_chart::models::common::Key;

fn main() {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);
    graph.append(
        0.0,
        &vec![Key::normal(), Key::empty(), Key::empty(), Key::empty()],
    );
    graph.append(
        1.0,
        &vec![Key::empty(), Key::empty(), Key::empty(), Key::normal()],
    );
    graph.append(
        2.0,
        &vec![Key::empty(), Key::empty(), Key::normal(), Key::empty()],
    );
    graph.append(
        3.0,
        &vec![Key::normal(), Key::empty(), Key::empty(), Key::normal()],
    );
    for i in 4..100000 {
        graph.append(
            i as f32,
            &vec![
                if i % 4 == 0 {
                    Key::normal()
                } else {
                    Key::empty()
                },
                if i % 4 == 1 {
                    Key::normal()
                } else {
                    Key::empty()
                },
                if i % 4 == 2 {
                    Key::normal()
                } else {
                    Key::empty()
                },
                if i % 4 == 3 {
                    Key::normal()
                } else {
                    Key::empty()
                },
            ],
        );
    }
    // use std::io::Write;
    // write!(
    //     std::fs::File::create("out.txt").unwrap(),
    //     "{}",
    //     Dot::with_config(&graph.graph, &[])
    // )
    // .unwrap();
    println!("{}", graph.graph.node_count());
    println!("{}", graph.graph.edge_count());
    // println!("{}", Dot::with_config(&graph.graph, &[]));
    // let path = graph.compute_path();
    // for placement in path {
    //     println!("{}", placement);
    // }
}
