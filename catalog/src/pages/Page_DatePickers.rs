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

pub struct DatePickersPage {
    weak: WeakEntity<Self>,
    date: DatePickerState,
    date_label: String,
}

impl DatePickersPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let weak = cx.entity().downgrade();
            let date = DatePickerState::today();
            let date_label = format!(
                "{:04}-{:02}-{:02}",
                date.year,
                date.month,
                date.selected_day.unwrap_or(1)
            );
            Self {
                weak,
                date,
                date_label,
            }
        })
    }
}

impl ComponentPage for DatePickersPage {
    fn description(&self) -> &'static str {
        "Date pickers let users select a date or range of dates."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[ExampleMeta {
            name: "DatePickerSample",
            description: "Date picker examples",
        }]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => {
                let theme = cx.theme();
                let colors = *theme.colors();
                let typography = *theme.typography();
                let weak = self.weak.clone();
                let date_label = self.date_label.clone();
                example_content([div()
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
                                    weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                        page.date = DatePickerState::new(year, month, Some(day));
                                        page.date_label = format!("{year:04}-{month:02}-{day:02}");
                                        cx.notify();
                                    })
                                    .log_err();
                                }
                            })
                            .on_month_change({
                                let weak = weak;
                                move |year, month, _, cx| {
                                    weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                        let day = page.date.selected_day;
                                        page.date = DatePickerState::new(year, month, day);
                                        cx.notify();
                                    })
                                    .log_err();
                                }
                            }),
                    )
                    .into_any_element()])
            }
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
