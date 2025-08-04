use crate::feet::{FootPart, FootPartIndices, Side};
use crate::stage::DanceStage;
use crate::state::State;

const MOVEMENT_COST: f32 = 6.0;
const FACING_COST: f32 = 2.0;
const DOUBLESTEP_COST: f32 = 8.0;

#[derive(Copy, Clone)]
#[non_exhaustive]
struct CostParams<'a> {
    stage: &'a DanceStage,
    prev: &'a State,
    next: &'a State,
    dt: f32,
}

pub fn total_cost(stage: &DanceStage, prev: &State, next: &State, dt: f32) -> f32 {
    let params = CostParams {
        stage,
        prev,
        next,
        dt,
    };

    let mut cost = 0.0;
    cost += movement_cost(params);
    cost += facing_cost(params);
    cost += doublestep_cost(params);

    cost
}

fn movement_cost(
    CostParams {
        stage,
        prev,
        next,
        dt,
        ..
    }: CostParams,
) -> f32 {
    let mut cost = 0.0;

    for part in FootPart::all_except_none() {
        if !next.foot_part_activated(part) {
            continue;
        }

        // If the foot part wasn't on the stage already, it shouldn't incur a "movement" cost.
        let Some(prev_position) = prev.final_columns.get_foot_part_index(part) else {
            continue;
        };

        let Some(next_position) = next.final_columns.get_foot_part_index(part) else {
            panic!("An activated foot part should have a placement in the final state")
        };

        // For tap -> bracket transitions: we don't incur movement cost twice
        // e.g `---R` -> `-R-r` should ignore R->r because we will count the [3]->[1] movement.
        if next.final_columns.get_foot_part_index(part.other_part()) == Some(prev_position) {
            continue;
        }

        let velocity = stage.distance_between(prev_position, next_position) / dt;
        cost += velocity * MOVEMENT_COST;
    }

    cost
}

fn facing_cost(CostParams { stage, next, .. }: CostParams) -> f32 {
    let FootPartIndices {
        left_heel,
        mut left_toe,
        right_heel,
        mut right_toe,
    } = next.final_columns.get_foot_part_indices();

    left_toe = left_toe.or(left_heel);
    right_toe = right_toe.or(right_heel);

    let heel_facing = if let (Some(left_heel), Some(right_heel)) = (left_heel, right_heel) {
        stage.x_difference(left_heel, right_heel)
    } else {
        0.0
    };
    let toe_facing = if let (Some(left_toe), Some(right_toe)) = (left_toe, right_toe) {
        stage.x_difference(left_toe, right_toe)
    } else {
        0.0
    };
    let left_facing = if let (Some(left_heel), Some(left_toe)) = (left_heel, left_toe) {
        stage.y_difference(left_heel, left_toe)
    } else {
        0.0
    };
    let right_facing = if let (Some(right_heel), Some(right_toe)) = (right_heel, right_toe) {
        stage.y_difference(right_heel, right_toe)
    } else {
        0.0
    };

    fn penalty(v: f32) -> f32 {
        (-1.0 * v.min(0.0)).powf(1.8)
    }

    let mut cost = 0.0;
    cost += penalty(heel_facing) * FACING_COST;
    cost += penalty(toe_facing) * FACING_COST;
    cost += penalty(left_facing) * FACING_COST;
    cost += penalty(right_facing) * FACING_COST;
    cost
}

fn doublestep_cost(
    CostParams {
        stage, prev, next, ..
    }: CostParams,
) -> f32 {
    let activated_one_side_only =
        next.side_activated(Side::Left) ^ next.side_activated(Side::Right);
    if !activated_one_side_only {
        return 0.0;
    }

    let activated_side = if next.side_activated(Side::Left) {
        Side::Left
    } else {
        Side::Right
    };

    let prev_heel = prev
        .activated_columns
        .get_foot_part_index(FootPart::heel(activated_side));
    let prev_toe = prev
        .activated_columns
        .get_foot_part_index(FootPart::toe(activated_side));
    let next_heel = next
        .activated_columns
        .get_foot_part_index(FootPart::heel(activated_side));
    let next_toe = next
        .activated_columns
        .get_foot_part_index(FootPart::toe(activated_side));

    if prev_heel == None {
        return 0.0;
    }

    let jacked = prev_heel == next_heel && prev_toe == next_toe;
    if jacked {
        return 0.0;
    }

    DOUBLESTEP_COST
        * stage
            .distance_between(prev_heel.unwrap(), next_heel.unwrap())
            .powi(3)
}
