//! MD3 Card（对应 material-web labs 的 `md-*-card`）
//!
//! 变体：Elevated / Filled / Outlined，圆角 12dp。
//! Card 实现了 `Styled` 与 `ParentElement`，可以像 `div` 一样追加样式和子元素。

use gpui::{
    AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
    div,
};

use crate::theme::ActiveTheme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    /// 浮起卡片。
    Elevated,
    /// 默认填充卡片。
    #[default]
    Filled,
    /// 描边卡片。
    Outlined,
}

/// MD3 卡片容器
#[derive(IntoElement)]
pub struct Card {
    base: Div,
    variant: CardVariant,
    children: Vec<AnyElement>,
}

impl Card {
    /// 创建默认的填充卡片。
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
            /// 创建此卡片变体。
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
    /// MD3 卡片样式。
    #[derive(Clone, Copy, Debug)]
    pub struct CardStyle {
        /// 容器色。
        pub container_color: Hsla,
        /// 内容色。
        pub content_color: Hsla,
        /// 描边色（`Some` 启用 1dp 描边）。
        pub outline_color: Option<Hsla>,
        /// 圆角。
        pub corner_radius: Pixels,
        /// 阴影颜色。
        pub shadow_color: Hsla,
        /// 阴影等级。
        pub elevation: crate::theme::Elevation,
    }
    impl CardStyle {
        /// 由令牌推导默认样式。
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
