//! MD3 Scaffold（对应 compose material3 的 `Scaffold`）
//!
//! 页面骨架:顶栏 / 底栏 / FAB 槽位 + 内容区。内容区自动填满剩余空间,
//! FAB 悬浮于内容区右下(在底栏之上):
//!
//! ```ignore
//! Scaffold::new("scaffold")
//!     .top_bar(TopAppBar::small("bar").title("Title"))
//!     .bottom_bar(BottomAppBar)
//!     .fab(Fab::new("fab", IconName::Add))
//!     .child(content)
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement as _, RenderOnce, Styled, Window, div,
    prelude::*, px,
};

use crate::prelude::ActiveTheme;

/// MD3 页面骨架。
#[derive(IntoElement)]
pub struct Scaffold {
    id: ElementId,
    top_bar: Option<AnyElement>,
    bottom_bar: Option<AnyElement>,
    fab: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl Scaffold {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            top_bar: None,
            bottom_bar: None,
            fab: None,
            children: Vec::new(),
        }
    }

    /// 顶部应用栏槽位。
    pub fn top_bar(mut self, top_bar: impl IntoElement) -> Self {
        self.top_bar = Some(top_bar.into_any_element());
        self
    }

    /// 底部应用栏槽位。
    pub fn bottom_bar(mut self, bottom_bar: impl IntoElement) -> Self {
        self.bottom_bar = Some(bottom_bar.into_any_element());
        self
    }

    /// 悬浮动作按钮槽位(内容区右下角)。
    pub fn fab(mut self, fab: impl IntoElement) -> Self {
        self.fab = Some(fab.into_any_element());
        self
    }
}

impl ParentElement for Scaffold {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Scaffold {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        let content = div()
            .relative()
            .flex_1()
            .min_h_0()
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(colors.surface)
            .children(self.children)
            .when_some(self.fab, |el, fab| {
                el.child(div().absolute().right(px(16.)).bottom(px(16.)).child(fab))
            });

        div()
            .id(self.id)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .bg(colors.surface)
            .when_some(self.top_bar, |el, top_bar| el.child(top_bar))
            .child(content)
            .when_some(self.bottom_bar, |el, bottom_bar| el.child(bottom_bar))
    }
}
