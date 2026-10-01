//! Slider 与 Progress 对比页，滑块数值驱动确定进度，刷新局限于本页。

use gpui::{
    App, AppContext as _, Entity, IntoElement, Render, Styled, WeakEntity, Window, div, prelude::*,
    px,
};
use material3_gpui::prelude::*;

use super::{LogErr as _, gallery, showcase_group};

/// Slider & Progress 页视图。
pub struct SliderProgressPage {
    weak: WeakEntity<Self>,
    slider: Entity<SliderState>,
    range: (f32, f32),
}

impl SliderProgressPage {
    /// 创建页面及其初始组件状态。
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let weak = cx.entity().downgrade();
            let slider = Slider::new(0., 100., 40.).step(1.).build(cx);
            // 滑块变化只刷新本页视图
            cx.observe(&slider, |_, _, cx| cx.notify()).detach();
            Self {
                weak,
                slider,
                range: (0.2, 0.78),
            }
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
                "Slider · determinate",
                [div()
                    .w_full()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .child(
                        div()
                            .flex()
                            .items_baseline()
                            .gap(px(8.))
                            .child(
                                typography
                                    .title_large
                                    .apply(div())
                                    .text_color(colors.on_surface)
                                    .child(format!("{slider_value:.0}%")),
                            )
                            .child(
                                typography
                                    .body_medium
                                    .apply(div())
                                    .text_color(colors.on_surface_variant)
                                    .child("Progress"),
                            ),
                    )
                    .child(div().w_full().child(self.slider.clone()))
                    .child(div().w_full().child(
                        material3_gpui::LinearProgress::new("lp-bound").value(slider_value / 100.),
                    ))
                    .into_any_element()],
            ),
            showcase_group(
                cx,
                "Progress variants",
                [
                    div()
                        .w_full()
                        .min_w_0()
                        .child(
                            material3_gpui::WavyProgressIndicator::new("wavy")
                                .value(slider_value / 100.),
                        )
                        .into_any_element(),
                    material3_gpui::CircularProgress::new()
                        .size(px(48.))
                        .into_any_element(),
                    material3_gpui::LoadingIndicator::new("loading")
                        .size(px(48.))
                        .into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Range & indeterminate",
                [
                    material3_gpui::RangeSlider::new("range", self.range.0, self.range.1)
                        .on_value_change({
                            let weak = self.weak.clone();
                            move |next, _, cx| {
                                weak.update(cx, |page, cx| {
                                    page.range = next;
                                    cx.notify();
                                })
                                .log_err();
                            }
                        })
                        .into_any_element(),
                    div()
                        .w_full()
                        .min_w_0()
                        .child(material3_gpui::LinearProgress::new("lp-ind").indeterminate())
                        .into_any_element(),
                ],
            ),
        ])
    }
}
