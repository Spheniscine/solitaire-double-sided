use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub allow_undo: bool,
}