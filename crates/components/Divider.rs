//! MD3 Divider（对应 material-web 的 `md-divider`）
//!
//! 1dp 分割线，颜色 outline-variant，支持水平/垂直与 inset。

use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px};

use crate::theme::ActiveTheme;

/// MD3 分割线
#[derive(IntoElement)]
pub struct Divider {
    vertical: bool,
    inset: bool,
}

impl Divider {
    /// 水平分割线
    pub fn horizontal() -> Self {
        Self {
            vertical: false,
            inset: false,
        }
    }

    /// 垂直分割线
    pub fn vertical() -> Self {
        Self {
            vertical: true,
            inset: false,
        }
    }

    /// 两端缩进 16dp
    pub fn inset(mut self) -> Self {
        self.inset = true;
        self
    }
}

impl RenderOnce for Divider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = cx.theme().colors().outline_variant;
        if self.vertical {
            // 外层占位，内层着色，避免 margin 溢出
            div()
                .w(px(1.))
                .h_full()
                .flex_none()
                .when(self.inset, |el| el.py(px(16.)))
                .child(div().w(px(1.)).h_full().bg(color))
        } else {
            div()
                .h(px(1.))
                .w_full()
                .flex_none()
                .when(self.inset, |el| el.px(px(16.)))
                .child(div().h(px(1.)).w_full().bg(color))
        }
    }
}
