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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/SwipeToDismissBox.kt

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    App, ElementId, IntoElement, MouseButton, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder as _, prelude::*,
};

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
