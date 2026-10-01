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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Divider.kt

use gpui::{App, Hsla, IntoElement, Pixels, RenderOnce, Window, div, prelude::*};

use crate::theme::ActiveTheme;

#[derive(IntoElement)]
pub struct Divider {
    vertical: bool,
    inset: bool,
    thickness: Option<Pixels>,
    color: Option<Hsla>,
}

pub type HorizontalDivider = Divider;

#[derive(IntoElement)]
pub struct VerticalDivider(Divider);

impl Divider {
    pub fn new() -> Self {
        Self::horizontal()
    }

    pub fn horizontal() -> Self {
        Self {
            vertical: false,
            inset: false,
            thickness: None,
            color: None,
        }
    }

    pub fn vertical() -> Self {
        Self {
            vertical: true,
            inset: false,
            thickness: None,
            color: None,
        }
    }

    pub fn inset(mut self) -> Self {
        self.inset = true;
        self
    }

    pub fn thickness(mut self, thickness: Pixels) -> Self {
        self.thickness = Some(thickness);
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl VerticalDivider {
    pub fn new() -> Self {
        Self(Divider::vertical())
    }

    pub fn thickness(mut self, thickness: Pixels) -> Self {
        self.0 = self.0.thickness(thickness);
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.0 = self.0.color(color);
        self
    }
}

impl Default for VerticalDivider {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for VerticalDivider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.0.render(window, cx)
    }
}

impl RenderOnce for Divider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut style = DividerStyle::resolve(cx.theme().token_set(), self.inset);
        if let Some(thickness) = self.thickness {
            style.thickness = thickness;
        }
        if let Some(color) = self.color {
            style.color = color;
        }
        let color = style.color;
        if self.vertical {
            div()
                .w(style.thickness)
                .h_full()
                .flex_none()
                .when(self.inset, |el| el.py(style.inset))
                .child(div().w(style.thickness).h_full().bg(color))
        } else {
            div()
                .h(style.thickness)
                .w_full()
                .flex_none()
                .when(self.inset, |el| el.px(style.inset))
                .child(div().h(style.thickness).w_full().bg(color))
        }
    }
}

pub use appearance::DividerStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};

    #[derive(Clone, Copy, Debug)]
    pub struct DividerStyle {
        pub color: Hsla,

        pub thickness: Pixels,

        pub inset: Pixels,
    }
    impl DividerStyle {
        pub fn resolve(tokens: &TokenSet, inset: bool) -> Self {
            use crate::tokens::DividerTokens;
            Self {
                color: DividerTokens::COLOR.resolve(tokens),
                thickness: DividerTokens::THICKNESS.pixels(),
                inset: if inset { px(16.) } else { px(0.) },
            }
        }
    }
}
