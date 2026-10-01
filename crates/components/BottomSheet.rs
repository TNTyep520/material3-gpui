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
    AnyElement, App, ElementId, Hsla, IntoElement, ParentElement as _, Pixels, RenderOnce, Window,
    anchored, deferred, div, point, prelude::*, px,
};
use std::rc::Rc;

use crate::prelude::ActiveTheme;
use crate::theme::TokenSet;
use crate::tokens::{ScrimTokens, SheetBottomTokens};

#[derive(Clone, Copy, Debug)]
pub struct BottomSheetStyle {
    pub container_color: Hsla,
    pub content_color: Hsla,
    pub scrim_color: Hsla,
    pub handle_color: Hsla,
    pub handle_size: (Pixels, Pixels),
    pub corner_radius: Pixels,
    pub max_width: Pixels,
}

impl BottomSheetStyle {
    pub fn resolve(tokens: &TokenSet) -> Self {
        Self {
            container_color: SheetBottomTokens::DOCKED_CONTAINER_COLOR.resolve(tokens),
            content_color: tokens.colors.on_surface,
            scrim_color: ScrimTokens::CONTAINER_COLOR
                .resolve(tokens)
                .opacity(ScrimTokens::CONTAINER_OPACITY),
            handle_color: SheetBottomTokens::DOCKED_DRAG_HANDLE_COLOR.resolve(tokens),
            handle_size: (
                SheetBottomTokens::DOCKED_DRAG_HANDLE_WIDTH.pixels(),
                SheetBottomTokens::DOCKED_DRAG_HANDLE_HEIGHT.pixels(),
            ),
            corner_radius: tokens.shapes.extra_large,
            max_width: px(640.),
        }
    }
}

type DismissHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
type ExpandedChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// MD3 模态底部弹层。
#[derive(IntoElement)]
pub struct ModalBottomSheet {
    id: ElementId,
    drag_handle: bool,
    visible: bool,
    children: Vec<AnyElement>,
    on_dismiss: Option<DismissHandler>,
}

/// AndroidX BottomSheetScaffold 对应的常驻底部面板布局。
#[derive(IntoElement)]
pub struct BottomSheetScaffold {
    id: ElementId,
    content: Vec<AnyElement>,
    sheet_content: Vec<AnyElement>,
    top_bar: Option<AnyElement>,
    expanded: bool,
    peek_height: Pixels,
    on_expanded_change: Option<ExpandedChangeHandler>,
}

impl BottomSheetScaffold {
    /// 创建常驻面板，默认折叠并露出 56dp。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            content: Vec::new(),
            sheet_content: Vec::new(),
            top_bar: None,
            expanded: false,
            peek_height: px(56.),
            on_expanded_change: None,
        }
    }

    /// 设置面板内容。
    pub fn sheet_content(mut self, content: impl IntoElement) -> Self {
        self.sheet_content.push(content.into_any_element());
        self
    }

    /// 设置页面顶部应用栏。
    pub fn top_bar(mut self, bar: impl IntoElement) -> Self {
        self.top_bar = Some(bar.into_any_element());
        self
    }

    /// 设置面板是否展开。
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// 设置折叠时露出的面板高度。
    pub fn sheet_peek_height(mut self, height: Pixels) -> Self {
        self.peek_height = height.max(px(0.));
        self
    }

    /// 设置拖动把手点击时的展开状态变化回调。
    pub fn on_expanded_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_expanded_change = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for BottomSheetScaffold {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.content.extend(elements);
    }
}

impl RenderOnce for BottomSheetScaffold {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = BottomSheetStyle::resolve(cx.theme().token_set());
        let content_id = (self.id.clone(), "sheet-content");
        let height = if self.expanded {
            window.viewport_size().height * 0.8
        } else {
            self.peek_height
        };
        let handle = div()
            .id((self.id.clone(), "handle"))
            .w_full()
            .h(px(32.))
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .when_some(self.on_expanded_change, |el, handler| {
                let expanded = self.expanded;
                el.cursor_pointer()
                    .on_click(move |_, window, cx| handler(!expanded, window, cx))
            })
            .child(
                div()
                    .w(style.handle_size.0)
                    .h(style.handle_size.1)
                    .rounded_full()
                    .bg(style.handle_color),
            );
        div()
            .id(self.id)
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .when_some(self.top_bar, |el, top_bar| el.child(top_bar))
            .child(div().flex_1().children(self.content))
            .child(
                div()
                    .absolute()
                    .bottom_0()
                    .w_full()
                    .h(height)
                    .flex()
                    .flex_col()
                    .rounded(style.corner_radius)
                    .rounded_b_none()
                    .bg(style.container_color)
                    .child(handle)
                    .child(
                        div()
                            .id(content_id)
                            .flex_1()
                            .overflow_y_scroll()
                            .children(self.sheet_content),
                    ),
            )
    }
}

impl ModalBottomSheet {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            drag_handle: true,
            visible: true,
            children: Vec::new(),
            on_dismiss: None,
        }
    }

    /// 是否显示顶部拖动把手（默认显示）。
    pub fn drag_handle(mut self, drag_handle: bool) -> Self {
        self.drag_handle = drag_handle;
        self
    }

    /// 设置面板可见状态；隐藏时不渲染 scrim 和面板。
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// 点击 scrim 时触发（不设置则点击 scrim 无效果）。
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }

    /// 设置关闭请求回调，对应 AndroidX onDismissRequest。
    pub fn on_dismiss_request(self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss(handler)
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
        let style = BottomSheetStyle::resolve(theme.token_set());
        let viewport = _window.viewport_size();

        let sheet = div()
            .id(self.id.clone())
            .w_full()
            .max_w(style.max_width)
            .on_click(|_, _, cx| cx.stop_propagation())
            .max_h(viewport.height * 0.8)
            .flex()
            .flex_col()
            .rounded(style.corner_radius)
            .rounded_b_none()
            .bg(style.container_color)
            .px(px(16.))
            .pt(px(6.))
            .pb(px(24.))
            .gap(px(12.))
            .when(self.drag_handle, |el| {
                el.child(
                    div().flex().justify_center().child(
                        div()
                            .w(style.handle_size.0)
                            .h(style.handle_size.1)
                            .rounded_full()
                            .bg(style.handle_color),
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
                    .text_color(style.content_color)
                    .children(self.children),
            );

        let scrim = div()
            .id("md3-sheet-scrim")
            .occlude()
            .w(viewport.width)
            .h(viewport.height)
            .flex()
            .items_end()
            .justify_center()
            .bg(style.scrim_color)
            .when_some(self.on_dismiss, |el, handler| {
                el.on_click(move |_, window, cx| handler(window, cx))
            })
            .child(sheet);

        div().when(self.visible, |el| {
            el.child(
                deferred(anchored().position(point(px(0.), px(0.))).child(scrim))
                    .with_priority(100),
            )
        })
    }
}
