//! Overlays 页：Snackbar / Menu / Tooltip（经窗口 OverlayHost 渲染）。

use gpui::{App, Bounds, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px, size};
use material3_gpui::overlay::{
    MenuItem, MenuState, Snackbar, close_tooltip, show_menu, show_snackbar, show_tooltip,
};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Overlays 页视图。
pub struct OverlaysPage {
    pub b_snack: Entity<ButtonState>,
    pub b_menu: Entity<ButtonState>,
    pub tooltip_trigger: Entity<ButtonState>,
}

impl OverlaysPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let b_snack = Button::new("b-snack", "Show snackbar")
            .filled()
            .on_click(|_, window, cx| {
                show_snackbar(
                    window,
                    cx,
                    Snackbar::new("File archived")
                        .action("UNDO")
                        .on_action(|window, cx| {
                            show_snackbar(window, cx, Snackbar::new("Restored"), None);
                        }),
                    None,
                );
            })
            .build(cx);

        let menu = MenuState::new()
            .item(
                MenuItem::new("Refresh")
                    .icon(IconName::Settings)
                    .on_click(|window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Refreshed"), None);
                    }),
            )
            .item(
                MenuItem::new("Send feedback")
                    .icon(IconName::Info)
                    .on_click(|window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Thanks for the feedback!"), None);
                    }),
            )
            .build(cx);
        let b_menu = Button::new("b-menu", "Show menu")
            .outlined()
            .on_click(move |event, window, cx| {
                let anchor = Bounds {
                    origin: event.position(),
                    size: size(px(0.), px(0.)),
                };
                show_menu(window, cx, menu.clone(), anchor);
            })
            .build(cx);

        let tooltip_trigger = Button::new("tooltip-trigger", "Hover me")
            .outlined()
            .build(cx);

        cx.new(|_| Self {
            b_snack,
            b_menu,
            tooltip_trigger,
        })
    }
}

impl Render for OverlaysPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let tooltip_trigger = self.tooltip_trigger.clone();

        gallery([
            showcase_group(cx, "Snackbar", [self.b_snack.clone().into_any_element()]),
            showcase_group(cx, "Menu", [self.b_menu.clone().into_any_element()]),
            showcase_group(
                cx,
                "Tooltip",
                [div()
                    .id("tooltip-wrap")
                    .cursor_pointer()
                    .on_hover(move |hovered, window, cx| {
                        if *hovered {
                            let bounds = tooltip_trigger.read(cx).bounds();
                            show_tooltip(window, cx, "Tooltip via overlay host", bounds);
                        } else {
                            close_tooltip(window, cx);
                        }
                    })
                    .child(self.tooltip_trigger.clone())
                    .into_any_element()],
            ),
        ])
    }
}
