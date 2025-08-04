use std::io::Write;
use petgraph::dot::Dot;
use crate::{DanceStage, FootPlacement, StepGraph};
use rgc_chart::models::common::Key;

#[test]
fn test_graph_brackets_instead_of_jumps() {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);
    graph.append(
        0.1,
        &vec![Key::normal(), Key::empty(), Key::empty(), Key::empty()],
    );
    graph.append(
        0.2,
        &vec![Key::empty(), Key::empty(), Key::normal(), Key::normal()],
    );
    graph.append(
        0.3,
        &vec![Key::normal(), Key::empty(), Key::empty(), Key::empty()],
    );
    graph.append(
        0.4,
        &vec![Key::empty(), Key::normal(), Key::empty(), Key::normal()],
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
    graph.append(
        1.0,
        &vec![Key::normal(), Key::empty(), Key::empty(), Key::empty()],
    );
    graph.append(
        2.0,
        &vec![Key::empty(), Key::empty(), Key::normal(), Key::normal()],
    );
    graph.append(
        3.0,
        &vec![Key::normal(), Key::empty(), Key::empty(), Key::empty()],
    );
    write!(
        std::fs::File::create("out.txt").unwrap(),
        "{}",
        Dot::with_config(&graph.graph, &[])
    )
        .unwrap();
    // graph.append(
    //     4.0,
    //     &vec![Key::empty(), Key::normal(), Key::empty(), Key::normal()],
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
//         &vec![Key::empty(), Key::empty(), Key::empty(), Key::normal()],
//     );
//     graph.append(
//         0.1,
//         &vec![Key::empty(), Key::normal(), Key::empty(), Key::empty()],
//     );
//     graph.append(
//         0.2,
//         &vec![Key::normal(), Key::empty(), Key::normal(), Key::empty()],
//     );
//     graph.append(
//         0.3,
//         &vec![Key::empty(), Key::normal(), Key::empty(), Key::empty()],
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
