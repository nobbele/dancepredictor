//! Heavily inspired by https://mjvotaw.github.io/posts/step-annotation/step-annotations
#![feature(let_chains)]

pub(crate) mod cost;
pub(crate) mod extensions;
pub(crate) mod feet;
pub(crate) mod graph;
pub(crate) mod stage;
pub(crate) mod state;

pub use crate::feet::{FootPart, FootPlacement};
pub use crate::graph::StepGraph;
pub use crate::stage::DanceStage;

// let chart = parse::from_sm(include_str!("../basic.sm")).expect("Failed to parse Stepmania chart");