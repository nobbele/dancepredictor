#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct PanelPosition(i32, i32);

#[derive(Debug, Clone, PartialEq)]
pub struct DanceStage {
    columns: Vec<PanelPosition>,
}

impl DanceStage {
    pub fn ddr_solo() -> Self {
        DanceStage {
            columns: vec![
                PanelPosition(-1, 0),
                PanelPosition(0, -1),
                PanelPosition(0, 1),
                PanelPosition(1, 0),
            ],
        }
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn is_valid_bracket(&self, a: usize, b: usize) -> bool {
        self.distance_between(a, b) < 2.
    }

    pub fn distance_between(&self, a: usize, b: usize) -> f32 {
        let a = self.columns[a];
        let b = self.columns[b];

        ((a.0 - b.0).pow(2) as f32 + (a.1 - b.1).pow(2) as f32).sqrt()
    }

    // Get the sine value of two panels on the stage
    pub fn sin(&self, a: usize, b: usize) -> f32 {
        let l = self.distance_between(a, b);
        (self.columns[b].1 - self.columns[a].1) as f32 / l
    }

    // Get the cosine value of two panels on the stage
    pub fn cos(&self, a: usize, b: usize) -> f32 {
        let l = self.distance_between(a, b);
        (self.columns[b].0 - self.columns[a].0) as f32 / l
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
