use crate::cost::{CostParams, SLOW_BRACKET_COST, SLOW_BRACKET_THRESHOLD, TWISTED_FOOT_COST};
use crate::feet::{FootPartIndices, Side};
use crate::stage::StagePosition;

pub fn twisted_foot_cost(CostParams { stage, next, .. }: CostParams) -> f32 {
    let FootPartIndices {
        left_heel,
        left_toe,
        right_heel,
        right_toe,
    } = next.final_columns.get_foot_part_indices();

    let left_position = match (left_heel, left_toe.or(left_heel)) {
        (Some(heel), Some(toe)) => stage.average_position(heel, toe),
        _ => StagePosition::new(0.0, 0.0),
    };
    let right_position = match (right_heel, right_toe.or(right_heel)) {
        (Some(heel), Some(toe)) => stage.average_position(heel, toe),
        _ => StagePosition::new(0.0, 0.0),
    };

    // Twisted foot doesn't apply when performing crossovers.
    let crossover = right_position.0 < left_position.0;
    if crossover {
        return 0.0;
    }

    let left_backwards = match (left_heel, left_toe) {
        (Some(heel), Some(toe)) => stage.position(toe).1 < stage.position(heel).1,
        _ => false,
    };
    let right_backwards = match (right_heel, right_toe) {
        (Some(heel), Some(toe)) => stage.position(toe).1 < stage.position(heel).1,
        _ => false,
    };

    if !left_backwards && !right_backwards {
        return 0.0;
    }

    TWISTED_FOOT_COST
}

pub fn slow_bracket_cost(CostParams { next, dt, .. }: CostParams) -> f32 {
    let is_bracketing = [Side::Left, Side::Right]
        .into_iter()
        .any(|s| next.activated_columns.is_bracketing(s));
    if !is_bracketing {
        return 0.0;
    }

    if dt < SLOW_BRACKET_THRESHOLD {
        return 0.0;
    }

    let time_diff = dt - SLOW_BRACKET_THRESHOLD;
    time_diff * SLOW_BRACKET_COST
}

// #[test]
// fn test() {
//     let dance_stage = DanceStage::ddr_solo();
//     let mut graph = StepGraph::new(dance_stage);
//     graph.append(
//         0.041,
//         &vec![Key::empty(), Key::normal(), Key::empty(), Key::empty()],
//     );
//     graph.append(
//         0.927,
//         &vec![Key::empty(), Key::empty(), Key::empty(), Key::normal()],
//     );
//     graph.append(
//         1.814,
//         &vec![Key::empty(), Key::empty(), Key::normal(), Key::normal()],
//     );
//     graph.append(
//         2.258,
//         &vec![Key::normal(), Key::empty(), Key::empty(), Key::normal()],
//     );
//     assert_eq!(
//         graph.compute_path(),
//         vec![
//             FootPlacement::parse("-L--").unwrap(),
//             FootPlacement::parse("-L-R").unwrap(),
//             FootPlacement::parse("--LR").unwrap(),
//             FootPlacement::parse("L--R").unwrap(),
//         ]
//     );
// }
