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
    AnyElement, App, AppContext as _, Entity, IntoElement, ParentElement, SharedString, Styled,
    div, px,
};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content, example_row};

pub struct FloatingToolbarsPage {
    standard_icons: Vec<Entity<IconButtonState>>,
    collapsed_icons: Vec<Entity<IconButtonState>>,
    vibrant_icons: Vec<Entity<IconButtonState>>,
    vertical_icons: Vec<Entity<IconButtonState>>,
}

impl FloatingToolbarsPage {
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

impl ComponentPage for FloatingToolbarsPage {
    fn description(&self) -> &'static str {
        "A floating toolbar displays key actions above the content."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "HorizontalFloatingToolbarWithFabSample",
                description: "Floating toolbar examples",
            },
            ExampleMeta {
                name: "VerticalFloatingToolbarWithFabSample",
                description: "Floating toolbar examples",
            },
            ExampleMeta {
                name: "ExpandableHorizontalFloatingToolbarSample",
                description: "Floating toolbar examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([
                example_row(
                    cx,
                    "Standard",
                    [FloatingToolbar::new("toolbar-standard")
                        .children(self.standard_icons.clone())
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "Collapsed",
                    [FloatingToolbar::new("toolbar-collapsed")
                        .state(FloatingToolbarState::new(false))
                        .children(self.collapsed_icons.clone())
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "Vibrant",
                    [FloatingToolbar::new("toolbar-vibrant")
                        .vibrant(true)
                        .children(self.vibrant_icons.clone())
                        .into_any_element()],
                ),
            ]),
            1 => example_content([example_row(
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
            )]),
            2 => example_content([
                example_row(
                    cx,
                    "Collapsed",
                    [FloatingToolbar::new("toolbar-collapsed")
                        .state(FloatingToolbarState::new(false))
                        .children(self.collapsed_icons.clone())
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "Expanded",
                    [FloatingToolbar::new("toolbar-standard")
                        .children(self.standard_icons.clone())
                        .into_any_element()],
                ),
            ]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
