#[derive(Debug)]
pub(crate) enum SimsClientState {
    Unauthenticated{password: String, error_message: Option<String>},
    Authenticating,
    AutomaticViewSelection,
    AllItemView,
    ShelfItemView,
    ShelfView,
    ManageShelf,
    ManageItem,
    AwaitDatabaseConfirmation,
    DisplaySuggestions{suggestion: String}
}