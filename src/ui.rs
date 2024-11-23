use std::io;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize },
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

#[derive(PartialEq)]
#[derive(Clone)]
enum UIState {
    Running,
    Stopped,
    MainMenu
}

pub(crate) fn run_terminal() -> io::Result<()> {
    // initialize terminal and state of the UI and set the state to main menu
    let mut terminal = ratatui::init();
    let mut state = UIState::MainMenu;
    let mut previous_state = UIState::MainMenu;

    // until we see 'q' pressed, continue to render the UI
    loop {
        // if we've changed state clear terminal and redraw
        if previous_state != state.clone(){
            previous_state = state.clone();
            terminal.clear()?;

            // Render the current state we're in
            match state {
                UIState::MainMenu => {
                    terminal.draw(main_menu_draw)?;
                }
                UIState::Running => {
                    terminal.draw(running_draw)?;
                }
                UIState::Stopped => {
                    break;
                }
            }
        }
        // else we match the current state and handle events passed in
        // Allows the following key presses:
        // 'q' to quit
        // 's' to start the program
        else {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    state = UIState::Stopped;
                }
            }

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('s') {
                    state = UIState::Running;
                }
            }
        }
    }

    // Post TUI run clean up by clearing terminal and returning Ok
    terminal.clear()?;
    Ok(())
}


/// Calculate the layout of the UI elements.
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


fn running_draw(frame: &mut Frame) {
    let (title_area, main_areas) = calculate_layout(frame.area());
    render_title(frame, title_area);
    let text = Text::raw("Run time log");
    frame.render_widget(text, frame.area());
    let widget = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(widget, main_areas[0][0]);
    write_to_output(frame, main_areas[1][0], "Running");
}

/// Handles terminal UI window
fn main_menu_draw(frame: &mut Frame) {
    let (title_area, main_areas) = calculate_layout(frame.area());
    render_title(frame, title_area);
    let text = Text::raw("Output Window");
    frame.render_widget(text, frame.area());
    let widget = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(widget, main_areas[0][0]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Data Comparison Tool. Press q to quit")
            .bold()
            .white()
            .alignment(Alignment::Center),
        area,
    );
}

fn write_to_output(frame: &mut Frame, area: Rect, text: &str) {
    frame.render_widget(
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left)
            .style(Style::default().fg(Color::White)),
        area,
    );
}
