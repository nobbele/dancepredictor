use ordered_float::NotNan;
use std::ops::{Add, Div};

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct StagePosition(pub NotNan<f32>, pub NotNan<f32>);

impl StagePosition {
    pub fn new(x: f32, y: f32) -> StagePosition {
        StagePosition(NotNan::new(x).unwrap(), NotNan::new(y).unwrap())
    }
}

impl Add for StagePosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        StagePosition(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Div<f32> for StagePosition {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        StagePosition(
            NotNan::new(self.0 / rhs).unwrap(),
            NotNan::new(self.1 / rhs).unwrap(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DanceStage {
    columns: Vec<StagePosition>,
}

impl DanceStage {
    pub fn ddr_solo() -> Self {
        DanceStage {
            columns: vec![
                StagePosition::new(-1.0, 0.0),
                StagePosition::new(0.0, -1.0),
                StagePosition::new(0.0, 1.0),
                StagePosition::new(1.0, 0.0),
            ],
        }
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn is_valid_bracket(&self, a: usize, b: usize) -> bool {
        self.distance_between(a, b) < 2.
    }

    pub fn is_side_panel(&self, p: usize) -> bool {
        self.y(p) == 0.0 && self.x(p).abs() >= 1.0
    }

    pub fn distance_between(&self, a: usize, b: usize) -> f32 {
        let a = self.columns[a];
        let b = self.columns[b];

        ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
    }

    // Get the sine value of two panels on the stage
    pub fn sin(&self, a: usize, b: usize) -> f32 {
        let l = self.distance_between(a, b);
        (self.columns[b].1 - self.columns[a].1) / l
    }

    // Get the cosine value of two panels on the stage
    pub fn cos(&self, a: usize, b: usize) -> f32 {
        let l = self.distance_between(a, b);
        (self.columns[b].0 - self.columns[a].0) / l
    }

    pub fn x_difference(&self, left: usize, right: usize) -> f32 {
        if left == right {
            return 0.0;
        }

        let sign = (self.columns[right].0 - self.columns[left].0).signum() as f32;
        self.cos(left, right).powf(4.0) * sign
    }

    pub fn y_difference(&self, left: usize, right: usize) -> f32 {
        if left == right {
            return 0.0;
        }

        let sign = (self.columns[right].1 - self.columns[left].1).signum() as f32;
        self.sin(left, right).powf(4.0) * sign
    }

    pub fn position(&self, index: usize) -> StagePosition {
        self.columns[index]
    }

    pub fn x(&self, index: usize) -> f32 {
        self.position(index).0.into_inner()
    }

    pub fn y(&self, index: usize) -> f32 {
        self.position(index).1.into_inner()
    }

    pub fn average_position(&self, a: usize, b: usize) -> StagePosition {
        (self.position(a) + self.position(b)) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_valid_brackets() {
        let stage = DanceStage::ddr_solo();
        assert_eq!(stage.is_valid_bracket(0, 1), true);
        assert_eq!(stage.is_valid_bracket(0, 2), true);
        assert_eq!(stage.is_valid_bracket(3, 1), true);
        assert_eq!(stage.is_valid_bracket(3, 2), true);
    }

    #[test]
    fn test_invalid_brackets() {
        let stage = DanceStage::ddr_solo();
        assert_ne!(stage.is_valid_bracket(0, 3), true);
        assert_ne!(stage.is_valid_bracket(1, 2), true);
    }

    #[test]
    fn test_xy_difference() {
        let stage = DanceStage::ddr_solo();
        assert_relative_eq!(stage.x_difference(0, 0), 0.0);
        assert_relative_eq!(stage.x_difference(0, 3), 1.0);
        assert_relative_eq!(stage.x_difference(3, 0), -1.0);
        assert_relative_eq!(stage.x_difference(1, 0), -0.25);
        assert_relative_eq!(stage.x_difference(1, 3), 0.25);
    }
}
