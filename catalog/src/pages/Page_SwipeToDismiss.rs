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

use gpui::{AnyElement, App, AppContext as _, Entity, IntoElement, Styled, div};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, catalog_card, example_content, example_row};

pub struct SwipeToDismissPage;

impl SwipeToDismissPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

impl ComponentPage for SwipeToDismissPage {
    fn description(&self) -> &'static str {
        "Swipe to dismiss is a gesture used to dismiss items in a list."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[ExampleMeta {
            name: "SwipeToDismiss",
            description: "Swipe to dismiss examples",
        }]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
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
