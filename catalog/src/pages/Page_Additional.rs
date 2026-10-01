//! Lab bench 页：浮动工具栏、拾取器、搜索、扩展 FAB 与滚动条等补充组件。

use gpui::{App, AppContext as _, Entity, IntoElement, Render, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Lab bench 页视图。
pub struct AdditionalPage;

impl AdditionalPage {
    /// 创建页面及其初始组件状态。
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }
}

impl Render for AdditionalPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let toolbar = FloatingToolbar::new("toolbar").children([
            IconButton::new("tool-add", IconName::Add)
                .on_click(|_, window, cx| {
                    show_snackbar(window, cx, Snackbar::new("Toolbar: add"), None);
                })
                .build(cx),
            IconButton::new("tool-edit", IconName::Edit)
                .on_click(|_, window, cx| {
                    show_snackbar(window, cx, Snackbar::new("Toolbar: edit"), None);
                })
                .build(cx),
        ]);
        let fab_menu = FabMenu::new("fab-menu")
            .expanded(true)
            .action(Fab::new("fab-add", IconName::Add).build(cx));
        let swiping = Card::new()
            .filled()
            .w(px(280.))
            .max_w_full()
            .p(px(16.))
            .child("Swipe me away");
        let wide_rail = WideNavigationRail::new("wide-rail").children([
            Button::new("rail-home", "Home").text().build(cx),
            Button::new("rail-settings", "Settings").text().build(cx),
        ]);

        gallery([
            showcase_group(
                cx,
                "Indicators",
                [
                    LoadingIndicator::new("loading")
                        .size(px(40.))
                        .into_any_element(),
                    WavyProgressIndicator::new("wavy")
                        .value(0.64)
                        .into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Pickers & search",
                [div()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .w_full()
                    .min_w_0()
                    .child(
                        SearchBar::new("search")
                            .query("Search the bench")
                            .into_any_element(),
                    )
                    .child(
                        SecureTextField::new("secure", "Passphrase")
                            .value("secret")
                            .into_any_element(),
                    )
                    .child(
                        DatePicker::new("date")
                            .value("2026-10-01")
                            .into_any_element(),
                    )
                    .child(TimePicker::new("time").value("10:30").into_any_element())
                    .into_any_element()],
            ),
            showcase_group(
                cx,
                "Toolbars & navigation",
                [
                    toolbar.into_any_element(),
                    fab_menu.into_any_element(),
                    wide_rail.into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Dismiss & scroll",
                [div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(24.))
                    .w_full()
                    .min_w_0()
                    .child(
                        SwipeToDismissBox::new("dismiss", swiping)
                            .background(div().bg(cx.theme().colors().error_container))
                            .into_any_element(),
                    )
                    .child(
                        div()
                            .relative()
                            .h(px(72.))
                            .w(px(280.))
                            .max_w_full()
                            .child("Scrollable stack")
                            .child(Scrollbar::new("scrollbar").position(0.35))
                            .into_any_element(),
                    )
                    .into_any_element()],
            ),
            showcase_group(
                cx,
                "Range slider",
                [RangeSlider::new("range-tune", 0.2, 0.78).into_any_element()],
            ),
        ])
    }
}
