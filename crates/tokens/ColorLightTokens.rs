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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tokens.kt

use super::{ColorDarkTokens, ColorValue, PaletteTokens, TokenEntry, TokenValue};
use crate::theme::ColorScheme;

#[derive(Clone, Copy, Debug, Default)]
pub struct ColorLightTokens;
impl ColorLightTokens {
    pub const BACKGROUND: ColorValue = PaletteTokens::NEUTRAL98;
    pub const ERROR: ColorValue = PaletteTokens::ERROR40;
    pub const ERROR_CONTAINER: ColorValue = PaletteTokens::ERROR90;
    pub const INVERSE_ON_SURFACE: ColorValue = PaletteTokens::NEUTRAL95;
    pub const INVERSE_PRIMARY: ColorValue = PaletteTokens::PRIMARY80;
    pub const INVERSE_SURFACE: ColorValue = PaletteTokens::NEUTRAL20;
    pub const ON_BACKGROUND: ColorValue = PaletteTokens::NEUTRAL10;
    pub const ON_ERROR: ColorValue = PaletteTokens::ERROR100;
    pub const ON_ERROR_CONTAINER: ColorValue = PaletteTokens::ERROR10;
    pub const ON_PRIMARY: ColorValue = PaletteTokens::PRIMARY100;
    pub const ON_PRIMARY_CONTAINER: ColorValue = PaletteTokens::PRIMARY10;
    pub const ON_PRIMARY_FIXED: ColorValue = PaletteTokens::PRIMARY10;
    pub const ON_PRIMARY_FIXED_VARIANT: ColorValue = PaletteTokens::PRIMARY30;
    pub const ON_SECONDARY: ColorValue = PaletteTokens::SECONDARY100;
    pub const ON_SECONDARY_CONTAINER: ColorValue = PaletteTokens::SECONDARY10;
    pub const ON_SECONDARY_FIXED: ColorValue = PaletteTokens::SECONDARY10;
    pub const ON_SECONDARY_FIXED_VARIANT: ColorValue = PaletteTokens::SECONDARY30;
    pub const ON_SURFACE: ColorValue = PaletteTokens::NEUTRAL10;
    pub const ON_SURFACE_VARIANT: ColorValue = PaletteTokens::NEUTRAL_VARIANT30;
    pub const ON_TERTIARY: ColorValue = PaletteTokens::TERTIARY100;
    pub const ON_TERTIARY_CONTAINER: ColorValue = PaletteTokens::TERTIARY10;
    pub const ON_TERTIARY_FIXED: ColorValue = PaletteTokens::TERTIARY10;
    pub const ON_TERTIARY_FIXED_VARIANT: ColorValue = PaletteTokens::TERTIARY30;
    pub const OUTLINE: ColorValue = PaletteTokens::NEUTRAL_VARIANT50;
    pub const OUTLINE_VARIANT: ColorValue = PaletteTokens::NEUTRAL_VARIANT80;
    pub const PRIMARY: ColorValue = PaletteTokens::PRIMARY40;
    pub const PRIMARY_CONTAINER: ColorValue = PaletteTokens::PRIMARY90;
    pub const PRIMARY_FIXED: ColorValue = PaletteTokens::PRIMARY90;
    pub const PRIMARY_FIXED_DIM: ColorValue = PaletteTokens::PRIMARY80;
    pub const SCRIM: ColorValue = PaletteTokens::NEUTRAL0;
    pub const SECONDARY: ColorValue = PaletteTokens::SECONDARY40;
    pub const SECONDARY_CONTAINER: ColorValue = PaletteTokens::SECONDARY90;
    pub const SECONDARY_FIXED: ColorValue = PaletteTokens::SECONDARY90;
    pub const SECONDARY_FIXED_DIM: ColorValue = PaletteTokens::SECONDARY80;
    pub const SURFACE: ColorValue = PaletteTokens::NEUTRAL98;
    pub const SURFACE_BRIGHT: ColorValue = PaletteTokens::NEUTRAL98;
    pub const SURFACE_CONTAINER: ColorValue = PaletteTokens::NEUTRAL94;
    pub const SURFACE_CONTAINER_HIGH: ColorValue = PaletteTokens::NEUTRAL92;
    pub const SURFACE_CONTAINER_HIGHEST: ColorValue = PaletteTokens::NEUTRAL90;
    pub const SURFACE_CONTAINER_LOW: ColorValue = PaletteTokens::NEUTRAL96;
    pub const SURFACE_CONTAINER_LOWEST: ColorValue = PaletteTokens::NEUTRAL100;
    pub const SURFACE_DIM: ColorValue = PaletteTokens::NEUTRAL87;
    pub const SURFACE_TINT: ColorValue = ColorLightTokens::PRIMARY;
    pub const SURFACE_VARIANT: ColorValue = PaletteTokens::NEUTRAL_VARIANT90;
    pub const TERTIARY: ColorValue = PaletteTokens::TERTIARY40;
    pub const TERTIARY_CONTAINER: ColorValue = PaletteTokens::TERTIARY90;
    pub const TERTIARY_FIXED: ColorValue = PaletteTokens::TERTIARY90;
    pub const TERTIARY_FIXED_DIM: ColorValue = PaletteTokens::TERTIARY80;
}
impl ColorScheme {
    pub fn androidx_light() -> Self {
        Self {
            background: ColorLightTokens::BACKGROUND.resolve(),
            error: ColorLightTokens::ERROR.resolve(),
            error_container: ColorLightTokens::ERROR_CONTAINER.resolve(),
            inverse_on_surface: ColorLightTokens::INVERSE_ON_SURFACE.resolve(),
            inverse_primary: ColorLightTokens::INVERSE_PRIMARY.resolve(),
            inverse_surface: ColorLightTokens::INVERSE_SURFACE.resolve(),
            on_background: ColorLightTokens::ON_BACKGROUND.resolve(),
            on_error: ColorLightTokens::ON_ERROR.resolve(),
            on_error_container: ColorLightTokens::ON_ERROR_CONTAINER.resolve(),
            on_primary: ColorLightTokens::ON_PRIMARY.resolve(),
            on_primary_container: ColorLightTokens::ON_PRIMARY_CONTAINER.resolve(),
            on_primary_fixed: ColorLightTokens::ON_PRIMARY_FIXED.resolve(),
            on_primary_fixed_variant: ColorLightTokens::ON_PRIMARY_FIXED_VARIANT.resolve(),
            on_secondary: ColorLightTokens::ON_SECONDARY.resolve(),
            on_secondary_container: ColorLightTokens::ON_SECONDARY_CONTAINER.resolve(),
            on_secondary_fixed: ColorLightTokens::ON_SECONDARY_FIXED.resolve(),
            on_secondary_fixed_variant: ColorLightTokens::ON_SECONDARY_FIXED_VARIANT.resolve(),
            on_surface: ColorLightTokens::ON_SURFACE.resolve(),
            on_surface_variant: ColorLightTokens::ON_SURFACE_VARIANT.resolve(),
            on_tertiary: ColorLightTokens::ON_TERTIARY.resolve(),
            on_tertiary_container: ColorLightTokens::ON_TERTIARY_CONTAINER.resolve(),
            on_tertiary_fixed: ColorLightTokens::ON_TERTIARY_FIXED.resolve(),
            on_tertiary_fixed_variant: ColorLightTokens::ON_TERTIARY_FIXED_VARIANT.resolve(),
            outline: ColorLightTokens::OUTLINE.resolve(),
            outline_variant: ColorLightTokens::OUTLINE_VARIANT.resolve(),
            primary: ColorLightTokens::PRIMARY.resolve(),
            primary_container: ColorLightTokens::PRIMARY_CONTAINER.resolve(),
            primary_fixed: ColorLightTokens::PRIMARY_FIXED.resolve(),
            primary_fixed_dim: ColorLightTokens::PRIMARY_FIXED_DIM.resolve(),
            scrim: ColorLightTokens::SCRIM.resolve(),
            secondary: ColorLightTokens::SECONDARY.resolve(),
            secondary_container: ColorLightTokens::SECONDARY_CONTAINER.resolve(),
            secondary_fixed: ColorLightTokens::SECONDARY_FIXED.resolve(),
            secondary_fixed_dim: ColorLightTokens::SECONDARY_FIXED_DIM.resolve(),
            surface: ColorLightTokens::SURFACE.resolve(),
            surface_bright: ColorLightTokens::SURFACE_BRIGHT.resolve(),
            surface_container: ColorLightTokens::SURFACE_CONTAINER.resolve(),
            surface_container_high: ColorLightTokens::SURFACE_CONTAINER_HIGH.resolve(),
            surface_container_highest: ColorLightTokens::SURFACE_CONTAINER_HIGHEST.resolve(),
            surface_container_low: ColorLightTokens::SURFACE_CONTAINER_LOW.resolve(),
            surface_container_lowest: ColorLightTokens::SURFACE_CONTAINER_LOWEST.resolve(),
            surface_dim: ColorLightTokens::SURFACE_DIM.resolve(),
            surface_tint: ColorLightTokens::SURFACE_TINT.resolve(),
            surface_variant: ColorLightTokens::SURFACE_VARIANT.resolve(),
            tertiary: ColorLightTokens::TERTIARY.resolve(),
            tertiary_container: ColorLightTokens::TERTIARY_CONTAINER.resolve(),
            tertiary_fixed: ColorLightTokens::TERTIARY_FIXED.resolve(),
            tertiary_fixed_dim: ColorLightTokens::TERTIARY_FIXED_DIM.resolve(),
            shadow: PaletteTokens::BLACK.resolve(),
        }
    }
    pub fn androidx_dark() -> Self {
        Self {
            background: ColorDarkTokens::BACKGROUND.resolve(),
            error: ColorDarkTokens::ERROR.resolve(),
            error_container: ColorDarkTokens::ERROR_CONTAINER.resolve(),
            inverse_on_surface: ColorDarkTokens::INVERSE_ON_SURFACE.resolve(),
            inverse_primary: ColorDarkTokens::INVERSE_PRIMARY.resolve(),
            inverse_surface: ColorDarkTokens::INVERSE_SURFACE.resolve(),
            on_background: ColorDarkTokens::ON_BACKGROUND.resolve(),
            on_error: ColorDarkTokens::ON_ERROR.resolve(),
            on_error_container: ColorDarkTokens::ON_ERROR_CONTAINER.resolve(),
            on_primary: ColorDarkTokens::ON_PRIMARY.resolve(),
            on_primary_container: ColorDarkTokens::ON_PRIMARY_CONTAINER.resolve(),
            on_primary_fixed: ColorDarkTokens::ON_PRIMARY_FIXED.resolve(),
            on_primary_fixed_variant: ColorDarkTokens::ON_PRIMARY_FIXED_VARIANT.resolve(),
            on_secondary: ColorDarkTokens::ON_SECONDARY.resolve(),
            on_secondary_container: ColorDarkTokens::ON_SECONDARY_CONTAINER.resolve(),
            on_secondary_fixed: ColorDarkTokens::ON_SECONDARY_FIXED.resolve(),
            on_secondary_fixed_variant: ColorDarkTokens::ON_SECONDARY_FIXED_VARIANT.resolve(),
            on_surface: ColorDarkTokens::ON_SURFACE.resolve(),
            on_surface_variant: ColorDarkTokens::ON_SURFACE_VARIANT.resolve(),
            on_tertiary: ColorDarkTokens::ON_TERTIARY.resolve(),
            on_tertiary_container: ColorDarkTokens::ON_TERTIARY_CONTAINER.resolve(),
            on_tertiary_fixed: ColorDarkTokens::ON_TERTIARY_FIXED.resolve(),
            on_tertiary_fixed_variant: ColorDarkTokens::ON_TERTIARY_FIXED_VARIANT.resolve(),
            outline: ColorDarkTokens::OUTLINE.resolve(),
            outline_variant: ColorDarkTokens::OUTLINE_VARIANT.resolve(),
            primary: ColorDarkTokens::PRIMARY.resolve(),
            primary_container: ColorDarkTokens::PRIMARY_CONTAINER.resolve(),
            primary_fixed: ColorDarkTokens::PRIMARY_FIXED.resolve(),
            primary_fixed_dim: ColorDarkTokens::PRIMARY_FIXED_DIM.resolve(),
            scrim: ColorDarkTokens::SCRIM.resolve(),
            secondary: ColorDarkTokens::SECONDARY.resolve(),
            secondary_container: ColorDarkTokens::SECONDARY_CONTAINER.resolve(),
            secondary_fixed: ColorDarkTokens::SECONDARY_FIXED.resolve(),
            secondary_fixed_dim: ColorDarkTokens::SECONDARY_FIXED_DIM.resolve(),
            surface: ColorDarkTokens::SURFACE.resolve(),
            surface_bright: ColorDarkTokens::SURFACE_BRIGHT.resolve(),
            surface_container: ColorDarkTokens::SURFACE_CONTAINER.resolve(),
            surface_container_high: ColorDarkTokens::SURFACE_CONTAINER_HIGH.resolve(),
            surface_container_highest: ColorDarkTokens::SURFACE_CONTAINER_HIGHEST.resolve(),
            surface_container_low: ColorDarkTokens::SURFACE_CONTAINER_LOW.resolve(),
            surface_container_lowest: ColorDarkTokens::SURFACE_CONTAINER_LOWEST.resolve(),
            surface_dim: ColorDarkTokens::SURFACE_DIM.resolve(),
            surface_tint: ColorDarkTokens::SURFACE_TINT.resolve(),
            surface_variant: ColorDarkTokens::SURFACE_VARIANT.resolve(),
            tertiary: ColorDarkTokens::TERTIARY.resolve(),
            tertiary_container: ColorDarkTokens::TERTIARY_CONTAINER.resolve(),
            tertiary_fixed: ColorDarkTokens::TERTIARY_FIXED.resolve(),
            tertiary_fixed_dim: ColorDarkTokens::TERTIARY_FIXED_DIM.resolve(),
            shadow: PaletteTokens::BLACK.resolve(),
        }
    }
}
impl ColorLightTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ColorLightTokens",
            name: "Background",
            value: TokenValue::Color(ColorLightTokens::BACKGROUND),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "Error",
            value: TokenValue::Color(ColorLightTokens::ERROR),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "ErrorContainer",
            value: TokenValue::Color(ColorLightTokens::ERROR_CONTAINER),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "InverseOnSurface",
            value: TokenValue::Color(ColorLightTokens::INVERSE_ON_SURFACE),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "InversePrimary",
            value: TokenValue::Color(ColorLightTokens::INVERSE_PRIMARY),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "InverseSurface",
            value: TokenValue::Color(ColorLightTokens::INVERSE_SURFACE),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnBackground",
            value: TokenValue::Color(ColorLightTokens::ON_BACKGROUND),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnError",
            value: TokenValue::Color(ColorLightTokens::ON_ERROR),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnErrorContainer",
            value: TokenValue::Color(ColorLightTokens::ON_ERROR_CONTAINER),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnPrimary",
            value: TokenValue::Color(ColorLightTokens::ON_PRIMARY),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnPrimaryContainer",
            value: TokenValue::Color(ColorLightTokens::ON_PRIMARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnPrimaryFixed",
            value: TokenValue::Color(ColorLightTokens::ON_PRIMARY_FIXED),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnPrimaryFixedVariant",
            value: TokenValue::Color(ColorLightTokens::ON_PRIMARY_FIXED_VARIANT),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnSecondary",
            value: TokenValue::Color(ColorLightTokens::ON_SECONDARY),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnSecondaryContainer",
            value: TokenValue::Color(ColorLightTokens::ON_SECONDARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnSecondaryFixed",
            value: TokenValue::Color(ColorLightTokens::ON_SECONDARY_FIXED),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnSecondaryFixedVariant",
            value: TokenValue::Color(ColorLightTokens::ON_SECONDARY_FIXED_VARIANT),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnSurface",
            value: TokenValue::Color(ColorLightTokens::ON_SURFACE),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnSurfaceVariant",
            value: TokenValue::Color(ColorLightTokens::ON_SURFACE_VARIANT),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnTertiary",
            value: TokenValue::Color(ColorLightTokens::ON_TERTIARY),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnTertiaryContainer",
            value: TokenValue::Color(ColorLightTokens::ON_TERTIARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnTertiaryFixed",
            value: TokenValue::Color(ColorLightTokens::ON_TERTIARY_FIXED),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OnTertiaryFixedVariant",
            value: TokenValue::Color(ColorLightTokens::ON_TERTIARY_FIXED_VARIANT),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "Outline",
            value: TokenValue::Color(ColorLightTokens::OUTLINE),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "OutlineVariant",
            value: TokenValue::Color(ColorLightTokens::OUTLINE_VARIANT),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "Primary",
            value: TokenValue::Color(ColorLightTokens::PRIMARY),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "PrimaryContainer",
            value: TokenValue::Color(ColorLightTokens::PRIMARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "PrimaryFixed",
            value: TokenValue::Color(ColorLightTokens::PRIMARY_FIXED),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "PrimaryFixedDim",
            value: TokenValue::Color(ColorLightTokens::PRIMARY_FIXED_DIM),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "Scrim",
            value: TokenValue::Color(ColorLightTokens::SCRIM),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "Secondary",
            value: TokenValue::Color(ColorLightTokens::SECONDARY),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SecondaryContainer",
            value: TokenValue::Color(ColorLightTokens::SECONDARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SecondaryFixed",
            value: TokenValue::Color(ColorLightTokens::SECONDARY_FIXED),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SecondaryFixedDim",
            value: TokenValue::Color(ColorLightTokens::SECONDARY_FIXED_DIM),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "Surface",
            value: TokenValue::Color(ColorLightTokens::SURFACE),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SurfaceBright",
            value: TokenValue::Color(ColorLightTokens::SURFACE_BRIGHT),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SurfaceContainer",
            value: TokenValue::Color(ColorLightTokens::SURFACE_CONTAINER),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SurfaceContainerHigh",
            value: TokenValue::Color(ColorLightTokens::SURFACE_CONTAINER_HIGH),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SurfaceContainerHighest",
            value: TokenValue::Color(ColorLightTokens::SURFACE_CONTAINER_HIGHEST),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SurfaceContainerLow",
            value: TokenValue::Color(ColorLightTokens::SURFACE_CONTAINER_LOW),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SurfaceContainerLowest",
            value: TokenValue::Color(ColorLightTokens::SURFACE_CONTAINER_LOWEST),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SurfaceDim",
            value: TokenValue::Color(ColorLightTokens::SURFACE_DIM),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SurfaceTint",
            value: TokenValue::Color(ColorLightTokens::SURFACE_TINT),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "SurfaceVariant",
            value: TokenValue::Color(ColorLightTokens::SURFACE_VARIANT),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "Tertiary",
            value: TokenValue::Color(ColorLightTokens::TERTIARY),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "TertiaryContainer",
            value: TokenValue::Color(ColorLightTokens::TERTIARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "TertiaryFixed",
            value: TokenValue::Color(ColorLightTokens::TERTIARY_FIXED),
        },
        TokenEntry {
            group: "ColorLightTokens",
            name: "TertiaryFixedDim",
            value: TokenValue::Color(ColorLightTokens::TERTIARY_FIXED_DIM),
        },
    ];
}
