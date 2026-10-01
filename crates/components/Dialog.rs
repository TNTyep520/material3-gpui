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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AlertDialog.kt

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Window,
    anchored, deferred, div, point, prelude::*, px,
};
use std::rc::Rc;

use crate::icon::{Icon, IconName};
use crate::theme::ActiveTheme;

type DismissHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    icon: Option<IconName>,
    title: Option<SharedString>,
    children: Vec<AnyElement>,
    actions: Vec<AnyElement>,
    on_dismiss: Option<DismissHandler>,
}

pub type AlertDialog = Dialog;

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

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }

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
            pub fn new(id: impl Into<ElementId>, picker: impl IntoElement) -> Self {
                Self(Dialog::new(id).child(picker))
            }

            pub fn title(mut self, title: impl Into<SharedString>) -> Self {
                self.0 = self.0.title(title);
                self
            }

            pub fn action(mut self, action: impl IntoElement) -> Self {
                self.0 = self.0.action(action);
                self
            }

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
            .when_some(self.icon.clone(), |el, icon| {
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

    #[derive(Clone, Copy, Debug)]
    pub struct DialogStyle {
        pub container_color: Hsla,

        pub content_color: Hsla,

        pub supporting_color: Hsla,

        pub icon_color: Hsla,

        pub scrim_color: Hsla,

        pub scrim_opacity: f32,

        pub corner_radius: Pixels,

        pub width_range: (Pixels, Pixels),

        pub padding: Pixels,

        pub gap: Pixels,

        pub shadow_color: Hsla,

        pub elevation: crate::theme::Elevation,

        pub title: crate::theme::TypeStyle,

        pub body: crate::theme::TypeStyle,
    }
    impl DialogStyle {
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
