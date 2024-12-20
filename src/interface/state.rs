
#[derive(PartialEq, Clone)]
pub enum UIState {
    Running,
    MainMenu,
    StartUp,
    Results,
    TearDown,
}

pub(crate) fn get_string_from_state(state: UIState) -> String {
    match state {
        UIState::MainMenu => "Main Menu".to_string(),
        UIState::Running => "Running".to_string(),
        UIState::Results => "Results".to_string(),
        UIState::TearDown => "Tear Down".to_string(),
        UIState::StartUp => "Start Up".to_string(),
    }
}
