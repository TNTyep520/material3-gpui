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
    WeakEntity, div, px,
};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, LogErr as _, example_content};

pub struct TimePickersPage {
    weak: WeakEntity<Self>,
    time_input: Entity<TextFieldState>,
    time: TimePickerState,
    time_label: String,
    time_input_value: String,
}

impl TimePickersPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let weak = cx.entity().downgrade();
            let time_input = {
                let weak = weak.clone();
                TimeInput::new("time-input", TimePickerState::new(10, 30))
                    .on_time_change(move |state, _, cx| {
                        weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                            page.time_input_value =
                                format!("{:02}:{:02}", state.hour, state.minute);
                            cx.notify();
                        })
                        .log_err();
                    })
                    .build(cx)
            };
            Self {
                weak,
                time_input,
                time: TimePickerState::new(10, 30),
                time_label: "10:30".to_string(),
                time_input_value: "10:30".to_string(),
            }
        })
    }
}

impl ComponentPage for TimePickersPage {
    fn description(&self) -> &'static str {
        "Time picker allows the user to choose time of day."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "TimePickerSample",
                description: "Time Picker examples",
            },
            ExampleMeta {
                name: "TimeInputSample",
                description: "Time Picker examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        let theme = cx.theme();
        let typography = *theme.typography();
        let colors = *theme.colors();
        match index {
            0 => example_content([div()
                .w(px(240.))
                .max_w_full()
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(
                    typography
                        .body_medium
                        .apply(div())
                        .text_color(colors.on_surface_variant)
                        .child(self.time_label.clone()),
                )
                .child(
                    TimePicker::new("time")
                        .value(self.time_label.clone())
                        .state(self.time)
                        .on_time_change({
                            let weak = self.weak.clone();
                            move |state, _, cx| {
                                weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                    page.time = state;
                                    page.time_label =
                                        format!("{:02}:{:02}", state.hour, state.minute);
                                    cx.notify();
                                })
                                .log_err();
                            }
                        }),
                )
                .into_any_element()]),
            1 => example_content([div()
                .w(px(240.))
                .max_w_full()
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(self.time_input.clone())
                .child(
                    typography
                        .body_medium
                        .apply(div())
                        .text_color(colors.on_surface_variant)
                        .child(format!("Parsed value: {}", self.time_input_value)),
                )
                .into_any_element()]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
