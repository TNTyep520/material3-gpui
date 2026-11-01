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
    AnyElement, App, AppContext as _, Entity, Hsla, IntoElement, ParentElement as _, Styled, div,
};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content};

pub struct TypographyPage;

impl TypographyPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }
}

/// 用指定字阶样式与文字色渲染一行样式名,便于对照各变体。
fn typography_row(style: TypeStyle, color: Hsla, text: &'static str) -> AnyElement {
    style
        .apply(div())
        .text_color(color)
        .child(text)
        .into_any_element()
}

impl ComponentPage for TypographyPage {
    fn description(&self) -> &'static str {
        "The Material Design type scale includes a range of contrasting styles that \
         support the needs of your product and its content."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "TypographySample",
                description: "Typography examples",
            },
            ExampleMeta {
                name: "TypographyCustomFontFamilySample",
                description: "Typography examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        let theme = cx.theme();
        let typography = *theme.typography();
        let colors = *theme.colors();
        match index {
            0 => example_content([
                typography_row(typography.display_large, colors.on_surface, "Display Large"),
                typography_row(
                    typography.display_medium,
                    colors.on_surface,
                    "Display Medium",
                ),
                typography_row(typography.display_small, colors.on_surface, "Display Small"),
                typography_row(
                    typography.headline_large,
                    colors.on_surface,
                    "Headline Large",
                ),
                typography_row(
                    typography.headline_medium,
                    colors.on_surface,
                    "Headline Medium",
                ),
                typography_row(
                    typography.headline_small,
                    colors.on_surface,
                    "Headline Small",
                ),
                typography_row(typography.title_large, colors.on_surface, "Title Large"),
                typography_row(typography.title_medium, colors.on_surface, "Title Medium"),
                typography_row(typography.title_small, colors.on_surface, "Title Small"),
                typography_row(typography.body_large, colors.on_surface, "Body Large"),
                typography_row(typography.body_medium, colors.on_surface, "Body Medium"),
                typography_row(typography.body_small, colors.on_surface, "Body Small"),
                typography_row(typography.label_large, colors.on_surface, "Label Large"),
                typography_row(typography.label_medium, colors.on_surface, "Label Medium"),
                typography_row(typography.label_small, colors.on_surface, "Label Small"),
            ]),
            1 => example_content([
                typography_row(typography.display_large, colors.primary, "Display Large"),
                typography_row(
                    typography.headline_medium,
                    colors.secondary,
                    "Headline Medium",
                ),
                typography_row(typography.title_large, colors.tertiary, "Title Large"),
                typography_row(typography.body_large, colors.error, "Body Large"),
            ]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
