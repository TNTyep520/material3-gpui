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
    AnyElement, App, AppContext as _, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Styled, div, px,
};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content, example_row};

pub struct TooltipsPage;

impl TooltipsPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

fn hover_target(cx: &App, label: &'static str) -> impl IntoElement {
    let colors = *cx.theme().colors();
    div()
        .size(px(48.))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .bg(colors.surface_container_high)
        .child(
            Icon::new(IconName::new("favorite"))
                .size(px(24.))
                .color(colors.on_surface_variant),
        )
        .hover(|style| style.bg(colors.surface_container_highest))
        .child(label)
        .text_color(colors.on_surface_variant)
}

impl ComponentPage for TooltipsPage {
    fn description(&self) -> &'static str {
        "Tooltips call user attention to an anchor component."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "PlainTooltipSample",
                description: "Tooltips examples",
            },
            ExampleMeta {
                name: "RichTooltipSample",
                description: "Tooltips examples",
            },
            ExampleMeta {
                name: "RichTooltipWithCaretSample",
                description: "Tooltips examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Plain tooltip",
                [TooltipBox::plain(
                    "tooltip-plain",
                    hover_target(cx, "Anchor"),
                    PlainTooltip::new("Plain tooltip"),
                )
                .into_any_element()],
            )]),
            1 => example_content([example_row(
                cx,
                "Rich tooltip",
                [TooltipBox::rich(
                    "tooltip-rich",
                    hover_target(cx, "Anchor"),
                    RichTooltip::new("Rich tooltip", "Supporting text for the tooltip"),
                )
                .into_any_element()],
            )]),
            2 => example_content([example_row(
                cx,
                "Rich tooltip · caret",
                [TooltipBox::rich(
                    "tooltip-caret",
                    hover_target(cx, "Anchor"),
                    RichTooltip::new(
                        "With caret",
                        "Tooltips call user attention to an anchor component.",
                    ),
                )
                .into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
