use crate::{
    argument_parser,
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
    widgets::{Block, BorderType, Borders, List, ListDirection, ListState, Paragraph, Row,Table, Wrap},
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
    // set the current state as the previous state and set the current state
    // to whatever is passed in
    unsafe {
        PREV_STATE = get_state();
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

pub fn set_comparison_data(data: ComparisonData) {
    unsafe {
        COMPARISON_DATA = Some(data);
    }
}

// TODO: the rust-analyzer suggests that we should use addr_or!()
// here instead to create a raw pointer
// doing that would require changing the signature of this
// which then causes the fact that ComparisonData does not have the
// clone or copy trait on the sqlx types
// so for now we'll just ignore this for now
pub fn get_comparison_data() -> Option<&'static ComparisonData> {
    unsafe {
        match &COMPARISON_DATA {
            Some(data) => Some(data),
            None => None,
        }
    }
}

fn get_string_from_state(state: UIState) -> String {
    match state {
        UIState::MainMenu => "Main Menu".to_string(),
        UIState::Running => "Running".to_string(),
        UIState::Results => "Results".to_string(),
        UIState::TearDown => "Tear Down".to_string(),
        UIState::StartUp => "Start Up".to_string(),
    }
}

fn draw_and_handle_state(terminal: &mut ratatui::Terminal<CrosstermBackend<Stdout>>, log: &Log, args: &argument_parser::Arguments) -> Result<(), std::io::Error> {
    // if state is startup, do start up stuff
    // else Handle terminal if we've changed state
    if get_state() == UIState::StartUp {
        log.info("performing terminal initialization tasks");
        set_state(UIState::MainMenu);
        terminal.draw(draw_main_menu)?;
        return Ok(());
    }

    if get_prev_state() == get_state() {
        let log_message = format!("no state change detected, returning ok. Current state: {}", get_string_from_state(get_state()));
        log.info(&log_message);
        return Ok(());
    }

    let log_message = format!("State change detected, pev_state:{} current_state:{}", get_string_from_state(get_state()), get_string_from_state(get_prev_state()));
    log.info(&log_message);
    // set the previous state to the current state,
    terminal.clear()?;
    match get_state() {
        UIState::StartUp | UIState::MainMenu => {
            log.info("performing terminal initialization tasks");
            terminal.draw(draw_main_menu)?;
        }
        UIState::Running => {
            let result = draw_and_render_comparison(terminal, args, log);
            match result {
                Ok(()) => {
                    log.info("Comparison ran successfully");
                    set_state(UIState::Results);
                }
                Err(e) => {
                    log.error(&format!("Error running comparison: {:?}", e));
                }
            }
        }
        UIState::Results => {
            terminal.draw(draw_results)?;
        }
        UIState::TearDown => {
            log.info("Tearing down terminal and quitting");
            terminal.clear()?;
        }
    }

    // finally return that we've processed ok
    Ok(())
}

fn handle_main_menu_keys (key: KeyCode, log: &Log) {
    match key {
        KeyCode::Char('s') => {
            set_state(UIState::Running);
            log.info("setting state to running from main menu key press");
        }
        KeyCode::Char('q') => {
            set_state(UIState::TearDown);
            log.info("setting state to tear down from main menu key press");
        }
        _ => {
            log.info(&format!("unrecognized main menu selection Key pressed: {:?}", key));
        }
    }
}

fn runtime_key_events(key: KeyCode, log: &Log) {
    match key {
        KeyCode::Char('q') => {
            set_state(UIState::TearDown);
            log.info("setting state to tear down from runtime menu key press");
        }
        _ => {
            log.warn(&format!("unrecognized runtime menu Key pressed: {:?}", key));
        }
    }
}

fn result_key_events(key: KeyCode, log: &Log) {
    match key {
        KeyCode::Char('q') => {
            set_state(UIState::TearDown);
            log.info("setting state to tear down from results menu key press");
        }
        _ => {
            log.warn(&format!("unrecognized results menu Key pressed: {:?}", key));
        }
    }
}

/// Initialize the terminal UI, run start up tasks, and then display
/// the main menu to the user
pub(crate) fn run_terminal(args: &argument_parser::Arguments, log: &Log) -> io::Result<()> {
    // initialize terminal and state of the UI and set the state to main menu
    let mut terminal = ratatui::init();
    log.info("ratatui Terminal initialized");

    loop {
        // handle and render the current state and after the state has changed hanlde key events
        match draw_and_handle_state(&mut terminal, log, args) {
            Ok(()) => {
                log.info(&format!("State handled successfully: {:?}", get_string_from_state(get_state())));
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match get_state() {
                            UIState::MainMenu => {
                                handle_main_menu_keys(key.code, log);
                            }
                            UIState::Running => {
                                runtime_key_events(key.code, log);
                            }
                            UIState::Results => {
                                result_key_events(key.code, log);
                            },
                            _ => {
                                log.warn(&format!("unrecognized Key pressed: {:?}", key.code));
                            }

                        }
                    }
                }
            }
            Err(e) => {
                let log_string = format!("Error handling state: {:?}", e);
                log.error(&log_string);
            }
        }
        if get_state() == UIState::TearDown {
            // if we're still in the tear down state at the end of the loop just break and finish
            // execution
            break;
        }
    }

    // Post TUI run clean up by clearing terminal and returning Ok
    terminal.clear()?;
    Ok(())
}

fn draw_results(frame: &mut Frame) {
    // get the title area and main area of the terminal layout
    let (title_area, main_areas) = calculate_layout(frame.area());

    // render the title of the widget
    render_title(frame, title_area);

    // create widget data
    let comparison_data = get_comparison_data().unwrap();
    let unique_table_1_rows_str = comparison_data.unique_table_1_rows.len().to_string();
    let unique_table_2_rows_str = comparison_data.unique_table_2_rows.len().to_string();
    let rows = [
        Row::new(vec!["Results:"]),
        Row::new(vec!["Unique Table 1 rows", &unique_table_1_rows_str]),
        Row::new(vec!["Unique Table 2 rows", &unique_table_2_rows_str]),
    ];

    //TODO: maybe do this smarter? slapped length of column
    //text to be arbitrary for now but might be able to always set to longest
    // data point and add a couple extra characters for padding
    let widths = [Constraint::Length(10), Constraint::Length(10)];
    let table_widget = Table::new(rows,widths)
        .block(Block::default())
        .highlight_symbol(">>")
        .row_highlight_style(Style::new().on_blue());

    frame.render_widget(table_widget, main_areas[0][0]);
}

fn draw_and_render_comparison(
    terminal: &mut ratatui::Terminal<CrosstermBackend<Stdout>>,
    args: &argument_parser::Arguments,
    log: &Log
) -> Result<(), std::io::Error> {
    // pressing 's' will stop and take us back to the main menu
    log.info("Running comparison");
    terminal.draw(draw_running)?;

    let comparison_data = processor::run_comparison(args, log);
    set_comparison_data(comparison_data);
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

