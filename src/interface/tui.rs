use crate::{
    interface::{
        state::{
            UIState,
            get_string_from_state
        },
        log::Log,
        config::Config
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

/// First, let's create a new struct to hold our UI state
#[derive(Clone)]
pub struct TuiState {
    current_state: UIState,
    previous_state: UIState,
    comparison_data: Option<ComparisonData>,
}

impl TuiState {
    pub fn new() -> Self {
        Self {
            current_state: UIState::StartUp,
            previous_state: UIState::StartUp,
            comparison_data: None,
        }
    }

    pub fn set_state(&mut self, new_state: UIState, log: &Log) {
        let log_message = format!(
            "Setting state to: {:?} from: {:?}",
            get_string_from_state(new_state.clone()),
            get_string_from_state(self.current_state.clone())
        );
        log.debug(&log_message);

        self.previous_state = self.current_state.clone();
        self.current_state = new_state;
    }

    pub fn get_state(&self) -> UIState {
        self.current_state.clone()
    }

    pub fn get_prev_state(&self) -> UIState {
        self.previous_state.clone()
    }

    pub fn set_comparison_data(&mut self, data: ComparisonData) {
        self.comparison_data = Some(data);
    }

    pub fn get_comparison_data(&self) -> Option<&ComparisonData> {
        self.comparison_data.as_ref()
    }

    pub fn has_state_changed(&self) -> bool {
        self.current_state != self.previous_state
    }
}

/// Handle the rendering of the terminal UI based on the current state
/// of the UI.
fn draw_and_handle_state(
    terminal: &mut ratatui::Terminal<CrosstermBackend<Stdout>>,
    log: &Log,
    config: &Config,
    state: &mut TuiState,
) -> Result<(), std::io::Error> {
    // if state is startup, do start up stuff
    if state.get_state() == UIState::StartUp {
        log.debug("performing terminal initialization tasks");
        state.set_state(UIState::MainMenu, log);
        terminal.draw(|f| draw_main_menu(f, state))?;
        return Ok(());
    }

    // if no state change, return early
    if !state.has_state_changed() {
        let log_message = format!(
            "no state change detected, returning ok. Current state: {}",
            get_string_from_state(state.get_state())
        );
        log.debug(&log_message);
        return Ok(());
    }

    // generate log message that state has changed
    log.debug(
        &format!(
            "State change detected, pev_state:{} current_state:{}",
            get_string_from_state(state.get_state()),
            get_string_from_state(state.get_prev_state())
        )
    );

    // set the previous state to the current state,
    terminal.clear()?;

    // match on the current state and render the appropriate new UI
    match state.get_state() {
        UIState::StartUp | UIState::MainMenu => {
            log.debug("performing terminal initialization tasks");
            terminal.draw(|f| draw_main_menu(f, state))?;
        }
        UIState::Running => {
            terminal.draw(|f| draw_running(f))?;

            // TODO: This should be done in draw_running but is done
            // here to avoid lifetime and ownership conflictions
            let comparison_data =
                processor::run(config, log);
            state.set_comparison_data(comparison_data);
            log.debug("comparison complete, setting state to results");
            state.set_state(UIState::Results, log);
            terminal.clear()?;
            terminal.draw(|f| draw_results(f, state))?;
        }
        UIState::Results => {
            terminal.draw(|f| draw_results(f, state))?;
        }
        UIState::TearDown => {
            log.debug("Tearing down terminal and quitting");
            terminal.clear()?;
        }
    }

    // finally return that we've processed ok
    Ok(())
}

fn handle_main_menu_keys(key: KeyCode, log: &Log, state: &mut TuiState) {
    match key {
        KeyCode::Char('s') => {
            state.set_state(UIState::Running, log);
        }
        KeyCode::Char('q') => {
            state.set_state(UIState::TearDown, log);
        }
        _ => {
            log.warn(&format!(
                "unrecognized main menu selection Key pressed: {:?}",
                key
            ));
        }
    }
}

fn runtime_key_events(key: KeyCode, log: &Log, state: &mut TuiState) {
    match key {
        KeyCode::Char('q') => {
            state.set_state(UIState::TearDown, log);
        }
        _ => {
            log.warn(&format!("unrecognized runtime menu Key pressed: {:?}", key));
        }
    }
}

fn result_key_events(key: KeyCode, log: &Log, state: &mut TuiState) {
    match key {
        KeyCode::Char('q') => {
            state.set_state(UIState::TearDown, log);
        }
        KeyCode::Char('m') => {
            state.set_state(UIState::MainMenu, log);
        }
        _ => {
            log.warn(&format!("unrecognized results menu Key pressed: {:?}", key));
        }
    }
}

/// Initialize the terminal UI, run start up tasks, and then display
/// the main menu to the user
pub fn run_terminal(config: &Config, log: &Log) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut tui_state = TuiState::new();
    log.debug("ratatui Terminal initialized");

    loop {
        match draw_and_handle_state(&mut terminal, log, config, &mut tui_state) {
            Ok(()) => {
                log.info(&format!("current state: {:?}",
                    get_string_from_state(tui_state.get_state())
                ));
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match tui_state.get_state() {
                            UIState::MainMenu => {
                                handle_main_menu_keys(key.code, log, &mut tui_state);
                            }
                            UIState::Running => {
                                log.info("running state key press detected");
                                runtime_key_events(key.code, log, &mut tui_state);
                            }
                            UIState::Results => {
                                result_key_events(key.code, log, &mut tui_state);
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

        if tui_state.get_state() == UIState::TearDown {
            break;
        }
    }

    terminal.clear()?;
    Ok(())
}

/// handle rendering of the comparison results in a nice little
/// table
fn draw_results(frame: &mut Frame, state: &TuiState) {
    if let Some(comparison_data) = state.get_comparison_data() {
        // create widget data
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
fn draw_main_menu(frame: &mut Frame, state: &TuiState) {
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
