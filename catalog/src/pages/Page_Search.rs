// Copyright (c) 2026 TNTyep520
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gpui::{
    App, AppContext as _, Context, Entity, IntoElement, Render, Window, div, prelude::*, px,
};
use material3_gpui::prelude::*;

use super::{LogErr as _, page, showcase_group};

pub struct SearchPage {
    search_field: Entity<TextFieldState>,
    expanded_field: Entity<TextFieldState>,
    secure_field: Entity<TextFieldState>,
    query: String,
}

impl SearchPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let weak = cx.entity().downgrade();
            let search_field = {
                let weak = weak.clone();
                TextField::new("search-query", "Search")
                    .leading_icon(IconName::new("search"))
                    .on_value_change(move |value, _, cx| {
                        weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                            if page.query != value {
                                page.query = value.to_string();
                                cx.notify();
                            }
                        })
                        .log_err();
                    })
                    .build(cx)
            };
            let expanded_field = {
                TextField::new("search-expanded", "Recent searches")
                    .leading_icon(IconName::new("search"))
                    .on_value_change(move |value, _, cx| {
                        weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                            if page.query != value {
                                page.query = value.to_string();
                                cx.notify();
                            }
                        })
                        .log_err();
                    })
                    .build(cx)
            };
            let secure_field = SecureTextField::new("secure", "Passphrase")
                .value("secret")
                .build(cx);
            Self {
                search_field,
                expanded_field,
                secure_field,
                query: String::new(),
            }
        })
    }
}

impl Render for SearchPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();

        page(
            cx,
            "Search",
            "Search bar in collapsed and expanded forms, plus a secure text field.",
            [
                showcase_group(
                    cx,
                    "Search bar",
                    [div()
                        .w_full()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(8.))
                        .child(SearchBar::new("search", self.search_field.clone()))
                        .child(
                            typography
                                .body_medium
                                .apply(div())
                                .text_color(colors.on_surface_variant)
                                .child(if self.query.is_empty() {
                                    "Type to search".to_string()
                                } else {
                                    format!("Query: {}", self.query)
                                }),
                        )
                        .into_any_element()],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Expanded search",
                    [div()
                        .w_full()
                        .min_w_0()
                        .child(
                            SearchBar::new("search-expanded", self.expanded_field.clone())
                                .expanded(true)
                                .child(
                                    ListItem::new(("suggestion-1", 0usize), "Riso poster drafts")
                                        .leading_icon(IconName::new("history")),
                                )
                                .child(
                                    ListItem::new(("suggestion-2", 1usize), "Cutting mat restock")
                                        .leading_icon(IconName::new("history")),
                                )
                                .child(
                                    ListItem::new(("suggestion-3", 2usize), "Proof reading queue")
                                        .leading_icon(IconName::new("history")),
                                ),
                        )
                        .into_any_element()],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Secure text field",
                    [div()
                        .w(px(280.))
                        .max_w_full()
                        .child(self.secure_field.clone())
                        .into_any_element()],
                )
                .into_any_element(),
            ],
        )
    }
}
