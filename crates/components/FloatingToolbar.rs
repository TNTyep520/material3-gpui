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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingToolbar.kt

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement as _, RenderOnce, Styled, Window, div,
    prelude::*, px,
};

use crate::theme::ActiveTheme;

#[derive(IntoElement)]
pub struct FloatingToolbar {
    id: ElementId,
    children: Vec<AnyElement>,
    vertical: bool,
    vibrant: bool,
    state: FloatingToolbarState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FloatingToolbarState {
    pub expanded: bool,
}

impl FloatingToolbarState {
    pub fn new(expanded: bool) -> Self {
        Self { expanded }
    }

    pub fn expand(&mut self) {
        self.expanded = true;
    }

    pub fn collapse(&mut self) {
        self.expanded = false;
    }
}

impl Default for FloatingToolbarState {
    fn default() -> Self {
        Self::new(true)
    }
}

pub type HorizontalFloatingToolbar = FloatingToolbar;

#[derive(IntoElement)]
pub struct VerticalFloatingToolbar(FloatingToolbar);

impl VerticalFloatingToolbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self(FloatingToolbar::new(id).vertical())
    }

    pub fn state(mut self, state: FloatingToolbarState) -> Self {
        self.0 = self.0.state(state);
        self
    }

    pub fn vibrant(mut self, vibrant: bool) -> Self {
        self.0 = self.0.vibrant(vibrant);
        self
    }
}

impl ParentElement for VerticalFloatingToolbar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.0.extend(elements);
    }
}

impl RenderOnce for VerticalFloatingToolbar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.0.render(window, cx)
    }
}

impl FloatingToolbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            vertical: false,
            vibrant: false,
            state: FloatingToolbarState::default(),
        }
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    pub fn vibrant(mut self, vibrant: bool) -> Self {
        self.vibrant = vibrant;
        self
    }

    pub fn state(mut self, state: FloatingToolbarState) -> Self {
        self.state = state;
        self
    }
}

impl ParentElement for FloatingToolbar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for FloatingToolbar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        use crate::tokens::FloatingToolbarTokens as Tokens;
        let theme = cx.theme();
        let tokens = theme.token_set();
        let container_color = if self.vibrant {
            Tokens::VIBRANT_CONTAINER_COLOR.resolve(tokens)
        } else {
            Tokens::STANDARD_CONTAINER_COLOR.resolve(tokens)
        };
        let content_color = if self.vibrant {
            theme.colors().on_primary_container
        } else {
            theme.colors().on_surface_variant
        };
        let slot = px(48.);
        let container = if self.vertical {
            div().w(px(f32::from(slot) + 16.0)).h_auto()
        } else {
            div().h(Tokens::CONTAINER_HEIGHT.pixels())
        };
        container
            .id(self.id)
            .flex_none()
            .flex()
            .when(self.vertical, |el| el.flex_col())
            .items_center()
            .justify_center()
            .gap(Tokens::CONTAINER_BETWEEN_SPACE.pixels())
            .p(px(8.))
            .rounded_full()
            .bg(container_color)
            .text_color(content_color)
            .shadow_lg()
            .when(self.state.expanded, |el| {
                el.children(self.children.into_iter().map(|child| {
                    div()
                        .size(slot)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(child)
                }))
            })
    }
}
