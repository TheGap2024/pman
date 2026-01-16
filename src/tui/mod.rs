pub mod terminal;
mod event;

pub use terminal::Tui;
pub use event::{Event, EventHandler, key_to_action};
