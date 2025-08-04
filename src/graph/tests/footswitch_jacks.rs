use crate::{DanceStage, FootPlacement, StepGraph};
use rgc_chart::models::common::Key;

#[test]
fn test_graph_footswitch_instead_of_jack() {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);
    graph.append(
        0.1,
        &vec![Key::normal(), Key::empty(), Key::empty(), Key::empty()],
    );
    graph.append(
        0.2,
        &vec![Key::empty(), Key::normal(), Key::empty(), Key::empty()],
    );
    graph.append(
        0.3,
        &vec![Key::empty(), Key::normal(), Key::empty(), Key::empty()],
    );
    assert_eq!(
        graph.compute_path(),
        vec![
            FootPlacement::parse("L---").unwrap(),
            FootPlacement::parse("LR--").unwrap(),
            FootPlacement::parse("LR--").unwrap(),
        ]
    );
}

#[test]
fn test_graph_jack_instead_of_footswitch() {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);
    graph.append(
        1.0,
        &vec![Key::normal(), Key::empty(), Key::empty(), Key::empty()],
    );
    graph.append(
        2.0,
        &vec![Key::empty(), Key::normal(), Key::empty(), Key::empty()],
    );
    graph.append(
        3.0,
        &vec![Key::empty(), Key::normal(), Key::empty(), Key::empty()],
    );
    assert_eq!(
        graph.compute_path(),
        vec![
            FootPlacement::parse("L---").unwrap(),
            FootPlacement::parse("LR--").unwrap(),
            FootPlacement::parse("LR--").unwrap(),
        ]
    );
}
