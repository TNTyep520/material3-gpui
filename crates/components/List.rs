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
        let height = if two_line { px(72.) } else { px(56.) };
        let layer = colors.on_surface;

        let headline_style = theme.typography().body_large;
        let supporting_style = theme.typography().body_medium;
        let trailing_style = theme.typography().label_small;

        div()
            .id(self.id)
            .h(height)
            .w_full()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(16.))
            .px(px(16.))
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
