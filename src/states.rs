#[derive(Debug)]
pub(crate) enum SimsClientState {
    Unauthenticated{password: String},
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