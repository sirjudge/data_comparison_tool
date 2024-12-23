#[derive(Clone)]
pub enum LogVerbosity {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3
}

#[derive(Clone,serde::Serialize,serde::Deserialize)]
pub enum LogOutput {
    StdOut,
    File,
    Console
}

