use iced::{Element, theme};
use iced::widget::{Button, Column, Container, Image, Row, Text, TextInput, text, Rule, Space, Scrollable, column, row};
use iced::{Length};
use iced::Length::{Fill, Shrink};
use iced::widget::image as iced_image;
use iced_aw::{Modal};
use iced_aw::floating_element::FloatingElement;
use crate::{assets, ClientState};
use crate::assets::get_icon;
use crate::frontend::{create_tab, EditTarget, TabId};
use crate::frontend::EditTarget::{NewItem, NewShelf};
use crate::styles::Fab;
use crate::ui_messages::Message;
use crate::ui_messages::Message::{StartEditing, StopEditing};

pub(crate) fn unauthenticated_view<'a>(state: &ClientState, password: &String, error_message: &'a Option<String>) -> Element<'a, Message> {
    let elements = Row::new()
                    .push(Space::with_width(Length::FillPortion(3)))
                    .push(
                        Column::new()
                            .push(Container::new(Image::new(iced_image::Handle::from_memory(Vec::from(assets::SIMS_LOGO_SQUARE))).height(Length::Units(110))).width(Fill).center_x())
                            .push(Container::new(Text::new("SIMS IMS").size(30)).width(Fill).center_x())
                            .push(Rule::horizontal(20))
                            .push(TextInput::new("Username", &state.username, Message::UsernameInputChanged).padding(10))
                            .push(TextInput::new("Password", password, Message::PasswordInputChanged).padding(10).password())
                            .push(Rule::horizontal(20))
                            .push(Button::new("Login").on_press(Message::LoginButtonClicked).width(Fill))
                            .push(Space::with_height(Length::Units(3)))
                            .push(Button::new("Register & Login").on_press(Message::RegisterButtonClicked).width(Fill))
                            .push(Container::new(Text::new(match error_message{Some(message)=>message, None=>""})).height(Length::Units(50)).center_x().center_y()
                            )
                            .width(Length::FillPortion(2)))
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
                    TabId::AllShelves => Row::new()
                        .push(Text::new("All shelves view"))
                        .push(Button::new("Meep").on_press(Message::OpenShelf(TabId::ShelfView("shelf0".to_owned()))))
                        .push(Button::new("Meep2").on_press(Message::OpenShelf(TabId::ShelfView("shelf1".to_owned()))))
                        .into(),
                    TabId::AllItems => Text::new("All items view").into(),
                    TabId::ShelfView(shelf_id) => {
                        let text_content = format!("Shelf Items view for shelf {}", shelf_id);
                        Scrollable::new(
                            column(
                                state.shelves.iter()
                                    .map(|s|row![
                                        Text::new(s.shelf_id.clone()),
                                        Text::new(format!("Slots: {}", s.shelf_count))
                                    ].into())
                                    .collect()
                            )
                        ).into()
                    }
                };

                let tabs = state.tabs.iter()
                    .map(| tab_info| match tab_info {
                        TabId::AllShelves => create_tab(tab_info.clone(), "Shelves".to_owned(), false, Some('\u{F685}')),
                        TabId::AllItems => create_tab(tab_info.clone(), "Items".to_owned(), false, Some('\u{F7D3}')),
                        TabId::ShelfView(shelf_id) => {
                            create_tab(tab_info.clone(), shelf_id.clone(), true, Some('\u{F1C8}'))
                        }
                    })
                    .fold(
                        Row::new(),
                        |tabs_container, tab|{
                            tabs_container.push(Space::with_width(Length::Units(2))).push(tab)
                        }
                    );

                let page = Column::new()
                    .push(
                    Container::new(
                            tabs
                        )
                        .width(Fill)
                        .height(Shrink)
                        .padding(5))
                    .push(
                        Container::new(page_content)
                        .width(Fill)
                        .height(Fill)
                    );

                Modal::new(state.edit_item.is_some(), FloatingElement::new(
                    Container::new(page).width(Fill).height(Fill),
                    || {
                            Button::new(Container::new(get_icon('\u{F4FE}').size(30)).width(Length::Fill).height(Length::Fill).center_x().center_y())
                                .style(theme::Button::Custom(Box::new(Fab)))
                                .width(Length::Units(50))
                                .height(Length::Units(50))
                                .on_press(
                                match state.current_tab.last().unwrap_or_default() {
                                    TabId::AllShelves => StartEditing(NewShelf),
                                    TabId::AllItems => StartEditing(NewItem{shelf_id: None}),
                                    TabId::ShelfView(shelf_id) => StartEditing(NewItem{shelf_id: Some(shelf_id.clone())})
                                }
                            ).into()
                        }),
                        move || {
                            match state.edit_item.as_ref() {
                                None => text("Nothing to edit"),
                                Some(target) => match target {
                                    EditTarget::EditItem { shelf_id, item_id } => text(format!("Editing item with id {} in shelf with id {}", item_id, shelf_id)),
                                    EditTarget::NewItem { shelf_id } => match shelf_id {
                                        None => text(format!("Creating new item in shelf with no default shelf id")),
                                        Some(shelf_id) => text(format!("Creating new item in shelf with default shelf id {}", shelf_id))
                                    },
                                    EditTarget::EditSlot { shelf_id, slot_id } => text(format!("Editing slot with id {} in shelf with id {}", slot_id, shelf_id)),
                                    EditTarget::NewShelf => text("Creating new shelf"),
                                    EditTarget::EditShelf { shelf_id } => text(format!("Editing shelf with id {}", shelf_id)),
                                }
                            }.into()
                        }).backdrop(StopEditing).into()
}