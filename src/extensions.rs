use rgc_chart::models::common::KeyType;

pub trait HasPressRequirement {
    fn require_press(&self) -> bool;
    #[allow(dead_code)]
    fn require_release(&self) -> bool;
}

impl HasPressRequirement for KeyType {
    fn require_press(&self) -> bool {
        matches!(self, KeyType::Normal | KeyType::SliderStart)
    }

    fn require_release(&self) -> bool {
        matches!(self, KeyType::Mine)
    }
}
