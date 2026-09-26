//! MD3 Card（对应 material-web labs 的 `md-*-card`）
//!
//! 变体：Elevated / Filled / Outlined，圆角 12dp。
//! Card 实现了 `Styled` 与 `ParentElement`，可以像 `div` 一样追加样式和子元素。

use gpui::{
    AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
    div,
};

use crate::theme::{ActiveTheme, Elevation};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    #[default]
    Elevated,
    Filled,
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
        let colors = theme.colors();

        let base = self
            .base
            .rounded(theme.shapes().medium)
            .text_color(colors.on_surface);

        let base = match self.variant {
            CardVariant::Elevated => base
                .bg(colors.surface_container_low)
                .shadow(Elevation::Level1.shadows(colors.shadow)),
            CardVariant::Filled => base.bg(colors.surface_container_highest),
            CardVariant::Outlined => base
                .bg(colors.surface)
                .border_1()
                .border_color(colors.outline_variant),
        };

        base.children(self.children)
    }
}
