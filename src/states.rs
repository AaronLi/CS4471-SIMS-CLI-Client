#[derive(Debug)]
pub(crate) enum SimsClientState {
    Unauthenticated{password: String, error_message: Option<String>},
    Authenticating,
    InventoryView,
    AwaitDatabaseConfirmation,
    DisplaySuggestions{suggestion: String}
}