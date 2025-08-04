#[cfg(test)]
mod tests;

use crate::cost::total_cost;
use crate::feet::{foot_placement_permutations, FootPlacement};
use crate::stage::DanceStage;
use crate::state::State;
use ordered_float::OrderedFloat;
use petgraph::algo::astar;
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use rgc_chart::models::common::Key;
use rustc_hash::FxBuildHasher;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphState {
    time: OrderedFloat<f32>,
    state: State,
}

impl Display for GraphState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: ", self.time)?;
        <State as Display>::fmt(&self.state, f)
    }
}

impl GraphState {
    pub fn new(time: f32, state: State) -> Self {
        GraphState {
            time: OrderedFloat(time),
            state,
        }
    }
}

pub struct StepGraph {
    dance_stage: DanceStage,

    queue: VecDeque<NodeIndex>,
    pub graph: DiGraph<GraphState, f32>,
    node_cache: HashMap<GraphState, NodeIndex, FxBuildHasher>,
    edge_cache: HashMap<(NodeIndex, NodeIndex, OrderedFloat<f32>), EdgeIndex, FxBuildHasher>,

    start_node: NodeIndex,
}

impl StepGraph {
    pub fn new(dance_stage: DanceStage) -> Self {
        let mut graph = DiGraph::new();
        let mut state_map = HashMap::default();

        let start_state =
            GraphState::new(f32::NEG_INFINITY, State::new(dance_stage.column_count()));
        let start_node = graph.add_node(start_state.clone());
        state_map.insert(start_state, start_node);

        let mut queue = VecDeque::new();
        queue.push_back(start_node);

        StepGraph {
            dance_stage,

            queue,
            graph,
            node_cache: state_map,
            edge_cache: HashMap::default(),

            start_node,
        }
    }

    pub fn append(&mut self, time: f32, row: &[Key]) {
        assert_eq!(row.len(), self.dance_stage.column_count());

        let permutations = foot_placement_permutations(&self.dance_stage, row);

        let mut new_states = Vec::new();
        while let Some(prev) = self.queue.pop_front() {
            for permutation in &permutations {
                let next_state = GraphState::new(time, self.graph[prev].state.append(permutation));
                let prev_state = &self.graph[prev];
                let cost = total_cost(
                    &self.dance_stage,
                    row,
                    &prev_state.state,
                    &next_state.state,
                    next_state.time.0 - prev_state.time.0,
                );

                let next = if let Some(&next) = self.node_cache.get(&next_state) {
                    next
                } else {
                    let next = self.graph.add_node(next_state.clone());
                    self.node_cache.insert(next_state, next);
                    new_states.push(next);
                    next
                };

                self.add_edge(prev, next, cost);
            }
        }

        self.queue.extend(new_states);
    }

    fn add_edge(&mut self, prev: NodeIndex, next: NodeIndex, cost: f32) {
        if !self
            .edge_cache
            .contains_key(&(prev, next, OrderedFloat(cost)))
        {
            let edge = self.graph.add_edge(prev, next, cost);
            self.edge_cache
                .insert((prev, next, OrderedFloat(cost)), edge);
        }
    }

    pub fn compute_path(&mut self) -> Vec<FootPlacement> {
        // Final empty state, just to set as a goal
        let final_state = GraphState::new(f32::NAN, State::new(self.dance_stage.column_count()));
        let final_node = self.graph.add_node(final_state);
        while let Some(prev) = self.queue.pop_front() {
            self.graph.add_edge(prev, final_node, 0.0);
        }

        let res = astar(
            &self.graph,
            self.start_node,
            |node| node == final_node,
            |edge| *edge.weight(),
            |_| 0.0,
        );

        let path = if let Some((_cost, path)) = res {
            // Ignore empty start and end nodes
            let path = &path[1..path.len() - 1];
            path.iter()
                .map(|node| self.graph[*node].state.clone().final_columns)
                .collect()
        } else {
            Vec::new()
        };

        // Make the graph re-usable again
        self.graph.remove_node(final_node);

        path
    }
}
