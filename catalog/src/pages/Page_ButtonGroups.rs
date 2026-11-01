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
    AnyElement, App, AppContext as _, Entity, IntoElement, ParentElement as _, Styled, div, px,
};
use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content, example_row};

pub struct ButtonGroupsPage {
    connected: Vec<Entity<ButtonState>>,
    overflow: Vec<Entity<ButtonState>>,
}

impl ButtonGroupsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let connected = vec![
            Button::new("bg-day", "Day").tonal().build(cx),
            Button::new("bg-week", "Week").outlined().build(cx),
            Button::new("bg-month", "Month").outlined().build(cx),
        ];
        let overflow = vec![
            Button::new("bg-add", "Add").filled().build(cx),
            Button::new("bg-remove", "Remove").outlined().build(cx),
        ];
        cx.new(|_| Self {
            connected,
            overflow,
        })
    }
}

impl ComponentPage for ButtonGroupsPage {
    fn description(&self) -> &'static str {
        "button groups is a container for material components that adds an animation on \
         press"
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "ButtonGroupSample",
                description: "ButtonGroup examples",
            },
            ExampleMeta {
                name: "VerticalButtonGroupSample",
                description: "ButtonGroup examples",
            },
            ExampleMeta {
                name: "SingleSelectConnectedButtonGroupSample",
                description: "ButtonGroup examples",
            },
        ]
    }

    fn example(&self, index: usize, _cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                _cx,
                "Horizontal",
                [ButtonGroup::new("bg-connected")
                    .children(self.connected.clone())
                    .into_any_element()],
            )]),
            1 => example_content([example_row(
                _cx,
                "Vertical",
                [div()
                    .h(px(160.))
                    .child(
                        ButtonGroup::new("bg-vertical")
                            .vertical()
                            .children(self.overflow.clone()),
                    )
                    .into_any_element()],
            )]),
            2 => example_content([example_row(
                _cx,
                "Connected",
                [ButtonGroup::new("bg-connected-select")
                    .children(self.connected.clone())
                    .into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
