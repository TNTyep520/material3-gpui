//! Cards 页：使用相同内容比较三种表面变体。

use gpui::{App, Entity, IntoElement, Render, Window, prelude::*};
use material3_gpui::prelude::*;

use super::{catalog_card, gallery, showcase_group};

/// Cards 页视图。
pub struct CardsPage;

impl CardsPage {
    /// 创建页面及其初始组件状态。
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }
}

impl Render for CardsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gallery([showcase_group(
            cx,
            "Variants",
            [
                catalog_card(cx, Card::new().filled(), "Filled").into_any_element(),
                catalog_card(cx, Card::new().elevated(), "Elevated").into_any_element(),
                catalog_card(cx, Card::new().outlined(), "Outlined").into_any_element(),
            ],
        )])
    }
}
