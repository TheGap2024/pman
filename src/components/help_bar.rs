use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct HelpBar;

impl HelpBar {
    pub fn render(frame: &mut Frame, area: Rect, help_text: &str) {
        let spans: Vec<Span> = help_text
            .split("  ")
            .flat_map(|part| {
                if let Some((key, desc)) = part.split_once(':') {
                    vec![
                        Span::styled(key, Style::default().fg(Color::Yellow)),
                        Span::raw(":"),
                        Span::styled(desc, Style::default().fg(Color::Gray)),
                        Span::raw("  "),
                    ]
                } else {
                    vec![Span::raw(part), Span::raw("  ")]
                }
            })
            .collect();

        let line = Line::from(spans);
        let paragraph = Paragraph::new(line).style(Style::default().bg(Color::DarkGray));

        frame.render_widget(paragraph, area);
    }
}
