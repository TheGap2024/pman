use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub struct FuzzyList<T> {
    items: Vec<T>,
    filtered_indices: Vec<usize>,
    query: String,
    list_state: ListState,
    matcher: Matcher,
    title: String,
    display_fn: fn(&T) -> String,
    search_fn: fn(&T) -> String,
}

impl<T: Clone> FuzzyList<T> {
    pub fn new(
        title: impl Into<String>,
        display_fn: fn(&T) -> String,
        search_fn: fn(&T) -> String,
    ) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            items: Vec::new(),
            filtered_indices: Vec::new(),
            query: String::new(),
            list_state,
            matcher: Matcher::new(Config::DEFAULT),
            title: title.into(),
            display_fn,
            search_fn,
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.update_filter();
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn push_char(&mut self, c: char) {
        self.query.push(c);
        self.update_filter();
    }

    pub fn pop_char(&mut self) {
        self.query.pop();
        self.update_filter();
    }

    pub fn clear_query(&mut self) {
        self.query.clear();
        self.update_filter();
    }

    pub fn move_up(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0);
        let new = if current == 0 {
            self.filtered_indices.len() - 1
        } else {
            current - 1
        };
        self.list_state.select(Some(new));
    }

    pub fn move_down(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0);
        let new = if current >= self.filtered_indices.len() - 1 {
            0
        } else {
            current + 1
        };
        self.list_state.select(Some(new));
    }

    pub fn page_up(&mut self, page_size: usize) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0);
        let new = current.saturating_sub(page_size);
        self.list_state.select(Some(new));
    }

    pub fn page_down(&mut self, page_size: usize) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0);
        let new = (current + page_size).min(self.filtered_indices.len() - 1);
        self.list_state.select(Some(new));
    }

    pub fn selected(&self) -> Option<&T> {
        let selected_idx = self.list_state.selected()?;
        let item_idx = self.filtered_indices.get(selected_idx)?;
        self.items.get(*item_idx)
    }

    pub fn selected_index(&self) -> Option<usize> {
        let selected_idx = self.list_state.selected()?;
        self.filtered_indices.get(selected_idx).copied()
    }

    fn update_filter(&mut self) {
        if self.query.is_empty() {
            self.filtered_indices = (0..self.items.len()).collect();
        } else {
            let pattern = Pattern::parse(&self.query, CaseMatching::Ignore, Normalization::Smart);
            let mut scored: Vec<(usize, u32)> = self
                .items
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    let text = (self.search_fn)(item);
                    let mut buf = Vec::new();
                    let score = pattern.score(nucleo_matcher::Utf32Str::new(&text, &mut buf), &mut self.matcher)?;
                    Some((idx, score))
                })
                .collect();

            scored.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered_indices = scored.into_iter().map(|(idx, _)| idx).collect();
        }

        if !self.filtered_indices.is_empty() {
            let current = self.list_state.selected().unwrap_or(0);
            if current >= self.filtered_indices.len() {
                self.list_state.select(Some(0));
            }
        } else {
            self.list_state.select(None);
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        // Search input
        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", self.title));

        let input = Paragraph::new(Line::from(vec![
            Span::styled("> ", Style::default().fg(Color::Yellow)),
            Span::raw(&self.query),
            Span::styled("│", Style::default().fg(Color::Gray)),
        ]))
        .block(input_block);

        frame.render_widget(input, chunks[0]);

        // List
        let items: Vec<ListItem> = self
            .filtered_indices
            .iter()
            .map(|&idx| {
                let item = &self.items[idx];
                let display = (self.display_fn)(item);
                ListItem::new(display)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        frame.render_stateful_widget(list, chunks[1], &mut self.list_state);
    }
}
