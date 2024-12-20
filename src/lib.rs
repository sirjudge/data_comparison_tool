pub mod database {
    pub mod mysql;
    pub mod sqlite;
}

pub mod models {
    pub mod argument_parser;
    pub mod comparison_data;
    pub mod table_data;
}

pub mod data_creator;
pub mod data_exporter;
pub mod data_querier;
pub mod log;
pub mod processor;
