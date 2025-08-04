use crate::feet::{FootPart, FootPlacement, Side};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    // modified_columns: FootPlacement,
    pub activated_columns: FootPlacement,
    pub final_columns: FootPlacement,
}

impl State {
    pub fn new(column_count: usize) -> State {
        State {
            // modified_columns: FootPlacement::new(column_count),
            activated_columns: FootPlacement::new(column_count),
            final_columns: FootPlacement::new(column_count),
        }
    }

    pub fn append(&self, columns: &FootPlacement) -> State {
        let column_count = columns.0.len();
        debug_assert_eq!(column_count, self.activated_columns.0.len());
        debug_assert_eq!(column_count, self.final_columns.0.len());

        let mut result = State::new(column_count);

        // let mut moved_part = HashSet::new();
        for column in 0..column_count {
            let column_value = columns.at(column);
            if column_value != FootPart::None {
                *result.activated_columns.at_mut(column) = column_value;

                // moved_part.insert(column_value);
            }
        }

        for column in 0..column_count {
            if result.activated_columns.at(column) != FootPart::None {
                *result.final_columns.at_mut(column) = result.activated_columns.at(column);
                continue;
            };

            let prev_foot_part = self.final_columns.at(column);
            if prev_foot_part != FootPart::None && !result.activated_columns.contains(prev_foot_part)
            {
                let moved_heel = result
                    .activated_columns
                    .contains(FootPart::heel(prev_foot_part.side().unwrap()));
                let bracket_to_tap_transition = prev_foot_part.is_toe() && moved_heel;

                // In case it's a bracket to tap transition,
                // we should not keep the toe state.
                if !bracket_to_tap_transition {
                    *result.final_columns.at_mut(column) = prev_foot_part;
                }
            }
        }

        result
    }

    pub fn foot_part_activated(&self, part: FootPart) -> bool {
        self.activated_columns.contains(part)
    }

    pub fn side_activated(&self, side: Side) -> bool {
        self.foot_part_activated(FootPart::heel(side))
            || self.foot_part_activated(FootPart::toe(side))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State ")?;
        <FootPlacement as Display>::fmt(&self.final_columns, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::smallvec;

    #[test]
    fn test_state_progression() {
        let state = State::new(4);
        let state = state.append(&FootPlacement(smallvec![
            FootPart::LeftHeel,
            FootPart::None,
            FootPart::None,
            FootPart::None,
        ]));
        assert_eq!(
            state.final_columns,
            FootPlacement(smallvec![
                FootPart::LeftHeel,
                FootPart::None,
                FootPart::None,
                FootPart::None,
            ])
        );
        let state = state.append(&FootPlacement(smallvec![
            FootPart::None,
            FootPart::None,
            FootPart::None,
            FootPart::RightHeel,
        ]));
        assert_eq!(
            state.final_columns,
            FootPlacement(smallvec![
                FootPart::LeftHeel,
                FootPart::None,
                FootPart::None,
                FootPart::RightHeel,
            ])
        );
        let state = state.append(&FootPlacement(smallvec![
            FootPart::None,
            FootPart::LeftHeel,
            FootPart::None,
            FootPart::None,
        ]));
        assert_eq!(
            state.final_columns,
            FootPlacement(smallvec![
                FootPart::None,
                FootPart::LeftHeel,
                FootPart::None,
                FootPart::RightHeel,
            ])
        );
        assert_eq!(
            state.activated_columns,
            FootPlacement(smallvec![
                FootPart::None,
                FootPart::LeftHeel,
                FootPart::None,
                FootPart::None,
            ])
        );
    }

    #[test]
    fn test_state_progression_brackets() {
        let state = State::new(4);
        let state = state.append(&FootPlacement(smallvec![
            FootPart::LeftHeel,
            FootPart::None,
            FootPart::LeftToe,
            FootPart::None,
        ]));
        assert_eq!(
            state.final_columns,
            FootPlacement(smallvec![
                FootPart::LeftHeel,
                FootPart::None,
                FootPart::LeftToe,
                FootPart::None,
            ])
        );
        let state = state.append(&FootPlacement(smallvec![
            FootPart::None,
            FootPart::None,
            FootPart::None,
            FootPart::RightHeel,
        ]));
        assert_eq!(
            state.final_columns,
            FootPlacement(smallvec![
                FootPart::LeftHeel,
                FootPart::None,
                FootPart::LeftToe,
                FootPart::RightHeel,
            ])
        );
        let state = state.append(&FootPlacement(smallvec![
            FootPart::None,
            FootPart::LeftHeel,
            FootPart::None,
            FootPart::None,
        ]));
        assert_eq!(
            state.final_columns,
            FootPlacement(smallvec![
                FootPart::None,
                FootPart::LeftHeel,
                FootPart::None,
                FootPart::RightHeel,
            ])
        );
    }

    #[test]
    fn test_state_progression_bracket_consecutive() {
        let state = State::new(4);
        let state = state.append(&FootPlacement(smallvec![
            FootPart::LeftHeel,
            FootPart::None,
            FootPart::LeftToe,
            FootPart::None,
        ]));
        let state = state.append(&FootPlacement(smallvec![
            FootPart::LeftToe,
            FootPart::LeftHeel,
            FootPart::None,
            FootPart::None,
        ]));
        assert_eq!(
            state.final_columns,
            FootPlacement(smallvec![
                FootPart::LeftToe,
                FootPart::LeftHeel,
                FootPart::None,
                FootPart::None,
            ])
        );
    }

    #[test]
    fn test_state_progression_toe_to_other_heel() {
        let state = State::new(4);
        let state = state.append(&FootPlacement(smallvec![
            FootPart::LeftHeel,
            FootPart::None,
            FootPart::None,
            FootPart::None,
        ]));
        let state = state.append(&FootPlacement(smallvec![
            FootPart::RightHeel,
            FootPart::RightToe,
            FootPart::None,
            FootPart::None,
        ]));
        assert_eq!(
            state.final_columns,
            FootPlacement(smallvec![
                FootPart::RightHeel,
                FootPart::RightToe,
                FootPart::None,
                FootPart::None,
            ])
        );
    }

    #[test]
    fn test_state_progression_jack() {
        let state = State::new(4);
        let state = state.append(&FootPlacement(smallvec![
            FootPart::LeftHeel,
            FootPart::None,
            FootPart::None,
            FootPart::None,
        ]));
        let state = state.append(&FootPlacement(smallvec![
            FootPart::None,
            FootPart::None,
            FootPart::None,
            FootPart::RightHeel,
        ]));
        let state = state.append(&FootPlacement(smallvec![
            FootPart::None,
            FootPart::None,
            FootPart::None,
            FootPart::RightHeel,
        ]));
        assert_eq!(
            state.final_columns,
            FootPlacement(smallvec![
                FootPart::LeftHeel,
                FootPart::None,
                FootPart::None,
                FootPart::RightHeel,
            ])
        );
    }
}
