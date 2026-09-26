//! Chips 页。

use gpui::{AnyElement, App, AppContext as _, Entity, IntoElement, Render, Window};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Chips 页视图。
pub struct ChipsPage {
    pub chip_assist: Entity<ChipState>,
    pub chip_filters: Vec<Entity<ChipState>>,
    /// Input chip：点击移除按钮后从页面消失。
    pub chip_input: Option<Entity<ChipState>>,
    pub chip_suggestion: Entity<ChipState>,
}

impl ChipsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let chip_assist = Chip::new("chip-assist", "Assist")
            .assist()
            .leading_icon(IconName::Info)
            .build(cx);
        let chip_filters = ["Alpha", "Beta", "Gamma"]
            .into_iter()
            .enumerate()
            .map(|(ix, label)| Chip::new(("chip-filter", ix), label).filter().build(cx))
            .collect();
        let chip_suggestion = Chip::new("chip-suggestion", "Suggestion")
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
            let chip_input = Chip::new("chip-input", "Rust")
                .input()
                .on_remove(move |_, window, cx| {
                    page_entity.update(cx, |page, cx| {
                        page.chip_input = None;
                        cx.notify();
                    });
                    material3_gpui::overlay::show_snackbar(
                        window,
                        cx,
                        material3_gpui::overlay::Snackbar::new("Chip removed"),
                        None,
                    );
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
        let filter_items: Vec<AnyElement> = self
            .chip_filters
            .iter()
            .map(|chip| chip.clone().into_any_element())
            .collect();
        let input_items: Vec<AnyElement> = self
            .chip_input
            .iter()
            .map(|chip| chip.clone().into_any_element())
            .collect();

        gallery([
            showcase_group(
                cx,
                "Assist Chips",
                [self.chip_assist.clone().into_any_element()],
            ),
            showcase_group(cx, "Filter Chips", filter_items),
            showcase_group(cx, "Input Chips", input_items),
            showcase_group(
                cx,
                "Suggestion Chips",
                [self.chip_suggestion.clone().into_any_element()],
            ),
        ])
    }
}
