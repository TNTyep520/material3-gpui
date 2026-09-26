//! Text fields 页：含动态色种子输入（输入 hex 实时应用主题）。

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Text fields 页视图。
pub struct TextFieldsPage {
    pub seed_field: Entity<TextFieldState>,
    tf_name: Entity<TextFieldState>,
    tf_error: Entity<TextFieldState>,
    tf_disabled: Entity<TextFieldState>,
    /// 种子色解析成功回调（由根视图接线；参数为 ARGB 种子色）。
    pub(crate) on_seed_changed: Option<super::PageCallback<u32>>,
}

impl TextFieldsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let seed_field = TextField::new("seed-field", "Seed color (hex)")
            .value("6750A4")
            .helper("Type a hex color (like 6750A4); the theme applies live")
            .build(cx);
        let tf_name = TextField::new("tf-name", "Name").build(cx);
        let tf_error = TextField::new("tf-error", "Email")
            .error("Please enter a valid email address")
            .build(cx);
        let tf_disabled = TextField::new("tf-disabled", "Disabled")
            .disabled(true)
            .build(cx);

        cx.new(|cx: &mut gpui::Context<Self>| {
            // 种子色输入实时应用主题（回调由根视图接线）
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

    /// 设置种子色变化回调（根视图首帧接线）。
    pub fn set_on_seed_changed(&mut self, handler: super::PageCallback<u32>) {
        self.on_seed_changed = Some(handler);
    }
}

impl Render for TextFieldsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gallery([
            showcase_group(
                cx,
                "Text Fields",
                [div()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .w_full()
                    .child(self.tf_name.clone())
                    .child(self.tf_error.clone())
                    .child(self.tf_disabled.clone())
                    .into_any_element()],
            ),
            showcase_group(
                cx,
                "Seed Color",
                [div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .child(self.seed_field.clone())
                    .into_any_element()],
            ),
        ])
    }
}
