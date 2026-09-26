//! MD3 Dialog（对应 material-web 的 `md-dialog`）
//!
//! 规格：min 280 / max 560dp 宽、圆角 28dp、surface-container-high 底色、
//! Level3 阴影、32% scrim。作为元素条件渲染：
//!
//! ```ignore
//! div().when(self.dialog_open, |el| {
//!     el.child(
//!         Dialog::new("confirm")
//!             .title("Delete item?")
//!             .child("This action cannot be undone.")
//!             .action(Button::new("cancel", "Cancel").text().on_click(...))
//!             .action(Button::new("ok", "Delete").text().on_click(...))
//!             .on_dismiss(|window, cx| { /* 点击 scrim 关闭 */ }),
//!     )
//! })
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, RenderOnce, SharedString, Window, anchored, deferred,
    div, point, prelude::*, px,
};
use std::rc::Rc;

use crate::icon::{Icon, IconName};
use crate::theme::{ActiveTheme, Elevation};

type DismissHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// MD3 对话框
#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    icon: Option<IconName>,
    title: Option<SharedString>,
    children: Vec<AnyElement>,
    actions: Vec<AnyElement>,
    on_dismiss: Option<DismissHandler>,
}

impl Dialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            icon: None,
            title: None,
            children: Vec::new(),
            actions: Vec::new(),
            on_dismiss: None,
        }
    }

    /// 顶部居中的图标（hero icon）
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 底部操作按钮（从左到右追加，一般用 Text Button）
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }

    /// 点击 scrim 时触发（不设置则点击 scrim 无效果）
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }
}

impl gpui::ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let viewport = window.viewport_size();

        let title_style = theme.typography().headline_small;
        let body_style = theme.typography().body_medium;

        let container = div()
            .id(self.id.clone())
            .occlude()
            // 阻止容器内点击冒泡到 scrim 触发关闭
            .on_click(|_, _, cx| cx.stop_propagation())
            .min_w(px(280.))
            .max_w(px(560.))
            .max_h(viewport.height - px(96.))
            .flex()
            .flex_col()
            .rounded(theme.shapes().extra_large)
            .bg(colors.surface_container_high)
            .shadow(Elevation::Level3.shadows(colors.shadow))
            .p(px(24.))
            .gap(px(16.))
            .when_some(self.icon, |el, icon| {
                el.child(
                    div()
                        .flex()
                        .justify_center()
                        .child(Icon::new(icon).size(px(24.)).color(colors.secondary)),
                )
            })
            .when_some(self.title, |el, title| {
                let centered = self.icon.is_some();
                el.child(
                    title_style
                        .apply(div())
                        .text_color(colors.on_surface)
                        .when(centered, |t| t.text_center())
                        .child(title),
                )
            })
            .child(
                body_style
                    .apply(div())
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .overflow_hidden()
                    .text_color(colors.on_surface_variant)
                    .children(self.children),
            )
            .when(!self.actions.is_empty(), |el| {
                el.child(
                    div()
                        .flex()
                        .justify_end()
                        .items_center()
                        .gap(px(8.))
                        .pt(px(8.))
                        .children(self.actions),
                )
            });

        let scrim = div()
            .id("md3-dialog-scrim")
            .occlude()
            .w(viewport.width)
            .h(viewport.height)
            .flex()
            .items_center()
            .justify_center()
            .bg(colors.scrim.opacity(0.32))
            .when_some(self.on_dismiss, |el, handler| {
                el.on_click(move |_, window, cx| handler(window, cx))
            })
            .child(container);

        deferred(anchored().position(point(px(0.), px(0.))).child(scrim)).with_priority(100)
    }
}
