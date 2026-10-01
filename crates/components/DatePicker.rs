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

use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use gpui::{
    App, ElementId, IntoElement, RenderOnce, SharedString, Styled, Window, div,
    prelude::FluentBuilder as _, prelude::*, px,
};

use crate::theme::ActiveTheme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DatePickerState {
    pub year: i32,

    pub month: u32,

    pub selected_day: Option<u32>,
}

impl DatePickerState {
    pub fn new(year: i32, month: u32, selected_day: Option<u32>) -> Self {
        let month = month.clamp(1, 12);
        Self {
            year,
            month,
            selected_day: selected_day
                .filter(|day| *day >= 1 && *day <= days_in_month(year, month)),
        }
    }

    pub fn today() -> Self {
        let days = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            / 86_400;
        let (year, month) = year_month_from_days(days as i64);
        Self::new(year, month, None)
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 31,
    }
}

fn weekday_of_first(year: i32, month: u32) -> u32 {
    let offsets = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = year - i32::from(month < 3);
    (year + year / 4 - year / 100 + year / 400 + offsets[(month - 1) as usize]).rem_euclid(7) as u32
}

fn year_month_from_days(days: i64) -> (i32, u32) {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let year = year + i64::from(month <= 2);
    (year as i32, month as u32)
}

type DateChangeHandler = Rc<dyn Fn(i32, u32, u32, &mut Window, &mut App)>;
type MonthChangeHandler = Rc<dyn Fn(i32, u32, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct DatePicker {
    id: ElementId,
    value: SharedString,
    state: DatePickerState,
    on_date_change: Option<DateChangeHandler>,
    on_month_change: Option<MonthChangeHandler>,
}
impl DatePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: "Select date".into(),
            state: DatePickerState::today(),
            on_date_change: None,
            on_month_change: None,
        }
    }

    pub fn value(mut self, v: impl Into<SharedString>) -> Self {
        self.value = v.into();
        self
    }

    pub fn state(mut self, state: DatePickerState) -> Self {
        self.state = state;
        self
    }

    pub fn on_date_change(
        mut self,
        handler: impl Fn(i32, u32, u32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_date_change = Some(Rc::new(handler));
        self
    }

    pub fn on_month_change(
        mut self,
        handler: impl Fn(i32, u32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_month_change = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for DatePicker {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let year = self.state.year;
        let month = self.state.month;
        let first = weekday_of_first(year, month);
        let days = days_in_month(year, month);
        let calendar_id = self.id.clone();
        let previous = if month == 1 {
            (year - 1, 12)
        } else {
            (year, month - 1)
        };
        let next = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
        let month_change = self.on_month_change.clone();
        let previous_button = div()
            .id((self.id.clone(), "previous"))
            .cursor_pointer()
            .child("‹")
            .when_some(month_change, |el, handler| {
                el.on_click(move |_, window, cx| handler(previous.0, previous.1, window, cx))
            });
        let next_button = div()
            .id((self.id.clone(), "next"))
            .cursor_pointer()
            .child("›")
            .when_some(self.on_month_change, |el, handler| {
                el.on_click(move |_, window, cx| handler(next.0, next.1, window, cx))
            });
        div()
            .id(self.id)
            .p(px(16.))
            .rounded(px(12.))
            .bg(colors.surface_container_high)
            .text_color(colors.on_surface)
            .child(self.value)
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(previous_button)
                    .child(format!("{year:04}-{month:02}"))
                    .child(next_button),
            )
            .children((0_u32..6).map(|week| {
                div().flex().children((0_u32..7).map(|weekday| {
                    let day: u32 = week * 7 + weekday + 1;
                    let day = day
                        .checked_sub(first)
                        .filter(|day| *day >= 1 && *day <= days);
                    let selected = day == self.state.selected_day && day.is_some();
                    let handler = self.on_date_change.clone();
                    div()
                        .id((calendar_id.clone(), format!("day-{}", week * 7 + weekday)))
                        .size(px(36.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .when(selected, |el| {
                            el.bg(colors.primary).text_color(colors.on_primary)
                        })
                        .when_some(day, |el, day| {
                            el.child(day.to_string()).when_some(handler, |el, handler| {
                                el.cursor_pointer().on_click(move |_, window, cx| {
                                    handler(year, month, day, window, cx)
                                })
                            })
                        })
                }))
            }))
    }
}
pub type ExposedDatePicker = DatePicker;

#[cfg(test)]
mod tests {
    use super::{DatePickerState, days_in_month, weekday_of_first, year_month_from_days};

    #[test]
    fn gregorian_month_lengths_and_weekdays_are_valid() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2100, 2), 28);
        assert_eq!(days_in_month(2000, 2), 29);
        assert_eq!(weekday_of_first(2026, 10), 3);
        assert_eq!(year_month_from_days(0), (1970, 1));
        assert_eq!(DatePickerState::new(2025, 2, Some(29)).selected_day, None);
    }
}
