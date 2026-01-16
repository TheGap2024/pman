use ratatui::Frame;

use crate::actions::Action;
use crate::error::Result;

pub trait Component {
    fn handle_action(&mut self, action: &Action) -> Result<Option<Action>>;
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect);
    fn help_text(&self) -> &'static str;
}
