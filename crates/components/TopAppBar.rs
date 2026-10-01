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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled,
    Window, div, prelude::*, px,
};

use crate::prelude::ActiveTheme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAppBarVariant {
    Small,

    CenterAligned,

    Medium,

    Large,

    MediumFlexible,

    LargeFlexible,
}

#[derive(IntoElement)]
pub struct TopAppBar {
    id: ElementId,
    variant: TopAppBarVariant,
    title: SharedString,
    leading: Vec<AnyElement>,
    actions: Vec<AnyElement>,
    collapsed_fraction: f32,
}

impl TopAppBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: TopAppBarVariant::Small,
            title: SharedString::default(),
            leading: Vec::new(),
            actions: Vec::new(),
            collapsed_fraction: 0.,
        }
    }

    pub fn small(id: impl Into<ElementId>) -> Self {
        Self::new(id).variant(TopAppBarVariant::Small)
    }

    pub fn medium(id: impl Into<ElementId>) -> Self {
        Self::new(id).variant(TopAppBarVariant::Medium)
    }

    pub fn center_aligned(id: impl Into<ElementId>) -> Self {
        Self::new(id).variant(TopAppBarVariant::CenterAligned)
    }

    pub fn large(id: impl Into<ElementId>) -> Self {
        Self::new(id).variant(TopAppBarVariant::Large)
    }

    pub fn variant(mut self, variant: TopAppBarVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn collapsed_fraction(mut self, fraction: f32) -> Self {
        self.collapsed_fraction = fraction.clamp(0., 1.);
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    pub fn leading(mut self, leading: impl IntoElement) -> Self {
        self.leading.push(leading.into_any_element());
        self
    }

    pub fn navigation_icon(self, content: impl IntoElement) -> Self {
        self.leading(content)
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }
}

macro_rules! top_app_bar_variant {
    ($name:ident, $variant:ident) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的顶部应用栏。")]
        #[derive(IntoElement)]
        pub struct $name(TopAppBar);

        impl $name {
            pub fn new(id: impl Into<ElementId>) -> Self {
                Self(TopAppBar::new(id).variant(TopAppBarVariant::$variant))
            }

            pub fn title(mut self, title: impl Into<SharedString>) -> Self {
                self.0 = self.0.title(title);
                self
            }

            pub fn collapsed_fraction(mut self, fraction: f32) -> Self {
                self.0 = self.0.collapsed_fraction(fraction);
                self
            }

            pub fn navigation_icon(mut self, content: impl IntoElement) -> Self {
                self.0 = self.0.leading(content);
                self
            }

            pub fn action(mut self, content: impl IntoElement) -> Self {
                self.0 = self.0.action(content);
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

top_app_bar_variant!(CenterAlignedTopAppBar, CenterAligned);
top_app_bar_variant!(MediumTopAppBar, Medium);
top_app_bar_variant!(LargeTopAppBar, Large);
top_app_bar_variant!(MediumFlexibleTopAppBar, MediumFlexible);
top_app_bar_variant!(LargeFlexibleTopAppBar, LargeFlexible);

pub type TwoRowsTopAppBar = LargeTopAppBar;

#[derive(IntoElement)]
pub struct BottomAppBar {
    id: ElementId,
    actions: Vec<AnyElement>,
    floating_action_button: Option<AnyElement>,
    collapsed_fraction: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BottomAppBarState {
    pub collapsed_fraction: f32,
}

impl BottomAppBarState {
    pub fn new(collapsed_fraction: f32) -> Self {
        Self {
            collapsed_fraction: collapsed_fraction.clamp(0., 1.),
        }
    }
}

impl BottomAppBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            actions: Vec::new(),
            floating_action_button: None,
            collapsed_fraction: 0.,
        }
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }

    pub fn floating_action_button(mut self, button: impl IntoElement) -> Self {
        self.floating_action_button = Some(button.into_any_element());
        self
    }

    pub fn state(mut self, state: BottomAppBarState) -> Self {
        self.collapsed_fraction = state.collapsed_fraction;
        self
    }
}

#[derive(IntoElement)]
pub struct FlexibleBottomAppBar(BottomAppBar);

impl FlexibleBottomAppBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self(BottomAppBar::new(id))
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.0 = self.0.action(action);
        self
    }

    pub fn floating_action_button(mut self, button: impl IntoElement) -> Self {
        self.0 = self.0.floating_action_button(button);
        self
    }

    pub fn state(mut self, state: BottomAppBarState) -> Self {
        self.0 = self.0.state(state);
        self
    }
}

impl ParentElement for FlexibleBottomAppBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.0.extend(elements);
    }
}

impl RenderOnce for FlexibleBottomAppBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.0.render(window, cx)
    }
}

impl ParentElement for BottomAppBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.actions.extend(elements);
    }
}

impl RenderOnce for BottomAppBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        div()
            .id(self.id)
            .w_full()
            .h(px(80. - 16. * self.collapsed_fraction))
            .flex()
            .flex_none()
            .items_center()
            .gap(px(8.))
            .px(px(16.))
            .bg(colors.surface_container)
            .children(self.actions)
            .when_some(self.floating_action_button, |el, button| {
                el.child(div().ml_auto().child(button))
            })
    }
}

impl ParentElement for TopAppBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.actions.extend(elements)
    }
}

impl RenderOnce for TopAppBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let mut style = TopAppBarStyle::resolve(theme.token_set(), self.variant);
        if matches!(
            self.variant,
            TopAppBarVariant::MediumFlexible | TopAppBarVariant::LargeFlexible
        ) {
            style.height -= (style.height - px(64.)) * self.collapsed_fraction;
        }

        let small = matches!(
            self.variant,
            TopAppBarVariant::Small | TopAppBarVariant::CenterAligned
        ) || matches!(
            self.variant,
            TopAppBarVariant::MediumFlexible | TopAppBarVariant::LargeFlexible
        ) && self.collapsed_fraction >= 0.5;
        let centered = self.variant == TopAppBarVariant::CenterAligned;

        let leading_row = div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(4.))
            .children(self.leading);

        let actions_row = div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(4.))
            .children(self.actions);

        let title_element = style
            .title
            .apply(div())
            .text_color(style.title_color)
            .truncate()
            .child(self.title);

        let bar = div()
            .id(self.id)
            .w_full()
            .h(style.height)
            .flex_none()
            .flex()
            .flex_col()
            .bg(style.container_color);

        if small {
            bar.child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .items_center()
                    .px(px(4.))
                    .gap(px(4.))
                    .child(leading_row)
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .when(centered, |el| el.justify_center())
                            .when(!centered, |el| el.pl(px(12.)))
                            .child(title_element),
                    )
                    .child(actions_row),
            )
        } else {
            bar.child(
                div()
                    .h(px(64.))
                    .flex_none()
                    .flex()
                    .items_center()
                    .px(px(4.))
                    .gap(px(4.))
                    .child(leading_row)
                    .child(div().flex_1())
                    .child(actions_row),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .items_end()
                    .px(px(16.))
                    .pb(px(12.))
                    .child(title_element),
            )
        }
    }
}

pub use appearance::TopAppBarStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};

    #[derive(Clone, Copy, Debug)]
    pub struct TopAppBarStyle {
        pub container_color: Hsla,

        pub title_color: Hsla,

        pub icon_color: Hsla,

        pub height: Pixels,

        pub horizontal_padding: Pixels,

        pub gap: Pixels,

        pub title: crate::theme::TypeStyle,
    }
    impl TopAppBarStyle {
        pub fn resolve(tokens: &TokenSet, variant: super::TopAppBarVariant) -> Self {
            use crate::tokens::{AppBarLargeTokens, AppBarMediumTokens, AppBarSmallTokens};
            let (height, title) = match variant {
                super::TopAppBarVariant::Small | super::TopAppBarVariant::CenterAligned => (
                    AppBarSmallTokens::CONTAINER_HEIGHT.pixels(),
                    AppBarSmallTokens::TITLE_FONT.resolve(tokens),
                ),
                super::TopAppBarVariant::Medium | super::TopAppBarVariant::MediumFlexible => (
                    AppBarMediumTokens::CONTAINER_HEIGHT.pixels(),
                    AppBarMediumTokens::TITLE_FONT.resolve(tokens),
                ),
                super::TopAppBarVariant::Large | super::TopAppBarVariant::LargeFlexible => (
                    AppBarLargeTokens::CONTAINER_HEIGHT.pixels(),
                    AppBarLargeTokens::TITLE_FONT.resolve(tokens),
                ),
            };
            Self {
                container_color: tokens.colors.surface,
                title_color: tokens.colors.on_surface,
                icon_color: tokens.colors.on_surface_variant,
                height,
                horizontal_padding: px(16.),
                gap: px(8.),
                title,
            }
        }
    }
}
