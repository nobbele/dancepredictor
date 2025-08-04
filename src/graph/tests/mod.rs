use rgc_chart::models::common::Key;
use crate::{DanceStage, FootPart, FootPlacement, StepGraph};

mod basic;
mod brackets;
mod footswitch_jacks;

#[test]
fn walk_into_jump() {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);
    graph.append(
        0.0,
        &vec![Key::normal(), Key::empty(), Key::empty(), Key::empty()],
    );
    graph.append(
        1.0,
        &vec![Key::empty(), Key::normal(), Key::empty(), Key::empty()],
    );
    graph.append(
        2.0,
        &vec![Key::empty(), Key::empty(), Key::normal(), Key::empty()],
    );
    graph.append(
        3.0,
        &vec![Key::empty(), Key::empty(), Key::empty(), Key::normal()],
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