//! Chips 页：分组展示标签类型及可移除状态。

use gpui::{App, AppContext as _, Entity, IntoElement, Render, Window};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group, specimen};

/// Chips 页视图。
pub struct ChipsPage {
    chip_assist: Entity<ChipState>,
    chip_filters: Vec<Entity<ChipState>>,
    /// Input chip：点击移除按钮后从页面消失。
    chip_input: Option<Entity<ChipState>>,
    chip_suggestion: Entity<ChipState>,
}

impl ChipsPage {
    /// 创建页面及其初始组件状态。
    pub fn new(cx: &mut App) -> Entity<Self> {
        let chip_assist = Chip::new("chip-assist", "Draft reply")
            .assist()
            .leading_icon(IconName::Edit)
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Reply drafted"), None);
            })
            .build(cx);
        let chip_filters = ["Photos", "Receipts", "Travel"]
            .into_iter()
            .enumerate()
            .map(|(ix, label)| {
                Chip::new(("chip-filter", ix), label)
                    .filter()
                    .on_click(move |_, window, cx| {
                        show_snackbar(
                            window,
                            cx,
                            Snackbar::new(format!("\"{label}\" filter applied")),
                            None,
                        );
                    })
                    .build(cx)
            })
            .collect();
        let chip_suggestion = Chip::new("chip-suggestion", "Not now")
            .suggestion()
            .elevated(true)
            .build(cx);

        let page = cx.new(|_| Self {
            chip_assist,
            chip_filters,
            chip_input: None,
            chip_suggestion,
        });

        // Input chip 需要在回调里更新页面状态，因此拿到页面句柄后再构建
        page.update(cx, |page, cx| {
            let page_entity = cx.entity();
            let chip_input = Chip::new("chip-input", "On vacation")
                .input()
                .on_remove(move |_, window, cx| {
                    page_entity.update(cx, |page, cx| {
                        page.chip_input = None;
                        cx.notify();
                    });
                    show_snackbar(window, cx, Snackbar::new("Filter removed"), None);
                })
                .build(cx);
            page.chip_input = Some(chip_input);
            cx.notify();
        });
        page
    }
}

impl Render for ChipsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gallery([
            showcase_group(
                cx,
                "Variants",
                [
                    specimen(cx, "Assist", self.chip_assist.clone()),
                    specimen(cx, "Suggestion · elevated", self.chip_suggestion.clone()),
                ],
            ),
            showcase_group(
                cx,
                "Filter chips",
                self.chip_filters
                    .iter()
                    .map(|chip| specimen(cx, "Selectable", chip.clone())),
            ),
            showcase_group(
                cx,
                "Input chip",
                self.chip_input
                    .iter()
                    .map(|chip| specimen(cx, "Removable", chip.clone())),
            ),
        ])
    }
}
