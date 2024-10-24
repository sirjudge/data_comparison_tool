use std::{io, thread, time::Duration};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line,Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    DefaultTerminal, Frame,
};

pub(crate) fn run_terminal() -> io::Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw);
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

/// Calculate the layout of the UI elements.
///
/// Returns a tuple of the title area and the main areas.
fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>) {
    let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    let block_layout = Layout::vertical([Constraint::Max(4); 9]);
    let [title_area, main_area] = main_layout.areas(area);
    let main_areas = block_layout
        .split(main_area)
        .iter()
        .map(|&area| {
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area)
                .to_vec()
        })
        .collect();
    (title_area, main_areas)
}


/// Handles terminal UI window
fn draw(frame: &mut Frame) {
    let (title_area, main_areas) = calculate_layout(frame.size());

    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());


}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Block example. Press q to quit")
            .dark_gray()
            .alignment(Alignment::Center),
        area,
    );
}

