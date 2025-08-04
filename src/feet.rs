use crate::extensions::HasPressRequirement;
use crate::stage::DanceStage;
use rgc_chart::models::common::Row;
use smallvec::{smallvec, SmallVec};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Hash)]
pub enum Side {
    Left,
    Right,
}

// Possible foot placement states for a column.
#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Hash)]
pub enum FootPart {
    None,
    LeftHeel,
    LeftToe,
    RightHeel,
    RightToe,
}

impl FootPart {
    pub(crate) fn all_except_none() -> [Self; 4] {
        [
            Self::LeftHeel,
            Self::LeftToe,
            Self::RightHeel,
            Self::RightToe,
        ]
    }

    pub(crate) fn is_toe(&self) -> bool {
        matches!(self, FootPart::LeftToe | FootPart::RightToe)
    }

    pub(crate) fn is_heel(&self) -> bool {
        matches!(self, FootPart::LeftHeel | FootPart::RightHeel)
    }

    pub(crate) fn side(&self) -> Option<Side> {
        match self {
            FootPart::None => None,
            FootPart::LeftHeel => Some(Side::Left),
            FootPart::LeftToe => Some(Side::Left),
            FootPart::RightHeel => Some(Side::Right),
            FootPart::RightToe => Some(Side::Right),
        }
    }

    pub(crate) fn heel(side: Side) -> FootPart {
        match side {
            Side::Left => FootPart::LeftHeel,
            Side::Right => FootPart::RightHeel,
        }
    }

    pub(crate) fn toe(side: Side) -> FootPart {
        match side {
            Side::Left => FootPart::LeftToe,
            Side::Right => FootPart::RightToe,
        }
    }

    pub(crate) fn other_part(&self) -> FootPart {
        if self.is_toe() {
            FootPart::heel(self.side().unwrap())
        } else if self.is_heel() {
            FootPart::toe(self.side().unwrap())
        } else {
            FootPart::None
        }
    }
}

impl Debug for FootPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for FootPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FootPart::None => write!(f, "-"),
            FootPart::LeftHeel => write!(f, "L"),
            FootPart::LeftToe => write!(f, "l"),
            FootPart::RightHeel => write!(f, "R"),
            FootPart::RightToe => write!(f, "r"),
        }
    }
}

/// Represents what foot part is on a column.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FootPlacement(pub SmallVec<[FootPart; 4]>);

impl FootPlacement {
    pub(crate) fn new(columns: usize) -> Self {
        FootPlacement(smallvec![FootPart::None; columns])
    }

    pub(crate) fn get_foot_part_index(&self, part: FootPart) -> Option<usize> {
        self.0.iter().position(|&x| x == part)
    }

    pub(crate) fn get_foot_part_indices(&self) -> FootPartIndices {
        FootPartIndices {
            left_heel: self.get_foot_part_index(FootPart::LeftHeel),
            left_toe: self.get_foot_part_index(FootPart::LeftToe),
            right_heel: self.get_foot_part_index(FootPart::RightHeel),
            right_toe: self.get_foot_part_index(FootPart::RightToe),
        }
    }

    pub(crate) fn contains(&self, part: FootPart) -> bool {
        self.0.iter().any(|&x| x == part)
    }

    pub(crate) fn is_bracketing(&self, side: Side) -> bool {
        match side {
            Side::Left => self.contains(FootPart::LeftToe) && self.contains(FootPart::LeftHeel),
            Side::Right => self.contains(FootPart::RightToe) && self.contains(FootPart::RightHeel),
        }
    }

    pub(crate) fn at(&self, column_idx: usize) -> FootPart {
        self.0[column_idx]
    }

    pub(crate) fn at_mut(&mut self, column_idx: usize) -> &mut FootPart {
        &mut self.0[column_idx]
    }
}

impl Debug for FootPlacement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for FootPlacement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        for part in &self.0 {
            <FootPart as Display>::fmt(&part, f)?;
        }
        write!(f, "\"")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FootPartIndices {
    pub left_heel: Option<usize>,
    pub left_toe: Option<usize>,
    pub right_heel: Option<usize>,
    pub right_toe: Option<usize>,
}

pub(crate) fn foot_placement_permutations(stage: &DanceStage, row: &Row) -> Vec<FootPlacement> {
    let mut permutations = Vec::new();

    permute_foot_placement(
        &mut permutations,
        stage,
        row,
        &FootPlacement::new(row.len()),
        0,
    );

    permutations
}

fn permute_foot_placement(
    permutations: &mut Vec<FootPlacement>,
    stage: &DanceStage,
    row: &Row,
    current_placement: &FootPlacement,
    column: usize,
) {
    if column >= row.len() {
        let indices = current_placement.get_foot_part_indices();

        // TODO This algorithm assumes you will always use the heel at least,
        // TODO and never exclusively the toes (unlike what a player should do)
        let invalid_left_toe = matches!((indices.left_heel, indices.left_toe), (None, Some(_)));
        let invalid_right_toe = matches!((indices.right_heel, indices.right_toe), (None, Some(_)));
        if invalid_left_toe || invalid_right_toe {
            return;
        }

        // Check for impossible brackets (e.g UP + DOWN, LEFT + RIGHT)
        let valid_left_bracket = !current_placement.is_bracketing(Side::Left)
            || stage.is_valid_bracket(indices.left_heel.unwrap(), indices.left_toe.unwrap());
        let valid_right_bracket = !current_placement.is_bracketing(Side::Right)
            || stage.is_valid_bracket(indices.right_heel.unwrap(), indices.right_toe.unwrap());
        if !valid_left_bracket || !valid_right_bracket {
            return;
        }

        permutations.push(current_placement.clone());
        return;
    }

    if let Some(column_key) = row.get(column)
        && column_key.key_type.require_press()
    {
        let mut new_placement = current_placement.clone();
        for foot_part in FootPart::all_except_none() {
            if current_placement.contains(foot_part) {
                continue;
            }

            new_placement.0[column] = foot_part;
            permute_foot_placement(permutations, stage, row, &new_placement, column + 1);
        }

        return;
    }

    permute_foot_placement(permutations, stage, row, &current_placement, column + 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rgc_chart::models::common::Key;

    #[test]
    fn test_tap_permutations() {
        let permutations = foot_placement_permutations(
            &DanceStage::ddr_solo(),
            &vec![Key::normal(), Key::empty(), Key::empty(), Key::empty()],
        );

        assert_eq!(
            permutations,
            vec![
                FootPlacement(smallvec![
                    FootPart::LeftHeel,
                    FootPart::None,
                    FootPart::None,
                    FootPart::None,
                ]),
                FootPlacement(smallvec![
                    FootPart::RightHeel,
                    FootPart::None,
                    FootPart::None,
                    FootPart::None,
                ])
            ]
        );
    }

    #[test]
    fn test_jump_permutations() {
        let permutations = foot_placement_permutations(
            &DanceStage::ddr_solo(),
            &vec![Key::normal(), Key::empty(), Key::empty(), Key::normal()],
        );

        assert_eq!(
            permutations,
            vec![
                FootPlacement(smallvec![
                    FootPart::LeftHeel,
                    FootPart::None,
                    FootPart::None,
                    FootPart::RightHeel,
                ]),
                FootPlacement(smallvec![
                    FootPart::RightHeel,
                    FootPart::None,
                    FootPart::None,
                    FootPart::LeftHeel,
                ])
            ]
        );
    }

    #[test]
    fn test_bracket_permutations() {
        let permutations = foot_placement_permutations(
            &DanceStage::ddr_solo(),
            &vec![Key::normal(), Key::normal(), Key::empty(), Key::empty()],
        );

        assert_eq!(
            permutations,
            vec![
                FootPlacement(smallvec![
                    FootPart::LeftHeel,
                    FootPart::LeftToe,
                    FootPart::None,
                    FootPart::None
                ]),
                FootPlacement(smallvec![
                    FootPart::LeftHeel,
                    FootPart::RightHeel,
                    FootPart::None,
                    FootPart::None
                ]),
                FootPlacement(smallvec![
                    FootPart::LeftToe,
                    FootPart::LeftHeel,
                    FootPart::None,
                    FootPart::None
                ]),
                FootPlacement(smallvec![
                    FootPart::RightHeel,
                    FootPart::LeftHeel,
                    FootPart::None,
                    FootPart::None
                ]),
                FootPlacement(smallvec![
                    FootPart::RightHeel,
                    FootPart::RightToe,
                    FootPart::None,
                    FootPart::None
                ]),
                FootPlacement(smallvec![
                    FootPart::RightToe,
                    FootPart::RightHeel,
                    FootPart::None,
                    FootPart::None
                ])
            ]
        );
    }
}
