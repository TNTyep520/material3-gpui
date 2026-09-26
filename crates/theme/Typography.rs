//! MD3 字体排印比例（Type Scale）令牌。
//!
//! 对应 [m3fx](https://github.com/Glavo/m3fx) 的 `M3TypographyTokens`
//! （Apache-2.0，© 2026 Glavo），数值与 Compose Material 3 一致；
//! 同时兼容 material-web 的 `md-sys-typescale` 令牌命名。
//! 参考: <https://m3.material.io/styles/typography/type-scale-tokens>

use gpui::{FontWeight, Pixels, Styled, px};

/// 单个字体样式令牌。
#[derive(Clone, Copy, Debug)]
pub struct TypeStyle {
    /// 字号。
    pub size: Pixels,
    /// 行高。
    pub line_height: Pixels,
    /// 字重。
    pub weight: FontWeight,
    /// 字距（px）。gpui 暂不支持字距渲染，此字段为令牌数据预留。
    pub tracking: f32,
}

impl TypeStyle {
    /// 创建字体样式令牌（tracking 默认为 0）。
    pub const fn new(size: f32, line_height: f32, weight: FontWeight) -> Self {
        Self {
            size: px(size),
            line_height: px(line_height),
            weight,
            tracking: 0.0,
        }
    }

    /// 创建带字距的字体样式令牌。
    pub const fn with_tracking(
        size: f32,
        line_height: f32,
        weight: FontWeight,
        tracking: f32,
    ) -> Self {
        Self {
            size: px(size),
            line_height: px(line_height),
            weight,
            tracking,
        }
    }

    /// 把该字体令牌应用到任意 Styled 元素上。
    ///
    /// 注意：gpui 暂不支持字距，`tracking` 不参与渲染。
    pub fn apply<E: Styled>(&self, el: E) -> E {
        el.text_size(self.size)
            .line_height(self.line_height)
            .font_weight(self.weight)
    }
}

/// MD3 完整 Type Scale（display / headline / title / body / label × large / medium / small）。
#[derive(Clone, Copy, Debug)]
pub struct TypeScale {
    /// display-large。
    pub display_large: TypeStyle,
    /// display-medium。
    pub display_medium: TypeStyle,
    /// display-small。
    pub display_small: TypeStyle,
    /// headline-large。
    pub headline_large: TypeStyle,
    /// headline-medium。
    pub headline_medium: TypeStyle,
    /// headline-small。
    pub headline_small: TypeStyle,
    /// title-large。
    pub title_large: TypeStyle,
    /// title-medium。
    pub title_medium: TypeStyle,
    /// title-small。
    pub title_small: TypeStyle,
    /// body-large。
    pub body_large: TypeStyle,
    /// body-medium。
    pub body_medium: TypeStyle,
    /// body-small。
    pub body_small: TypeStyle,
    /// label-large。
    pub label_large: TypeStyle,
    /// label-medium。
    pub label_medium: TypeStyle,
    /// label-small。
    pub label_small: TypeStyle,
}

impl Default for TypeScale {
    /// 2021 基线字型比例（与 material-web `md-sys-typescale` 一致）。
    fn default() -> Self {
        Self::baseline()
    }
}

impl TypeScale {
    /// 基线（2021）字型比例。
    pub fn baseline() -> Self {
        Self {
            display_large: TypeStyle::with_tracking(57., 64., FontWeight::NORMAL, -0.25),
            display_medium: TypeStyle::new(45., 52., FontWeight::NORMAL),
            display_small: TypeStyle::new(36., 44., FontWeight::NORMAL),
            headline_large: TypeStyle::new(32., 40., FontWeight::NORMAL),
            headline_medium: TypeStyle::new(28., 36., FontWeight::NORMAL),
            headline_small: TypeStyle::new(24., 32., FontWeight::NORMAL),
            title_large: TypeStyle::new(22., 28., FontWeight::NORMAL),
            title_medium: TypeStyle::with_tracking(16., 24., FontWeight::MEDIUM, 0.15),
            title_small: TypeStyle::with_tracking(14., 20., FontWeight::MEDIUM, 0.10),
            body_large: TypeStyle::with_tracking(16., 24., FontWeight::NORMAL, 0.50),
            body_medium: TypeStyle::with_tracking(14., 20., FontWeight::NORMAL, 0.25),
            body_small: TypeStyle::with_tracking(12., 16., FontWeight::NORMAL, 0.40),
            label_large: TypeStyle::with_tracking(14., 20., FontWeight::MEDIUM, 0.10),
            label_medium: TypeStyle::with_tracking(12., 16., FontWeight::MEDIUM, 0.50),
            label_small: TypeStyle::with_tracking(11., 16., FontWeight::MEDIUM, 0.50),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_matches_material_web() {
        let t = TypeScale::baseline();
        assert_eq!(t.display_large.size, px(57.));
        assert_eq!(t.label_large.size, px(14.));
        assert_eq!(t.label_large.weight, FontWeight::MEDIUM);
        // m3fx：titleSmall 与 labelLarge 同值（14/20/500/0.10）
        assert_eq!(t.title_small.size, t.label_large.size);
        assert_eq!(t.title_small.line_height, t.label_large.line_height);
    }
}
