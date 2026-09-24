//! Tabs 页：指示条以 fastSpatial 弹簧滑动。

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use md3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Tabs 页视图。
pub struct TabsPage {
    tabbar: Entity<TabBarState>,
}

impl TabsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let tabbar = TabBar::new("tabs")
            .tab(Tab::new("Home").icon(IconName::Home))
            .tab(Tab::new("Search").icon(IconName::Search))
            .tab(Tab::new("Profile").icon(IconName::Person))
            .selected(0)
            .build(cx);
        cx.new(|cx| {
            cx.observe(&tabbar, |_, _, cx| cx.notify()).detach();
            Self { tabbar }
        })
    }
}

impl Render for TabsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();
        let selected_tab = self.tabbar.read(cx).selected();

        gallery([showcase_group(
            cx,
            "Primary Tabs",
            [div()
                .flex()
                .flex_col()
                .w_full()
                .child(self.tabbar.clone())
                .child(
                    div().pt(px(12.)).child(
                        typography
                            .body_medium
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(format!("Tab {} selected", selected_tab + 1)),
                    ),
                )
                .into_any_element()],
        )])
    }
}
