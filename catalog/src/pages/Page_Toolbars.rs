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

use gpui::{App, AppContext as _, Entity, IntoElement, Render, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{page, showcase_group};

pub struct ToolbarsPage {
    icons: Vec<Entity<IconButtonState>>,
}

impl ToolbarsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let icon = |cx: &mut App, id: &'static str, name: &'static str, message: &'static str| {
            IconButton::new(id, IconName::new(name))
                .on_click(move |_, window, cx| {
                    show_snackbar(window, cx, Snackbar::new(message), None);
                })
                .build(cx)
        };
        let icons = vec![
            icon(cx, "tool-add", "add", "Toolbar: add"),
            icon(cx, "tool-edit", "edit", "Toolbar: edit"),
            icon(cx, "tool-more", "more_vert", "Toolbar: more"),
        ];
        cx.new(|_| Self { icons })
    }
}

impl Render for ToolbarsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let buttons = self.icons.clone();

        page(
            cx,
            "Toolbars",
            "Floating toolbars in standard and vibrant color styles, horizontal or vertical.",
            [
                showcase_group(
                    cx,
                    "Horizontal",
                    [
                        FloatingToolbar::new("toolbar-standard").children(buttons.clone()),
                        FloatingToolbar::new("toolbar-collapsed")
                            .state(FloatingToolbarState::new(false))
                            .children(buttons.clone()),
                    ],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Vibrant",
                    [FloatingToolbar::new("toolbar-vibrant")
                        .vibrant(true)
                        .children(buttons.clone())],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Vertical",
                    [div()
                        .h(px(240.))
                        .flex_none()
                        .child(VerticalFloatingToolbar::new("toolbar-vertical").children(buttons))
                        .into_any_element()],
                )
                .into_any_element(),
            ],
        )
    }
}
