pub(crate) enum SimsClientState {
    Unauthenticated{username: String, password: String},
    Authenticating{username: String, password: String},
    AutomaticViewSelection,
    AllItemView,
    ShelfItemView,
    ShelfView,
    ManageShelf,
    ManageItem,
    AwaitDatabaseConfirmation,
    DisplaySuggestions{suggestion: String}
}