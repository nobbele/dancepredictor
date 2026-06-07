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
        graph.append(time, row);
    }

    graph.compute_steps()
}

#[cfg(test)]
mod tests {
    use super::*;
    use danceparser::{NoteKind, SMChart};
    use std::io::Cursor;

    #[test]
    fn test_eientewi() {
        let chart = SMChart::from_sm(Cursor::new(include_str!("../Eientewi Set 12A.sm")))
            .expect("Failed to parse StepMania chart");

        let diffs = chart.notes.iter().filter(|c| c.style == "dance-single");
        for diff in diffs {
            let expected_total_steps = diff
                .measures
                .iter()
                .flat_map(|m| m.rows.iter())
                .filter(|row| row.columns.iter().any(|&n| n != NoteKind::Empty))
                .count();

            let steps = generate_steps(NoteViewer::new(&chart, diff));
            assert_eq!(steps.len(), expected_total_steps);
        }
    }
}
