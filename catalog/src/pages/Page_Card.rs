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

use gpui::{AnyElement, App, AppContext as _, Entity, IntoElement};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, catalog_card, example_content, example_row};

pub struct CardPage;

impl CardPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

impl ComponentPage for CardPage {
    fn description(&self) -> &'static str {
        "Cards contain content and actions that relate information about a subject."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "CardSample",
                description: "Cards examples",
            },
            ExampleMeta {
                name: "ClickableCardSample",
                description: "Cards examples",
            },
            ExampleMeta {
                name: "ElevatedCardSample",
                description: "Cards examples",
            },
            ExampleMeta {
                name: "ClickableElevatedCardSample",
                description: "Cards examples",
            },
            ExampleMeta {
                name: "OutlinedCardSample",
                description: "Cards examples",
            },
            ExampleMeta {
                name: "ClickableOutlinedCardSample",
                description: "Cards examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Filled",
                [catalog_card(cx, Card::new().filled(), "Filled").into_any_element()],
            )]),
            1 => example_content([example_row(
                cx,
                "Clickable",
                [catalog_card(cx, Card::new().filled(), "Clickable").into_any_element()],
            )]),
            2 => example_content([example_row(
                cx,
                "Elevated",
                [catalog_card(cx, Card::new().elevated(), "Elevated").into_any_element()],
            )]),
            3 => example_content([example_row(
                cx,
                "Clickable",
                [
                    catalog_card(cx, Card::new().elevated(), "Clickable elevated")
                        .into_any_element(),
                ],
            )]),
            4 => example_content([example_row(
                cx,
                "Outlined",
                [catalog_card(cx, Card::new().outlined(), "Outlined").into_any_element()],
            )]),
            5 => example_content([example_row(
                cx,
                "Clickable",
                [
                    catalog_card(cx, Card::new().outlined(), "Clickable outlined")
                        .into_any_element(),
                ],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
