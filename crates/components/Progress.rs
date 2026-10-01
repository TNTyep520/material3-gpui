//! MD3 Progress Indicators（对应 material-web 的 `md-linear-progress` / `md-circular-progress`）
//!
//! - LinearProgress：4dp 轨道；`value(Some(f))` 为确定进度，`None` 为不确定动画。
//! - CircularProgress：48dp 旋转圆弧（不确定进度）。

use gpui::{
    Animation, AnimationExt, App, ElementId, IntoElement, RenderOnce, Transformation, Window, div,
    ease_in_out, percentage, prelude::*, relative, svg,
};
use std::time::Duration;

use crate::theme::ActiveTheme;

/// MD3 线性进度条
#[derive(IntoElement)]
pub struct LinearProgress {
    id: ElementId,
    /// Some(0.0..=1.0) 确定进度；None 为不确定动画
    value: Option<f32>,
}

/// AndroidX LinearProgressIndicator 对应的线性进度指示器。
pub type LinearProgressIndicator = LinearProgress;

impl LinearProgress {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = Some(value.clamp(0., 1.));
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.value = None;
        self
    }
}

impl RenderOnce for LinearProgress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = LinearProgressStyle::resolve(cx.theme().token_set());
        let active = style.active_color;
        let track = style.track_color;

        let container = div()
            .id(self.id)
            .w_full()
            .h(style.height)
            .rounded(style.corner_radius)
            .bg(track)
            .overflow_hidden();

        match self.value {
            Some(value) => {
                container.child(div().h_full().w(relative(value)).rounded_full().bg(active))
            }
            None => container.child(
                div().relative().size_full().child(
                    div()
                        .absolute()
                        .top_0()
                        .h_full()
                        .w(relative(0.4))
                        .rounded_full()
                        .bg(active)
                        .with_animation(
                            "md3-linear-indeterminate",
                            Animation::new(Duration::from_millis(1200))
                                .repeat()
                                .with_easing(ease_in_out),
                            |el, delta| el.left(relative(-0.4 + delta * 1.4)),
                        ),
                ),
            ),
        }
    }
}

/// MD3 环形进度指示器（不确定进度）
#[derive(IntoElement)]
pub struct CircularProgress {
    size: Option<gpui::Pixels>,
}

/// AndroidX CircularProgressIndicator 对应的环形进度指示器。
pub type CircularProgressIndicator = CircularProgress;

impl CircularProgress {
    pub fn new() -> Self {
        Self { size: None }
    }

    /// 覆盖默认尺寸(默认取环形进度令牌的 40dp)。
    pub fn size(mut self, size: gpui::Pixels) -> Self {
        self.size = Some(size);
        self
    }
}

impl Default for CircularProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for CircularProgress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = CircularProgressStyle::resolve(cx.theme().token_set());
        svg()
            .path(crate::assets::PROGRESS_ARC_SVG_PATH)
            .size(self.size.unwrap_or(style.size))
            .text_color(style.color)
            .with_animation(
                "md3-circular-indeterminate",
                Animation::new(Duration::from_millis(1000)).repeat(),
                |el, delta| el.with_transformation(Transformation::rotate(percentage(delta))),
            )
    }
}

pub use appearance::{CircularProgressStyle, LinearProgressStyle};

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels};
    /// 线性进度条样式。
    #[derive(Clone, Copy, Debug)]
    pub struct LinearProgressStyle {
        /// 活动条颜色。
        pub active_color: Hsla,
        /// 轨道颜色。
        pub track_color: Hsla,
        /// 高度。
        pub height: Pixels,
        /// 圆角。
        pub corner_radius: Pixels,
    }
    impl LinearProgressStyle {
        /// 由令牌推导默认样式(ProgressIndicatorTokens / LinearProgressIndicatorTokens)。
        pub fn resolve(tokens: &TokenSet) -> Self {
            use crate::tokens::{LinearProgressIndicatorTokens, ProgressIndicatorTokens};
            Self {
                active_color: ProgressIndicatorTokens::ACTIVE_INDICATOR_COLOR.resolve(tokens),
                track_color: ProgressIndicatorTokens::TRACK_COLOR.resolve(tokens),
                height: LinearProgressIndicatorTokens::HEIGHT.pixels(),
                corner_radius: tokens.shapes.full,
            }
        }
    }
    /// 环形进度指示器样式。
    #[derive(Clone, Copy, Debug)]
    pub struct CircularProgressStyle {
        /// 颜色。
        pub color: Hsla,
        /// 默认尺寸。
        pub size: Pixels,
    }
    impl CircularProgressStyle {
        /// 由令牌推导默认样式(CircularProgressIndicatorTokens,默认 40dp)。
        pub fn resolve(tokens: &TokenSet) -> Self {
            use crate::tokens::{CircularProgressIndicatorTokens, ProgressIndicatorTokens};
            Self {
                color: ProgressIndicatorTokens::ACTIVE_INDICATOR_COLOR.resolve(tokens),
                size: CircularProgressIndicatorTokens::SIZE.pixels(),
            }
        }
    }
}
