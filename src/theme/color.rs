//! MD3 颜色方案（Color Scheme）
//!
//! 完整实现 Material Design 3 的颜色角色（color roles），
//! 基线（baseline）取值与 material-web 的 `md-sys-color` 令牌一致。
//! 参考: <https://m3.material.io/styles/color/roles>

use gpui::{Hsla, rgb};

/// 十六进制 -> Hsla 便捷函数
pub fn hex(value: u32) -> Hsla {
    rgb(value).into()
}

/// MD3 颜色角色全集（对应 material-web 的 `--md-sys-color-*` 令牌）
#[derive(Clone, Copy, Debug)]
pub struct ColorScheme {
    // Primary
    pub primary: Hsla,
    pub on_primary: Hsla,
    pub primary_container: Hsla,
    pub on_primary_container: Hsla,
    pub inverse_primary: Hsla,
    // Secondary
    pub secondary: Hsla,
    pub on_secondary: Hsla,
    pub secondary_container: Hsla,
    pub on_secondary_container: Hsla,
    // Tertiary
    pub tertiary: Hsla,
    pub on_tertiary: Hsla,
    pub tertiary_container: Hsla,
    pub on_tertiary_container: Hsla,
    // Error
    pub error: Hsla,
    pub on_error: Hsla,
    pub error_container: Hsla,
    pub on_error_container: Hsla,
    // Surface
    pub surface: Hsla,
    pub on_surface: Hsla,
    pub surface_variant: Hsla,
    pub on_surface_variant: Hsla,
    pub surface_dim: Hsla,
    pub surface_bright: Hsla,
    pub surface_container_lowest: Hsla,
    pub surface_container_low: Hsla,
    pub surface_container: Hsla,
    pub surface_container_high: Hsla,
    pub surface_container_highest: Hsla,
    pub inverse_surface: Hsla,
    pub inverse_on_surface: Hsla,
    // Outline & misc
    pub outline: Hsla,
    pub outline_variant: Hsla,
    pub scrim: Hsla,
    pub shadow: Hsla,
    pub surface_tint: Hsla,
    // 兼容旧命名
    pub background: Hsla,
    pub on_background: Hsla,
}

impl ColorScheme {
    /// MD3 基线亮色方案（material-web baseline light）
    pub fn light() -> Self {
        Self {
            primary: hex(0x6750A4),
            on_primary: hex(0xFFFFFF),
            primary_container: hex(0xEADDFF),
            on_primary_container: hex(0x4F378B),
            inverse_primary: hex(0xD0BCFF),

            secondary: hex(0x625B71),
            on_secondary: hex(0xFFFFFF),
            secondary_container: hex(0xE8DEF8),
            on_secondary_container: hex(0x4A4458),

            tertiary: hex(0x7D5260),
            on_tertiary: hex(0xFFFFFF),
            tertiary_container: hex(0xFFD8E4),
            on_tertiary_container: hex(0x633B48),

            error: hex(0xB3261E),
            on_error: hex(0xFFFFFF),
            error_container: hex(0xF9DEDC),
            on_error_container: hex(0x8C1D18),

            surface: hex(0xFEF7FF),
            on_surface: hex(0x1D1B20),
            surface_variant: hex(0xE7E0EC),
            on_surface_variant: hex(0x49454F),
            surface_dim: hex(0xDED8E1),
            surface_bright: hex(0xFEF7FF),
            surface_container_lowest: hex(0xFFFFFF),
            surface_container_low: hex(0xF7F2FA),
            surface_container: hex(0xF3EDF7),
            surface_container_high: hex(0xECE6F0),
            surface_container_highest: hex(0xE6E0E9),
            inverse_surface: hex(0x322F35),
            inverse_on_surface: hex(0xF5EFF7),

            outline: hex(0x79747E),
            outline_variant: hex(0xCAC4D0),
            scrim: hex(0x000000),
            shadow: hex(0x000000),
            surface_tint: hex(0x6750A4),

            background: hex(0xFEF7FF),
            on_background: hex(0x1D1B20),
        }
    }

    /// MD3 基线暗色方案（material-web baseline dark）
    pub fn dark() -> Self {
        Self {
            primary: hex(0xD0BCFF),
            on_primary: hex(0x381E72),
            primary_container: hex(0x4F378B),
            on_primary_container: hex(0xEADDFF),
            inverse_primary: hex(0x6750A4),

            secondary: hex(0xCCC2DC),
            on_secondary: hex(0x332D41),
            secondary_container: hex(0x4A4458),
            on_secondary_container: hex(0xE8DEF8),

            tertiary: hex(0xEFB8C8),
            on_tertiary: hex(0x492532),
            tertiary_container: hex(0x633B48),
            on_tertiary_container: hex(0xFFD8E4),

            error: hex(0xF2B8B5),
            on_error: hex(0x601410),
            error_container: hex(0x8C1D18),
            on_error_container: hex(0xF9DEDC),

            surface: hex(0x141218),
            on_surface: hex(0xE6E0E9),
            surface_variant: hex(0x49454F),
            on_surface_variant: hex(0xCAC4D0),
            surface_dim: hex(0x141218),
            surface_bright: hex(0x3B383E),
            surface_container_lowest: hex(0x0F0D13),
            surface_container_low: hex(0x1D1B20),
            surface_container: hex(0x211F26),
            surface_container_high: hex(0x2B2930),
            surface_container_highest: hex(0x36343B),
            inverse_surface: hex(0xE6E0E9),
            inverse_on_surface: hex(0x322F35),

            outline: hex(0x938F99),
            outline_variant: hex(0x49454F),
            scrim: hex(0x000000),
            shadow: hex(0x000000),
            surface_tint: hex(0xD0BCFF),

            background: hex(0x141218),
            on_background: hex(0xE6E0E9),
        }
    }

    /// 禁用态内容色(on_surface × 禁用内容不透明度 38%)。
    pub fn disabled_content(&self, state: &crate::theme::state::StateLayerTokens) -> Hsla {
        self.on_surface.opacity(state.disabled_content)
    }

    /// 禁用态容器色(on_surface × 禁用容器不透明度 12%)。
    pub fn disabled_container(&self, state: &crate::theme::state::StateLayerTokens) -> Hsla {
        self.on_surface.opacity(state.disabled_container)
    }
}
