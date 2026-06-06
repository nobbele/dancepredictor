mod basic;
mod brackets;
mod footswitch_jacks;

use crate::cost::footswitch_jacks::footswitch_cost;
use crate::stage::DanceStage;
use crate::state::State;
use basic::*;
use brackets::*;
use danceparser::Row;
use footswitch_jacks::*;

// TODO Don't ignore holds for costs (esp. when it comes to doublesteps).

const MOVEMENT_COST: f64 = 6.0;
const FACING_COST: f64 = 2000.0;
const DOUBLESTEP_COST: f64 = 850.0;
const MINE_COST: f64 = 10000.0;
const TWISTED_FOOT_COST: f64 = 1000.0;
const SLOW_BRACKET_COST: f64 = 300.0;
const JACK_COST: f64 = 30.0;
const SLOW_FOOTSWITCH_COST: f64 = 325.0;
const SIDESWITCH_COST: f64 = 130.0;

const SLOW_BRACKET_THRESHOLD: f64 = 0.15;
const JACK_THRESHOLD: f64 = 0.1;
const SLOW_FOOTSWITCH_THRESHOLD: f64 = 0.2;

#[derive(Copy, Clone)]
#[non_exhaustive]
struct CostParams<'a> {
    stage: &'a DanceStage,
    row: &'a Row,
    prev: &'a State,
    next: &'a State,
    dt: f64,
}

pub fn total_cost(stage: &DanceStage, row: &Row, prev: &State, next: &State, dt: f64) -> f64 {
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

    cost
}
