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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Card.kt

use gpui::{
    AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
    div,
};

use crate::theme::ActiveTheme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    Elevated,

    #[default]
    Filled,

    Outlined,
}

#[derive(IntoElement)]
pub struct Card {
    base: Div,
    variant: CardVariant,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            base: div(),
            variant: CardVariant::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn elevated(self) -> Self {
        self.variant(CardVariant::Elevated)
    }

    pub fn filled(self) -> Self {
        self.variant(CardVariant::Filled)
    }

    pub fn outlined(self) -> Self {
        self.variant(CardVariant::Outlined)
    }
}

macro_rules! card_variant {
    ($name:ident, $variant:ident) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的卡片容器。")]
        #[derive(IntoElement)]
        pub struct $name(Card);

        impl $name {
            pub fn new() -> Self {
                Self(Card::new().variant(CardVariant::$variant))
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Styled for $name {
            fn style(&mut self) -> &mut StyleRefinement {
                self.0.style()
            }
        }

        impl ParentElement for $name {
            fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
                self.0.extend(elements);
            }
        }

        impl RenderOnce for $name {
            fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
                self.0.render(window, cx)
            }
        }
    };
}

card_variant!(ElevatedCard, Elevated);
card_variant!(OutlinedCard, Outlined);

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Card {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let style = CardStyle::resolve(theme.token_set(), self.variant);
        let base = self
            .base
            .rounded(style.corner_radius)
            .text_color(style.content_color)
            .bg(style.container_color)
            .shadow(style.elevation.shadows(style.shadow_color));
        let base = if let Some(outline) = style.outline_color {
            base.border_1().border_color(outline)
        } else {
            base
        };

        base.children(self.children)
    }
}

pub use appearance::CardStyle;

mod appearance {
    use super::CardVariant;
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels};

    #[derive(Clone, Copy, Debug)]
    pub struct CardStyle {
        pub container_color: Hsla,

        pub content_color: Hsla,

        pub outline_color: Option<Hsla>,

        pub corner_radius: Pixels,

        pub shadow_color: Hsla,

        pub elevation: crate::theme::Elevation,
    }
    impl CardStyle {
        pub fn resolve(tokens: &TokenSet, variant: CardVariant) -> Self {
            let colors = &tokens.colors;
            let (container, outline, elevation) = match variant {
                CardVariant::Elevated => (
                    colors.surface_container_low,
                    None,
                    crate::theme::Elevation::Level1,
                ),
                CardVariant::Filled => (
                    colors.surface_container_highest,
                    None,
                    crate::theme::Elevation::Level0,
                ),
                CardVariant::Outlined => (
                    colors.surface,
                    Some(colors.outline_variant),
                    crate::theme::Elevation::Level0,
                ),
            };
            Self {
                container_color: container,
                content_color: colors.on_surface,
                outline_color: outline,
                corner_radius: tokens.shapes.medium,
                shadow_color: colors.shadow,
                elevation,
            }
        }
    }
}
