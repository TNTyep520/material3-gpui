//! MD3 List / ListItem（对应 material-web 的 `md-list` / `md-list-item`）
//!
//! 规格：单行 56dp、双行 72dp，左右内边距 16dp，headline 用 body-large、
//! supporting 用 body-medium、trailing 用 label-small。

use gpui::{
    AnyElement, App, ClickEvent, ElementId, IntoElement, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};

use crate::icon::{Icon, IconName};
use crate::theme::{ActiveTheme, HOVER_OPACITY, PRESSED_OPACITY};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// MD3 列表容器（简单的纵向 flex + 上下 8dp 内边距）
#[derive(IntoElement)]
pub struct List {
    children: Vec<AnyElement>,
}

impl List {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl gpui::ParentElement for List {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for List {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        div()
            .flex()
            .flex_col()
            .w_full()
            .py(px(8.))
            .bg(colors.surface)
            .children(self.children)
    }
}

/// MD3 列表项
#[derive(IntoElement)]
pub struct ListItem {
    id: ElementId,
    headline: SharedString,
    supporting_text: Option<SharedString>,
    trailing_text: Option<SharedString>,
    leading_icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    leading: Option<AnyElement>,
    trailing: Option<AnyElement>,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

impl ListItem {
    pub fn new(id: impl Into<ElementId>, headline: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            headline: headline.into(),
            supporting_text: None,
            trailing_text: None,
            leading_icon: None,
            trailing_icon: None,
            leading: None,
            trailing: None,
            disabled: false,
            on_click: None,
        }
    }

    /// 第二行辅助文字（设置后条目高度变为 72dp）
    pub fn supporting_text(mut self, text: impl Into<SharedString>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// 尾部说明文字（如时间戳）
    pub fn trailing_text(mut self, text: impl Into<SharedString>) -> Self {
        self.trailing_text = Some(text.into());
        self
    }

    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    /// 自定义头部元素（头像等），优先于 leading_icon
    pub fn leading(mut self, element: impl IntoElement) -> Self {
        self.leading = Some(element.into_any_element());
        self
    }

    /// 自定义尾部元素（如 Checkbox / Switch），优先于 trailing_icon
    pub fn trailing(mut self, element: impl IntoElement) -> Self {
        self.trailing = Some(element.into_any_element());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置条目的启用状态；禁用时移除点击交互。
    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ListItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();
        let disabled = self.disabled;
        let two_line = self.supporting_text.is_some();
        let style = ListItemStyle::resolve(theme.token_set());
        let height = if two_line {
            style.height_two_line
        } else {
            style.height_single_line
        };
        let layer = colors.on_surface;

        let headline_style = style.headline;
        let supporting_style = style.supporting;
        let trailing_style = style.trailing;

        div()
            .id(self.id)
            .min_h(height)
            .w_full()
            .flex()
            .flex_none()
            .items_center()
            .gap(style.gap)
            .px(style.horizontal_padding)
            .py(style.vertical_padding)
            .when(!disabled && self.on_click.is_some(), |el| {
                el.cursor_pointer()
                    .hover(move |s| s.bg(layer.opacity(HOVER_OPACITY)))
                    .active(move |s| s.bg(layer.opacity(PRESSED_OPACITY)))
            })
            .when_some(self.on_click.filter(|_| !disabled), |el, handler| {
                el.on_click(move |event, window, cx| handler(event, window, cx))
            })
            // leading（自定义元素优先于图标）
            .when_some(
                self.leading_icon.filter(|_| self.leading.is_none()),
                |el, icon| {
                    el.child(
                        Icon::new(icon)
                            .size(px(24.))
                            .color(colors.on_surface_variant),
                    )
                },
            )
            .when_some(self.leading, |el, leading| el.child(leading))
            // 中间文本区
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        headline_style
                            .apply(div())
                            .text_color(colors.on_surface)
                            .child(self.headline),
                    )
                    .when_some(self.supporting_text, |el, text| {
                        el.child(
                            supporting_style
                                .apply(div())
                                .text_color(colors.on_surface_variant)
                                .child(text),
                        )
                    }),
            )
            // trailing
            .when_some(self.trailing_text, |el, text| {
                el.child(
                    trailing_style
                        .apply(div())
                        .text_color(colors.on_surface_variant)
                        .child(text),
                )
            })
            .when_some(self.trailing_icon, |el, icon| {
                el.child(
                    Icon::new(icon)
                        .size(px(24.))
                        .color(colors.on_surface_variant),
                )
            })
            .when_some(self.trailing, |el, trailing| el.child(trailing))
    }
}

/// AndroidX SegmentedListItem 对应的连组列表项。
#[derive(IntoElement)]
pub struct SegmentedListItem {
    item: ListItem,
    index: usize,
    count: usize,
}

impl SegmentedListItem {
    /// 创建位于指定位置的连组列表项。
    pub fn new(
        id: impl Into<ElementId>,
        headline: impl Into<SharedString>,
        index: usize,
        count: usize,
    ) -> Self {
        Self {
            item: ListItem::new(id, headline),
            index,
            count,
        }
    }

    /// 设置辅助文字。
    pub fn supporting_text(mut self, text: impl Into<SharedString>) -> Self {
        self.item = self.item.supporting_text(text);
        self
    }

    /// 设置条目是否可交互。
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.item = self.item.enabled(enabled);
        self
    }

    /// 设置点击回调。
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.item = self.item.on_click(handler);
        self
    }
}

impl RenderOnce for SegmentedListItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let radius = cx.theme().shapes().medium;
        let first = self.index == 0;
        let last = self.index + 1 >= self.count;
        div()
            .w_full()
            .overflow_hidden()
            .bg(cx.theme().colors().surface_container)
            .when(first, |el| el.rounded_tl(radius).rounded_tr(radius))
            .when(last, |el| el.rounded_bl(radius).rounded_br(radius))
            .child(self.item)
    }
}

pub use appearance::ListItemStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};
    /// MD3 列表项样式。
    #[derive(Clone, Copy, Debug)]
    pub struct ListItemStyle {
        /// 单行高度。
        pub height_single_line: Pixels,
        /// 双行高度。
        pub height_two_line: Pixels,
        /// 内容色。
        pub content_color: Hsla,
        /// 支撑文本色。
        pub supporting_color: Hsla,
        /// 尾随元素色。
        pub trailing_color: Hsla,
        /// 水平内边距。
        pub horizontal_padding: Pixels,
        /// 垂直内边距。
        pub vertical_padding: Pixels,
        /// 元素间距。
        pub gap: Pixels,
        /// 图标尺寸。
        pub icon_size: Pixels,
        /// hover 状态层不透明度。
        pub hover_opacity: f32,
        /// 标题字型。
        pub headline: crate::theme::TypeStyle,
        /// 支撑文本字型。
        pub supporting: crate::theme::TypeStyle,
        /// 尾随字型。
        pub trailing: crate::theme::TypeStyle,
    }
    impl ListItemStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(tokens: &TokenSet) -> Self {
            let colors = &tokens.colors;
            Self {
                height_single_line: px(56.),
                height_two_line: px(72.),
                content_color: colors.on_surface,
                supporting_color: colors.on_surface_variant,
                trailing_color: colors.on_surface_variant,
                horizontal_padding: px(16.),
                vertical_padding: px(8.),
                gap: px(16.),
                icon_size: px(24.),
                hover_opacity: crate::theme::HOVER_OPACITY,
                headline: tokens.typography.body_large,
                supporting: tokens.typography.body_medium,
                trailing: tokens.typography.label_small,
            }
        }
    }
}
