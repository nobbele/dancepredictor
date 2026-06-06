use crate::{DanceStage, FootPart, FootPlacement, StepGraph};
use danceparser::{NoteKind, Row};

#[test]
fn test_graph_walk() {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);
    graph.append(
        0.0,
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
        1.0,
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
        2.0,
        &Row {
            columns: vec![
                NoteKind::Empty,
                NoteKind::Empty,
                NoteKind::Tap,
                NoteKind::Empty,
            ],
        },
    );
    graph.append(
        3.0,
        &Row {
            columns: vec![
                NoteKind::Empty,
                NoteKind::Empty,
                NoteKind::Empty,
                NoteKind::Tap,
            ],
        },
    );
    assert_eq!(
        graph.compute_path(),
        vec![
            FootPlacement::from_ddr_solo(
                FootPart::LeftHeel,
                FootPart::None,
                FootPart::None,
                FootPart::None
            ),
            FootPlacement::from_ddr_solo(
                FootPart::LeftHeel,
                FootPart::RightHeel,
                FootPart::None,
                FootPart::None
            ),
            FootPlacement::from_ddr_solo(
                FootPart::None,
                FootPart::RightHeel,
                FootPart::LeftHeel,
                FootPart::None
            ),
            FootPlacement::from_ddr_solo(
                FootPart::None,
                FootPart::None,
                FootPart::LeftHeel,
                FootPart::RightHeel
            )
        ]
    );
}
