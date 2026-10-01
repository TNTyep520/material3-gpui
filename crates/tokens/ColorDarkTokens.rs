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

use super::{ColorValue, PaletteTokens, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct ColorDarkTokens;
impl ColorDarkTokens {
    pub const BACKGROUND: ColorValue = PaletteTokens::NEUTRAL6;
    pub const ERROR: ColorValue = PaletteTokens::ERROR80;
    pub const ERROR_CONTAINER: ColorValue = PaletteTokens::ERROR30;
    pub const INVERSE_ON_SURFACE: ColorValue = PaletteTokens::NEUTRAL20;
    pub const INVERSE_PRIMARY: ColorValue = PaletteTokens::PRIMARY40;
    pub const INVERSE_SURFACE: ColorValue = PaletteTokens::NEUTRAL90;
    pub const ON_BACKGROUND: ColorValue = PaletteTokens::NEUTRAL90;
    pub const ON_ERROR: ColorValue = PaletteTokens::ERROR20;
    pub const ON_ERROR_CONTAINER: ColorValue = PaletteTokens::ERROR90;
    pub const ON_PRIMARY: ColorValue = PaletteTokens::PRIMARY20;
    pub const ON_PRIMARY_CONTAINER: ColorValue = PaletteTokens::PRIMARY90;
    pub const ON_PRIMARY_FIXED: ColorValue = PaletteTokens::PRIMARY10;
    pub const ON_PRIMARY_FIXED_VARIANT: ColorValue = PaletteTokens::PRIMARY30;
    pub const ON_SECONDARY: ColorValue = PaletteTokens::SECONDARY20;
    pub const ON_SECONDARY_CONTAINER: ColorValue = PaletteTokens::SECONDARY90;
    pub const ON_SECONDARY_FIXED: ColorValue = PaletteTokens::SECONDARY10;
    pub const ON_SECONDARY_FIXED_VARIANT: ColorValue = PaletteTokens::SECONDARY30;
    pub const ON_SURFACE: ColorValue = PaletteTokens::NEUTRAL90;
    pub const ON_SURFACE_VARIANT: ColorValue = PaletteTokens::NEUTRAL_VARIANT80;
    pub const ON_TERTIARY: ColorValue = PaletteTokens::TERTIARY20;
    pub const ON_TERTIARY_CONTAINER: ColorValue = PaletteTokens::TERTIARY90;
    pub const ON_TERTIARY_FIXED: ColorValue = PaletteTokens::TERTIARY10;
    pub const ON_TERTIARY_FIXED_VARIANT: ColorValue = PaletteTokens::TERTIARY30;
    pub const OUTLINE: ColorValue = PaletteTokens::NEUTRAL_VARIANT60;
    pub const OUTLINE_VARIANT: ColorValue = PaletteTokens::NEUTRAL_VARIANT30;
    pub const PRIMARY: ColorValue = PaletteTokens::PRIMARY80;
    pub const PRIMARY_CONTAINER: ColorValue = PaletteTokens::PRIMARY30;
    pub const PRIMARY_FIXED: ColorValue = PaletteTokens::PRIMARY90;
    pub const PRIMARY_FIXED_DIM: ColorValue = PaletteTokens::PRIMARY80;
    pub const SCRIM: ColorValue = PaletteTokens::NEUTRAL0;
    pub const SECONDARY: ColorValue = PaletteTokens::SECONDARY80;
    pub const SECONDARY_CONTAINER: ColorValue = PaletteTokens::SECONDARY30;
    pub const SECONDARY_FIXED: ColorValue = PaletteTokens::SECONDARY90;
    pub const SECONDARY_FIXED_DIM: ColorValue = PaletteTokens::SECONDARY80;
    pub const SURFACE: ColorValue = PaletteTokens::NEUTRAL6;
    pub const SURFACE_BRIGHT: ColorValue = PaletteTokens::NEUTRAL24;
    pub const SURFACE_CONTAINER: ColorValue = PaletteTokens::NEUTRAL12;
    pub const SURFACE_CONTAINER_HIGH: ColorValue = PaletteTokens::NEUTRAL17;
    pub const SURFACE_CONTAINER_HIGHEST: ColorValue = PaletteTokens::NEUTRAL22;
    pub const SURFACE_CONTAINER_LOW: ColorValue = PaletteTokens::NEUTRAL10;
    pub const SURFACE_CONTAINER_LOWEST: ColorValue = PaletteTokens::NEUTRAL4;
    pub const SURFACE_DIM: ColorValue = PaletteTokens::NEUTRAL6;
    pub const SURFACE_TINT: ColorValue = ColorDarkTokens::PRIMARY;
    pub const SURFACE_VARIANT: ColorValue = PaletteTokens::NEUTRAL_VARIANT30;
    pub const TERTIARY: ColorValue = PaletteTokens::TERTIARY80;
    pub const TERTIARY_CONTAINER: ColorValue = PaletteTokens::TERTIARY30;
    pub const TERTIARY_FIXED: ColorValue = PaletteTokens::TERTIARY90;
    pub const TERTIARY_FIXED_DIM: ColorValue = PaletteTokens::TERTIARY80;
}
impl ColorDarkTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ColorDarkTokens",
            name: "Background",
            value: TokenValue::Color(ColorDarkTokens::BACKGROUND),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "Error",
            value: TokenValue::Color(ColorDarkTokens::ERROR),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "ErrorContainer",
            value: TokenValue::Color(ColorDarkTokens::ERROR_CONTAINER),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "InverseOnSurface",
            value: TokenValue::Color(ColorDarkTokens::INVERSE_ON_SURFACE),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "InversePrimary",
            value: TokenValue::Color(ColorDarkTokens::INVERSE_PRIMARY),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "InverseSurface",
            value: TokenValue::Color(ColorDarkTokens::INVERSE_SURFACE),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnBackground",
            value: TokenValue::Color(ColorDarkTokens::ON_BACKGROUND),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnError",
            value: TokenValue::Color(ColorDarkTokens::ON_ERROR),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnErrorContainer",
            value: TokenValue::Color(ColorDarkTokens::ON_ERROR_CONTAINER),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnPrimary",
            value: TokenValue::Color(ColorDarkTokens::ON_PRIMARY),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnPrimaryContainer",
            value: TokenValue::Color(ColorDarkTokens::ON_PRIMARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnPrimaryFixed",
            value: TokenValue::Color(ColorDarkTokens::ON_PRIMARY_FIXED),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnPrimaryFixedVariant",
            value: TokenValue::Color(ColorDarkTokens::ON_PRIMARY_FIXED_VARIANT),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnSecondary",
            value: TokenValue::Color(ColorDarkTokens::ON_SECONDARY),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnSecondaryContainer",
            value: TokenValue::Color(ColorDarkTokens::ON_SECONDARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnSecondaryFixed",
            value: TokenValue::Color(ColorDarkTokens::ON_SECONDARY_FIXED),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnSecondaryFixedVariant",
            value: TokenValue::Color(ColorDarkTokens::ON_SECONDARY_FIXED_VARIANT),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnSurface",
            value: TokenValue::Color(ColorDarkTokens::ON_SURFACE),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnSurfaceVariant",
            value: TokenValue::Color(ColorDarkTokens::ON_SURFACE_VARIANT),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnTertiary",
            value: TokenValue::Color(ColorDarkTokens::ON_TERTIARY),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnTertiaryContainer",
            value: TokenValue::Color(ColorDarkTokens::ON_TERTIARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnTertiaryFixed",
            value: TokenValue::Color(ColorDarkTokens::ON_TERTIARY_FIXED),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OnTertiaryFixedVariant",
            value: TokenValue::Color(ColorDarkTokens::ON_TERTIARY_FIXED_VARIANT),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "Outline",
            value: TokenValue::Color(ColorDarkTokens::OUTLINE),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "OutlineVariant",
            value: TokenValue::Color(ColorDarkTokens::OUTLINE_VARIANT),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "Primary",
            value: TokenValue::Color(ColorDarkTokens::PRIMARY),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "PrimaryContainer",
            value: TokenValue::Color(ColorDarkTokens::PRIMARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "PrimaryFixed",
            value: TokenValue::Color(ColorDarkTokens::PRIMARY_FIXED),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "PrimaryFixedDim",
            value: TokenValue::Color(ColorDarkTokens::PRIMARY_FIXED_DIM),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "Scrim",
            value: TokenValue::Color(ColorDarkTokens::SCRIM),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "Secondary",
            value: TokenValue::Color(ColorDarkTokens::SECONDARY),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SecondaryContainer",
            value: TokenValue::Color(ColorDarkTokens::SECONDARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SecondaryFixed",
            value: TokenValue::Color(ColorDarkTokens::SECONDARY_FIXED),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SecondaryFixedDim",
            value: TokenValue::Color(ColorDarkTokens::SECONDARY_FIXED_DIM),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "Surface",
            value: TokenValue::Color(ColorDarkTokens::SURFACE),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SurfaceBright",
            value: TokenValue::Color(ColorDarkTokens::SURFACE_BRIGHT),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SurfaceContainer",
            value: TokenValue::Color(ColorDarkTokens::SURFACE_CONTAINER),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SurfaceContainerHigh",
            value: TokenValue::Color(ColorDarkTokens::SURFACE_CONTAINER_HIGH),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SurfaceContainerHighest",
            value: TokenValue::Color(ColorDarkTokens::SURFACE_CONTAINER_HIGHEST),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SurfaceContainerLow",
            value: TokenValue::Color(ColorDarkTokens::SURFACE_CONTAINER_LOW),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SurfaceContainerLowest",
            value: TokenValue::Color(ColorDarkTokens::SURFACE_CONTAINER_LOWEST),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SurfaceDim",
            value: TokenValue::Color(ColorDarkTokens::SURFACE_DIM),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SurfaceTint",
            value: TokenValue::Color(ColorDarkTokens::SURFACE_TINT),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "SurfaceVariant",
            value: TokenValue::Color(ColorDarkTokens::SURFACE_VARIANT),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "Tertiary",
            value: TokenValue::Color(ColorDarkTokens::TERTIARY),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "TertiaryContainer",
            value: TokenValue::Color(ColorDarkTokens::TERTIARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "TertiaryFixed",
            value: TokenValue::Color(ColorDarkTokens::TERTIARY_FIXED),
        },
        TokenEntry {
            group: "ColorDarkTokens",
            name: "TertiaryFixedDim",
            value: TokenValue::Color(ColorDarkTokens::TERTIARY_FIXED_DIM),
        },
    ];
}
