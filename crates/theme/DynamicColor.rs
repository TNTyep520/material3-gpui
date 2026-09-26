//! 动态色（Dynamic Color）：由种子色生成 MD3 颜色方案。
//!
//! 对应 [m3fx](https://github.com/Glavo/m3fx) 依赖的 MonetFX
//! `ColorScheme` 派生逻辑（Apache-2.0），色度学实现使用
//! [`mcu_dynamiccolor`](https://crates.io/crates/mcu-dynamiccolor)
//! （material-color-utilities 的 Rust 移植，支持 2021/2025 双规格）。

use mcu_dynamiccolor::{
    DynamicColor, DynamicScheme, DynamicSchemeOptions, MaterialDynamicColors, SpecVersion,
};
use mcu_hct::Hct;
use mcu_palettes::TonalPalette;

use super::color::{ColorScheme, hex};
use super::profile::Profile;

/// 按官方 material-color-utilities 的 2021 规格生成六组调色板。
///
/// 注：`mcu-dynamiccolor` 0.2.4 的 2021 TonalSpot 主色板 chroma 与官方
/// 实现不一致（36.0，官方为 48.0），导致基线种子色无法还原出
/// material-web 基线色板；为保证与 m3fx（MonetFX）输出一致，
/// 这里按官方常量自行生成调色板并经 `DynamicSchemeOptions` 显式传入。
fn official_palettes_2021(source: &Hct) -> [TonalPalette; 6] {
    let hue = source.hue();
    let from_hue_chroma = |h: f64, c: f64| TonalPalette::from_hue_and_chroma(h, c);
    [
        from_hue_chroma(hue, 48.0),        // primary
        from_hue_chroma(hue, 16.0),        // secondary
        from_hue_chroma(hue + 60.0, 24.0), // tertiary
        from_hue_chroma(hue, 4.0),         // neutral
        from_hue_chroma(hue, 8.0),         // neutral variant
        from_hue_chroma(25.0, 84.0),       // error
    ]
}

/// 由种子色生成完整 MD3 颜色方案。
///
/// `seed` 为 ARGB 颜色（如 `0x6750A4`）；`is_dark` 决定亮/暗色模式；
/// `profile` 决定动态色规格（2021 baseline / 2025 expressive）与色彩风格。
///
/// 与 m3fx `M3Theme::fromSeed(seed, brightness, profile, …)` 语义一致，
/// 对比度取标准值 `0.0`。
pub fn color_scheme_from_seed(seed: u32, is_dark: bool, profile: Profile) -> ColorScheme {
    let source = Hct::from_int(seed);
    let mut options = DynamicSchemeOptions::new(source, profile.color_style(), 0.0, is_dark);
    options.spec_version = Some(profile.color_spec_version());
    // 2021 规格使用本地官方参数色板（见 `official_palettes_2021` 文档）；
    // 2025 规格沿用 crate 委托生成
    if profile.color_spec_version() == SpecVersion::Spec2021 {
        let [
            primary,
            secondary,
            tertiary,
            neutral,
            neutral_variant,
            error,
        ] = official_palettes_2021(&source);
        options.primary_palette = Some(primary);
        options.secondary_palette = Some(secondary);
        options.tertiary_palette = Some(tertiary);
        options.neutral_palette = Some(neutral);
        options.neutral_variant_palette = Some(neutral_variant);
        options.error_palette = Some(error);
    }
    let scheme = DynamicScheme::new(options);

    // 角色解析辅助：DynamicColor -> gpui Hsla
    fn role(scheme: &DynamicScheme, dc: DynamicColor) -> gpui::Hsla {
        hex(dc.get_argb(scheme))
    }

    ColorScheme {
        primary: role(&scheme, MaterialDynamicColors::primary()),
        on_primary: role(&scheme, MaterialDynamicColors::on_primary()),
        primary_container: role(&scheme, MaterialDynamicColors::primary_container()),
        on_primary_container: role(&scheme, MaterialDynamicColors::on_primary_container()),
        inverse_primary: role(&scheme, MaterialDynamicColors::inverse_primary()),

        secondary: role(&scheme, MaterialDynamicColors::secondary()),
        on_secondary: role(&scheme, MaterialDynamicColors::on_secondary()),
        secondary_container: role(&scheme, MaterialDynamicColors::secondary_container()),
        on_secondary_container: role(&scheme, MaterialDynamicColors::on_secondary_container()),

        tertiary: role(&scheme, MaterialDynamicColors::tertiary()),
        on_tertiary: role(&scheme, MaterialDynamicColors::on_tertiary()),
        tertiary_container: role(&scheme, MaterialDynamicColors::tertiary_container()),
        on_tertiary_container: role(&scheme, MaterialDynamicColors::on_tertiary_container()),

        error: role(&scheme, MaterialDynamicColors::error()),
        on_error: role(&scheme, MaterialDynamicColors::on_error()),
        error_container: role(&scheme, MaterialDynamicColors::error_container()),
        on_error_container: role(&scheme, MaterialDynamicColors::on_error_container()),

        surface: role(&scheme, MaterialDynamicColors::surface()),
        on_surface: role(&scheme, MaterialDynamicColors::on_surface()),
        surface_variant: role(&scheme, MaterialDynamicColors::surface_variant()),
        on_surface_variant: role(&scheme, MaterialDynamicColors::on_surface_variant()),
        surface_dim: role(&scheme, MaterialDynamicColors::surface_dim()),
        surface_bright: role(&scheme, MaterialDynamicColors::surface_bright()),
        surface_container_lowest: role(&scheme, MaterialDynamicColors::surface_container_lowest()),
        surface_container_low: role(&scheme, MaterialDynamicColors::surface_container_low()),
        surface_container: role(&scheme, MaterialDynamicColors::surface_container()),
        surface_container_high: role(&scheme, MaterialDynamicColors::surface_container_high()),
        surface_container_highest: role(
            &scheme,
            MaterialDynamicColors::surface_container_highest(),
        ),
        inverse_surface: role(&scheme, MaterialDynamicColors::inverse_surface()),
        inverse_on_surface: role(&scheme, MaterialDynamicColors::inverse_on_surface()),

        outline: role(&scheme, MaterialDynamicColors::outline()),
        outline_variant: role(&scheme, MaterialDynamicColors::outline_variant()),
        scrim: role(&scheme, MaterialDynamicColors::scrim()),
        shadow: role(&scheme, MaterialDynamicColors::shadow()),
        surface_tint: role(&scheme, MaterialDynamicColors::surface_tint()),

        background: role(&scheme, MaterialDynamicColors::background()),
        on_background: role(&scheme, MaterialDynamicColors::on_background()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_seed_matches_baseline_scheme() {
        // 基线种子色 #6750A4 在 2021 spec（官方 chroma=48）下应还原为
        // material-web 基线 primary（tone 40 的主色板颜色）
        let scheme = color_scheme_from_seed(0x6750A4, false, Profile::Baseline2021);
        let primary: gpui::Rgba = scheme.primary.into();
        assert!((primary.r - 0.404).abs() < 0.05, "r={}", primary.r);
        assert!((primary.g - 0.314).abs() < 0.05, "g={}", primary.g);
        assert!((primary.b - 0.643).abs() < 0.05, "b={}", primary.b);
    }

    #[test]
    fn dark_scheme_has_dark_surface() {
        let scheme = color_scheme_from_seed(0x6750A4, true, Profile::Baseline2021);
        let surface: gpui::Rgba = scheme.surface.into();
        assert!(surface.r < 0.2 && surface.g < 0.2 && surface.b < 0.2);
    }
}
