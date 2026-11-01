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
    AnyElement, App, AppContext as _, Context, Entity, IntoElement, ParentElement as _, Styled,
    div, px,
};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, LogErr as _, example_content, example_row};

pub struct TextFieldsPage {
    tf_name: Entity<TextFieldState>,
    tf_error: Entity<TextFieldState>,
    tf_disabled: Entity<TextFieldState>,
    tf_outlined: Entity<TextFieldState>,
    tf_password: Entity<TextFieldState>,
    value_label: String,
}

impl TextFieldsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx: &mut Context<Self>| {
            let weak = cx.entity().downgrade();
            let tf_name = {
                let weak = weak;
                TextField::new("tf-name", "Display name")
                    .on_value_change(move |value, _, cx| {
                        weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                            page.value_label = if value.is_empty() {
                                "(empty)".to_string()
                            } else {
                                value.to_string()
                            };
                            cx.notify();
                        })
                        .log_err();
                    })
                    .build(cx)
            };
            let tf_error = TextField::new("tf-error", "Handle")
                .error("@name already exists in this studio")
                .build(cx);
            let tf_disabled = TextField::new("tf-disabled", "Workspace")
                .value("Milk crate studio")
                .disabled(true)
                .build(cx);
            let tf_outlined = TextField::new("tf-outlined", "Outlined")
                .outlined()
                .build(cx);
            let tf_password = SecureTextField::new("tf-password", "Passphrase")
                .value("secret")
                .build(cx);

            Self {
                tf_name,
                tf_error,
                tf_disabled,
                tf_outlined,
                tf_password,
                value_label: "(empty)".to_string(),
            }
        })
    }
}

/// 固定宽度包裹文本框,保持各行布局与旧示例一致。
fn field_at(field: impl IntoElement) -> AnyElement {
    div()
        .w(px(280.))
        .max_w_full()
        .child(field)
        .into_any_element()
}

impl ComponentPage for TextFieldsPage {
    fn description(&self) -> &'static str {
        "Text fields let users enter and edit text."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "SimpleTextFieldSample",
                description: "Text fields examples",
            },
            ExampleMeta {
                name: "SimpleOutlinedTextFieldSample",
                description: "Text fields examples",
            },
            ExampleMeta {
                name: "TextFieldWithErrorState",
                description: "Text fields examples",
            },
            ExampleMeta {
                name: "PasswordTextField",
                description: "Text fields examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        let theme = cx.theme();
        let typography = *theme.typography();
        let colors = *theme.colors();
        match index {
            0 => example_content([
                example_row(cx, "Text field", [field_at(self.tf_name.clone())]),
                example_row(
                    cx,
                    "Value",
                    [field_at(
                        typography
                            .body_medium
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(format!("Value: {}", self.value_label)),
                    )],
                ),
                example_row(cx, "Disabled", [field_at(self.tf_disabled.clone())]),
            ]),
            1 => example_content([example_row(
                cx,
                "Outlined",
                [field_at(self.tf_outlined.clone())],
            )]),
            2 => example_content([example_row(cx, "Error", [field_at(self.tf_error.clone())])]),
            3 => example_content([example_row(
                cx,
                "Secure",
                [field_at(self.tf_password.clone())],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
