//! App bars & Scaffold 页：顶栏三档 densities、Badge 与 Scaffold 舞台演示。

use gpui::{AnyElement, App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use material3_gpui::icon::IconName;
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// App bars & Scaffold 页视图。
pub struct AppBarsPage {
    /// Scaffold 槽位里的 FAB(状态组件,构造期一次 build)。
    fab: Entity<FabState>,
}

impl AppBarsPage {
    /// 创建页面及其初始组件状态。
    pub fn new(cx: &mut App) -> Entity<Self> {
        let fab = Fab::new("scaffold-fab", IconName::Add).build(cx);
        cx.new(|_| Self { fab })
    }
}

/// 为顶栏变体添加标签，各预览占据完整一行。
fn bar_row(bar: impl IntoElement, caption: &'static str, cx: &App) -> AnyElement {
    let typography = *cx.theme().typography();
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            typography
                .label_medium
                .apply(div())
                .text_color(cx.theme().colors().on_surface_variant)
                .child(caption),
        )
        .child(bar)
        .into_any_element()
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
                "Top app bar variants",
                [
                    bar_row(
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
                            ),
                        "Small app bar",
                        cx,
                    ),
                    bar_row(
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
                            ),
                        "Medium app bar",
                        cx,
                    ),
                    bar_row(
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
                            ),
                        "Large app bar",
                        cx,
                    ),
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
                    .w_full()
                    .min_w_0()
                    .h(px(320.))
                    .overflow_hidden()
                    .rounded(px(24.))
                    .child(
                        Scaffold::new("scaffold-demo")
                            .top_bar(
                                TopAppBar::small("scaffold-bar")
                                    .title("Kiln report")
                                    .action(
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
                                        "The FAB floats above the bottom-right corner and the \
                                     top bar stays pinned.",
                                    ),
                            ),
                    )
                    .into_any_element()],
            ),
        ])
    }
}
