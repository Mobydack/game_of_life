use crate::diagnostics::CELLS_COUNT;
use bevy::{diagnostic::DiagnosticsStore, ecs::system::lifetimeless::SRes, prelude::*};
use iyes_perf_ui::{entry::PerfUiEntry, utils};

#[derive(Component)]
pub struct PerfUiCellsCount {
    pub label: String,
    pub sort_key: i32,
}

impl Default for PerfUiCellsCount {
    fn default() -> Self {
        Self {
            label: String::new(),
            sort_key: utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiCellsCount {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = u32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Cells"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as bevy::ecs::system::SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(diagnostics.get(&CELLS_COUNT)?.value()? as u32)
    }
}
