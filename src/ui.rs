use crate::{argument_parser,
    data_comparer::ComparisonData,
    processor,
    log::Log
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListDirection, ListState, Paragraph, Wrap},
    Frame,
};
use std::{io, io::Stdout};

#[derive(PartialEq, Clone)]
pub enum UIState {
    Running,
    MainMenu,
    StartUp,
    Results,
    TearDown,
}

/// State management for the UI
/// I'm aware this may not be the best way to do this
/// but as a wise sage once said "I'm just a girl trying to do her best"
static mut STATE: UIState = UIState::StartUp;
static mut PREV_STATE: UIState = UIState::StartUp;
static mut COMPARISON_DATA: Option<ComparisonData> = None;

pub fn set_state(state: UIState) {
    unsafe {
        STATE = state;
    }
}
pub fn get_state() -> UIState {
    unsafe { STATE.clone() }
}
pub fn get_prev_state() -> UIState {
    unsafe { PREV_STATE.clone() }
}
pub fn set_prev_state(state: UIState) {
    unsafe { PREV_STATE = state; }
}
pub fn set_comparison_data(data: ComparisonData) {
    unsafe { COMPARISON_DATA = Some(data); }
}

pub fn get_comparison_data() -> Option<&'static ComparisonData> {
    unsafe {
        // TODO: the rust-analyzer suggests that we should use addr_or!()
        // here instead to create a raw pointer
        // doing that would require changing the signature of this
        // which then causes the fact that ComparisonData does not have the
        // clone or copy trait on the sqlx types
        // so for now we'll just ignore this for now
        match &COMPARISON_DATA {
            Some(data) => Some(data),
            None => None,
        }
    }
}

fn handle_state(terminal: &mut ratatui::Terminal<CrosstermBackend<Stdout>>, log: &Log) -> Result<(), std::io::Error> {
    // if state is startup, do start up stuff
    // else Handle terminal if we've changed state
    if get_state() == UIState::StartUp {
        set_state(UIState::MainMenu);
        terminal.draw(draw_main_menu)?;
    } else if get_prev_state() != get_state() {
        // set the previous state to the current state,
        // clear the terminal, and draw the new state
        set_prev_state(get_state());
        //terminal.clear()?;
        match get_state() {
            UIState::StartUp | UIState::MainMenu => {
                terminal.draw(draw_main_menu)?;
            }
            UIState::Running => {
                terminal.draw(draw_running)?;
            }
            UIState::Results => {
                terminal.draw(draw_results)?;
            }
            UIState::TearDown => {
                log.info("Tearing down terminal and quitting");
                terminal.clear()?;
                return Ok(());
            }
        }
    }

    Ok(())
}


/// Initialize the terminal UI, run start up tasks, and then display
/// the main menu to the user
pub(crate) fn run_terminal(args: &argument_parser::Arguments, log: &Log) -> io::Result<()> {
    // initialize terminal and state of the UI and set the state to main menu
    let mut terminal = ratatui::init();

    // ensure we correctly handle the state
    match handle_state(&mut terminal, log) {
        Ok(()) => {
            log.info("State handled successfully");
            loop {
                // if event is a key press and it's pressed down
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match get_state() {
                            UIState::MainMenu => {
                                //TODO: handle menu navigation using vim
                                //and arrow keys
                                match key.code {
                                    KeyCode::Char('s') => {
                                        set_state(UIState::Running);
                                    }
                                    KeyCode::Char('q') => {
                                        set_state(UIState::TearDown);
                                        break;
                                    }
                                    _ => {
                                        let log_string = format!("unrecognized Key pressed: {:?}", key.code);
                                        log.info(&log_string);
                                    }
                                }
                            }
                            UIState::Running => {
                                let result = draw_and_render_comparison(&mut terminal, args, log);
                                match result {
                                    Ok(()) => {
                                        log.info("Comparison complete, displaying results");
                                        set_state(UIState::Results);
                                    }
                                    Err(e) => {
                                        let log_string = format!("Error running comparison: {:?}", e);
                                        log.info(&log_string);
                                        set_state(UIState::MainMenu);
                                    }
                                }
                            }
                            UIState::Results => match key.code {
                                KeyCode::Char('q') => {
                                    terminal.clear()?;
                                    terminal.draw(draw_results)?;
                                    set_state(UIState::TearDown);
                                }
                                _ => {
                                    let log_string = format!("unrecognized Key pressed: {:?}", key.code);
                                    log.info(&log_string);
                                }
                            },
                            _ => {
                                let log_string = format!("unrecognized Key pressed: {:?}", key.code);
                                log.info(&log_string);
                                break;
                            }

                        }
                    }
                }
            }
        }
        Err(e) => {
            let log_string = format!("Error handling state: {:?}", e);
            log.error(&log_string);
            // return Err(e);
        }
    }

    // Post TUI run clean up by clearing terminal and returning Ok
    terminal.clear()?;
    Ok(())
}

fn draw_results(frame: &mut Frame) {
    let (title_area, main_areas) = calculate_layout(frame.area());
    render_title(frame, title_area);
    let text = Text::raw("Results");
    frame.render_widget(text, frame.area());
    let widget = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .style(Style::default().bg(Color::Black));

    let comparison_data = get_comparison_data().unwrap();
    let unique_table_1_rows_str = comparison_data.unique_table_1_rows.len().to_string();
    let unique_table_2_rows_str = comparison_data.unique_table_2_rows.len().to_string();

    frame.render_widget(widget, main_areas[0][0]);
    write_to_output(frame, main_areas[1][0], "Results");
    write_to_output(frame, main_areas[2][0], "unique rows in table 1");
    write_to_output(frame, main_areas[2][1], &unique_table_1_rows_str);
    write_to_output(frame, main_areas[3][0], "unique rows in table 2");
    write_to_output(frame, main_areas[3][1], &unique_table_2_rows_str);
}

fn draw_and_render_comparison(
    terminal: &mut ratatui::Terminal<CrosstermBackend<Stdout>>,
    args: &argument_parser::Arguments,
    log: &Log
) -> Result<(), std::io::Error> {
    // pressing 's' will stop and take us back to the main menu
    let _ = log.info("Running comparison");
    terminal.draw(draw_running)?;
    let comparison_data = processor::run_comparison(args, log);
    set_comparison_data(comparison_data);
    let _ = log.info("Displaying results");
    terminal.draw(draw_results)?;
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

/// Render the main menu of the terminal UI
fn draw_main_menu(frame: &mut Frame) {
    // init possible items
    let items = ["[S]tart", "[Q]uit", "[V]iew Past Results"];

    // create widget
    let list = List::new(items)
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

