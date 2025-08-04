use crate::cost::{
    CostParams, JACK_COST, JACK_THRESHOLD, SIDESWITCH_COST, SLOW_FOOTSWITCH_COST,
    SLOW_FOOTSWITCH_THRESHOLD,
};
use crate::feet::Side;
use crate::FootPart;

fn jacked_side(CostParams { prev, next, .. }: CostParams, side: Side) -> bool {
    let heel = FootPart::heel(side);
    let toe = FootPart::toe(side);

    let Some(heel_idx) = next.final_columns.get_foot_part_index(heel) else {
        return false;
    };
    let Some(toe_idx) = next.final_columns.get_foot_part_index(toe) else {
        return false;
    };

    let jacked_heel = prev.final_columns.0[heel_idx] == heel;
    let jacked_toe = prev.final_columns.0[toe_idx] == toe;

    jacked_heel || jacked_toe
}

pub fn jack_cost(params @ CostParams { dt, .. }: CostParams) -> f32 {
    if dt > JACK_THRESHOLD {
        return 0.0;
    }

    let time_diff = JACK_THRESHOLD - dt;
    let time_cost = (1.0 / time_diff) - (1.0 / JACK_THRESHOLD);

    let mut cost = 0.0;
    if jacked_side(params, Side::Left) {
        cost += time_cost;
    }
    if jacked_side(params, Side::Right) {
        cost += time_cost;
    }

    cost * JACK_COST
}

pub fn footswitch_cost(
    CostParams {
        stage,
        prev,
        next,
        dt,
        ..
    }: CostParams,
) -> f32 {
    if dt < SLOW_FOOTSWITCH_THRESHOLD {
        return 0.0;
    }

    let activated_one_side_only =
        next.side_activated(Side::Left) ^ next.side_activated(Side::Right);
    if !activated_one_side_only {
        return 0.0;
    }

    let mut has_footswitch = false;
    let mut has_sideswitch = false;
    for column in 0..stage.column_count() {
        // Ignore column if there was no foot on it, or no foot is hitting it (i.e not a footswitch)
        if prev.final_columns.0[column] == FootPart::None
            || next.activated_columns.0[column] == FootPart::None
        {
            continue;
        }

        if prev.final_columns.0[column].side() != next.activated_columns.0[column].side() {
            has_footswitch = true;
            if stage.is_side_panel(column) {
                has_sideswitch = true;
            }
            break;
        }
    }

    if !has_footswitch {
        return 0.0;
    }

    let time_diff = SLOW_FOOTSWITCH_THRESHOLD - dt;
    let time_cost = time_diff / (SLOW_FOOTSWITCH_THRESHOLD + time_diff);

    let mut cost = time_cost * SLOW_FOOTSWITCH_COST;
    if has_sideswitch {
        cost += SIDESWITCH_COST;
    }
    cost
}
