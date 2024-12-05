use std::io;
use crate::processor;
use crate::argument_parser;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap, List, ListDirection, ListState},
    Frame,
};

#[derive(PartialEq)]
#[derive(Clone)]
pub enum UIState {
    Running,
    MainMenu,
    StartUp
}

/// State management for the UI
static mut STATE: UIState = UIState::StartUp;
static mut PREV_STATE: UIState = UIState::StartUp;

pub fn set_state(state: UIState) {
    unsafe {
        STATE = state;
    }
}
pub fn get_state() -> UIState {
    unsafe {
        STATE.clone()
    }
}
pub fn get_prev_state() -> UIState {
    unsafe {
        PREV_STATE.clone()
    }
}
pub fn set_prev_state(state: UIState) {
    unsafe {
        PREV_STATE = state;
    }
}


pub(crate) fn run_terminal(args: &argument_parser::Arguments) -> io::Result<()> {
    // initialize terminal and state of the UI and set the state to main menu
    let mut terminal = ratatui::init();
    let mut previous_state = UIState::MainMenu;

    // until we see 'q' pressed, continue to render the UI
    loop {
        // Handle terminal startup intiialization
        if get_state() == UIState::StartUp {
            set_state(UIState::MainMenu);
            terminal.draw(draw_main_menu)?;
        }
        // Handle terminal state change
        else if get_prev_state() != get_state() {
            set_prev_state(get_state());
            terminal.clear()?;
            match get_state() {
                UIState::StartUp |
                UIState::MainMenu => {
                    terminal.draw(draw_main_menu)?;
                }
                UIState::Running => {
                    terminal.draw(draw_running)?;
                }
            }
        }
        // Else handle new terminal events based on state
        else {
            // if event is a key press read it
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match get_state() {
                        UIState::MainMenu => {
                            match key.code {
                                KeyCode::Up |
                                KeyCode::Char('j') => {
                                    // select widget and move selection up or down
                                },
                                KeyCode::Down |
                                KeyCode::Char('k') => {
                                    // TODO: make list go up?
                                },
                                KeyCode::Char('s') => {
                                    set_state(UIState::Running);
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
                            terminal.draw(draw_running)?;

                            // todo: once the comparison is done we need
                            // to display the results to the user
                            let comparison_data = processor::run_comparison(args);
                            println!("Comparison ran to completion");
                            set_state(UIState::MainMenu);
                        }
                        _ => {
                            println!("Unrecognized key pressed: {:?}", key.code);
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

fn draw_results_of_comparison(frame: &mut Frame){

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
fn draw_running(frame: &mut Frame) {
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

fn draw_main_menu(frame: &mut Frame) {
    // init possible items
    let items = ["[S]tart", "[Q]uit", "[V]iew Past Results"];

    // create widget
    let list = List::new (items)
        .block(Block::bordered().title("Menu options"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    // render the list
    let mut state = ListState::default();
    frame.render_stateful_widget(list, frame.area(), &mut state);
}

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
