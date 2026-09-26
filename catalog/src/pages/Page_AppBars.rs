//! App bars & Scaffold 页:顶栏三档变体、Badge 与 Scaffold 槽位演示。

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use material3_gpui::icon::IconName;
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// App bars & Scaffold 页视图。
pub struct AppBarsPage {
    /// Scaffold 槽位里的 FAB(状态组件,构造期一次 build)。
    fab: Entity<material3_gpui::FabState>,
}

impl AppBarsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let fab = material3_gpui::Fab::new("scaffold-fab", IconName::Add).build(cx);
        cx.new(|_| Self { fab })
    }
}

impl Render for AppBarsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();

        // 带 Badge 的顶栏动作图标(静态预览)
        let bell_with_badge = div().child(badged(
            Icon::new(IconName::Menu)
                .size(px(24.))
                .color(colors.on_surface_variant),
            Badge::new("appbar-bell-badge").label("3"),
        ));

        gallery([
            showcase_group(
                cx,
                "Top app bars",
                [
                    TopAppBar::small("bar-small")
                        .title("Small")
                        .leading(
                            Icon::new(IconName::ArrowBack)
                                .size(px(24.))
                                .color(colors.on_surface),
                        )
                        .action(bell_with_badge)
                        .action(
                            Icon::new(IconName::MoreVert)
                                .size(px(24.))
                                .color(colors.on_surface_variant),
                        )
                        .into_any_element(),
                    TopAppBar::medium("bar-medium")
                        .title("Medium")
                        .leading(
                            Icon::new(IconName::ArrowBack)
                                .size(px(24.))
                                .color(colors.on_surface),
                        )
                        .action(
                            Icon::new(IconName::MoreVert)
                                .size(px(24.))
                                .color(colors.on_surface_variant),
                        )
                        .into_any_element(),
                    TopAppBar::large("bar-large")
                        .title("Large")
                        .leading(
                            Icon::new(IconName::ArrowBack)
                                .size(px(24.))
                                .color(colors.on_surface),
                        )
                        .action(
                            Icon::new(IconName::MoreVert)
                                .size(px(24.))
                                .color(colors.on_surface_variant),
                        )
                        .into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Badges",
                [div()
                    .flex()
                    .items_center()
                    .gap(px(32.))
                    .child(badged(
                        Icon::new(IconName::Menu)
                            .size(px(24.))
                            .color(colors.on_surface_variant),
                        Badge::new("badge-bell-count").label("9"),
                    ))
                    .child(badged(
                        Icon::new(IconName::Info)
                            .size(px(24.))
                            .color(colors.on_surface_variant),
                        Badge::new("badge-inbox-dot"),
                    ))
                    .into_any_element()],
            ),
            showcase_group(
                cx,
                "Scaffold",
                [div()
                    .h(px(360.))
                    .overflow_hidden()
                    .rounded(px(12.))
                    .child(
                        Scaffold::new("scaffold-demo")
                            .top_bar(
                                TopAppBar::small("scaffold-bar").title("Scaffold").action(
                                    Icon::new(IconName::MoreVert)
                                        .size(px(24.))
                                        .color(colors.on_surface_variant),
                                ),
                            )
                            .fab(self.fab.clone())
                            .child(
                                div()
                                    .p(px(16.))
                                    .text_color(colors.on_surface_variant)
                                    .child(
                                        "Content area. The FAB floats above the bottom-right \
                                         corner and the top bar stays pinned.",
                                    ),
                            ),
                    )
                    .into_any_element()],
            ),
        ])
    }
}
