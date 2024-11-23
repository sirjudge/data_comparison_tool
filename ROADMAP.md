# Current To-dos
## Terminal User Interface (TUI)
### Description
Adds a TUI using [Ratatui](https://ratatui.rs/https://ratatui.rs/) to give
the user a visual representation of the work being done.
### Definition of Done
- Allows user to pass in a flag to the CLI that allows a TUI to launch
- TUI has the following options to select options when in the main menu
    - *[S]tart*:
    - *[Q]uit*:
    - *[M]ain menu*:

# Road-map
## New Input types
- Add [tiberius](https://docs.rs/tiberius/latest/tiberius/) crate and relevant
logic for MSSQL support as sqlx only has support for PostgreSQL,MySQL,
MariaDB, and sqlite.
- CSV files
## Comparison logic refresh
- Add custom column mapping
- Add data filtration to further refine comparison
## Output file type support
- Add support to export comparison data in formats other than a csv or sqlite files
## Test History
- Add a test history portion to the UI and CLI flags that allow the user
to View the currently saved history of preveiously run test configurations
- Allow the selection and re-running of a previously ran configuration
## Test configuration + Test History
- Add support for `.toml` file test configuration instead of CLI flags or TUI selections
to be able to save and load test configurations to improve scriptablity.
- Add support for viewing previous run data
