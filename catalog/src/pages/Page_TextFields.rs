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

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{gallery, palette_strip, showcase_group};

pub struct TextFieldsPage {
    pub seed_field: Entity<TextFieldState>,
    tf_name: Entity<TextFieldState>,
    tf_error: Entity<TextFieldState>,
    tf_disabled: Entity<TextFieldState>,

    pub(crate) on_seed_changed: Option<super::PageCallback<u32>>,
}

impl TextFieldsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let seed_field = TextField::new("seed-field", "Seed color (hex)")
            .value("6750A4")
            .helper("Type a hex color (like 6750A4); the theme applies live")
            .build(cx);
        let tf_name = TextField::new("tf-name", "Display name").build(cx);
        let tf_error = TextField::new("tf-error", "Handle")
            .error("@name already exists in this studio")
            .build(cx);
        let tf_disabled = TextField::new("tf-disabled", "Workspace")
            .value("Milk crate studio")
            .disabled(true)
            .build(cx);

        cx.new(|cx: &mut gpui::Context<Self>| {
            cx.observe(&seed_field, |this: &mut Self, field, cx| {
                let value = field
                    .read(cx)
                    .value()
                    .trim()
                    .trim_start_matches('#')
                    .to_string();
                if let Ok(seed) = u32::from_str_radix(&value, 16)
                    && seed != 0
                    && let Some(handler) = this.on_seed_changed.clone()
                {
                    handler(seed, cx);
                }
            })
            .detach();

            Self {
                seed_field,
                tf_name,
                tf_error,
                tf_disabled,
                on_seed_changed: None,
            }
        })
    }

    pub fn set_on_seed_changed(&mut self, handler: super::PageCallback<u32>) {
        self.on_seed_changed = Some(handler);
    }
}

impl Render for TextFieldsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let typography = *theme.typography();
        let colors = *theme.colors();

        gallery([
            showcase_group(
                cx,
                "States",
                [
                    div()
                        .w(px(280.))
                        .max_w_full()
                        .child(self.tf_name.clone())
                        .into_any_element(),
                    div()
                        .w(px(280.))
                        .max_w_full()
                        .child(self.tf_error.clone())
                        .into_any_element(),
                    div()
                        .w(px(280.))
                        .max_w_full()
                        .child(self.tf_disabled.clone())
                        .into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Theme input",
                [div()
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .w_full()
                    .min_w_0()
                    .child(self.seed_field.clone())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.))
                            .flex_wrap()
                            .child(
                                typography
                                    .label_large
                                    .apply(div())
                                    .text_color(colors.on_surface_variant)
                                    .child("Palette preview"),
                            )
                            .child(palette_strip(cx)),
                    )
                    .into_any_element()],
            ),
        ])
    }
}
