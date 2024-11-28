use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap, List, ListDirection},
    Frame,
};

#[derive(PartialEq)]
#[derive(Clone)]
enum UIState {
    Running,
    MainMenu,
    StartUp
}

pub(crate) fn run_terminal() -> io::Result<()> {
    // initialize terminal and state of the UI and set the state to main menu
    let mut terminal = ratatui::init();
    let mut state = UIState::StartUp;
    let mut previous_state = UIState::MainMenu;

    // until we see 'q' pressed, continue to render the UI
    loop {
        // Handle terminal startup intiialization
        if state == UIState::StartUp {
            state = UIState::MainMenu;
            terminal.draw(main_menu_draw)?;
        }
        // Handle terminal state change
        else if previous_state != state.clone() || state == UIState::StartUp {
            previous_state = state.clone();
            terminal.clear()?;
            match state {
                UIState::StartUp |
                UIState::MainMenu => {
                    terminal.draw(main_menu_draw)?;
                }
                UIState::Running => {
                    terminal.draw(running_draw)?;
                }
            }
        }
        // Else handle new terminal events based on state
        else {
            // if event is a key press read it
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match state {
                        UIState::MainMenu => {
                            match key.code {
                                KeyCode::Char('j') => {
                                    // TODO: make list go down?

                                },
                                KeyCode::Char('k') => {
                                    // TODO: make list go up?
                                },
                                KeyCode::Char('s') => {
                                    state = UIState::Running;
                                },
                                KeyCode::Char('q') => {
                                    println!("Quitting");
                                    break;
                                }
                                _ => println!("unrecognized Key pressed: {:?}", key.code),
                            }
                        }
                        UIState::Running => {
                            // pressing 's' will stop and take us back to the main menu
                            terminal.draw(running_draw)?;
                            if let Event::Key(key) = event::read()? {
                                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('s') {
                                    state = UIState::MainMenu;
                                }
                            }
                        }
                        _ => {
                            print_unrecognized_key();
                            break;
                        }
                    }
                }

            }
        }
    }

    // Post TUI run clean up by clearing terminal and returning Ok
    terminal.clear()?;
    Ok(())
}

fn print_unrecognized_key() {
    //TODO: Add some kind of user response for unkown keys
    println!("Unrecognized key pressed. Press 'q' to quit");
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

/// Handles the termina UI for the running state
/// of running the current data comparison
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

fn main_menu_draw(frame: &mut Frame) {
    // init possible items
    let items = ["Start", "Quit", "View Past Results"];

    // create widget
    let list = List::new (items)
        .block(Block::bordered().title("List"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    // render the list
    frame.render_widget(list, frame.area());
}

/*
/// Handles terminal UI window for the main menu
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
    write_to_output(frame, main_areas[1][0], "main menu");
}
*/

/// Renders the title of the terminal UI
fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Data Comparison Tool. Press q to quit")
            .bold()
            .white()
            .alignment(Alignment::Center),
        area,
    );
}

/// Writes test to the passed in frame and area
fn write_to_output(frame: &mut Frame, area: Rect, text: &str) {
    frame.render_widget(
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left)
            .style(Style::default().fg(Color::White)),
        area,
    );
}
