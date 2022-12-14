use crate::assets::get_icon;
use crate::frontend::EditTarget::{NewItem, NewShelf};
use crate::frontend::{create_tab, EditTarget, TabId};
use crate::styles::Fab;
use crate::ui_messages::Message;
use crate::ui_messages::Message::{StartEditing, StopEditing, UpdateItems, UpdateShelves};
use crate::{assets, ui_messages, ClientState};
use iced::widget::image as iced_image;
use iced::widget::{
    button, column, row, text, Button, Column, Container, Image, Row, Rule, Scrollable, Space,
    Text, TextInput,PickList
};
use iced::Length;
use iced::Length::{Fill, Shrink};
use iced::{theme, Element};
use iced_aw::floating_element::FloatingElement;
use iced_aw::{Card, Modal};
use std::iter;

pub(crate) fn unauthenticated_view<'a>(
    state: &ClientState,
    password: &String,
    error_message: &'a Option<String>,
) -> Element<'a, Message> {
    let elements = Row::new()
        .push(Space::with_width(Length::FillPortion(3)))
        .push(
            Column::new()
                .push(
                    Container::new(
                        Image::new(iced_image::Handle::from_memory(Vec::from(
                            assets::SIMS_LOGO_SQUARE,
                        )))
                        .height(Length::Units(110)),
                    )
                    .width(Fill)
                    .center_x(),
                )
                .push(
                    Container::new(Text::new("SIMS IMS").size(30))
                        .width(Fill)
                        .center_x(),
                )
                .push(Rule::horizontal(20))
                .push(
                    TextInput::new("Username", &state.username, Message::UsernameInputChanged)
                        .padding(10),
                )
                .push(
                    TextInput::new("Password", password, Message::PasswordInputChanged)
                        .padding(10)
                        .password(),
                )
                .push(Rule::horizontal(20))
                .push(
                    Button::new("Login")
                        .on_press(Message::LoginButtonClicked)
                        .width(Fill),
                )
                .push(Space::with_height(Length::Units(3)))
                .push(
                    Button::new("Register & Login")
                        .on_press(Message::RegisterButtonClicked)
                        .width(Fill),
                )
                .push(
                    Container::new(Text::new(match error_message {
                        Some(message) => message,
                        None => "",
                    }))
                    .height(Length::Units(50))
                    .center_x()
                    .center_y(),
                )
                .width(Length::FillPortion(2)),
        )
        .push(Space::with_width(Length::FillPortion(3)));

    Container::new(elements)
        .width(Fill)
        .height(Fill)
        .center_x()
        .center_y()
        .into()
}

pub(crate) fn inventory_view(state: &ClientState) -> Element<Message> {
    let page_content: Element<'_, Message> = match state.current_tab.last().unwrap_or_default() {
        TabId::AllShelves => Column::new()
            .push(
                Row::new().push(
                    Container::new(Text::new("My Shelves").size(30))
                        .width(Length::Fill)
                        .center_x(),
                ),
            )
            .push(row![
                Text::new("Name").width(Length::FillPortion(3)),
                Text::new("# of Slots").width(Length::FillPortion(3)),
                Text::new("Actions").width(Length::FillPortion(1))
            ])
            .push(Rule::horizontal(2))
            .push(
                // list all shelves
                Scrollable::new(state.shelves.iter().fold(Column::new(), |c, s| {
                    c.push(
                        Container::new(row![
                            Text::new(s.shelf_id.clone()).width(Length::FillPortion(3)),
                            Text::new(format!("Slots: {}", s.shelf_count))
                                .width(Length::FillPortion(3)),
                            Container::new(
                                Button::new("Open")
                                    .on_press(Message::OpenShelf(TabId::ShelfView(
                                        s.shelf_id.clone()
                                    )))
                                    .width(Length::Shrink)
                            )
                            .width(Length::FillPortion(1))
                        ])
                        .height(Length::Units(40))
                        .center_y(),
                    )
                })),
            )
            .into(),
        TabId::AllItems => {
            Column::new()
                .push(
                    Container::new(text("All Items").size(30))
                        .width(Length::Fill)
                        .center_x(),
                )
                .push(
                    row![
                        text("Item").width(Length::FillPortion(1)),
                        // Rule::vertical(2),
                        text("Description").width(Length::FillPortion(3)),
                        text("Shelf").width(Length::FillPortion(1)),
                        // Rule::vertical(2),
                        text("Price").width(Length::FillPortion(1)),
                        // Rule::vertical(2),
                        text("Stock").width(Length::FillPortion(1))
                    ]
                    .height(Length::Shrink),
                )
                .push(Rule::horizontal(2))
                .push(Scrollable::new(
                    state
                        .all_items
                        .iter()
                        .flat_map(|(k, v)| v.iter().zip(iter::repeat(k)))
                        .map(|(item, shelf)| {
                            Container::new(row![
                                text(item.object_id.clone()).width(Length::FillPortion(1)),
                                // Rule::vertical(2),
                                text(item.description.clone()).width(Length::FillPortion(3)),
                                text(shelf.clone()).width(Length::FillPortion(1)),
                                // Rule::vertical(2),
                                text(item.price).width(Length::FillPortion(1)),
                                // Rule::vertical(2),
                                text(item.stock).width(Length::FillPortion(1))
                            ])
                            .height(Length::Units(40))
                            .center_y()
                        })
                        .fold(Column::new(), |c, v| c.push(v))
                        .width(Length::Fill),
                ))
                .into()
        }
        TabId::ShelfView(shelf_id) => {
            match state.all_items.get(shelf_id) {
                Some(shelf_items) => {
                    Column::new()
                        .push(
                            Container::new(text(format!("Viewing {}", shelf_id)).size(30))
                                .width(Length::Fill)
                                .center_x(),
                        )
                        .push(
                            row![
                                text("ID").width(Length::FillPortion(1)),
                                // Rule::vertical(2),
                                text("Name").width(Length::FillPortion(4)),
                                // Rule::vertical(2),
                                text("Price").width(Length::FillPortion(1)),
                                // Rule::vertical(2),
                                text("Stock").width(Length::FillPortion(1))
                            ]
                            .height(Length::Shrink),
                        )
                        .push(Rule::horizontal(2))
                        .push(Scrollable::new(
                            shelf_items
                                .iter()
                                .map(|s| {
                                    Container::new(row![
                                        text(s.object_id.clone()).width(Length::FillPortion(1)),
                                        // Rule::vertical(2),
                                        text(s.description.clone()).width(Length::FillPortion(4)),
                                        // Rule::vertical(2),
                                        text(s.price).width(Length::FillPortion(1)),
                                        // Rule::vertical(2),
                                        text(s.stock).width(Length::FillPortion(1))
                                    ])
                                    .height(Length::Units(40))
                                    .center_y()
                                })
                                .fold(Column::new(), |c, v| c.push(v))
                                .width(Length::Fill),
                        ))
                        .into()
                }
                None => text("Invalid shelf tab").into(),
            }
        }
    };

    let mut tabs = state
        .tabs
        .iter()
        .map(|tab_info| match tab_info {
            TabId::AllShelves => create_tab(
                tab_info.clone(),
                "Shelves".to_owned(),
                false,
                Some('\u{F685}'),
            ),
            TabId::AllItems => create_tab(
                tab_info.clone(),
                "Items".to_owned(),
                false,
                Some('\u{F7D3}'),
            ),
            TabId::ShelfView(shelf_id) => {
                create_tab(tab_info.clone(), shelf_id.clone(), true, Some('\u{F1C8}'))
            }
        })
        .fold(Row::new(), |tabs_container, tab| {
            tabs_container
                .push(Space::with_width(Length::Units(2)))
                .push(tab)
        })
        .push(Space::with_width(Length::Fill));

    tabs = match state.current_tab.last().unwrap_or_default() {
        TabId::AllShelves => tabs.push(button(get_icon('\u{F116}')).on_press(UpdateShelves(None))),
        TabId::AllItems => tabs.push(button(get_icon('\u{F116}')).on_press(UpdateItems(None))),
        TabId::ShelfView(shelf_id) => {
            tabs.push(button(get_icon('\u{F116}')).on_press(UpdateItems(Some(shelf_id.clone()))))
        }
        _ => tabs,
    };

    let page = Column::new()
        .push(Container::new(tabs).width(Fill).height(Shrink).padding(5))
        .push(Container::new(page_content).width(Fill).height(Fill));

    Modal::new(
        state.edit_item.is_some(),
        FloatingElement::new(Container::new(page).width(Fill).height(Fill), || {
            Button::new(
                Container::new(get_icon('\u{F4FE}').size(30))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y(),
            )
            .style(theme::Button::Custom(Box::new(Fab)))
            .width(Length::Units(50))
            .height(Length::Units(50))
            .on_press(match state.current_tab.last().unwrap_or_default() {
                TabId::AllShelves => StartEditing(NewShelf {
                    shelf_name: "".to_owned(),
                    slots: "0".to_owned(),
                    error_message: None,
                }),
                TabId::AllItems => StartEditing(NewItem { shelf_id: "".to_owned(), item_name: "".to_owned(), item_count: "0".to_owned(), error_message: None }),
                TabId::ShelfView(shelf_id) => StartEditing(NewItem {
                    shelf_id: shelf_id.clone(),
                    item_name: "".to_owned(),
                    item_count: "0".to_owned(),
                    error_message: None
                }),
            })
            .into()
        }),
        move || match state.edit_item.as_ref() {
            None => text("Nothing to edit").into(),
            Some(target) => match target {
                EditTarget::EditItem { shelf_id, item_id } => text(format!(
                    "Editing item with id {} in shelf with id {}",
                    item_id, shelf_id
                ))
                .into(),
                EditTarget::NewItem { shelf_id, item_name, item_count, error_message } => {
                        let elements = state.shelves.iter().map(|s|s.shelf_id.clone()).collect::<Vec<String>>();
                        println!("Shelves {:?}", elements);
                        let mut card_content = column![
                            row![
                                Text::new("Shelf").width(Length::Units(60)),
                                TextInput::new("Enter a shelf id", shelf_id, Message::SlotPicked)
                            ],
                            Space::with_height(Length::Units(5)),
                            row![
                                Text::new("Name").width(Length::Units(60)),
                                TextInput::new(
                                    "Item name",
                                    item_name,
                                    Message::CreateObjectNameInputChanged
                                )
                            ],
                            Space::with_height(Length::Units(5)),
                            row![
                                Text::new("Stock").width(Length::Units(60)),
                                TextInput::new(
                                    "Item Stock",
                                    &item_count,
                                    Message::ShelfSlotCountInputChanged
                                )
                            ],
                            Space::with_height(Length::Units(5)),
                        ];
                        if let Some(message) = error_message {
                            card_content = card_content
                                .push(text(message))
                                .push(Space::with_height(Length::Units(5)));
                        }
                        card_content = card_content.push(row![
                            Space::with_width(Length::FillPortion(5)),
                            Button::new("Create")
                                .width(Length::FillPortion(1))
                                .on_press(Message::CreateTarget)
                        ]);

                        Card::new(text("New Item").width(Length::Fill), card_content)
                            .width(Length::Units(400))
                            .into()
                },
                EditTarget::EditSlot { shelf_id, slot_id } => text(format!(
                    "Editing slot with id {} in shelf with id {}",
                    slot_id, shelf_id
                ))
                .into(),
                EditTarget::NewShelf {
                    shelf_name,
                    slots,
                    error_message,
                } => {
                    let mut card_content = column![
                        row![
                            Text::new("Name").width(Length::Units(60)),
                            TextInput::new(
                                "Shelf name",
                                shelf_name,
                                Message::CreateObjectNameInputChanged
                            )
                        ],
                        Space::with_height(Length::Units(5)),
                        row![
                            Text::new("Slots").width(Length::Units(60)),
                            TextInput::new(
                                "Slots",
                                &format!("{}", slots),
                                Message::ShelfSlotCountInputChanged
                            )
                        ],
                        Space::with_height(Length::Units(5)),
                    ];
                    if let Some(message) = error_message {
                        card_content = card_content
                            .push(text(message))
                            .push(Space::with_height(Length::Units(5)));
                    }
                    card_content = card_content.push(row![
                        Space::with_width(Length::FillPortion(5)),
                        Button::new("Create")
                            .width(Length::FillPortion(1))
                            .on_press(Message::CreateTarget)
                    ]);

                    Card::new(text("New Shelf").width(Length::Fill), card_content)
                        .width(Length::Units(400))
                        .into()
                }
                EditTarget::EditShelf { shelf_id } => {
                    text(format!("Editing shelf with id {}", shelf_id)).into()
                }
            },
        },
    )
    .backdrop(StopEditing)
    .into()
}
