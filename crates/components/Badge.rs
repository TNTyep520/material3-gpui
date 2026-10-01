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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Badge.kt

use gpui::{
    AnyElement, App, ElementId, Hsla, IntoElement, ParentElement as _, Pixels, RenderOnce,
    SharedString, Styled, Window, div, prelude::*, px,
};

use crate::prelude::ActiveTheme;
use crate::theme::{TokenSet, TypeStyle};
use crate::tokens::BadgeTokens;

#[derive(Clone, Copy, Debug)]
pub struct BadgeStyle {
    pub container_color: Hsla,
    pub content_color: Hsla,
    pub size: Pixels,
    pub horizontal_padding: Pixels,
    pub label: TypeStyle,
}

impl BadgeStyle {
    pub fn resolve(tokens: &TokenSet, has_label: bool) -> Self {
        Self {
            container_color: BadgeTokens::COLOR.resolve(tokens),
            content_color: BadgeTokens::LARGE_LABEL_TEXT_COLOR.resolve(tokens),
            size: if has_label {
                BadgeTokens::LARGE_SIZE.pixels()
            } else {
                BadgeTokens::SIZE.pixels()
            },
            horizontal_padding: px(4.),
            label: BadgeTokens::LARGE_LABEL_TEXT_FONT.resolve(tokens),
        }
    }
}

#[derive(IntoElement)]
pub struct Badge {
    id: ElementId,
    label: Option<SharedString>,
}

impl Badge {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let style = BadgeStyle::resolve(theme.token_set(), self.label.is_some());

        match self.label {
            Some(label) => style
                .label
                .apply(div())
                .id(self.id)
                .min_w(style.size)
                .h(style.size)
                .px(style.horizontal_padding)
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .bg(style.container_color)
                .text_color(style.content_color)
                .child(label),
            None => div()
                .id(self.id)
                .size(style.size)
                .rounded_full()
                .bg(style.container_color),
        }
    }
}

pub fn badged(anchor: impl IntoElement, badge: Badge) -> AnyElement {
    div()
        .relative()
        .child(anchor.into_any_element())
        .child(div().absolute().top(px(-4.)).right(px(-6.)).child(badge))
        .into_any_element()
}

#[derive(IntoElement)]
pub struct BadgedBox {
    anchor: AnyElement,
    badge: Badge,
}

impl BadgedBox {
    pub fn new(anchor: impl IntoElement, badge: Badge) -> Self {
        Self {
            anchor: anchor.into_any_element(),
            badge,
        }
    }
}

impl RenderOnce for BadgedBox {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        badged(self.anchor, self.badge)
    }
}
