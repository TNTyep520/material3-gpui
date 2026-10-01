//! MD3 Badge（对应 compose material3 的 `BadgedBox` + `Badge`）
//!
//! 规格：纯圆点 6dp、带数字徽标最小 16dp 高、error 底 + on_error 文字、
//! 圆角全圆。与任意元素组合时以相对定位叠放在右上角：
//!
//! ```ignore
//! badged(Icon::new(IconName::Notifications).size(px(24.)), Badge::new().label("3"))
//! ```

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

/// MD3 徽标：未设置 label 时渲染为圆点，否则渲染为带文字的胶囊。
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

    /// 徽标文字（如未读数）；不设置则为纯圆点。
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

/// 把徽标叠放到任意元素的右上角（compose `BadgedBox` 的等价用法）。
pub fn badged(anchor: impl IntoElement, badge: Badge) -> AnyElement {
    div()
        .relative()
        .child(anchor.into_any_element())
        .child(div().absolute().top(px(-4.)).right(px(-6.)).child(badge))
        .into_any_element()
}

/// AndroidX BadgedBox 对应的徽标叠放容器。
#[derive(IntoElement)]
pub struct BadgedBox {
    anchor: AnyElement,
    badge: Badge,
}

impl BadgedBox {
    /// 创建带徽标的内容容器。
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
