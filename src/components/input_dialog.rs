use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::actions::{Action, InputCallback};
use crate::error::Result;

pub struct InputDialog {
    title: String,
    input: String,
    callback: InputCallback,
}

impl InputDialog {
    pub fn new(title: impl Into<String>, callback: InputCallback) -> Self {
        Self {
            title: title.into(),
            input: String::new(),
            callback,
        }
    }

    pub fn handle_action(&mut self, action: &Action) -> Result<Option<Action>> {
        match action {
            Action::Character(c) => {
                self.input.push(*c);
                Ok(Some(Action::Render))
            }
            Action::Backspace => {
                self.input.pop();
                Ok(Some(Action::Render))
            }
            Action::Enter => {
                if self.input.is_empty() {
                    return Ok(Some(Action::CloseDialog));
                }
                let result = match &self.callback {
                    InputCallback::CreateSession => {
                        Action::CreateSession(self.input.clone(), None)
                    }
                    InputCallback::CreateWorktree => {
                        Action::CreateWorktree(self.input.clone())
                    }
                    InputCallback::RenameSession => {
                        // Could be implemented later
                        Action::CloseDialog
                    }
                };
                Ok(Some(result))
            }
            Action::Escape => Ok(Some(Action::CloseDialog)),
            _ => Ok(None),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let dialog_width = 50.min(area.width.saturating_sub(4));
        let dialog_height = 5;

        let dialog_area = centered_rect(dialog_width, dialog_height, area);

        frame.render_widget(Clear, dialog_area);

        let block = Block::default()
            .title(format!(" {} ", self.title))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(dialog_area);
        frame.render_widget(block, dialog_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .margin(1)
            .split(inner);

        let input_line = Line::from(vec![
            Span::styled("> ", Style::default().fg(Color::Yellow)),
            Span::raw(&self.input),
            Span::styled("│", Style::default().fg(Color::Gray)),
        ]);

        let input = Paragraph::new(input_line);
        frame.render_widget(input, chunks[0]);

        let hint = Paragraph::new("Enter: confirm  Esc: cancel")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[1]);
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
