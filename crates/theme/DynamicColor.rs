// Copyright (c) 2026 TNTyep520
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use mcu_dynamiccolor::{
    DynamicColor, DynamicScheme, DynamicSchemeOptions, MaterialDynamicColors, SpecVersion,
};
use mcu_hct::Hct;
use mcu_palettes::TonalPalette;

use super::color::{ColorScheme, hex};
use super::profile::Profile;

fn official_palettes_2021(source: &Hct) -> [TonalPalette; 6] {
    let hue = source.hue();
    let from_hue_chroma = |h: f64, c: f64| TonalPalette::from_hue_and_chroma(h, c);
    [
        from_hue_chroma(hue, 48.0),
        from_hue_chroma(hue, 16.0),
        from_hue_chroma(hue + 60.0, 24.0),
        from_hue_chroma(hue, 4.0),
        from_hue_chroma(hue, 8.0),
        from_hue_chroma(25.0, 84.0),
    ]
}

pub fn color_scheme_from_seed(seed: u32, is_dark: bool, profile: Profile) -> ColorScheme {
    let source = Hct::from_int(seed);
    let mut options = DynamicSchemeOptions::new(source, profile.color_style(), 0.0, is_dark);
    options.spec_version = Some(profile.color_spec_version());

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

    fn role(scheme: &DynamicScheme, dc: DynamicColor) -> gpui::Hsla {
        hex(dc.get_argb(scheme))
    }

    ColorScheme {
        on_primary_fixed: role(&scheme, MaterialDynamicColors::on_primary_fixed()),
        on_primary_fixed_variant: role(&scheme, MaterialDynamicColors::on_primary_fixed_variant()),
        on_secondary_fixed: role(&scheme, MaterialDynamicColors::on_secondary_fixed()),
        on_secondary_fixed_variant: role(
            &scheme,
            MaterialDynamicColors::on_secondary_fixed_variant(),
        ),
        on_tertiary_fixed: role(&scheme, MaterialDynamicColors::on_tertiary_fixed()),
        on_tertiary_fixed_variant: role(
            &scheme,
            MaterialDynamicColors::on_tertiary_fixed_variant(),
        ),
        primary_fixed: role(&scheme, MaterialDynamicColors::primary_fixed()),
        primary_fixed_dim: role(&scheme, MaterialDynamicColors::primary_fixed_dim()),
        secondary_fixed: role(&scheme, MaterialDynamicColors::secondary_fixed()),
        secondary_fixed_dim: role(&scheme, MaterialDynamicColors::secondary_fixed_dim()),
        tertiary_fixed: role(&scheme, MaterialDynamicColors::tertiary_fixed()),
        tertiary_fixed_dim: role(&scheme, MaterialDynamicColors::tertiary_fixed_dim()),
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
