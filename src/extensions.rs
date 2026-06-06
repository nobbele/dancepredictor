use danceparser::NoteKind;

pub trait HasPressRequirement {
    fn require_press(&self) -> bool;
    #[allow(dead_code)]
    fn require_release(&self) -> bool;
}

impl HasPressRequirement for NoteKind {
    fn require_press(&self) -> bool {
        matches!(
            self,
            NoteKind::Tap | NoteKind::HoldHead | NoteKind::RollHead
        )
    }

    fn require_release(&self) -> bool {
        matches!(self, NoteKind::Mine)
    }
}
