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

use gpui::{
    AnyElement, App, AppContext as _, Entity, IntoElement, ParentElement as _, Styled, div, px,
};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content, example_row};

fn drawer_example(id: &'static str, modal: bool, cx: &mut App) -> Entity<NavigationDrawerState> {
    NavigationDrawer::new(id)
        .item(NavigationItemSpec::new("Inbox", IconName::new("info")).badge("24"))
        .item(NavigationItemSpec::new("Starred", IconName::new("star")))
        .section("Labels")
        .item(NavigationItemSpec::new("Work", IconName::new("edit")))
        .item(NavigationItemSpec::new("Personal", IconName::new("person")))
        .modal(modal)
        .selected(0)
        .build(cx)
}

pub struct NavigationDrawersPage {
    modal_drawer: Entity<NavigationDrawerState>,
    dismissible_drawer: Entity<NavigationDrawerState>,
    permanent_drawer: Entity<NavigationDrawerState>,
}

impl NavigationDrawersPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let modal_drawer = drawer_example("drawer-modal", true, cx);
        let dismissible_drawer = drawer_example("drawer-dismissible", false, cx);
        let permanent_drawer = PermanentNavigationDrawer::new("drawer-permanent")
            .item(NavigationItemSpec::new("Inbox", IconName::new("info")))
            .item(NavigationItemSpec::new("Starred", IconName::new("star")))
            .selected(0)
            .build(cx);
        cx.new(|_| Self {
            modal_drawer,
            dismissible_drawer,
            permanent_drawer,
        })
    }
}

impl ComponentPage for NavigationDrawersPage {
    fn description(&self) -> &'static str {
        "Navigation drawers provide ergonomic access to destinations in an app."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "ModalNavigationDrawerSample",
                description: "Navigation drawer examples",
            },
            ExampleMeta {
                name: "DismissibleNavigationDrawerSample",
                description: "Navigation drawer examples",
            },
            ExampleMeta {
                name: "PermanentNavigationDrawerSample",
                description: "Navigation drawer examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Modal",
                [div()
                    .w(px(360.))
                    .max_w_full()
                    .h(px(360.))
                    .overflow_hidden()
                    .rounded(px(24.))
                    .child(self.modal_drawer.clone())
                    .into_any_element()],
            )]),
            1 => example_content([example_row(
                cx,
                "Dismissible",
                [div()
                    .w(px(360.))
                    .max_w_full()
                    .h(px(360.))
                    .overflow_hidden()
                    .rounded(px(24.))
                    .child(self.dismissible_drawer.clone())
                    .into_any_element()],
            )]),
            2 => {
                let colors = *cx.theme().colors();
                example_content([example_row(
                    cx,
                    "Permanent",
                    [div()
                        .w_full()
                        .min_w_0()
                        .flex()
                        .gap(px(16.))
                        .child(
                            div()
                                .w(px(280.))
                                .flex_none()
                                .overflow_hidden()
                                .rounded(px(24.))
                                .child(self.permanent_drawer.clone()),
                        )
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(24.))
                                .border_1()
                                .border_color(colors.outline_variant)
                                .h(px(360.))
                                .text_color(colors.on_surface_variant)
                                .child("Content area"),
                        )
                        .into_any_element()],
                )])
            }
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
