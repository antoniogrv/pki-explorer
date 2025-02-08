use color_eyre::eyre::Error;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, StatefulWidget, Widget, Wrap},
    DefaultTerminal,
};

use crate::{x509::X509, x509_tui::X509TUIList};

pub struct PKIExplorerApp {
    done: bool,
    x509_tui_list: X509TUIList,
    workdir: String,
}

impl Widget for &mut PKIExplorerApp {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [header_area, content_area, footer_area] = Layout::vertical([
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ])
        .areas(area);

        self.render_header_area(header_area, buf);
        self.render_content_area(content_area, buf);
        self.render_footer_area(footer_area, buf);
    }
}

impl PKIExplorerApp {
    pub fn new(x509s: Vec<X509>, workdir: String) -> Self {
        Self {
            done: Default::default(),
            x509_tui_list: X509TUIList::new(x509s),
            workdir: workdir,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Error> {
        while !self.done {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.done = true,
            KeyCode::Up => self.x509_tui_list.state.select_previous(),
            KeyCode::Down => self.x509_tui_list.state.select_next(),
            _ => {}
        }
    }

    fn render_header_area(&mut self, area: Rect, buf: &mut Buffer) {
        let header_content: Text = Text::from(vec![
            Line::from(vec![Span::styled(
                "pki-explorer",
                Style::default().fg(Color::LightBlue).bold(),
            )]),
            Line::from("use the arrow keys to move; press 'q' to exit; use '-h' for help"),
        ]);

        Paragraph::new(header_content)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false })
            .block(Block::default())
            .render(area, buf);
    }

    fn render_footer_area(&mut self, area: Rect, buf: &mut Buffer) {
        let footer_content: Text = Text::from(format!(
            "currently exploring the directory: {}",
            self.workdir
        ));

        Paragraph::new(footer_content)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false })
            .block(Block::default())
            .render(area, buf);
    }

    fn render_content_area(&mut self, area: Rect, buf: &mut Buffer) {
        let [list_area, content_area] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(area);

        self.render_list_area(list_area, buf);
        self.render_content_display_area(content_area, buf);
    }

    fn render_list_area(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .padding(Padding::uniform(1))
            .borders(Borders::NONE);

        let items: Vec<ListItem> = self
            .x509_tui_list
            .items
            .iter()
            .map(ListItem::from)
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.x509_tui_list.state);
    }

    fn render_content_display_area(&self, area: Rect, buf: &mut Buffer) {
        let mut lines: Vec<Line> = Vec::new();

        if self.x509_tui_list.items.len() == 0 {
            lines.push(Line::from(
                "This path contains no valid X509 certificates. Please use -p to specifiy a path.",
            ).alignment(Alignment::Center));
        } else if let Some(index) = self.x509_tui_list.state.selected() {
            if let Some(x509) = self.x509_tui_list.items.get(index) {
                let mut default_lines: Vec<Line> = Vec::from(x509.get_default_lines());
                lines.append(&mut default_lines);
            } else {
                lines.push(Line::from("Couldn't parse the X509 TUI item."));
            }
        } else {
            lines.push(Line::from("No X509 selected."));
        };

        let content: Text = Text::from(lines);

        let block: Block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::symmetric(2, 1))
            .border_style(Style::default().fg(Color::Black));

        Paragraph::new(content)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: false })
            .block(block)
            .render(area, buf);
    }
}
