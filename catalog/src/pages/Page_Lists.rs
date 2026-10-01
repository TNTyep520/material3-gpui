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

use gpui::{AnyElement, App, Entity, IntoElement, Render, Window, div, prelude::*};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

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
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
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

impl Render for ListsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
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

        gallery([showcase_group(
            cx,
            "List items & dividers",
            [div()
                .w_full()
                .min_w_0()
                .child(List::new().children(rows))
                .into_any_element()],
        )])
    }
}
