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
    App, AppContext as _, Context, Entity, IntoElement, Render, WeakEntity, Window, div,
    prelude::*, px,
};
use material3_gpui::prelude::*;

use super::{LogErr as _, page, showcase_group};

pub struct PickersPage {
    weak: WeakEntity<Self>,
    time_input: Entity<TextFieldState>,
    date: DatePickerState,
    time: TimePickerState,
    time_input_value: String,
}

impl PickersPage {
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
                date: DatePickerState::today(),
                time: TimePickerState::new(10, 30),
                time_input_value: "10:30".to_string(),
            }
        })
    }
}

impl Render for PickersPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();
        let weak = self.weak.clone();

        let date_label = self
            .date
            .selected_day
            .map(|day| format!("{:04}-{:02}-{:02}", self.date.year, self.date.month, day))
            .unwrap_or_else(|| "Select date".to_string());
        let time_label = format!("{:02}:{:02}", self.time.hour, self.time.minute);

        page(
            cx,
            "Date & time pickers",
            "Calendar month picker and 24-hour time controls.",
            [
                showcase_group(
                    cx,
                    "Date picker",
                    [div()
                        .w(px(292.))
                        .max_w_full()
                        .flex()
                        .flex_col()
                        .gap(px(8.))
                        .child(
                            typography
                                .body_medium
                                .apply(div())
                                .text_color(colors.on_surface_variant)
                                .child(date_label.clone()),
                        )
                        .child(
                            DatePicker::new("date")
                                .value(date_label)
                                .state(self.date)
                                .on_date_change({
                                    let weak = weak.clone();
                                    move |year, month, day, _, cx| {
                                        weak.update(
                                            cx,
                                            |page: &mut Self, cx: &mut Context<Self>| {
                                                page.date =
                                                    DatePickerState::new(year, month, Some(day));
                                                cx.notify();
                                            },
                                        )
                                        .log_err();
                                    }
                                })
                                .on_month_change({
                                    let weak = weak.clone();
                                    move |year, month, _, cx| {
                                        weak.update(
                                            cx,
                                            |page: &mut Self, cx: &mut Context<Self>| {
                                                let day = page.date.selected_day;
                                                page.date = DatePickerState::new(year, month, day);
                                                cx.notify();
                                            },
                                        )
                                        .log_err();
                                    }
                                }),
                        )
                        .into_any_element()],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Time picker",
                    [div()
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
                                .child(time_label.clone()),
                        )
                        .child(
                            TimePicker::new("time")
                                .value(time_label)
                                .state(self.time)
                                .on_time_change({
                                    move |state, _, cx| {
                                        weak.update(
                                            cx,
                                            |page: &mut Self, cx: &mut Context<Self>| {
                                                page.time = state;
                                                cx.notify();
                                            },
                                        )
                                        .log_err();
                                    }
                                }),
                        )
                        .into_any_element()],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Time input",
                    [div()
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
                        .into_any_element()],
                )
                .into_any_element(),
            ],
        )
    }
}
