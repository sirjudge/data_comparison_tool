use crate::{
    interface::{
        state::{
            UIState,
            get_string_from_state
        },
        log::Log,
        argument_parser,
    },
    models::comparison_data::ComparisonData,
    processor,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style, Stylize},
    widgets::{
        Block, List, ListDirection, ListState, Paragraph, Row, Table, Wrap,
    },
    Frame,
};
use std::{io, io::Stdout};

/// State management for the UI
/// I'm aware this may not be the best way to do this
/// but as a wise sage once said "I'm just a girl trying to do her best"
static mut CURRENT_STATE: UIState = UIState::StartUp;
static mut PREVIOUS_STATE: UIState = UIState::StartUp;
static mut COMPARISON_DATA: Option<ComparisonData> = None;

pub fn set_state(state: UIState, log: &Log) {
    // set the current state as the previous state and set the current state
    // to whatever is passed in
    unsafe {
        // log the state change
        let current_state_string = get_string_from_state(get_state());
        let new_state_string = get_string_from_state(state.clone());
        let log_message = format!("Setting state to: {:?} from: {:?}",new_state_string,current_state_string);
        log.debug(&log_message);

        // current state becomes previous state
        PREVIOUS_STATE = get_state();

        // passed in state becomes the current state
        CURRENT_STATE = state;
    }
}

pub fn get_state() -> UIState {
    unsafe {
        CURRENT_STATE.clone()
    }
}

pub fn get_prev_state() -> UIState {
    unsafe {
        PREVIOUS_STATE.clone()
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
            Some(data) => {
                Some(data)
            }
            None => {
                None
            }
        }
    }
}

/// Handle the rendering of the terminal UI based on the current state
/// of the UI.
fn draw_and_handle_state(
    terminal: &mut ratatui::Terminal<CrosstermBackend<Stdout>>,
    log: &Log,
    args: &argument_parser::Arguments,
) -> Result<(), std::io::Error> {
    // if state is startup, do start up stuff
    if get_state() == UIState::StartUp {
        log.debug("performing terminal initialization tasks");
        set_state(UIState::MainMenu, log);
        terminal.draw(draw_main_menu)?;
        return Ok(());
    }

    // if the previous state and current state are the same
    if get_prev_state() == get_state() {
        let log_message = format!(
            "no state change detected, returning ok. Current state: {}",
            get_string_from_state(get_state())
        );
        log.debug(&log_message);
        return Ok(());
    }

    // generate log message that state has changed
    log.debug(
        &format!(
            "State change detected, pev_state:{} current_state:{}",
            get_string_from_state(get_state()),
            get_string_from_state(get_prev_state())
        )
    );

    // set the previous state to the current state,
    terminal.clear()?;

    // match on the current state and render the appropriate new UI
    match get_state() {
        UIState::StartUp | UIState::MainMenu => {
            log.debug("performing terminal initialization tasks");
            terminal.draw(draw_main_menu)?;
        }
        UIState::Running => {
            terminal.draw(draw_running)?;

            // TOOD: This should be done in draw_running but is done
            // here to avoid lifetime and ownership conflictions
            let comparison_data = processor::run_comparison(args, log);
            set_comparison_data(comparison_data);
            log.debug("comparison complete, setting state to results");
            set_state(UIState::Results, log);
            terminal.clear()?;
            terminal.draw(draw_results)?;
        }
        UIState::Results => {
            terminal.draw(draw_results)?;
        }
        UIState::TearDown => {
            log.debug("Tearing down terminal and quitting");
            terminal.clear()?;
        }
    }

    // finally return that we've processed ok
    Ok(())
}

fn handle_main_menu_keys(key: KeyCode, log: &Log) {
    match key {
        KeyCode::Char('s') => {
            set_state(UIState::Running, log);
        }
        KeyCode::Char('q') => {
            set_state(UIState::TearDown, log);
        }
        _ => {
            log.warn(&format!(
                "unrecognized main menu selection Key pressed: {:?}",
                key
            ));
        }
    }
}

fn runtime_key_events(key: KeyCode, log: &Log) {
    match key {
        KeyCode::Char('q') => {
            set_state(UIState::TearDown, log);
        }
        _ => {
            log.warn(&format!("unrecognized runtime menu Key pressed: {:?}", key));
        }
    }
}

fn result_key_events(key: KeyCode, log: &Log) {
    match key {
        KeyCode::Char('q') => {
            set_state(UIState::TearDown, log);
        }
        KeyCode::Char('m') => {
            set_state(UIState::MainMenu, log);
        }
        _ => {
            log.warn(&format!("unrecognized results menu Key pressed: {:?}", key));
        }
    }
}

/// Initialize the terminal UI, run start up tasks, and then display
/// the main menu to the user
pub fn run_terminal(args: &argument_parser::Arguments, log: &Log) -> io::Result<()> {
    // initialize terminal and state of the UI and set the state to main menu
    let mut terminal = ratatui::init();
    log.debug("ratatui Terminal initialized");
    //BUG: the following key presses cause a crash from a table already existing
    //Main menu > running > results > main menu > running
    // this is probably an error with the table name for either
    // mysql or sqlite not being re-initialized
    loop {
        // handle and render the current state and after the state has changed hanlde key events
        match draw_and_handle_state(&mut terminal, log, args) {
            Ok(()) => {
                log.info(&format!("current state: {:?}",
                    get_string_from_state(get_state())
                ));
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match get_state() {
                            UIState::MainMenu => {
                                handle_main_menu_keys(key.code, log);
                            }
                            UIState::Running => {
                                log.info("running state key press detected");
                                runtime_key_events(key.code, log);
                            }
                            UIState::Results => {
                                result_key_events(key.code, log);
                            }
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

        // if we're still in the tear down state at the end of the loop
        // break and finish execution
        if get_state() == UIState::TearDown {
            break;
        }
    }

    // Post TUI run clean up by clearing terminal and returning Ok
    terminal.clear()?;
    Ok(())
}

/// handle rendering of the comparison results in a nice little
/// table
fn draw_results(frame: &mut Frame) {
    // create widget data
    let comparison_data = get_comparison_data().unwrap();
    let unique_table_1_rows_str = comparison_data.unique_table_1_rows.len().to_string();
    let unique_table_2_rows_str = comparison_data.unique_table_2_rows.len().to_string();
    let changed_rows_str = comparison_data.changed_rows.len().to_string();

    // initialize the rows of the table
    let rows = [
        Row::new(vec!["Results:"]),
        Row::new(vec!["Unique Table 1 rows", &unique_table_1_rows_str]),
        Row::new(vec!["Unique Table 2 rows", &unique_table_2_rows_str]),
        Row::new(vec!["Changed rows", &changed_rows_str]),
        Row::new(vec!["Press [q] to exit"]),
        Row::new(vec!["Press [m] to return to the main menu"]),
    ];

    // set column widths
    let column_1_width = Constraint::Length(20);
    let column_2_width = Constraint::Length(20);
    let widths = [column_1_width, column_2_width];

    // generate the table widget
    let table_widget =
        Table::new(rows, widths)
        .block(Block::default());

    frame.render_widget(table_widget, frame.area());
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
    frame.render_widget(
        Paragraph::new("Data Comparison Tool. Press q to quit")
            .bold()
            .white()
            .alignment(Alignment::Center),
        title_area
    );

    let text_widget = Paragraph::new("Running comparison")
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));
    frame.render_widget(text_widget, main_areas[0][0]);
}

/// Render the main menu of the terminal UI
fn draw_main_menu(frame: &mut Frame) {
    // init possible items
    let items = ["[S]tart", "[Q]uit"];

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
