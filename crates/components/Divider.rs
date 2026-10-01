//! MD3 Divider（对应 material-web 的 `md-divider`）
//!
//! 1dp 分割线，颜色 outline-variant，支持水平/垂直与 inset。

use gpui::{App, Hsla, IntoElement, Pixels, RenderOnce, Window, div, prelude::*};

use crate::theme::ActiveTheme;

/// MD3 分割线
#[derive(IntoElement)]
pub struct Divider {
    vertical: bool,
    inset: bool,
    thickness: Option<Pixels>,
    color: Option<Hsla>,
}

/// AndroidX HorizontalDivider 对应的水平分割线。
pub type HorizontalDivider = Divider;

/// AndroidX VerticalDivider 对应的垂直分割线。
#[derive(IntoElement)]
pub struct VerticalDivider(Divider);

impl Divider {
    /// 创建默认的水平分割线。
    pub fn new() -> Self {
        Self::horizontal()
    }

    /// 水平分割线
    pub fn horizontal() -> Self {
        Self {
            vertical: false,
            inset: false,
            thickness: None,
            color: None,
        }
    }

    /// 垂直分割线
    pub fn vertical() -> Self {
        Self {
            vertical: true,
            inset: false,
            thickness: None,
            color: None,
        }
    }

    /// 两端缩进 16dp
    pub fn inset(mut self) -> Self {
        self.inset = true;
        self
    }

    /// 设置线条厚度。
    pub fn thickness(mut self, thickness: Pixels) -> Self {
        self.thickness = Some(thickness);
        self
    }

    /// 设置线条颜色。
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
    /// 创建默认的垂直分割线。
    pub fn new() -> Self {
        Self(Divider::vertical())
    }

    /// 设置线条厚度。
    pub fn thickness(mut self, thickness: Pixels) -> Self {
        self.0 = self.0.thickness(thickness);
        self
    }

    /// 设置线条颜色。
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
            // 外层占位，内层着色，避免 margin 溢出
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
    /// MD3 分隔线样式。
    #[derive(Clone, Copy, Debug)]
    pub struct DividerStyle {
        /// 颜色。
        pub color: Hsla,
        /// 厚度。
        pub thickness: Pixels,
        /// inset 缩进。
        pub inset: Pixels,
    }
    impl DividerStyle {
        /// 由令牌推导默认样式。
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
