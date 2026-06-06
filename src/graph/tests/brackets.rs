use crate::{DanceStage, FootPlacement, StepGraph};
use danceparser::{NoteKind, Row};

#[test]
fn test_graph_brackets_instead_of_jumps() {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);
    let dt = 0.1;
    graph.append(
        1.0 * dt,
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
        2.0 * dt,
        &Row {
            columns: vec![
                NoteKind::Empty,
                NoteKind::Empty,
                NoteKind::Tap,
                NoteKind::Tap,
            ],
        },
    );
    graph.append(
        3.0 * dt,
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
        4.0 * dt,
        &Row {
            columns: vec![
                NoteKind::Empty,
                NoteKind::Tap,
                NoteKind::Empty,
                NoteKind::Tap,
            ],
        },
    );
    assert_eq!(
        graph.compute_path(),
        vec![
            FootPlacement::parse("L---").unwrap(),
            FootPlacement::parse("L-rR").unwrap(),
            FootPlacement::parse("L-rR").unwrap(),
            FootPlacement::parse("LR-r").unwrap(),
        ]
    );
}

#[test]
fn test_graph_jumps_instead_of_brackets() {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);
    let dt = 0.5;
    graph.append(
        1.0 * dt,
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
        2.0 * dt,
        &Row {
            columns: vec![
                NoteKind::Empty,
                NoteKind::Empty,
                NoteKind::Tap,
                NoteKind::Tap,
            ],
        },
    );
    graph.append(
        3.0 * dt,
        &Row {
            columns: vec![
                NoteKind::Tap,
                NoteKind::Empty,
                NoteKind::Empty,
                NoteKind::Empty,
            ],
        },
    );
    assert_eq!(
        graph.compute_path(),
        vec![
            FootPlacement::parse("L---").unwrap(),
            FootPlacement::parse("--LR").unwrap(),
            FootPlacement::parse("L--R").unwrap(),
        ]
    );
}
