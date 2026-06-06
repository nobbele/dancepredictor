use crate::{DanceStage, FootPlacement, StepGraph};
use danceparser::{NoteKind, Row};

#[test]
fn test_graph_footswitch_instead_of_jack() {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);
    graph.append(
        0.1,
        &Row {
            columns: vec![
                NoteKind::Tap,
                NoteKind::Empty,
                NoteKind::Empty,
                NoteKind::Empty,
            ],
        },
    );
    graph.append(
        0.2,
        &Row {
            columns: vec![
                NoteKind::Empty,
                NoteKind::Tap,
                NoteKind::Empty,
                NoteKind::Empty,
            ],
        },
    );
    graph.append(
        0.3,
        &Row {
            columns: vec![
                NoteKind::Empty,
                NoteKind::Tap,
                NoteKind::Empty,
                NoteKind::Empty,
            ],
        },
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
        &Row {
            columns: vec![
                NoteKind::Tap,
                NoteKind::Empty,
                NoteKind::Empty,
                NoteKind::Empty,
            ],
        },
    );
    graph.append(
        2.0,
        &Row {
            columns: vec![
                NoteKind::Empty,
                NoteKind::Tap,
                NoteKind::Empty,
                NoteKind::Empty,
            ],
        },
    );
    graph.append(
        3.0,
        &Row {
            columns: vec![
                NoteKind::Empty,
                NoteKind::Tap,
                NoteKind::Empty,
                NoteKind::Empty,
            ],
        },
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
