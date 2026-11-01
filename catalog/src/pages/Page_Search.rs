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
    AnyElement, App, AppContext as _, Context, Entity, IntoElement, ParentElement as _, Styled,
    div, px,
};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, LogErr as _, example_content, example_row};

pub struct SearchPage {
    search_field: Entity<TextFieldState>,
    expanded_field: Entity<TextFieldState>,
    fullscreen_field: Entity<TextFieldState>,
    query: String,
}

impl SearchPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let weak = cx.entity().downgrade();
            let search_field = {
                let weak = weak;
                TextField::new("search-query", "Search")
                    .plain(true)
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
                    .plain(true)
                    .leading_icon(IconName::new("search"))
                    .build(cx)
            };
            let fullscreen_field = {
                TextField::new("search-fullscreen", "Search")
                    .plain(true)
                    .leading_icon(IconName::new("search"))
                    .build(cx)
            };
            Self {
                search_field,
                expanded_field,
                fullscreen_field,
                query: String::new(),
            }
        })
    }
}

impl ComponentPage for SearchPage {
    fn description(&self) -> &'static str {
        "Search bars allow users to enter a keyword or phrase and get relevant information."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "SimpleSearchBarSample",
                description: "Search bar examples",
            },
            ExampleMeta {
                name: "DockedSearchBarScaffoldSample",
                description: "Search bar examples",
            },
            ExampleMeta {
                name: "FullScreenSearchBarScaffoldSample",
                description: "Search bar examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => {
                let theme = cx.theme();
                let colors = *theme.colors();
                let typography = *theme.typography();
                example_content([example_row(
                    cx,
                    "Collapsed",
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
                )])
            }
            1 => example_content([example_row(
                cx,
                "Expanded · suggestions",
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
            )]),
            2 => example_content([example_row(
                cx,
                "Expanded · full width",
                [div()
                    .w_full()
                    .min_w_0()
                    .child(
                        SearchBar::new("search-fullscreen", self.fullscreen_field.clone())
                            .expanded(true),
                    )
                    .into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
