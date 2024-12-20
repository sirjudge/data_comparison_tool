pub mod database {
    pub mod mysql;
    pub mod sqlite;
}

pub mod models {
    pub mod comparison_data;
    pub mod table_data;
}

pub mod interface {
    pub mod argument_parser;
    pub mod ui;
    pub mod state;
    pub mod log_options;
    pub mod log;
}

pub mod data_creator;
pub mod data_exporter;
pub mod data_querier;
pub mod processor;

