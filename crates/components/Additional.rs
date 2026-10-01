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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/LoadingIndicator.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButtonMenu.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingToolbar.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/SwipeToDismissBox.kt

use crate::components::{TextField, TextFieldState};
use crate::theme::ActiveTheme;
use gpui::{
    AnyElement, App, ElementId, Entity, IntoElement, MouseButton, ParentElement, RenderOnce,
    SharedString, Styled, Window, div, prelude::*, px, relative,
};
use std::cell::Cell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

type RangeChangeHandler = Rc<dyn Fn((f32, f32), &mut Window, &mut App)>;
type ChangeHandler = Rc<dyn Fn(&str, &mut Window, &mut App)>;
#[derive(IntoElement)]
pub struct RangeSlider {
    id: ElementId,
    start: f32,
    end: f32,
    enabled: bool,
    on_value_change: Option<RangeChangeHandler>,
}
impl RangeSlider {
    pub fn new(id: impl Into<ElementId>, start: f32, end: f32) -> Self {
        Self {
            id: id.into(),
            start: start.clamp(0., 1.).min(end.clamp(0., 1.)),
            end: end.clamp(0., 1.).max(start.clamp(0., 1.)),
            enabled: true,
            on_value_change: None,
        }
    }
    pub fn range(mut self, start: f32, end: f32) -> Self {
        self.start = start.clamp(0., 1.).min(end.clamp(0., 1.));
        self.end = end.clamp(0., 1.).max(self.start);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn((f32, f32), &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for RangeSlider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        let bounds = crate::interaction::BoundsHandle::new();
        let values = Rc::new(Cell::new((self.start, self.end)));
        let active_start = Rc::new(Cell::new(None::<bool>));
        let on_mouse_down = {
            let bounds = bounds.clone();
            let values = values.clone();
            let active_start = active_start.clone();
            let callback = self.on_value_change.clone();
            move |event: &gpui::MouseDownEvent, window: &mut Window, cx: &mut App| {
                let rect = bounds.get();
                let width = f32::from(rect.size.width);
                if width <= 0. {
                    return;
                }
                let fraction = (f32::from(event.position.x - rect.origin.x) / width).clamp(0., 1.);
                let (start, end) = values.get();
                let is_start = (fraction - start).abs() <= (fraction - end).abs();
                active_start.set(Some(is_start));
                let next = if is_start {
                    (fraction.min(end), end)
                } else {
                    (start, fraction.max(start))
                };
                values.set(next);
                if let Some(handler) = &callback {
                    handler(next, window, cx);
                }
            }
        };
        let on_mouse_move = {
            let bounds = bounds.clone();
            let active_start = active_start.clone();
            let callback = self.on_value_change;
            move |event: &gpui::MouseMoveEvent, window: &mut Window, cx: &mut App| {
                let Some(is_start) = active_start.get() else {
                    return;
                };
                if event.pressed_button != Some(MouseButton::Left) {
                    return;
                }
                let rect = bounds.get();
                let width = f32::from(rect.size.width);
                if width <= 0. {
                    return;
                }
                let fraction = (f32::from(event.position.x - rect.origin.x) / width).clamp(0., 1.);
                let (start, end) = values.get();
                let next = if is_start {
                    (fraction.min(end), end)
                } else {
                    (start, fraction.max(start))
                };
                if next != (start, end) {
                    values.set(next);
                    if let Some(handler) = &callback {
                        handler(next, window, cx);
                    }
                }
            }
        };
        div()
            .id(self.id)
            .relative()
            .h(px(44.))
            .w_full()
            .when(self.enabled, |el| {
                el.cursor_pointer()
                    .on_mouse_down(MouseButton::Left, on_mouse_down)
                    .on_mouse_move(on_mouse_move)
                    .on_mouse_up(MouseButton::Left, move |_, _, _| active_start.set(None))
            })
            .child(
                div()
                    .absolute()
                    .top(px(20.))
                    .h(px(4.))
                    .w_full()
                    .rounded_full()
                    .bg(c.secondary_container),
            )
            .child(
                div()
                    .absolute()
                    .top(px(20.))
                    .left(relative(self.start))
                    .right(relative(1. - self.end))
                    .h(px(4.))
                    .bg(c.primary),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left(relative(self.start))
                    .ml(px(-2.))
                    .w(px(4.))
                    .h(px(44.))
                    .rounded_full()
                    .bg(c.primary),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left(relative(self.end))
                    .ml(px(-2.))
                    .w(px(4.))
                    .h(px(44.))
                    .rounded_full()
                    .bg(c.primary),
            )
            .child(bounds.capture_element())
    }
}

#[derive(IntoElement)]
pub struct Scrollbar {
    id: ElementId,
    position: f32,
    thickness: gpui::Pixels,
}
impl Scrollbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            position: 0.,
            thickness: px(4.),
        }
    }
    pub fn position(mut self, p: f32) -> Self {
        self.position = p.clamp(0., 1.);
        self
    }
    pub fn thickness(mut self, t: gpui::Pixels) -> Self {
        self.thickness = t;
        self
    }
}
impl RenderOnce for Scrollbar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .absolute()
            .right_0()
            .top(relative(self.position))
            .w(self.thickness)
            .h(px(48.))
            .rounded_full()
            .bg(cx.theme().colors().on_surface_variant.opacity(0.5))
    }
}

pub struct SecureTextField {
    id: ElementId,
    label: SharedString,
    value: SharedString,
    enabled: bool,
    on_value_change: Option<ChangeHandler>,
}
impl SecureTextField {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: SharedString::default(),
            enabled: true,
            on_value_change: None,
        }
    }
    pub fn value(mut self, v: impl Into<SharedString>) -> Self {
        self.value = v.into();
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn on_value_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}
impl SecureTextField {
    pub fn build(self, cx: &mut App) -> Entity<TextFieldState> {
        let mut field = TextField::new(self.id, self.label)
            .password(true)
            .value(self.value)
            .enabled(self.enabled);
        if let Some(handler) = self.on_value_change {
            field = field.on_value_change(move |value, window, cx| handler(value, window, cx));
        }
        field.build(cx)
    }
}

#[derive(IntoElement)]
pub struct SearchBar {
    id: ElementId,
    field: Entity<TextFieldState>,
    expanded: bool,
    suggestions: Vec<gpui::AnyElement>,
}
impl SearchBar {
    pub fn new(id: impl Into<ElementId>, field: Entity<TextFieldState>) -> Self {
        Self {
            id: id.into(),
            field,
            expanded: false,
            suggestions: Vec::new(),
        }
    }
    pub fn expanded(mut self, e: bool) -> Self {
        self.expanded = e;
        self
    }
}

impl ParentElement for SearchBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.suggestions.extend(elements);
    }
}
impl RenderOnce for SearchBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        div()
            .id(self.id)
            .w_full()
            .min_h(px(56.))
            .when(!self.expanded, |el| el.rounded_full())
            .when(self.expanded, |el| el.rounded(px(28.)))
            .bg(c.surface_container_high)
            .overflow_hidden()
            .flex()
            .flex_col()
            .text_color(c.on_surface)
            .child(self.field)
            .when(self.expanded, |el| el.children(self.suggestions))
    }
}

#[derive(IntoElement)]
pub struct SwipeToDismissBox {
    id: ElementId,
    content: gpui::AnyElement,
    background: Option<gpui::AnyElement>,
    enabled: bool,
    state: SwipeToDismissBoxState,
    on_value_change: Option<SwipeChangeHandler>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwipeToDismissBoxValue {
    #[default]
    Settled,

    StartToEnd,

    EndToStart,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SwipeToDismissBoxState {
    pub current_value: SwipeToDismissBoxValue,
}

type SwipeChangeHandler = Rc<dyn Fn(SwipeToDismissBoxValue, &mut Window, &mut App)>;

impl SwipeToDismissBox {
    pub fn new(id: impl Into<ElementId>, content: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            content: content.into_any_element(),
            background: None,
            enabled: true,
            state: SwipeToDismissBoxState::default(),
            on_value_change: None,
        }
    }

    pub fn background(mut self, b: impl IntoElement) -> Self {
        self.background = Some(b.into_any_element());
        self
    }

    pub fn state(mut self, state: SwipeToDismissBoxState) -> Self {
        self.state = state;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(SwipeToDismissBoxValue, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for SwipeToDismissBox {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let start = Rc::new(Cell::new(None));
        let pointer_down = start.clone();
        let callback = self.on_value_change;
        div()
            .id(self.id)
            .relative()
            .when(self.enabled, |el| {
                el.on_mouse_down(MouseButton::Left, move |event, _, _| {
                    pointer_down.set(Some(event.position.x));
                })
                .on_mouse_up(MouseButton::Left, move |event, window, cx| {
                    let Some(start_x) = start.replace(None) else {
                        return;
                    };
                    let distance = f32::from(event.position.x - start_x);
                    let result = if distance > 80. {
                        SwipeToDismissBoxValue::StartToEnd
                    } else if distance < -80. {
                        SwipeToDismissBoxValue::EndToStart
                    } else {
                        SwipeToDismissBoxValue::Settled
                    };
                    if result != SwipeToDismissBoxValue::Settled
                        && let Some(handler) = &callback
                    {
                        handler(result, window, cx);
                    }
                })
            })
            .when_some(self.background, |el, b| el.child(b))
            .when(
                self.state.current_value == SwipeToDismissBoxValue::Settled,
                |el| el.child(self.content),
            )
    }
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimePickerState {
    pub hour: u8,

    pub minute: u8,
}

impl TimePickerState {
    pub fn new(hour: u8, minute: u8) -> Self {
        Self {
            hour: hour.min(23),
            minute: minute.min(59),
        }
    }
}

type TimeChangeHandler = Rc<dyn Fn(TimePickerState, &mut Window, &mut App)>;

fn time_adjustment_button(
    id: ElementId,
    label: &'static str,
    next: TimePickerState,
    handler: Option<TimeChangeHandler>,
) -> AnyElement {
    div()
        .id(id)
        .size(px(36.))
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .when_some(handler, |el, handler| {
            el.cursor_pointer()
                .on_click(move |_, window, cx| handler(next, window, cx))
        })
        .child(label)
        .into_any_element()
}

#[derive(IntoElement)]
pub struct TimePicker {
    id: ElementId,
    value: SharedString,
    state: TimePickerState,
    on_time_change: Option<TimeChangeHandler>,
}
impl TimePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: "Select time".into(),
            state: TimePickerState::new(0, 0),
            on_time_change: None,
        }
    }

    pub fn value(mut self, v: impl Into<SharedString>) -> Self {
        self.value = v.into();
        self
    }

    pub fn state(mut self, state: TimePickerState) -> Self {
        self.state = state;
        self
    }

    pub fn on_time_change(
        mut self,
        handler: impl Fn(TimePickerState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_time_change = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for TimePicker {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let hour = self.state.hour;
        let minute = self.state.minute;
        let adjust = self.on_time_change.clone();
        let hour_controls = div()
            .flex()
            .flex_col()
            .items_center()
            .child(time_adjustment_button(
                (self.id.clone(), "hour-plus").into(),
                "+",
                TimePickerState::new((hour + 1) % 24, minute),
                adjust.clone(),
            ))
            .child(format!("{hour:02}"))
            .child(time_adjustment_button(
                (self.id.clone(), "hour-minus").into(),
                "−",
                TimePickerState::new((hour + 23) % 24, minute),
                adjust.clone(),
            ));
        let minute_controls = div()
            .flex()
            .flex_col()
            .items_center()
            .child(time_adjustment_button(
                (self.id.clone(), "minute-plus").into(),
                "+",
                TimePickerState::new(hour, (minute + 1) % 60),
                adjust.clone(),
            ))
            .child(format!("{minute:02}"))
            .child(time_adjustment_button(
                (self.id.clone(), "minute-minus").into(),
                "−",
                TimePickerState::new(hour, (minute + 59) % 60),
                adjust,
            ));
        div()
            .id(self.id)
            .p(px(16.))
            .rounded(px(12.))
            .bg(cx.theme().colors().surface_container_high)
            .text_color(cx.theme().colors().on_surface)
            .child(self.value)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(hour_controls)
                    .child(":")
                    .child(minute_controls),
            )
    }
}
pub type ExposedTimePicker = TimePicker;

pub struct TimeInput {
    id: ElementId,
    state: TimePickerState,
    enabled: bool,
    on_time_change: Option<TimeChangeHandler>,
}

impl TimeInput {
    pub fn new(id: impl Into<ElementId>, state: TimePickerState) -> Self {
        Self {
            id: id.into(),
            state,
            enabled: true,
            on_time_change: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn on_time_change(
        mut self,
        handler: impl Fn(TimePickerState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_time_change = Some(Rc::new(handler));
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<TextFieldState> {
        let mut input = TextField::new(self.id, "Time")
            .value(format!("{:02}:{:02}", self.state.hour, self.state.minute))
            .enabled(self.enabled);
        if let Some(handler) = self.on_time_change {
            input = input.on_value_change(move |text, window, cx| {
                if let Some((hour, minute)) = text.split_once(':')
                    && let (Ok(hour), Ok(minute)) = (hour.parse::<u8>(), minute.parse::<u8>())
                    && hour < 24
                    && minute < 60
                {
                    handler(TimePickerState::new(hour, minute), window, cx);
                }
            });
        }
        input.build(cx)
    }
}

#[derive(IntoElement)]
pub struct WideNavigationRail {
    id: ElementId,
    children: Vec<gpui::AnyElement>,
}
impl WideNavigationRail {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
        }
    }
}
impl ParentElement for WideNavigationRail {
    fn extend(&mut self, e: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(e)
    }
}
impl RenderOnce for WideNavigationRail {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .w(px(256.))
            .h_full()
            .flex()
            .flex_col()
            .gap(px(8.))
            .p(px(12.))
            .bg(cx.theme().colors().surface_container)
            .children(self.children)
    }
}

#[cfg(test)]
mod date_time_tests {
    use super::{
        DatePickerState, TimePickerState, days_in_month, weekday_of_first, year_month_from_days,
    };

    #[test]
    fn gregorian_month_lengths_and_weekdays_are_valid() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2100, 2), 28);
        assert_eq!(days_in_month(2000, 2), 29);
        assert_eq!(weekday_of_first(2026, 10), 3);
        assert_eq!(year_month_from_days(0), (1970, 1));
        assert_eq!(DatePickerState::new(2025, 2, Some(29)).selected_day, None);
    }

    #[test]
    fn time_state_limits_invalid_values() {
        assert_eq!(TimePickerState::new(24, 60), TimePickerState::new(23, 59));
    }
}
