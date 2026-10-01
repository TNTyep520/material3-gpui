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

use super::{TokenEntry, TokenValue};
use crate::theme::TokenSet;
use gpui::Hsla;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorToken {
    Background,
    Error,
    ErrorContainer,
    InverseOnSurface,
    InversePrimary,
    InverseSurface,
    OnBackground,
    OnError,
    OnErrorContainer,
    OnPrimary,
    OnPrimaryContainer,
    OnPrimaryFixed,
    OnPrimaryFixedVariant,
    OnSecondary,
    OnSecondaryContainer,
    OnSecondaryFixed,
    OnSecondaryFixedVariant,
    OnSurface,
    OnSurfaceVariant,
    OnTertiary,
    OnTertiaryContainer,
    OnTertiaryFixed,
    OnTertiaryFixedVariant,
    Outline,
    OutlineVariant,
    Primary,
    PrimaryContainer,
    PrimaryFixed,
    PrimaryFixedDim,
    Scrim,
    Secondary,
    SecondaryContainer,
    SecondaryFixed,
    SecondaryFixedDim,
    Surface,
    SurfaceBright,
    SurfaceContainer,
    SurfaceContainerHigh,
    SurfaceContainerHighest,
    SurfaceContainerLow,
    SurfaceContainerLowest,
    SurfaceDim,
    SurfaceTint,
    SurfaceVariant,
    Tertiary,
    TertiaryContainer,
    TertiaryFixed,
    TertiaryFixedDim,
}
impl ColorToken {
    pub const ALL: &'static [Self] = &[
        Self::Background,
        Self::Error,
        Self::ErrorContainer,
        Self::InverseOnSurface,
        Self::InversePrimary,
        Self::InverseSurface,
        Self::OnBackground,
        Self::OnError,
        Self::OnErrorContainer,
        Self::OnPrimary,
        Self::OnPrimaryContainer,
        Self::OnPrimaryFixed,
        Self::OnPrimaryFixedVariant,
        Self::OnSecondary,
        Self::OnSecondaryContainer,
        Self::OnSecondaryFixed,
        Self::OnSecondaryFixedVariant,
        Self::OnSurface,
        Self::OnSurfaceVariant,
        Self::OnTertiary,
        Self::OnTertiaryContainer,
        Self::OnTertiaryFixed,
        Self::OnTertiaryFixedVariant,
        Self::Outline,
        Self::OutlineVariant,
        Self::Primary,
        Self::PrimaryContainer,
        Self::PrimaryFixed,
        Self::PrimaryFixedDim,
        Self::Scrim,
        Self::Secondary,
        Self::SecondaryContainer,
        Self::SecondaryFixed,
        Self::SecondaryFixedDim,
        Self::Surface,
        Self::SurfaceBright,
        Self::SurfaceContainer,
        Self::SurfaceContainerHigh,
        Self::SurfaceContainerHighest,
        Self::SurfaceContainerLow,
        Self::SurfaceContainerLowest,
        Self::SurfaceDim,
        Self::SurfaceTint,
        Self::SurfaceVariant,
        Self::Tertiary,
        Self::TertiaryContainer,
        Self::TertiaryFixed,
        Self::TertiaryFixedDim,
    ];
    pub const fn id(self) -> u16 {
        match self {
            Self::Background => 0,
            Self::Error => 1,
            Self::ErrorContainer => 2,
            Self::InverseOnSurface => 3,
            Self::InversePrimary => 4,
            Self::InverseSurface => 5,
            Self::OnBackground => 6,
            Self::OnError => 7,
            Self::OnErrorContainer => 8,
            Self::OnPrimary => 9,
            Self::OnPrimaryContainer => 10,
            Self::OnPrimaryFixed => 11,
            Self::OnPrimaryFixedVariant => 12,
            Self::OnSecondary => 13,
            Self::OnSecondaryContainer => 14,
            Self::OnSecondaryFixed => 15,
            Self::OnSecondaryFixedVariant => 16,
            Self::OnSurface => 17,
            Self::OnSurfaceVariant => 18,
            Self::OnTertiary => 19,
            Self::OnTertiaryContainer => 20,
            Self::OnTertiaryFixed => 21,
            Self::OnTertiaryFixedVariant => 22,
            Self::Outline => 23,
            Self::OutlineVariant => 24,
            Self::Primary => 25,
            Self::PrimaryContainer => 26,
            Self::PrimaryFixed => 27,
            Self::PrimaryFixedDim => 28,
            Self::Scrim => 29,
            Self::Secondary => 30,
            Self::SecondaryContainer => 31,
            Self::SecondaryFixed => 32,
            Self::SecondaryFixedDim => 33,
            Self::Surface => 34,
            Self::SurfaceBright => 35,
            Self::SurfaceContainer => 36,
            Self::SurfaceContainerHigh => 37,
            Self::SurfaceContainerHighest => 38,
            Self::SurfaceContainerLow => 39,
            Self::SurfaceContainerLowest => 40,
            Self::SurfaceDim => 41,
            Self::SurfaceTint => 42,
            Self::SurfaceVariant => 43,
            Self::Tertiary => 44,
            Self::TertiaryContainer => 45,
            Self::TertiaryFixed => 46,
            Self::TertiaryFixedDim => 47,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorSchemeKeyTokens;
impl ColorSchemeKeyTokens {
    pub const BACKGROUND: ColorToken = ColorToken::Background;
    pub const ERROR: ColorToken = ColorToken::Error;
    pub const ERROR_CONTAINER: ColorToken = ColorToken::ErrorContainer;
    pub const INVERSE_ON_SURFACE: ColorToken = ColorToken::InverseOnSurface;
    pub const INVERSE_PRIMARY: ColorToken = ColorToken::InversePrimary;
    pub const INVERSE_SURFACE: ColorToken = ColorToken::InverseSurface;
    pub const ON_BACKGROUND: ColorToken = ColorToken::OnBackground;
    pub const ON_ERROR: ColorToken = ColorToken::OnError;
    pub const ON_ERROR_CONTAINER: ColorToken = ColorToken::OnErrorContainer;
    pub const ON_PRIMARY: ColorToken = ColorToken::OnPrimary;
    pub const ON_PRIMARY_CONTAINER: ColorToken = ColorToken::OnPrimaryContainer;
    pub const ON_PRIMARY_FIXED: ColorToken = ColorToken::OnPrimaryFixed;
    pub const ON_PRIMARY_FIXED_VARIANT: ColorToken = ColorToken::OnPrimaryFixedVariant;
    pub const ON_SECONDARY: ColorToken = ColorToken::OnSecondary;
    pub const ON_SECONDARY_CONTAINER: ColorToken = ColorToken::OnSecondaryContainer;
    pub const ON_SECONDARY_FIXED: ColorToken = ColorToken::OnSecondaryFixed;
    pub const ON_SECONDARY_FIXED_VARIANT: ColorToken = ColorToken::OnSecondaryFixedVariant;
    pub const ON_SURFACE: ColorToken = ColorToken::OnSurface;
    pub const ON_SURFACE_VARIANT: ColorToken = ColorToken::OnSurfaceVariant;
    pub const ON_TERTIARY: ColorToken = ColorToken::OnTertiary;
    pub const ON_TERTIARY_CONTAINER: ColorToken = ColorToken::OnTertiaryContainer;
    pub const ON_TERTIARY_FIXED: ColorToken = ColorToken::OnTertiaryFixed;
    pub const ON_TERTIARY_FIXED_VARIANT: ColorToken = ColorToken::OnTertiaryFixedVariant;
    pub const OUTLINE: ColorToken = ColorToken::Outline;
    pub const OUTLINE_VARIANT: ColorToken = ColorToken::OutlineVariant;
    pub const PRIMARY: ColorToken = ColorToken::Primary;
    pub const PRIMARY_CONTAINER: ColorToken = ColorToken::PrimaryContainer;
    pub const PRIMARY_FIXED: ColorToken = ColorToken::PrimaryFixed;
    pub const PRIMARY_FIXED_DIM: ColorToken = ColorToken::PrimaryFixedDim;
    pub const SCRIM: ColorToken = ColorToken::Scrim;
    pub const SECONDARY: ColorToken = ColorToken::Secondary;
    pub const SECONDARY_CONTAINER: ColorToken = ColorToken::SecondaryContainer;
    pub const SECONDARY_FIXED: ColorToken = ColorToken::SecondaryFixed;
    pub const SECONDARY_FIXED_DIM: ColorToken = ColorToken::SecondaryFixedDim;
    pub const SURFACE: ColorToken = ColorToken::Surface;
    pub const SURFACE_BRIGHT: ColorToken = ColorToken::SurfaceBright;
    pub const SURFACE_CONTAINER: ColorToken = ColorToken::SurfaceContainer;
    pub const SURFACE_CONTAINER_HIGH: ColorToken = ColorToken::SurfaceContainerHigh;
    pub const SURFACE_CONTAINER_HIGHEST: ColorToken = ColorToken::SurfaceContainerHighest;
    pub const SURFACE_CONTAINER_LOW: ColorToken = ColorToken::SurfaceContainerLow;
    pub const SURFACE_CONTAINER_LOWEST: ColorToken = ColorToken::SurfaceContainerLowest;
    pub const SURFACE_DIM: ColorToken = ColorToken::SurfaceDim;
    pub const SURFACE_TINT: ColorToken = ColorToken::SurfaceTint;
    pub const SURFACE_VARIANT: ColorToken = ColorToken::SurfaceVariant;
    pub const TERTIARY: ColorToken = ColorToken::Tertiary;
    pub const TERTIARY_CONTAINER: ColorToken = ColorToken::TertiaryContainer;
    pub const TERTIARY_FIXED: ColorToken = ColorToken::TertiaryFixed;
    pub const TERTIARY_FIXED_DIM: ColorToken = ColorToken::TertiaryFixedDim;
}
impl ColorToken {
    pub fn resolve(self, tokens: &TokenSet) -> Hsla {
        match self {
            Self::Background => tokens.colors.background,
            Self::Error => tokens.colors.error,
            Self::ErrorContainer => tokens.colors.error_container,
            Self::InverseOnSurface => tokens.colors.inverse_on_surface,
            Self::InversePrimary => tokens.colors.inverse_primary,
            Self::InverseSurface => tokens.colors.inverse_surface,
            Self::OnBackground => tokens.colors.on_background,
            Self::OnError => tokens.colors.on_error,
            Self::OnErrorContainer => tokens.colors.on_error_container,
            Self::OnPrimary => tokens.colors.on_primary,
            Self::OnPrimaryContainer => tokens.colors.on_primary_container,
            Self::OnPrimaryFixed => tokens.colors.on_primary_fixed,
            Self::OnPrimaryFixedVariant => tokens.colors.on_primary_fixed_variant,
            Self::OnSecondary => tokens.colors.on_secondary,
            Self::OnSecondaryContainer => tokens.colors.on_secondary_container,
            Self::OnSecondaryFixed => tokens.colors.on_secondary_fixed,
            Self::OnSecondaryFixedVariant => tokens.colors.on_secondary_fixed_variant,
            Self::OnSurface => tokens.colors.on_surface,
            Self::OnSurfaceVariant => tokens.colors.on_surface_variant,
            Self::OnTertiary => tokens.colors.on_tertiary,
            Self::OnTertiaryContainer => tokens.colors.on_tertiary_container,
            Self::OnTertiaryFixed => tokens.colors.on_tertiary_fixed,
            Self::OnTertiaryFixedVariant => tokens.colors.on_tertiary_fixed_variant,
            Self::Outline => tokens.colors.outline,
            Self::OutlineVariant => tokens.colors.outline_variant,
            Self::Primary => tokens.colors.primary,
            Self::PrimaryContainer => tokens.colors.primary_container,
            Self::PrimaryFixed => tokens.colors.primary_fixed,
            Self::PrimaryFixedDim => tokens.colors.primary_fixed_dim,
            Self::Scrim => tokens.colors.scrim,
            Self::Secondary => tokens.colors.secondary,
            Self::SecondaryContainer => tokens.colors.secondary_container,
            Self::SecondaryFixed => tokens.colors.secondary_fixed,
            Self::SecondaryFixedDim => tokens.colors.secondary_fixed_dim,
            Self::Surface => tokens.colors.surface,
            Self::SurfaceBright => tokens.colors.surface_bright,
            Self::SurfaceContainer => tokens.colors.surface_container,
            Self::SurfaceContainerHigh => tokens.colors.surface_container_high,
            Self::SurfaceContainerHighest => tokens.colors.surface_container_highest,
            Self::SurfaceContainerLow => tokens.colors.surface_container_low,
            Self::SurfaceContainerLowest => tokens.colors.surface_container_lowest,
            Self::SurfaceDim => tokens.colors.surface_dim,
            Self::SurfaceTint => tokens.colors.surface_tint,
            Self::SurfaceVariant => tokens.colors.surface_variant,
            Self::Tertiary => tokens.colors.tertiary,
            Self::TertiaryContainer => tokens.colors.tertiary_container,
            Self::TertiaryFixed => tokens.colors.tertiary_fixed,
            Self::TertiaryFixedDim => tokens.colors.tertiary_fixed_dim,
        }
    }
}
impl ColorSchemeKeyTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "Background",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::BACKGROUND),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "Error",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ERROR),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "ErrorContainer",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ERROR_CONTAINER),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "InverseOnSurface",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::INVERSE_ON_SURFACE),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "InversePrimary",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::INVERSE_PRIMARY),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "InverseSurface",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::INVERSE_SURFACE),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnBackground",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_BACKGROUND),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnError",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_ERROR),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnErrorContainer",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_ERROR_CONTAINER),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnPrimary",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_PRIMARY),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnPrimaryContainer",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnPrimaryFixed",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_PRIMARY_FIXED),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnPrimaryFixedVariant",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_PRIMARY_FIXED_VARIANT),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnSecondary",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_SECONDARY),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnSecondaryContainer",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnSecondaryFixed",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_SECONDARY_FIXED),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnSecondaryFixedVariant",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_SECONDARY_FIXED_VARIANT),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnSurface",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_SURFACE),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnSurfaceVariant",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_SURFACE_VARIANT),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnTertiary",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_TERTIARY),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnTertiaryContainer",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnTertiaryFixed",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_TERTIARY_FIXED),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OnTertiaryFixedVariant",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::ON_TERTIARY_FIXED_VARIANT),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "Outline",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::OUTLINE),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "OutlineVariant",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::OUTLINE_VARIANT),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "Primary",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::PRIMARY),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "PrimaryContainer",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::PRIMARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "PrimaryFixed",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::PRIMARY_FIXED),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "PrimaryFixedDim",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::PRIMARY_FIXED_DIM),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "Scrim",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SCRIM),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "Secondary",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SECONDARY),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SecondaryContainer",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SECONDARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SecondaryFixed",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SECONDARY_FIXED),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SecondaryFixedDim",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SECONDARY_FIXED_DIM),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "Surface",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SURFACE),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SurfaceBright",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SURFACE_BRIGHT),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SurfaceContainer",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SURFACE_CONTAINER),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SurfaceContainerHigh",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGH),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SurfaceContainerHighest",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SurfaceContainerLow",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SurfaceContainerLowest",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SURFACE_CONTAINER_LOWEST),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SurfaceDim",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SURFACE_DIM),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SurfaceTint",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SURFACE_TINT),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "SurfaceVariant",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::SURFACE_VARIANT),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "Tertiary",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::TERTIARY),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "TertiaryContainer",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::TERTIARY_CONTAINER),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "TertiaryFixed",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::TERTIARY_FIXED),
        },
        TokenEntry {
            group: "ColorSchemeKeyTokens",
            name: "TertiaryFixedDim",
            value: TokenValue::ColorRole(ColorSchemeKeyTokens::TERTIARY_FIXED_DIM),
        },
    ];
}
