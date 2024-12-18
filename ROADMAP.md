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
- TUI displays results and allowing user to return to the main menu or quit the  program
# Road-map ( in no particular order of importance)
## More expressive TUI
- MVP of the TUI is to lay the  framework of a more fully featured interface. Currently very limited in what it can do.
- Add scroll support with arrow and vim keys to when in a scroll list element
## Update logging output
- Need to standardize the programs logging to a single struct
- The base exists but due to needing the program args which is not archiectured to derive Clone or Copy and thus can not be passed in the code in it's current state
- also should add more proper  support for logging to a  more appropriate file location. This should should be handled by OS with linux support initally and windows support later
## Improve ease of configuration
- Some of the application flags can be grouped into smaller structs and passed more efficiently. The Arguments struct found in argument_parser.rs should be refactored to clean that up and modularize whatever makes sense (probably at least some of the input and output settings).
- this will allow greater ease of maintenance for supporting .toml configurations
as well as environment variables or CLI flags
## New Input types
- Add [tiberius](https://docs.rs/tiberius/latest/tiberius/) crate and relevant
logic for MSSQL support as sqlx only has support for PostgreSQL,MySQL,
MariaDB, and sqlite.
- CSV files
## Add custom column mapping support
- currently the comparison does not support comparing two columns with different names but is something that would be a good feature to support.
- This should be done by allowing a list of tuple
- Implementation is flexible
## Comparison logic refresh
- Add custom column mapping
- Add data filtration to further refine comparison
## Output file type support
- Add support to export comparison data in formats other than a csv or sqlite files
- consider adding support for an eport back to a supported database
## Test History
- Add a test history portion to the UI and CLI flags that allow the user
to View the currently saved history of preveiously run test configurations
- Allow the selection and re-running of a previously ran configuration
## Test configuration + Test History
- Add support for `.toml` file test configuration instead of CLI flags or TUI selections
to be able to save and load test configurations to improve scriptablity.
- Add support for viewing previous run data
