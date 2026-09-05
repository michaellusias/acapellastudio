//! AcapellaStudio — crate root
//!
//! Module layout matches docs/07_detailed_design.md exactly. Modules with
//! real, tested prototype code behind them are noted as such in their own
//! files. Modules with no prototype are stubs, also noted honestly.

pub mod audio;
pub mod dsp;
pub mod pitch_edit;
pub mod clip;
pub mod track;
pub mod mixer;
pub mod harmony;
pub mod project;
pub mod export;
// gui is intentionally NOT declared yet - no framework has been selected
// (Technology Selection §4), so there is nothing real to put here. Adding
// an empty gui module would misrepresent the project's actual state.
