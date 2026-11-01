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

use super::{ComponentPage, ExampleMeta, example_content, example_row};

pub struct RadioButtonsPage {
    radios: Vec<Entity<RadioState>>,
    radio_disabled: Entity<RadioState>,
}

impl RadioButtonsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        // 电台方案:第一档默认选中,第三档禁用;状态演示样本各自独立,不互斥
        let radios: Vec<Entity<RadioState>> = (0..3usize)
            .map(|ix| {
                RadioButton::new(("radio-plan", ix))
                    .selected(ix == 0)
                    .disabled(ix == 2)
                    .build(cx)
            })
            .collect();
        let radio_disabled = RadioButton::new("radio-disabled").disabled(true).build(cx);

        cx.new(|_| Self {
            radios,
            radio_disabled,
        })
    }
}

impl ComponentPage for RadioButtonsPage {
    fn description(&self) -> &'static str {
        "Radio buttons allow the user to select one option from a set."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "RadioButtonSample",
                description: "Radio buttons examples",
            },
            ExampleMeta {
                name: "RadioGroupSample",
                description: "Radio buttons examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => {
                let labels = ["Selected", "Unselected", "Disabled"];
                example_content(
                    self.radios
                        .iter()
                        .zip(labels)
                        .map(|(radio, label)| {
                            example_row(cx, label, [radio.clone().into_any_element()])
                        })
                        .collect::<Vec<_>>(),
                )
            }
            1 => example_content([example_row(
                cx,
                "Disabled group member",
                [self.radio_disabled.clone().into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
