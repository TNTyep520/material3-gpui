//! Cards 页。

use gpui::{App, Entity, IntoElement, Render, Window, prelude::*};
use material3_gpui::prelude::*;

use super::{catalog_card, gallery, showcase_group};

/// Cards 页视图。
pub struct CardsPage;

impl CardsPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

impl Render for CardsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gallery([showcase_group(
            cx,
            "Cards",
            [
                catalog_card(cx, Card::new().elevated(), "Elevated card").into_any_element(),
                catalog_card(cx, Card::new().filled(), "Filled card").into_any_element(),
                catalog_card(cx, Card::new().outlined(), "Outlined card").into_any_element(),
            ],
        )])
    }
}
