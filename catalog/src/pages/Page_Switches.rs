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

pub struct SwitchesPage {
    sw_dicts: Entity<SwitchState>,
    sw_sync: Entity<SwitchState>,
    sw_quiet: Entity<SwitchState>,
    sw_disabled_on: Entity<SwitchState>,
    sw_disabled_icon_on: Entity<SwitchState>,
    sw_disabled_off: Entity<SwitchState>,
}

impl SwitchesPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let sw_dicts = Switch::new("sw-dicts")
            .checked(true)
            .on_change(move |checked, window, cx| {
                let message = if checked {
                    "Spell-check on"
                } else {
                    "Spell-check off"
                };
                show_snackbar(window, cx, Snackbar::new(message), None);
            })
            .build(cx);
        let sw_sync = Switch::new("sw-sync").build(cx);
        let sw_quiet = Switch::new("sw-quiet")
            .checked(true)
            .with_check_icon(true)
            .build(cx);
        let sw_disabled_on = Switch::new("sw-dis-on")
            .disabled(true)
            .checked(true)
            .build(cx);
        let sw_disabled_icon_on = Switch::new("sw-dis-icon-on")
            .with_check_icon(true)
            .checked(true)
            .disabled(true)
            .build(cx);
        let sw_disabled_off = Switch::new("sw-dis-off").disabled(true).build(cx);

        cx.new(|_| Self {
            sw_dicts,
            sw_sync,
            sw_quiet,
            sw_disabled_on,
            sw_disabled_icon_on,
            sw_disabled_off,
        })
    }
}

impl ComponentPage for SwitchesPage {
    fn description(&self) -> &'static str {
        "Switches toggle the state of a single setting on or off."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "SwitchSample",
                description: "Switch examples",
            },
            ExampleMeta {
                name: "SwitchWithThumbIconSample",
                description: "Switch examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([
                example_row(cx, "Selected", [self.sw_dicts.clone().into_any_element()]),
                example_row(cx, "Unselected", [self.sw_sync.clone().into_any_element()]),
                example_row(
                    cx,
                    "Disabled · on",
                    [self.sw_disabled_on.clone().into_any_element()],
                ),
                example_row(
                    cx,
                    "Disabled · off",
                    [self.sw_disabled_off.clone().into_any_element()],
                ),
            ]),
            1 => example_content([
                example_row(cx, "With icon", [self.sw_quiet.clone().into_any_element()]),
                example_row(
                    cx,
                    "Disabled · icon on",
                    [self.sw_disabled_icon_on.clone().into_any_element()],
                ),
            ]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
