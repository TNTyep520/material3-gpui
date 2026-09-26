//! MD3 主题系统：颜色 / 字体 / 形状 / 高度 / 状态层 / 运动 / 组件令牌的聚合。
//!
//! 架构移植自 [m3fx](https://github.com/Glavo/m3fx) 的 `tokens` + `theme`
//! 包（Apache-2.0，© 2026 Glavo）：主题 = [`TokenSet`]（不可变令牌集合，
//! 支持 profile 预设与整组覆盖）+ 全局字体族。
//!
//! 用法：
//! ```ignore
//! use material3_gpui::prelude::*;
//!
//! application().run(|cx| {
//!     material3_gpui::init(cx);                          // 安装默认亮色主题
//!     // 或者：种子色动态色主题
//!     Theme::set(cx, Theme::from_seed(0x6750A4, ThemeMode::Light, Profile::Baseline2021));
//! });
//!
//! // 在任意 render 中：
//! let theme = cx.theme();
//! div().bg(theme.colors().surface)
//! ```

#[path = "theme/Color.rs"]
mod color;
#[path = "theme/ComponentTokens.rs"]
mod component_tokens;
#[path = "theme/Density.rs"]
mod density;
#[cfg(feature = "dynamic-color")]
#[path = "theme/DynamicColor.rs"]
mod dynamic_color;
#[path = "theme/Elevation.rs"]
mod elevation;
#[path = "theme/Profile.rs"]
mod profile;
#[path = "theme/Shape.rs"]
mod shape;
#[path = "theme/State.rs"]
mod state;
#[path = "theme/TokenSet.rs"]
mod token_set;
#[path = "theme/Typography.rs"]
mod typography;

pub use color::{ColorScheme, hex};
pub use component_tokens::{
    ButtonTokens, ComponentTokens, MenuTokens, SliderTokens, SnackbarTokens, SwitchTokens,
    TextFieldTokens, TooltipTokens,
};
pub use density::Density;
#[cfg(feature = "dynamic-color")]
pub use dynamic_color::color_scheme_from_seed;
pub use elevation::{Elevation, ElevationTokens};
pub use profile::Profile;
pub use shape::Shapes;
pub use state::*;
pub use token_set::{DEFAULT_FONT_FAMILY, TokenSet, TokenSetBuilder, font_family};
pub use typography::{TypeScale, TypeStyle};

use gpui::{App, Global, SharedString};

use crate::motion::MotionScheme;

/// 主题模式
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    /// 亮色。
    #[default]
    Light,
    /// 暗色。
    Dark,
}

/// MD3 主题（作为 gpui Global 存储）。
///
/// 主题 = [`TokenSet`] + 字体族；颜色/字型/形状/运动等一律经
/// 访问器读取。主题是纯数据对象，替换后需触发重绘
/// （如 `cx.refresh_windows()`）。
#[derive(Clone, Debug)]
pub struct Theme {
    mode: ThemeMode,
    tokens: TokenSet,
    font_family: SharedString,
}

impl Global for Theme {}

impl Theme {
    pub fn androidx(mode: ThemeMode) -> Self {
        Self::from_token_set(TokenSet::androidx(mode), mode, DEFAULT_FONT_FAMILY)
    }

    /// MD3 基线亮色主题（种子色 `#6750A4`，Baseline2021 profile）。
    pub fn light() -> Self {
        Self::from_seed(0x6750A4, ThemeMode::Light, Profile::Baseline2021)
    }

    /// MD3 基线暗色主题（种子色 `#6750A4`，Baseline2021 profile）。
    pub fn dark() -> Self {
        Self::from_seed(0x6750A4, ThemeMode::Dark, Profile::Baseline2021)
    }

    /// 由种子色生成主题。
    ///
    /// 基线种子色 `0x6750A4` + `Baseline2021` 直接使用 material-web /
    /// MD3 官方的精确 baseline 亮/暗常量表（与动态色推导存在 ±1~3/255
    /// 的 HCT 舍入差异，为保证 switch 手柄等角色与规范一致，此处精确对齐）。
    ///
    /// 其余种子色需启用 `dynamic-color` feature 经
    /// material-color-utilities 动态生成；未启用时回退到基线色表
    /// （仅亮/暗模式生效）。
    pub fn from_seed(seed: u32, mode: ThemeMode, profile: Profile) -> Self {
        let is_dark = mode == ThemeMode::Dark;

        #[cfg(feature = "dynamic-color")]
        let colors = if seed == 0x6750A4 && profile == Profile::Baseline2021 {
            if is_dark {
                ColorScheme::dark()
            } else {
                ColorScheme::light()
            }
        } else {
            color_scheme_from_seed(seed, is_dark, profile)
        };

        #[cfg(not(feature = "dynamic-color"))]
        let colors = if is_dark {
            ColorScheme::dark()
        } else {
            ColorScheme::light()
        };
        let _ = (seed, profile); // feature 关闭时参数仅由模式决定

        Self::from_token_set(TokenSet::new(profile, colors), mode, DEFAULT_FONT_FAMILY)
    }

    /// 由令牌集构建主题。
    pub fn from_token_set(
        tokens: TokenSet,
        mode: ThemeMode,
        font_family: impl Into<SharedString>,
    ) -> Self {
        Self {
            mode,
            tokens,
            font_family: font_family.into(),
        }
    }

    /// 主题模式。
    pub fn mode(&self) -> ThemeMode {
        self.mode
    }

    /// 是否暗色模式。
    pub fn is_dark(&self) -> bool {
        self.mode == ThemeMode::Dark
    }

    /// 全局字体族。
    pub fn font_family(&self) -> &SharedString {
        &self.font_family
    }

    /// 完整令牌集。
    pub fn token_set(&self) -> &TokenSet {
        &self.tokens
    }

    /// 替换令牌集（保持当前模式与字体族）。
    pub fn set_token_set(&mut self, tokens: TokenSet) {
        self.tokens = tokens;
    }

    /// 令牌 Profile。
    pub fn profile(&self) -> Profile {
        self.tokens.profile
    }

    /// 布局密度。
    pub fn density(&self) -> Density {
        self.tokens.density
    }

    /// 颜色角色令牌。
    pub fn colors(&self) -> &ColorScheme {
        &self.tokens.colors
    }

    /// 字体排印令牌。
    pub fn typography(&self) -> &TypeScale {
        &self.tokens.typography
    }

    /// 形状（圆角刻度）令牌。
    pub fn shapes(&self) -> &Shapes {
        &self.tokens.shapes
    }

    /// 高度令牌。
    pub fn elevation_tokens(&self) -> &ElevationTokens {
        &self.tokens.elevation
    }

    /// 运动方案。
    pub fn motion(&self) -> &MotionScheme {
        &self.tokens.motion
    }

    /// 状态层令牌。
    pub fn state_layer(&self) -> &StateLayerTokens {
        &self.tokens.state_layer
    }

    /// 组件令牌。
    pub fn component(&self) -> &ComponentTokens {
        &self.tokens.component
    }

    /// 读取全局主题。
    pub fn global(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    /// 替换全局主题（切换主题后请触发一次重绘，如 `cx.refresh_windows()`
    /// 或视图 `cx.notify()`）。
    pub fn set(cx: &mut App, theme: Theme) {
        cx.set_global(theme);
    }
}

/// 便捷 trait：`cx.theme()`
pub trait ActiveTheme {
    /// 读取全局主题。
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_seed_matches_baseline_defaults() {
        // 基线种子色的亮色主题应与旧版手写的基线方案一致
        let theme = Theme::light();
        assert_eq!(theme.mode(), ThemeMode::Light);
        assert_eq!(theme.profile(), Profile::Baseline2021);
    }
}
