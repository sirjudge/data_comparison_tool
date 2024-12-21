pub mod datastore {
    pub mod mysql;
    pub mod sqlite;
    pub mod csv;
    pub mod generator;
    pub mod transformer;
}

pub mod models {
    pub mod comparison_data;
    pub mod table_data;
}

pub mod interface {
    pub mod argument_parser;
    pub mod tui;
    pub mod state;
    pub mod log_options;
    pub mod log;
}

pub mod processor;

