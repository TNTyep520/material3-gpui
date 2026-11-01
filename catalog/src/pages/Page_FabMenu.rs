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

use gpui::{AnyElement, App, AppContext as _, Entity, IntoElement, ParentElement, Styled, div, px};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content};

pub struct FabMenuPage {
    fab_menu: Entity<FabMenuState>,
}

impl FabMenuPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let item = |cx: &mut App, id: &'static str, icon: &'static str, message: &'static str| {
            Fab::new(id, IconName::new(icon))
                .color(FabColor::Secondary)
                .on_click(move |_, window, cx| {
                    show_snackbar(window, cx, Snackbar::new(message), None);
                })
                .build(cx)
        };
        let fab_menu = FabMenu::new("fab-menu")
            .action(item(cx, "fab-action-edit", "edit", "Menu: edit draft"))
            .action(item(cx, "fab-action-image", "image", "Menu: attach image"))
            .action(item(
                cx,
                "fab-action-settings",
                "settings",
                "Menu: settings",
            ))
            .build(cx);
        cx.new(|cx| {
            cx.observe(&fab_menu, |_, _, cx| cx.notify()).detach();
            Self { fab_menu }
        })
    }
}

impl ComponentPage for FabMenuPage {
    fn description(&self) -> &'static str {
        "The FAB Menu displays additional key actions on click of a FAB."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[ExampleMeta {
            name: "FloatingActionButtonMenuSample",
            description: "FAB Menu examples",
        }]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => {
                let theme = cx.theme();
                let state = if self.fab_menu.read(cx).expanded() {
                    "expanded"
                } else {
                    "collapsed"
                };
                example_content([div()
                    .h(px(280.))
                    .w_full()
                    .flex()
                    .items_start()
                    .justify_end()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_end()
                            .gap(px(8.))
                            .child(
                                theme
                                    .typography()
                                    .body_medium
                                    .apply(div())
                                    .text_color(theme.colors().on_surface_variant)
                                    .child(state),
                            )
                            .child(self.fab_menu.clone()),
                    )
                    .into_any_element()])
            }
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
