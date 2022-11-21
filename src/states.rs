#[derive(Debug)]
pub(crate) enum SimsClientState {
    Unauthenticated{password: String, error_message: Option<String>},
    Authenticating,
    InventoryView,
    ManageShelf,
    ManageItem,
    AwaitDatabaseConfirmation,
    DisplaySuggestions{suggestion: String}
}