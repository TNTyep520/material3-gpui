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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ButtonGroup.kt

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, Pixels, RenderOnce, Window, div,
    prelude::*,
};

use crate::theme::{ActiveTheme, TokenSet};
use crate::tokens::ButtonGroupSmallTokens;

#[derive(Clone, Copy, Debug)]
pub struct ButtonGroupStyle {
    pub spacing: Pixels,
}

impl ButtonGroupStyle {
    pub fn resolve(_tokens: &TokenSet) -> Self {
        Self {
            spacing: ButtonGroupSmallTokens::BETWEEN_SPACE.pixels(),
        }
    }
}

#[derive(IntoElement)]
pub struct ButtonGroup {
    id: ElementId,
    children: Vec<AnyElement>,
    vertical: bool,
    spacing: Option<Pixels>,
    expanded_ratio: f32,
    overflow_indicator: Option<AnyElement>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ButtonGroupMenuState {
    is_showing: bool,
}

impl ButtonGroupMenuState {
    pub fn new(initial_is_showing: bool) -> Self {
        Self {
            is_showing: initial_is_showing,
        }
    }

    pub fn is_showing(&self) -> bool {
        self.is_showing
    }

    pub fn show(&mut self) {
        self.is_showing = true;
    }

    pub fn dismiss(&mut self) {
        self.is_showing = false;
    }
}

impl ButtonGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            vertical: false,
            spacing: None,
            expanded_ratio: 1.,
            overflow_indicator: None,
        }
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    pub fn spacing(mut self, spacing: Pixels) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn expanded_ratio(mut self, ratio: f32) -> Self {
        self.expanded_ratio = ratio.max(0.);
        self
    }

    pub fn overflow_indicator(mut self, indicator: impl IntoElement) -> Self {
        self.overflow_indicator = Some(indicator.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }
}

impl ParentElement for ButtonGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = ButtonGroupStyle::resolve(cx.theme().token_set());
        div()
            .id(self.id)
            .flex()
            .when(self.vertical, |el| el.flex_col())
            .gap(self.spacing.unwrap_or(style.spacing) * self.expanded_ratio)
            .children(self.children)
            .when_some(self.overflow_indicator, |el, indicator| el.child(indicator))
    }
}
