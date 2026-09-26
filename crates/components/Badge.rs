//! MD3 Badge（对应 compose material3 的 `BadgedBox` + `Badge`）
//!
//! 规格：纯圆点 6dp、带数字徽标最小 16dp 高、error 底 + on_error 文字、
//! 圆角全圆。与任意元素组合时以相对定位叠放在右上角：
//!
//! ```ignore
//! badged(Icon::new(IconName::Notifications).size(px(24.)), Badge::new().label("3"))
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled,
    Window, div, prelude::*, px,
};

use crate::prelude::ActiveTheme;

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
        let colors = theme.colors();

        match self.label {
            Some(label) => div()
                .id(self.id)
                .min_w(px(16.))
                .h(px(16.))
                .px(px(4.))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .bg(colors.error)
                .text_color(colors.on_error)
                .text_size(px(12.))
                .child(label),
            None => div()
                .id(self.id)
                .size(px(6.))
                .rounded_full()
                .bg(colors.error),
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
