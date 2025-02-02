use color_eyre::eyre::Error;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};
use x509_parser::parse_x509_certificate;

use crate::{x509::X509, x509_tui::X509TUIList};

pub struct PKIExplorerApp {
    done: bool,
    x509_tui_list: X509TUIList,
}

impl Widget for &mut PKIExplorerApp {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [list_area, content_area] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(50)])
                .areas(area);

        self.render_list_area(list_area, buf);
        self.render_content_area(content_area, buf);
    }
}

impl PKIExplorerApp {
    pub fn new(x509s: Vec<X509>) -> Self {
        Self {
            done: Default::default(),
            x509_tui_list: X509TUIList::new(x509s),
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

    fn render_list_area(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .padding(Padding::uniform(1))
            .borders(Borders::RIGHT);

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

    fn render_content_area(&self, area: Rect, buf: &mut Buffer) {
        let content: String = if let Some(index) = self.x509_tui_list.state.selected() {
            if let Some(x509) = self.x509_tui_list.items.get(index) {
                match parse_x509_certificate(&x509.pem.contents) {
                    Ok((_, certificate)) => certificate.subject.to_string(),
                    Err(_) => "Couldn't parse the X509 file.".to_string(),
                }
            } else {
                "Couldn't parse the X509 path.".to_string()
            }
        } else {
            "No x509 selected".to_string()
        };

        let block = Block::default()
            .title_alignment(Alignment::Center)
            .title("X509 Content");

        Paragraph::new(content)
            .style(Style::default().fg(Color::White))
            .block(block)
            .render(area, buf);
    }
}
