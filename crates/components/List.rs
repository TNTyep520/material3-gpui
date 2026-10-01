// Copyright (c) 2026 TNTyep520
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ListItem.kt

use gpui::{
    AnyElement, App, ClickEvent, ElementId, IntoElement, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};

use crate::icon::{Icon, IconName};
use crate::theme::{ActiveTheme, HOVER_OPACITY, PRESSED_OPACITY};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

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

    pub fn supporting_text(mut self, text: impl Into<SharedString>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

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

    pub fn leading(mut self, element: impl IntoElement) -> Self {
        self.leading = Some(element.into_any_element());
        self
    }

    pub fn trailing(mut self, element: impl IntoElement) -> Self {
        self.trailing = Some(element.into_any_element());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

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

#[derive(IntoElement)]
pub struct SegmentedListItem {
    item: ListItem,
    index: usize,
    count: usize,
}

impl SegmentedListItem {
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

    pub fn supporting_text(mut self, text: impl Into<SharedString>) -> Self {
        self.item = self.item.supporting_text(text);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.item = self.item.enabled(enabled);
        self
    }

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

    #[derive(Clone, Copy, Debug)]
    pub struct ListItemStyle {
        pub height_single_line: Pixels,

        pub height_two_line: Pixels,

        pub content_color: Hsla,

        pub supporting_color: Hsla,

        pub trailing_color: Hsla,

        pub horizontal_padding: Pixels,

        pub vertical_padding: Pixels,

        pub gap: Pixels,

        pub icon_size: Pixels,

        pub hover_opacity: f32,

        pub headline: crate::theme::TypeStyle,

        pub supporting: crate::theme::TypeStyle,

        pub trailing: crate::theme::TypeStyle,
    }
    impl ListItemStyle {
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
