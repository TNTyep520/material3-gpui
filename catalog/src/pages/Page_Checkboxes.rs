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

pub struct CheckboxesPage {
    cb_a: Entity<CheckboxState>,
    cb_b: Entity<CheckboxState>,
    cb_disabled_on: Entity<CheckboxState>,
    cb_disabled_off: Entity<CheckboxState>,
    tri: Entity<CheckboxState>,
    tri_disabled: Entity<CheckboxState>,
}

impl CheckboxesPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let cb_a = Checkbox::new("cb-a").checked(true).build(cx);
        let cb_b = Checkbox::new("cb-b").build(cx);
        let cb_disabled_on = Checkbox::new("cb-dis-on")
            .checked(true)
            .disabled(true)
            .build(cx);
        let cb_disabled_off = Checkbox::new("cb-dis-off").disabled(true).build(cx);
        let tri = TriStateCheckbox::new("cb-tri")
            .state(ToggleableState::Indeterminate)
            .build(cx);
        let tri_disabled = TriStateCheckbox::new("cb-tri-dis")
            .state(ToggleableState::On)
            .enabled(false)
            .build(cx);

        cx.new(|_| Self {
            cb_a,
            cb_b,
            cb_disabled_on,
            cb_disabled_off,
            tri,
            tri_disabled,
        })
    }
}

impl ComponentPage for CheckboxesPage {
    fn description(&self) -> &'static str {
        "Checkboxes allow the user to select one or more items from a set or turn an \
         option on or off."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "CheckboxSample",
                description: "Checkboxes examples",
            },
            ExampleMeta {
                name: "TriStateCheckboxSample",
                description: "Checkboxes examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([
                example_row(cx, "Checked", [self.cb_a.clone().into_any_element()]),
                example_row(cx, "Unchecked", [self.cb_b.clone().into_any_element()]),
                example_row(
                    cx,
                    "Disabled · checked",
                    [self.cb_disabled_on.clone().into_any_element()],
                ),
                example_row(
                    cx,
                    "Disabled · unchecked",
                    [self.cb_disabled_off.clone().into_any_element()],
                ),
            ]),
            1 => example_content([
                example_row(cx, "Indeterminate", [self.tri.clone().into_any_element()]),
                example_row(
                    cx,
                    "Disabled",
                    [self.tri_disabled.clone().into_any_element()],
                ),
            ]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
