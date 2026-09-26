//! Lists 页。

use gpui::{App, Entity, IntoElement, Render, Window, prelude::*};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Lists 页视图。
pub struct ListsPage;

impl ListsPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

impl Render for ListsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gallery([showcase_group(
            cx,
            "Lists",
            [Card::new()
                .outlined()
                .overflow_hidden()
                .child(
                    List::new()
                        .child(
                            ListItem::new("li-1", "Photos")
                                .supporting_text("Jan 9, 2026")
                                .leading_icon(IconName::Star)
                                .trailing_icon(IconName::MoreVert)
                                .on_click(|_, _, _| {}),
                        )
                        .child(Divider::horizontal().inset())
                        .child(
                            ListItem::new("li-2", "Recipes")
                                .supporting_text("Updated yesterday")
                                .leading_icon(IconName::Favorite)
                                .trailing_text("2")
                                .on_click(|_, _, _| {}),
                        )
                        .child(Divider::horizontal().inset())
                        .child(
                            ListItem::new("li-3", "Settings")
                                .leading_icon(IconName::Settings)
                                .on_click(|_, _, _| {}),
                        ),
                )
                .into_any_element()],
        )])
    }
}
