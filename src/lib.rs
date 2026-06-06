//! Heavily inspired by https://mjvotaw.github.io/posts/step-annotation/step-annotations
pub(crate) mod cost;
pub(crate) mod extensions;
pub(crate) mod feet;
pub(crate) mod graph;
pub(crate) mod stage;
pub(crate) mod state;

use danceparser::view::{NoteView, NoteViewer};

pub use crate::extensions::HasPressRequirement;
pub use crate::feet::{FootPart, FootPlacement};
pub use crate::graph::StepEntry;
pub use crate::graph::StepGraph;
pub use crate::stage::DanceStage;

pub fn generate_steps(viewer: NoteViewer) -> Vec<StepEntry> {
    let dance_stage = DanceStage::ddr_solo();
    let mut graph = StepGraph::new(dance_stage);

    for NoteView { time, row, .. } in viewer {
        graph.append(time / 1000.0, row);
    }

    graph.compute_steps()
}
