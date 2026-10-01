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
    App, AppContext as _, Entity, IntoElement, Render, SharedString, Window, div, prelude::*, px,
};
use material3_gpui::prelude::*;

use super::{page, showcase_group};

pub struct ToolbarsPage {
    standard_icons: Vec<Entity<IconButtonState>>,
    collapsed_icons: Vec<Entity<IconButtonState>>,
    vibrant_icons: Vec<Entity<IconButtonState>>,
    vertical_icons: Vec<Entity<IconButtonState>>,
}

impl ToolbarsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let icons = |cx: &mut App, prefix: &'static str| {
            let icon =
                |cx: &mut App, id: SharedString, name: &'static str, message: &'static str| {
                    IconButton::new(id, IconName::new(name))
                        .on_click(move |_, window, cx| {
                            show_snackbar(window, cx, Snackbar::new(message), None);
                        })
                        .build(cx)
                };
            vec![
                icon(
                    cx,
                    format!("tool-{prefix}-add").into(),
                    "add",
                    "Toolbar: add",
                ),
                icon(
                    cx,
                    format!("tool-{prefix}-edit").into(),
                    "edit",
                    "Toolbar: edit",
                ),
                icon(
                    cx,
                    format!("tool-{prefix}-more").into(),
                    "more_vert",
                    "Toolbar: more",
                ),
            ]
        };
        let standard_icons = icons(cx, "standard");
        let collapsed_icons = icons(cx, "collapsed");
        let vibrant_icons = icons(cx, "vibrant");
        let vertical_icons = icons(cx, "vertical");
        cx.new(|_| Self {
            standard_icons,
            collapsed_icons,
            vibrant_icons,
            vertical_icons,
        })
    }
}

impl Render for ToolbarsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        page(
            cx,
            "Toolbars",
            "Floating toolbars in standard and vibrant color styles, horizontal or vertical.",
            [
                showcase_group(
                    cx,
                    "Horizontal",
                    [
                        FloatingToolbar::new("toolbar-standard")
                            .children(self.standard_icons.clone()),
                        FloatingToolbar::new("toolbar-collapsed")
                            .state(FloatingToolbarState::new(false))
                            .children(self.collapsed_icons.clone()),
                    ],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Vibrant",
                    [FloatingToolbar::new("toolbar-vibrant")
                        .vibrant(true)
                        .children(self.vibrant_icons.clone())],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Vertical",
                    [div()
                        .h(px(240.))
                        .flex_none()
                        .child(
                            VerticalFloatingToolbar::new("toolbar-vertical")
                                .children(self.vertical_icons.clone()),
                        )
                        .into_any_element()],
                )
                .into_any_element(),
            ],
        )
    }
}
