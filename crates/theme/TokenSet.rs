//! MD3 令牌集合（Token Set）。
//!
//! 对应 [m3fx](https://github.com/Glavo/m3fx) 的 `M3TokenSet(+Builder)`
//! （Apache-2.0，© 2026 Glavo）：聚合主题所需的全部不可变令牌组。
//! 令牌组在构建后按引用共享，不做拷贝；覆盖以"组"为单位。

use gpui::SharedString;

use super::ThemeMode;
use crate::motion::MotionScheme;

use super::color::ColorScheme;
use super::component_tokens::ComponentTokens;
use super::density::Density;
use super::elevation::ElevationTokens;
use super::profile::Profile;
use super::shape::Shapes;
use super::state::StateLayerTokens;
use super::typography::TypeScale;

/// 完整的不可变令牌集合：主题的数据来源。
#[derive(Clone, Debug)]
pub struct TokenSet {
    /// 生成默认值的 Profile。
    pub profile: Profile,
    /// 布局密度。
    pub density: Density,
    /// 颜色角色令牌。
    pub colors: ColorScheme,
    /// 字体排印令牌。
    pub typography: TypeScale,
    /// 形状（圆角刻度）令牌。
    pub shapes: Shapes,
    /// 高度令牌。
    pub elevation: ElevationTokens,
    /// 运动方案。
    pub motion: MotionScheme,
    /// 状态层令牌。
    pub state_layer: StateLayerTokens,
    /// 组件令牌。
    pub component: ComponentTokens,
}

impl TokenSet {
    pub fn androidx(mode: ThemeMode) -> Self {
        let colors = match mode {
            ThemeMode::Light => ColorScheme::androidx_light(),
            ThemeMode::Dark => ColorScheme::androidx_dark(),
        };
        Self::builder(Profile::Baseline2021, colors)
            .with_typography(TypeScale::androidx())
            .with_component(ComponentTokens::androidx())
            .build()
    }

    /// 以 Profile 默认值 + 指定颜色方案构建。
    pub fn new(profile: Profile, colors: ColorScheme) -> Self {
        Self::builder(profile, colors).build()
    }

    /// 创建构建器：以 Profile 默认值 + 指定颜色方案初始化。
    pub fn builder(profile: Profile, colors: ColorScheme) -> TokenSetBuilder {
        TokenSetBuilder::from_profile(profile, colors)
    }

    /// 从既有令牌集派生构建器（整组覆盖）。
    pub fn rebuild(self) -> TokenSetBuilder {
        TokenSetBuilder::from_token_set(self)
    }
}

/// `TokenSet` 构建器：支持逐组覆盖。
///
/// 用法：
/// ```ignore
/// use material3_gpui::theme::{Profile, TokenSet};
/// use material3_gpui::motion::MotionScheme;
///
/// let tokens = TokenSet::builder(Profile::Baseline2021, colors)
///     .with_motion(MotionScheme::standard())
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct TokenSetBuilder {
    profile: Profile,
    density: Density,
    colors: ColorScheme,
    typography: TypeScale,
    shapes: Shapes,
    elevation: ElevationTokens,
    motion: MotionScheme,
    state_layer: StateLayerTokens,
    component: ComponentTokens,
}

impl TokenSetBuilder {
    /// 以 Profile 默认值初始化。
    fn from_profile(profile: Profile, colors: ColorScheme) -> Self {
        Self {
            profile,
            density: Density::default(),
            colors,
            typography: TypeScale::baseline(),
            shapes: Shapes::baseline(),
            elevation: ElevationTokens::baseline(),
            motion: MotionScheme::standard(),
            state_layer: StateLayerTokens::baseline(),
            component: ComponentTokens::default(),
        }
    }

    /// 从既有令牌集初始化（拷贝全部组）。
    fn from_token_set(tokens: TokenSet) -> Self {
        Self {
            profile: tokens.profile,
            density: tokens.density,
            colors: tokens.colors,
            typography: tokens.typography,
            shapes: tokens.shapes,
            elevation: tokens.elevation,
            motion: tokens.motion,
            state_layer: tokens.state_layer,
            component: tokens.component,
        }
    }

    /// 覆盖密度。
    pub fn with_density(mut self, density: Density) -> Self {
        self.density = density;
        self
    }

    /// 覆盖颜色方案组。
    pub fn with_colors(mut self, colors: ColorScheme) -> Self {
        self.colors = colors;
        self
    }

    /// 覆盖字体排印组。
    pub fn with_typography(mut self, typography: TypeScale) -> Self {
        self.typography = typography;
        self
    }

    /// 覆盖形状组。
    pub fn with_shapes(mut self, shapes: Shapes) -> Self {
        self.shapes = shapes;
        self
    }

    /// 覆盖高度组。
    pub fn with_elevation(mut self, elevation: ElevationTokens) -> Self {
        self.elevation = elevation;
        self
    }

    /// 覆盖运动方案组。
    pub fn with_motion(mut self, motion: MotionScheme) -> Self {
        self.motion = motion;
        self
    }

    /// 覆盖状态层组。
    pub fn with_state_layer(mut self, state_layer: StateLayerTokens) -> Self {
        self.state_layer = state_layer;
        self
    }

    /// 覆盖组件令牌组。
    pub fn with_component(mut self, component: ComponentTokens) -> Self {
        self.component = component;
        self
    }

    /// 构建不可变令牌集。
    pub fn build(self) -> TokenSet {
        TokenSet {
            profile: self.profile,
            density: self.density,
            colors: self.colors,
            typography: self.typography,
            shapes: self.shapes,
            elevation: self.elevation,
            motion: self.motion,
            state_layer: self.state_layer,
            component: self.component,
        }
    }
}

/// 全局主题字体族的默认值。
pub const DEFAULT_FONT_FAMILY: &str = "Roboto";

/// 便捷转换：字符串字体族。
pub fn font_family(name: &str) -> SharedString {
    SharedString::from(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::motion::MotionRole;
    use crate::theme::color::ColorScheme;

    #[test]
    fn profile_selects_token_families() {
        let baseline = TokenSet::new(Profile::Baseline2021, ColorScheme::light());
        assert_eq!(baseline.profile, Profile::Baseline2021);
        assert_eq!(
            baseline
                .motion
                .spec(MotionRole::DefaultSpatial)
                .spring
                .stiffness,
            700.0
        );
        assert_eq!(baseline.shapes.medium, gpui::px(12.));
    }

    #[test]
    fn builder_overrides_groups() {
        let tokens = TokenSet::builder(Profile::Baseline2021, ColorScheme::dark())
            .with_motion(MotionScheme::standard())
            .build();
        // 颜色组保持 dark，运动组被整体替换
        assert_eq!(tokens.colors.surface, ColorScheme::dark().surface);
        assert_eq!(tokens.shapes.extra_small, gpui::px(4.));
    }
}
