use crate::{DanceStage, FootPlacement, StepGraph};
use danceparser::{NoteKind, Row};
use petgraph::dot::Dot;
use std::io::Write;

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
    write!(
        std::fs::File::create("out.txt").unwrap(),
        "{}",
        Dot::with_config(&graph.graph, &[])
    )
    .unwrap();
    // graph.append(
    //     4.0,
    //     &Row { columns: &vec![NoteKind::Empty, NoteKind::Tap, NoteKind::Empty, NoteKind::Tap],
    // );
    assert_eq!(
        graph.compute_path(),
        vec![
            FootPlacement::parse("L---").unwrap(),
            FootPlacement::parse("--LR").unwrap(),
            FootPlacement::parse("L--R").unwrap(),
            // FootPlacement::parse("-L-R").unwrap(),
        ]
    );
}

// #[test]
// fn test_graph_crossover_foot_bracket() {
//     let dance_stage = DanceStage::ddr_solo();
//     let mut graph = StepGraph::new(dance_stage);
//     graph.append(
//         0.0,
//         &Row { columns: &vec![NoteKind::Empty, NoteKind::Empty, NoteKind::Empty, NoteKind::Tap],
//     );
//     graph.append(
//         0.1,
//         &Row { columns: &vec![NoteKind::Empty, NoteKind::Tap, NoteKind::Empty, NoteKind::Empty],
//     );
//     graph.append(
//         0.2,
//         &Row { columns: &vec![NoteKind::Tap, NoteKind::Empty, NoteKind::Tap, NoteKind::Empty],
//     );
//     graph.append(
//         0.3,
//         &Row { columns: &vec![NoteKind::Empty, NoteKind::Tap, NoteKind::Empty, NoteKind::Empty],
//     );
//     assert_eq!(
//         graph.compute_path(),
//         vec![
//             FootPlacement::from_ddr_solo(
//                 FootPart::None,
//                 FootPart::None,
//                 FootPart::None,
//                 FootPart::RightHeel
//             ),
//             FootPlacement::from_ddr_solo(
//                 FootPart::None,
//                 FootPart::LeftHeel,
//                 FootPart::None,
//                 FootPart::RightHeel
//             ),
//             FootPlacement::from_ddr_solo(
//                 FootPart::RightToe,
//                 FootPart::LeftHeel,
//                 FootPart::RightHeel,
//                 FootPart::None
//             ),
//             FootPlacement::from_ddr_solo(
//                 FootPart::RightToe,
//                 FootPart::LeftHeel,
//                 FootPart::RightHeel,
//                 FootPart::None
//             )
//         ]
//     );
// }
