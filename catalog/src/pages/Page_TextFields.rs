//! Text fields 页：默认、错误和禁用状态，以及动态主题输入。

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{gallery, palette_strip, showcase_group};

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
    /// 创建页面及其初始组件状态。
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
