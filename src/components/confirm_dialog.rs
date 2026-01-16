use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::actions::{Action, ConfirmCallback};
use crate::error::Result;

pub struct ConfirmDialog {
    title: String,
    message: String,
    callback: ConfirmCallback,
    selected: bool, // false = No, true = Yes
}

impl ConfirmDialog {
    pub fn new(title: impl Into<String>, message: impl Into<String>, callback: ConfirmCallback) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            callback,
            selected: false,
        }
    }

    pub fn handle_action(&mut self, action: &Action) -> Result<Option<Action>> {
        match action {
            Action::Character('y') | Action::Character('Y') => {
                self.selected = true;
                Ok(Some(self.confirm()))
            }
            Action::Character('n') | Action::Character('N') => {
                Ok(Some(Action::CloseDialog))
            }
            Action::MoveUp | Action::MoveDown | Action::Character('h') | Action::Character('l') => {
                self.selected = !self.selected;
                Ok(Some(Action::Render))
            }
            Action::Enter => {
                if self.selected {
                    Ok(Some(self.confirm()))
                } else {
                    Ok(Some(Action::CloseDialog))
                }
            }
            Action::Escape => Ok(Some(Action::CloseDialog)),
            _ => Ok(None),
        }
    }

    fn confirm(&self) -> Action {
        match &self.callback {
            ConfirmCallback::DeleteWorktree(path) => Action::DeleteWorktree(path.clone()),
            ConfirmCallback::MergeWorktree(path) => Action::MergeWorktree(path.clone()),
            ConfirmCallback::KillSession(name) => Action::KillSession(name.clone()),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let dialog_width = 50.min(area.width.saturating_sub(4));
        let dialog_height = 7;

        let dialog_area = centered_rect(dialog_width, dialog_height, area);

        frame.render_widget(Clear, dialog_area);

        let block = Block::default()
            .title(format!(" {} ", self.title))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        let inner = block.inner(dialog_area);
        frame.render_widget(block, dialog_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .margin(1)
            .split(inner);

        let message = Paragraph::new(self.message.as_str())
            .alignment(Alignment::Center);
        frame.render_widget(message, chunks[0]);

        let no_style = if !self.selected {
            Style::default().fg(Color::White).bg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        let yes_style = if self.selected {
            Style::default().fg(Color::White).bg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        let buttons = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(chunks[1]);

        let no_btn = Paragraph::new(" [N]o ")
            .style(no_style)
            .alignment(Alignment::Center);
        let yes_btn = Paragraph::new(" [Y]es ")
            .style(yes_style)
            .alignment(Alignment::Center);

        frame.render_widget(no_btn, buttons[0]);
        frame.render_widget(yes_btn, buttons[1]);
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
