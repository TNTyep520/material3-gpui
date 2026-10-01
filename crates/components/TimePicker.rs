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

use gpui::{
    AnyElement, App, ElementId, Entity, IntoElement, RenderOnce, SharedString, Styled, Window, div,
    prelude::FluentBuilder as _, prelude::*, px,
};

use crate::components::{TextField, TextFieldState};
use crate::theme::ActiveTheme;

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

#[cfg(test)]
mod tests {
    use super::TimePickerState;

    #[test]
    fn time_state_limits_invalid_values() {
        assert_eq!(TimePickerState::new(24, 60), TimePickerState::new(23, 59));
    }
}
