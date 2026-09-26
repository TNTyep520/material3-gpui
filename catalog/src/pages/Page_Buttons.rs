//! Buttons 页：五种变体按钮演示。

use gpui::{App, Entity, IntoElement, Render, Window, prelude::*};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Buttons 页视图。
pub struct ButtonsPage {
    pub b_filled: Entity<ButtonState>,
    pub b_tonal: Entity<ButtonState>,
    pub b_elevated: Entity<ButtonState>,
    pub b_outlined: Entity<ButtonState>,
    pub b_text: Entity<ButtonState>,
    pub b_icon: Entity<ButtonState>,
    pub b_disabled: Entity<ButtonState>,
}

impl ButtonsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let b_filled = Button::new("b-filled", "Filled")
            .filled()
            .on_click(|_, window, cx| {
                material3_gpui::overlay::show_snackbar(
                    window,
                    cx,
                    material3_gpui::overlay::Snackbar::new("Filled button clicked"),
                    None,
                );
            })
            .build(cx);
        let b_tonal = Button::new("b-tonal", "Tonal").tonal().build(cx);
        let b_elevated = Button::new("b-elevated", "Elevated").elevated().build(cx);
        let b_outlined = Button::new("b-outlined", "Outlined").outlined().build(cx);
        let b_text = Button::new("b-text", "Text").text().build(cx);
        let b_icon = Button::new("b-icon", "With icon")
            .filled()
            .leading_icon(IconName::Add)
            .build(cx);
        let b_disabled = Button::new("b-disabled", "Disabled")
            .filled()
            .disabled(true)
            .build(cx);

        cx.new(|_| Self {
            b_filled,
            b_tonal,
            b_elevated,
            b_outlined,
            b_text,
            b_icon,
            b_disabled,
        })
    }
}

impl Render for ButtonsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gallery([showcase_group(
            cx,
            "Button Variants",
            [
                self.b_filled.clone().into_any_element(),
                self.b_tonal.clone().into_any_element(),
                self.b_elevated.clone().into_any_element(),
                self.b_outlined.clone().into_any_element(),
                self.b_text.clone().into_any_element(),
                self.b_icon.clone().into_any_element(),
                self.b_disabled.clone().into_any_element(),
            ],
        )])
    }
}
