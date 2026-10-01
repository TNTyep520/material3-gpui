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
    AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Window,
    anchored, deferred, div, point, prelude::*, px,
};
use std::rc::Rc;

use crate::icon::{Icon, IconName};
use crate::theme::ActiveTheme;

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

/// AndroidX AlertDialog 对应的带标题、正文和操作区的对话框。
pub type AlertDialog = Dialog;
/// AndroidX BasicAlertDialog 对应的可自定义内容对话框。
pub type BasicAlertDialog = Dialog;

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

impl ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

macro_rules! picker_dialog {
    ($name:ident) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的选择器对话框。")]
        #[derive(IntoElement)]
        pub struct $name(Dialog);

        impl $name {
            /// 创建包含日期或时间选择器的对话框。
            pub fn new(id: impl Into<ElementId>, picker: impl IntoElement) -> Self {
                Self(Dialog::new(id).child(picker))
            }

            /// 设置对话框标题。
            pub fn title(mut self, title: impl Into<SharedString>) -> Self {
                self.0 = self.0.title(title);
                self
            }

            /// 添加操作按钮。
            pub fn action(mut self, action: impl IntoElement) -> Self {
                self.0 = self.0.action(action);
                self
            }

            /// 设置关闭请求回调。
            pub fn on_dismiss_request(
                mut self,
                handler: impl Fn(&mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_dismiss(handler);
                self
            }
        }

        impl RenderOnce for $name {
            fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
                self.0.render(window, cx)
            }
        }
    };
}

picker_dialog!(DatePickerDialog);
picker_dialog!(TimePickerDialog);

impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let style = DialogStyle::resolve(theme.token_set());
        let viewport = window.viewport_size();

        let title_style = style.title;
        let body_style = style.body;

        let container = div()
            .id(self.id.clone())
            .occlude()
            // 阻止容器内点击冒泡到 scrim 触发关闭
            .on_click(|_, _, cx| cx.stop_propagation())
            .min_w(style.width_range.0)
            .max_w(style.width_range.1)
            .max_h(viewport.height - px(96.))
            .flex()
            .flex_col()
            .rounded(style.corner_radius)
            .bg(style.container_color)
            .shadow(style.elevation.shadows(style.shadow_color))
            .p(style.padding)
            .gap(style.gap)
            .when_some(self.icon, |el, icon| {
                el.child(
                    div()
                        .flex()
                        .justify_center()
                        .child(Icon::new(icon).size(px(24.)).color(style.icon_color)),
                )
            })
            .when_some(self.title, |el, title| {
                let centered = self.icon.is_some();
                el.child(
                    title_style
                        .apply(div())
                        .text_color(style.content_color)
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
                    .text_color(style.supporting_color)
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
            .bg(style.scrim_color.opacity(style.scrim_opacity))
            .when_some(self.on_dismiss, |el, handler| {
                el.on_click(move |_, window, cx| handler(window, cx))
            })
            .child(container);

        deferred(anchored().position(point(px(0.), px(0.))).child(scrim)).with_priority(100)
    }
}

pub use appearance::DialogStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};
    /// MD3 对话框样式。
    #[derive(Clone, Copy, Debug)]
    pub struct DialogStyle {
        /// 容器色。
        pub container_color: Hsla,
        /// 内容色。
        pub content_color: Hsla,
        /// 辅助文本色。
        pub supporting_color: Hsla,
        /// 图标色。
        pub icon_color: Hsla,
        /// scrim 颜色。
        pub scrim_color: Hsla,
        /// scrim 不透明度。
        pub scrim_opacity: f32,
        /// 圆角。
        pub corner_radius: Pixels,
        /// 最小/最大宽度。
        pub width_range: (Pixels, Pixels),
        /// 内边距。
        pub padding: Pixels,
        /// 元素间距。
        pub gap: Pixels,
        /// 阴影颜色。
        pub shadow_color: Hsla,
        /// 阴影等级。
        pub elevation: crate::theme::Elevation,
        /// 标题字型。
        pub title: crate::theme::TypeStyle,
        /// 正文字型。
        pub body: crate::theme::TypeStyle,
    }
    impl DialogStyle {
        /// 由令牌推导默认样式(DialogTokens / ScrimTokens)。
        pub fn resolve(tokens: &TokenSet) -> Self {
            use crate::tokens::{DialogTokens, ScrimTokens};
            let colors = &tokens.colors;
            Self {
                container_color: DialogTokens::CONTAINER_COLOR.resolve(tokens),
                content_color: DialogTokens::HEADLINE_COLOR.resolve(tokens),
                supporting_color: DialogTokens::SUPPORTING_TEXT_COLOR.resolve(tokens),
                icon_color: DialogTokens::ICON_COLOR.resolve(tokens),
                scrim_color: ScrimTokens::CONTAINER_COLOR.resolve(tokens),
                scrim_opacity: ScrimTokens::CONTAINER_OPACITY,
                corner_radius: tokens.shapes.extra_large,
                width_range: (px(280.), px(560.)),
                padding: px(24.),
                gap: px(16.),
                shadow_color: colors.shadow,
                elevation: crate::theme::Elevation::Level3,
                title: DialogTokens::HEADLINE_FONT.resolve(tokens),
                body: DialogTokens::SUPPORTING_TEXT_FONT.resolve(tokens),
            }
        }
    }
}
