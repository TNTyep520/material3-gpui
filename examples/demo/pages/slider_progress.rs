//! Slider & Progress 页。
//!
//! Slider 的变化观察在本页面内部——拖动只重渲染本页视图，
//! 不再惊动整棵应用树（掉帧修复）。

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use md3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Slider & Progress 页视图。
pub struct SliderProgressPage {
    slider: Entity<SliderState>,
}

impl SliderProgressPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let slider = Slider::new(0., 100., 40.).step(1.).build(cx);
        cx.new(|cx| {
            // 滑块变化只刷新本页视图
            cx.observe(&slider, |_, _, cx| cx.notify()).detach();
            Self { slider }
        })
    }
}

impl Render for SliderProgressPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();
        let slider_value = self.slider.read(cx).value();

        gallery([
            showcase_group(
                cx,
                "Continuous",
                [div().w_full().child(self.slider.clone()).into_any_element()],
            ),
            showcase_group(
                cx,
                "Standard Linear",
                [
                    div()
                        .flex()
                        .items_center()
                        .gap(px(16.))
                        .w_full()
                        .child(
                            div().flex_1().child(
                                md3_gpui::LinearProgress::new("lp").value(slider_value / 100.),
                            ),
                        )
                        .child(
                            typography
                                .label_medium
                                .apply(div())
                                .text_color(colors.on_surface_variant)
                                .child(format!("{slider_value:.0}%")),
                        )
                        .into_any_element(),
                    md3_gpui::LinearProgress::new("lp-ind")
                        .indeterminate()
                        .into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Standard Circular",
                [md3_gpui::CircularProgress::new()
                    .size(px(40.))
                    .into_any_element()],
            ),
        ])
    }
}
