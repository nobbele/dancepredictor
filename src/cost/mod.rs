mod basic;
mod brackets;
mod footswitch_jacks;

use crate::cost::footswitch_jacks::footswitch_cost;
use crate::stage::DanceStage;
use crate::state::State;
use basic::*;
use brackets::*;
use footswitch_jacks::*;
use rgc_chart::models::common::Key;

// TODO Don't ignore holds for costs (esp. when it comes to doublesteps).

const MOVEMENT_COST: f32 = 6.0;
const FACING_COST: f32 = 200.0;
const DOUBLESTEP_COST: f32 = 850.0;
const MINE_COST: f32 = 10000.0;
const TWISTED_FOOT_COST: f32 = 100.0;
const SLOW_BRACKET_COST: f32 = 300.0;
const JACK_COST: f32 = 30.0;
const SLOW_FOOTSWITCH_COST: f32 = 325.0;
const SIDESWITCH_COST: f32 = 130.0;

const SLOW_BRACKET_THRESHOLD: f32 = 0.15;
const JACK_THRESHOLD: f32 = 0.1;
const SLOW_FOOTSWITCH_THRESHOLD: f32 = 0.2;

#[derive(Copy, Clone)]
#[non_exhaustive]
struct CostParams<'a> {
    stage: &'a DanceStage,
    row: &'a [Key],
    prev: &'a State,
    next: &'a State,
    dt: f32,
}

pub fn total_cost(stage: &DanceStage, row: &[Key], prev: &State, next: &State, dt: f32) -> f32 {
    let params = CostParams {
        stage,
        row,
        prev,
        next,
        dt,
    };

    let mut cost = 0.0;
    cost += movement_cost(params);
    cost += facing_cost(params);
    cost += doublestep_cost(params);
    cost += mine_cost(params);
    cost += twisted_foot_cost(params);
    cost += slow_bracket_cost(params);
    cost += jack_cost(params);
    cost += footswitch_cost(params);

    // let mut costs = HashMap::new();
    // costs.insert("doublestep_cost", doublestep_cost(params));
    // costs.insert("total", cost);
    // 
    // println!(
    //     "{} -> {} costs [{:?}]",
    //     prev.final_columns, next.final_columns, costs
    // );

    cost
}
