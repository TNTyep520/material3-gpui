//! MD3 ModalBottomSheet（对应 compose material3 的 `ModalBottomSheet`）
//!
//! 规格：面板沉底、顶部圆角 28dp、surface-container-low 底色、32% scrim、
//! 顶部居中 DragHandle。作为元素条件渲染（同 Dialog）：
//!
//! ```ignore
//! div().when(self.sheet_open, |el| {
//!     el.child(
//!         ModalBottomSheet::new("sheet")
//!             .child("Sheet content")
//!             .on_dismiss(|window, cx| { /* 点击 scrim 关闭 */ }),
//!     )
//! })
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement as _, RenderOnce, Window, anchored,
    deferred, div, point, prelude::*, px,
};
use std::rc::Rc;

use crate::prelude::ActiveTheme;

type DismissHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// MD3 模态底部弹层。
#[derive(IntoElement)]
pub struct ModalBottomSheet {
    id: ElementId,
    drag_handle: bool,
    children: Vec<AnyElement>,
    on_dismiss: Option<DismissHandler>,
}

impl ModalBottomSheet {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            drag_handle: true,
            children: Vec::new(),
            on_dismiss: None,
        }
    }

    /// 是否显示顶部拖动把手（默认显示）。
    pub fn drag_handle(mut self, drag_handle: bool) -> Self {
        self.drag_handle = drag_handle;
        self
    }

    /// 点击 scrim 时触发（不设置则点击 scrim 无效果）。
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for ModalBottomSheet {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ModalBottomSheet {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let viewport = _window.viewport_size();

        let sheet = div()
            .id(self.id.clone())
            .w_full()
            .max_h(viewport.height * 0.8)
            .flex()
            .flex_col()
            .rounded(theme.shapes().extra_large)
            .rounded_b_none()
            .bg(colors.surface_container_low)
            .px(px(16.))
            .pt(px(6.))
            .pb(px(24.))
            .gap(px(12.))
            .when(self.drag_handle, |el| {
                el.child(
                    div().flex().justify_center().child(
                        div()
                            .w(px(32.))
                            .h(px(4.))
                            .rounded_full()
                            .bg(colors.on_surface_variant.opacity(0.4)),
                    ),
                )
            })
            .child(
                div()
                    .id("md3-sheet-content")
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .overflow_y_scroll()
                    .text_color(colors.on_surface)
                    .children(self.children),
            );

        let scrim = div()
            .id("md3-sheet-scrim")
            .occlude()
            .w(viewport.width)
            .h(viewport.height)
            .flex()
            .items_end()
            .bg(colors.scrim.opacity(0.32))
            .when_some(self.on_dismiss, |el, handler| {
                el.on_click(move |_, window, cx| handler(window, cx))
            })
            .child(sheet);

        deferred(anchored().position(point(px(0.), px(0.))).child(scrim)).with_priority(100)
    }
}
