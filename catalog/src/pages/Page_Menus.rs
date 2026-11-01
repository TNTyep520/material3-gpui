// Copyright (c) 2026 TNTyep520
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gpui::{AnyElement, App, AppContext as _, Bounds, Entity, IntoElement, WeakEntity, px, size};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, LogErr as _, example_content, example_row};

pub struct MenusPage {
    weak: WeakEntity<Self>,
    menu: Entity<MenuState>,
    dropdown_field: Entity<ButtonState>,
    dropdown_expanded: bool,
    menu_anchor: Entity<ButtonState>,
    scroll_menu_anchor: Entity<ButtonState>,
}

impl MenusPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let weak = cx.entity().downgrade();
            let menu = MenuState::new()
                .item(
                    MenuItem::new("Refresh the queue")
                        .icon(IconName::new("progress_activity"))
                        .on_click(|window, cx| {
                            show_snackbar(window, cx, Snackbar::new("Queue refreshed"), None);
                        }),
                )
                .item(
                    MenuItem::new("Report a delivery")
                        .icon(IconName::new("info"))
                        .on_click(|window, cx| {
                            show_snackbar(
                                window,
                                cx,
                                Snackbar::new("Thanks for the report!"),
                                None,
                            );
                        }),
                )
                .item(
                    MenuItem::new("Open tracking sheet")
                        .icon(IconName::new("menu"))
                        .on_click(|window, cx| {
                            show_snackbar(window, cx, Snackbar::new("Sheet opened"), None);
                        }),
                )
                .build(cx);
            let mut scroll_menu_builder = MenuState::new();
            for index in 1..=15 {
                scroll_menu_builder =
                    scroll_menu_builder.item(MenuItem::new(format!("Item {index}")));
            }
            let scroll_menu = scroll_menu_builder.build(cx);
            let dropdown_field = Button::new("menu-exposed-field", "Open dropdown")
                .outlined()
                .build(cx);
            let menu_anchor = Button::new("menu-anchor", "Open menu")
                .filled()
                .on_click({
                    let menu = menu.clone();
                    move |event, window, cx| {
                        let anchor = Bounds {
                            origin: event.position(),
                            size: size(px(0.), px(0.)),
                        };
                        show_menu(window, cx, menu.clone(), anchor);
                    }
                })
                .build(cx);
            let scroll_menu_anchor = Button::new("menu-anchor-scroll", "Open scrolling menu")
                .filled()
                .on_click({
                    let scroll_menu = scroll_menu;
                    move |event, window, cx| {
                        let anchor = Bounds {
                            origin: event.position(),
                            size: size(px(0.), px(0.)),
                        };
                        show_menu(window, cx, scroll_menu.clone(), anchor);
                    }
                })
                .build(cx);
            Self {
                weak,
                menu,
                dropdown_field,
                dropdown_expanded: false,
                menu_anchor,
                scroll_menu_anchor,
            }
        })
    }
}

impl ComponentPage for MenusPage {
    fn description(&self) -> &'static str {
        "Menus display a list of choices on temporary surfaces."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "MenuSample",
                description: "Menus examples",
            },
            ExampleMeta {
                name: "MenuWithScrollStateSample",
                description: "Menus examples",
            },
            ExampleMeta {
                name: "ExposedDropdownMenuSample",
                description: "Menus examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Dropdown menu",
                [self.menu_anchor.clone().into_any_element()],
            )]),
            1 => example_content([example_row(
                cx,
                "Scrolling menu",
                [self.scroll_menu_anchor.clone().into_any_element()],
            )]),
            2 => {
                let weak = self.weak.clone();
                let dropdown =
                    ExposedDropdownMenu::new("menu-exposed", self.dropdown_field.clone())
                        .menu(self.menu.clone())
                        .expanded(self.dropdown_expanded)
                        .on_expanded_change(move |expanded, _, cx| {
                            weak.update(cx, |page, cx| {
                                page.dropdown_expanded = expanded;
                                cx.notify();
                            })
                            .log_err();
                        });
                example_content([example_row(
                    cx,
                    "Exposed dropdown",
                    [dropdown.into_any_element()],
                )])
            }
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
