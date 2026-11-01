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

use gpui::{AnyElement, App, AppContext as _, Entity, IntoElement, ParentElement, Styled, div, px};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, catalog_card, example_content, example_row};

const ENTRIES: [(&str, &str, &str, Option<&str>); 5] = [
    ("Proofs batch", "Jan 9, 2026", "star", Some("3")),
    (
        "Field recordings",
        "Updated yesterday",
        "favorite",
        Some("12"),
    ),
    ("Riso calendar", "Drafting", "edit", None),
    ("Cutting mats", "Restock requested", "settings", None),
    ("Archive", "Last opened in May", "delete", None),
];

pub struct ListsPage;

impl ListsPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

fn inbox_row(
    ix: usize,
    title: &'static str,
    supporting: &'static str,
    icon: &'static str,
    trailing: Option<&'static str>,
) -> AnyElement {
    let mut item = ListItem::new(("studio-inbox", ix), title)
        .supporting_text(supporting)
        .leading_icon(IconName::new(icon))
        .on_click(move |_, window, cx| {
            show_snackbar(
                window,
                cx,
                Snackbar::new(format!("Opened \"{title}\"")),
                None,
            );
        });
    if let Some(text) = trailing {
        item = item.trailing_text(text);
    } else {
        item = item.trailing_icon(IconName::new("chevron_right"));
    }
    div().child(item).into_any_element()
}

impl ComponentPage for ListsPage {
    fn description(&self) -> &'static str {
        "Lists are continuous, vertical indexes of text or images."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "ClickableListItemSample",
                description: "List examples",
            },
            ExampleMeta {
                name: "SwipeToDismiss",
                description: "Swipe to dismiss examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => {
                let rows = ENTRIES
                    .iter()
                    .enumerate()
                    .flat_map(|(ix, (title, supporting, icon, trailing))| {
                        let row = inbox_row(ix, title, supporting, icon, *trailing);
                        if ix + 1 < ENTRIES.len() {
                            vec![row, Divider::horizontal().inset().into_any_element()]
                        } else {
                            vec![row]
                        }
                    })
                    .collect::<Vec<_>>();
                example_content([
                    div()
                        .w_full()
                        .min_w_0()
                        .child(List::new().children(rows))
                        .into_any_element(),
                    example_row(
                        cx,
                        "Segmented list item",
                        [div()
                            .w(px(320.))
                            .max_w_full()
                            .child(
                                List::new().child(
                                    SegmentedListItem::new(
                                        "segmented-item",
                                        "Segmented item",
                                        0,
                                        2,
                                    )
                                    .supporting_text("The first of two joined rows")
                                    .on_click(
                                        |_, window, cx| {
                                            show_snackbar(
                                                window,
                                                cx,
                                                Snackbar::new("Segmented row tapped"),
                                                None,
                                            );
                                        },
                                    ),
                                ),
                            )
                            .into_any_element()],
                    ),
                ])
            }
            1 => example_content([example_row(
                cx,
                "Swipe to dismiss",
                [SwipeToDismissBox::new(
                    "swipe-dismiss",
                    catalog_card(cx, Card::new().filled(), "Swipe me away"),
                )
                .background(div().bg(cx.theme().colors().error_container))
                .into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
