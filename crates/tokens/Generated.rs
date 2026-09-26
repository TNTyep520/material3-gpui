/*
 * Copyright 2021 The Android Open Source Project
 * Copyright 2022 The Android Open Source Project
 * Copyright 2023 The Android Open Source Project
 * Copyright 2024 The Android Open Source Project
 * Copyright 2025 The Android Open Source Project
 * Copyright 2026 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::time::Duration;

use gpui::{FontWeight, Hsla};

use crate::motion::{Easing, MotionRole, MotionSpec};
use crate::theme::{ColorScheme, TokenSet, TypeScale, TypeStyle};

use super::{
    ColorValue, Dp, FontFamilyToken, ShapeValue, Sp, TextStyleToken, TokenEntry, TokenValue,
};
pub const ANDROIDX_COMMIT: &str = "a095da93f8e98dea8748ceed79ea8427aade245f";
pub const ANDROIDX_DIRECTORY: &str =
    "compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens";
pub const SOURCE_FILE_COUNT: usize = 120;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShapeToken {
    CornerExtraExtraLarge,
    CornerExtraLarge,
    CornerExtraLargeIncreased,
    CornerExtraLargeTop,
    CornerExtraSmall,
    CornerExtraSmallTop,
    CornerFull,
    CornerLarge,
    CornerLargeEnd,
    CornerLargeIncreased,
    CornerLargeStart,
    CornerLargeTop,
    CornerMedium,
    CornerNone,
    CornerSmall,
}
impl ShapeToken {
    pub const ALL: &'static [Self] = &[
        Self::CornerExtraExtraLarge,
        Self::CornerExtraLarge,
        Self::CornerExtraLargeIncreased,
        Self::CornerExtraLargeTop,
        Self::CornerExtraSmall,
        Self::CornerExtraSmallTop,
        Self::CornerFull,
        Self::CornerLarge,
        Self::CornerLargeEnd,
        Self::CornerLargeIncreased,
        Self::CornerLargeStart,
        Self::CornerLargeTop,
        Self::CornerMedium,
        Self::CornerNone,
        Self::CornerSmall,
    ];
    pub const fn id(self) -> u16 {
        match self {
            Self::CornerExtraExtraLarge => 0,
            Self::CornerExtraLarge => 1,
            Self::CornerExtraLargeIncreased => 2,
            Self::CornerExtraLargeTop => 3,
            Self::CornerExtraSmall => 4,
            Self::CornerExtraSmallTop => 5,
            Self::CornerFull => 6,
            Self::CornerLarge => 7,
            Self::CornerLargeEnd => 8,
            Self::CornerLargeIncreased => 9,
            Self::CornerLargeStart => 10,
            Self::CornerLargeTop => 11,
            Self::CornerMedium => 12,
            Self::CornerNone => 13,
            Self::CornerSmall => 14,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypographyToken {
    BodyLarge,
    BodyMedium,
    BodySmall,
    DisplayLarge,
    DisplayMedium,
    DisplaySmall,
    HeadlineLarge,
    HeadlineMedium,
    HeadlineSmall,
    LabelLarge,
    LabelMedium,
    LabelSmall,
    TitleLarge,
    TitleMedium,
    TitleSmall,
    BodyLargeEmphasized,
    BodyMediumEmphasized,
    BodySmallEmphasized,
    DisplayLargeEmphasized,
    DisplayMediumEmphasized,
    DisplaySmallEmphasized,
    HeadlineLargeEmphasized,
    HeadlineMediumEmphasized,
    HeadlineSmallEmphasized,
    LabelLargeEmphasized,
    LabelMediumEmphasized,
    LabelSmallEmphasized,
    TitleLargeEmphasized,
    TitleMediumEmphasized,
    TitleSmallEmphasized,
}
impl TypographyToken {
    pub const ALL: &'static [Self] = &[
        Self::BodyLarge,
        Self::BodyMedium,
        Self::BodySmall,
        Self::DisplayLarge,
        Self::DisplayMedium,
        Self::DisplaySmall,
        Self::HeadlineLarge,
        Self::HeadlineMedium,
        Self::HeadlineSmall,
        Self::LabelLarge,
        Self::LabelMedium,
        Self::LabelSmall,
        Self::TitleLarge,
        Self::TitleMedium,
        Self::TitleSmall,
        Self::BodyLargeEmphasized,
        Self::BodyMediumEmphasized,
        Self::BodySmallEmphasized,
        Self::DisplayLargeEmphasized,
        Self::DisplayMediumEmphasized,
        Self::DisplaySmallEmphasized,
        Self::HeadlineLargeEmphasized,
        Self::HeadlineMediumEmphasized,
        Self::HeadlineSmallEmphasized,
        Self::LabelLargeEmphasized,
        Self::LabelMediumEmphasized,
        Self::LabelSmallEmphasized,
        Self::TitleLargeEmphasized,
        Self::TitleMediumEmphasized,
        Self::TitleSmallEmphasized,
    ];
    pub const fn id(self) -> u16 {
        match self {
            Self::BodyLarge => 0,
            Self::BodyMedium => 1,
            Self::BodySmall => 2,
            Self::DisplayLarge => 3,
            Self::DisplayMedium => 4,
            Self::DisplaySmall => 5,
            Self::HeadlineLarge => 6,
            Self::HeadlineMedium => 7,
            Self::HeadlineSmall => 8,
            Self::LabelLarge => 9,
            Self::LabelMedium => 10,
            Self::LabelSmall => 11,
            Self::TitleLarge => 12,
            Self::TitleMedium => 13,
            Self::TitleSmall => 14,
            Self::BodyLargeEmphasized => 15,
            Self::BodyMediumEmphasized => 16,
            Self::BodySmallEmphasized => 17,
            Self::DisplayLargeEmphasized => 18,
            Self::DisplayMediumEmphasized => 19,
            Self::DisplaySmallEmphasized => 20,
            Self::HeadlineLargeEmphasized => 21,
            Self::HeadlineMediumEmphasized => 22,
            Self::HeadlineSmallEmphasized => 23,
            Self::LabelLargeEmphasized => 24,
            Self::LabelMediumEmphasized => 25,
            Self::LabelSmallEmphasized => 26,
            Self::TitleLargeEmphasized => 27,
            Self::TitleMediumEmphasized => 28,
            Self::TitleSmallEmphasized => 29,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MotionSchemeToken {
    DefaultSpatial,
    FastSpatial,
    SlowSpatial,
    DefaultEffects,
    FastEffects,
    SlowEffects,
}
impl MotionSchemeToken {
    pub const ALL: &'static [Self] = &[
        Self::DefaultSpatial,
        Self::FastSpatial,
        Self::SlowSpatial,
        Self::DefaultEffects,
        Self::FastEffects,
        Self::SlowEffects,
    ];
    pub const fn id(self) -> u16 {
        match self {
            Self::DefaultSpatial => 0,
            Self::FastSpatial => 1,
            Self::SlowSpatial => 2,
            Self::DefaultEffects => 3,
            Self::FastEffects => 4,
            Self::SlowEffects => 5,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct AppBarLargeFlexibleTokens;
impl AppBarLargeFlexibleTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(120.0);
    pub const SUBTITLE_FONT: TypographyToken = TypographyKeyTokens::TITLE_MEDIUM;
    pub const TITLE_FONT: TypographyToken = TypographyKeyTokens::DISPLAY_SMALL;
    pub const LARGE_CONTAINER_HEIGHT: Dp = Dp(152.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct AppBarLargeTokens;
impl AppBarLargeTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(152.0);
    pub const TITLE_FONT: TypographyToken = TypographyKeyTokens::HEADLINE_MEDIUM;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct AppBarMediumFlexibleTokens;
impl AppBarMediumFlexibleTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(112.0);
    pub const SUBTITLE_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const TITLE_FONT: TypographyToken = TypographyKeyTokens::HEADLINE_MEDIUM;
    pub const LARGE_CONTAINER_HEIGHT: Dp = Dp(136.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct AppBarMediumTokens;
impl AppBarMediumTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(112.0);
    pub const TITLE_FONT: TypographyToken = TypographyKeyTokens::HEADLINE_SMALL;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct AppBarSmallTokens;
impl AppBarSmallTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(64.0);
    pub const SUBTITLE_FONT: TypographyToken = TypographyKeyTokens::LABEL_MEDIUM;
    pub const TITLE_FONT: TypographyToken = TypographyKeyTokens::TITLE_LARGE;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct AppBarTokens;
impl AppBarTokens {
    pub const AVATAR_SIZE: Dp = Dp(32.0);
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const ICON_BUTTON_SPACE: Dp = Dp(0.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const LEADING_SPACE: Dp = Dp(4.0);
    pub const ON_SCROLL_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const ON_SCROLL_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const SUBTITLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TITLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TRAILING_SPACE: Dp = Dp(4.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct AssistChipTokens;
impl AssistChipTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const DRAGGED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ELEVATED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const ELEVATED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ELEVATED_DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const ELEVATED_DISABLED_CONTAINER_OPACITY: f32 = 0.12;
    pub const ELEVATED_FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const ELEVATED_PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const FLAT_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FLAT_DISABLED_OUTLINE_OPACITY: f32 = 0.12;
    pub const FLAT_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FLAT_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FLAT_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DRAGGED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_SIZE: Dp = Dp(18.0);
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct BadgeTokens;
impl BadgeTokens {
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const LARGE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const LARGE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR;
    pub const LARGE_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_SMALL;
    pub const LARGE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const LARGE_SIZE: Dp = Dp(16.0);
    pub const SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SIZE: Dp = Dp(6.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct BaselineButtonTokens;
impl BaselineButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const FOCUSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const HOVERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const ICON_SIZE: Dp = Dp(20.0);
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LEADING_SPACE: Dp = Dp(24.0);
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const SELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const TRAILING_SPACE: Dp = Dp(24.0);
    pub const UNSELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const UNSELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct BottomAppBarTokens;
impl BottomAppBarTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const CONTAINER_HEIGHT: Dp = Dp(80.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ButtonGroupSmallTokens;
impl ButtonGroupSmallTokens {
    pub const BETWEEN_SPACE: Dp = Dp(12.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ButtonLargeTokens;
impl ButtonLargeTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(96.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const ICON_LABEL_SPACE: Dp = Dp(12.0);
    pub const ICON_SIZE: Dp = Dp(32.0);
    pub const LEADING_SPACE: Dp = Dp(48.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const TRAILING_SPACE: Dp = Dp(48.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ButtonMediumTokens;
impl ButtonMediumTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const LEADING_SPACE: Dp = Dp(24.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const TRAILING_SPACE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ButtonSmallTokens;
impl ButtonSmallTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const ICON_SIZE: Dp = Dp(20.0);
    pub const LEADING_SPACE: Dp = Dp(16.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const TRAILING_SPACE: Dp = Dp(16.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ButtonXLargeTokens;
impl ButtonXLargeTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(136.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const ICON_LABEL_SPACE: Dp = Dp(16.0);
    pub const ICON_SIZE: Dp = Dp(40.0);
    pub const LEADING_SPACE: Dp = Dp(64.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(3.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const TRAILING_SPACE: Dp = Dp(64.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ButtonXSmallTokens;
impl ButtonXSmallTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const ICON_SIZE: Dp = Dp(20.0);
    pub const LEADING_SPACE: Dp = Dp(16.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const TRAILING_SPACE: Dp = Dp(16.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct CheckboxTokens;
impl CheckboxTokens {
    pub const CONTAINER_SHAPE: ShapeValue = ShapeValue::rounded(2.0);
    pub const CONTAINER_SIZE: Dp = Dp(18.0);
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const ICON_SIZE: Dp = Dp(18.0);
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const SELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const SELECTED_DISABLED_CONTAINER_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const SELECTED_ERROR_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const SELECTED_ERROR_FOCUS_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const SELECTED_ERROR_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR;
    pub const SELECTED_ERROR_HOVER_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const SELECTED_ERROR_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR;
    pub const SELECTED_ERROR_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR;
    pub const SELECTED_ERROR_PRESSED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const SELECTED_ERROR_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR;
    pub const SELECTED_FOCUS_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_FOCUS_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_HOVER_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVER_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_PRESSED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const STATE_LAYER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const STATE_LAYER_SIZE: Dp = Dp(40.0);
    pub const UNSELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const UNSELECTED_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_DISABLED_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const UNSELECTED_ERROR_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const UNSELECTED_ERROR_HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const UNSELECTED_ERROR_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const UNSELECTED_ERROR_PRESSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const UNSELECTED_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_FOCUS_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const UNSELECTED_HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_HOVER_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const UNSELECTED_PRESSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_PRESSED_OUTLINE_WIDTH: Dp = Dp(2.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ChipsTokens;
impl ChipsTokens {
    pub const AVATAR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const AVATAR_SIZE: Dp = Dp(24.0);
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const FOCUSED_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HEIGHT: Dp = Dp(32.0);
    pub const LABEL_TEXT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const LEADING_ICON_SIZE: Dp = Dp(18.0);
    pub const PRESSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const SELECTED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const SELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.12;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const TRAILING_ICON_SIZE: Dp = Dp(18.0);
    pub const UNSELECTED_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_DISABLED_OUTLINE_OPACITY: f32 = 0.1;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const UNSELECTED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const UNSELECTED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct CircularProgressIndicatorTokens;
impl CircularProgressIndicatorTokens {
    pub const ACTIVE_THICKNESS: Dp = Dp(4.0);
    pub const ACTIVE_WAVE_AMPLITUDE: Dp = Dp(1.6);
    pub const ACTIVE_WAVE_WAVELENGTH: Dp = Dp(15.0);
    pub const SIZE: Dp = Dp(40.0);
    pub const TRACK_ACTIVE_SPACE: Dp = Dp(4.0);
    pub const TRACK_THICKNESS: Dp = Dp(4.0);
    pub const WAVE_SIZE: Dp = Dp(48.0);
}
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
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectedButtonGroupSmallTokens;
impl ConnectedButtonGroupSmallTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_SMALL;
    pub const PRESSED_INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_EXTRA_SMALL;
    pub const SELECTED_INNER_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct DateInputModalTokens;
impl DateInputModalTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_HEIGHT: Dp = Dp(512.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const CONTAINER_SURFACE_TINT_LAYER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_TINT;
    pub const CONTAINER_WIDTH: Dp = Dp(328.0);
    pub const HEADER_CONTAINER_HEIGHT: Dp = Dp(120.0);
    pub const HEADER_CONTAINER_WIDTH: Dp = Dp(328.0);
    pub const HEADER_HEADLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADER_HEADLINE_FONT: TypographyToken = TypographyKeyTokens::HEADLINE_LARGE;
    pub const HEADER_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADER_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct DatePickerModalTokens;
impl DatePickerModalTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGH;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_HEIGHT: Dp = Dp(568.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const CONTAINER_WIDTH: Dp = Dp(360.0);
    pub const DATE_CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const DATE_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const DATE_CONTAINER_WIDTH: Dp = Dp(40.0);
    pub const DATE_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const DATE_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DATE_SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const DATE_STATE_LAYER_HEIGHT: Dp = Dp(40.0);
    pub const DATE_STATE_LAYER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const DATE_STATE_LAYER_WIDTH: Dp = Dp(40.0);
    pub const DATE_TODAY_CONTAINER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DATE_TODAY_CONTAINER_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const DATE_TODAY_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DATE_UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HEADER_CONTAINER_HEIGHT: Dp = Dp(120.0);
    pub const HEADER_CONTAINER_WIDTH: Dp = Dp(360.0);
    pub const HEADER_HEADLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADER_HEADLINE_FONT: TypographyToken = TypographyKeyTokens::HEADLINE_LARGE;
    pub const HEADER_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADER_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_FULL;
    pub const RANGE_SELECTION_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const RANGE_SELECTION_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const SELECTION_DATE_IN_RANGE_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const RANGE_SELECTION_HEADER_CONTAINER_HEIGHT: Dp = Dp(128.0);
    pub const RANGE_SELECTION_HEADER_HEADLINE_FONT: TypographyToken =
        TypographyKeyTokens::TITLE_LARGE;
    pub const RANGE_SELECTION_MONTH_SUBHEAD_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const RANGE_SELECTION_MONTH_SUBHEAD_FONT: TypographyToken =
        TypographyKeyTokens::TITLE_SMALL;
    pub const WEEKDAYS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const WEEKDAYS_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const SELECTION_YEAR_CONTAINER_HEIGHT: Dp = Dp(36.0);
    pub const SELECTION_YEAR_CONTAINER_WIDTH: Dp = Dp(72.0);
    pub const SELECTION_YEAR_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const SELECTION_YEAR_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTION_YEAR_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTION_YEAR_STATE_LAYER_HEIGHT: Dp = Dp(36.0);
    pub const SELECTION_YEAR_STATE_LAYER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTION_YEAR_STATE_LAYER_WIDTH: Dp = Dp(72.0);
    pub const SELECTION_YEAR_UNSELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct DialogTokens;
impl DialogTokens {
    pub const ACTION_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTION_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTION_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTION_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const ACTION_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGH;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const HEADLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HEADLINE_FONT: TypographyToken = TypographyKeyTokens::HEADLINE_SMALL;
    pub const SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_MEDIUM;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const ICON_SIZE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct DividerTokens;
impl DividerTokens {
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const THICKNESS: Dp = Dp(1.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct DockedToolbarTokens;
impl DockedToolbarTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const CONTAINER_HEIGHT: Dp = Dp(64.0);
    pub const CONTAINER_LEADING_SPACE: Dp = Dp(16.0);
    pub const CONTAINER_MAX_SPACING: Dp = Dp(32.0);
    pub const CONTAINER_MIN_SPACING: Dp = Dp(4.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const CONTAINER_TRAILING_SPACE: Dp = Dp(16.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct DragHandleTokens;
impl DragHandleTokens {
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const CONTAINER_WIDTH: Dp = Dp(24.0);
    pub const DRAGGED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DRAGGED_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DRAGGED_HEIGHT: Dp = Dp(52.0);
    pub const DRAGGED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DRAGGED_WIDTH: Dp = Dp(12.0);
    pub const ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const HEIGHT: Dp = Dp(48.0);
    pub const PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const PRESSED_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const PRESSED_HEIGHT: Dp = Dp(52.0);
    pub const PRESSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const PRESSED_WIDTH: Dp = Dp(12.0);
    pub const SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const WIDTH: Dp = Dp(4.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ElevatedButtonTokens;
impl ElevatedButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const FOCUSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HOVERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const LABEL_TEXT_SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const UNSELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const UNSELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ElevatedCardTokens;
impl ElevatedCardTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ElevationTokens;
impl ElevationTokens {
    pub const LEVEL0: Dp = Dp(0.0);
    pub const LEVEL1: Dp = Dp(1.0);
    pub const LEVEL2: Dp = Dp(3.0);
    pub const LEVEL3: Dp = Dp(6.0);
    pub const LEVEL4: Dp = Dp(8.0);
    pub const LEVEL5: Dp = Dp(12.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ExpandedListTokens;
impl ExpandedListTokens {
    pub const COLLAPSED_ITEM_TRAILING_ICON_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE;
    pub const COLLAPSED_ITEM_TRAILING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const EXPANDED_ITEM_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const EXPANDED_ITEM_SEGMENTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const EXPANDED_ITEM_TRAILING_ICON_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const EXPANDED_ITEM_TRAILING_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TRAILING_ICON_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ExpressiveMotionTokens;
impl ExpressiveMotionTokens {
    pub const SPRING_DEFAULT_SPATIAL_DAMPING: f32 = 0.8;
    pub const SPRING_DEFAULT_SPATIAL_STIFFNESS: f32 = 380.0;
    pub const SPRING_DEFAULT_EFFECTS_DAMPING: f32 = 1.0;
    pub const SPRING_DEFAULT_EFFECTS_STIFFNESS: f32 = 1600.0;
    pub const SPRING_FAST_SPATIAL_DAMPING: f32 = 0.6;
    pub const SPRING_FAST_SPATIAL_STIFFNESS: f32 = 800.0;
    pub const SPRING_FAST_EFFECTS_DAMPING: f32 = 1.0;
    pub const SPRING_FAST_EFFECTS_STIFFNESS: f32 = 3800.0;
    pub const SPRING_SLOW_SPATIAL_DAMPING: f32 = 0.8;
    pub const SPRING_SLOW_SPATIAL_STIFFNESS: f32 = 200.0;
    pub const SPRING_SLOW_EFFECTS_DAMPING: f32 = 1.0;
    pub const SPRING_SLOW_EFFECTS_STIFFNESS: f32 = 800.0;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ExtendedFabLargeTokens;
impl ExtendedFabLargeTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(96.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const ICON_LABEL_SPACE: Dp = Dp(20.0);
    pub const ICON_SIZE: Dp = Dp(32.0);
    pub const LEADING_SPACE: Dp = Dp(28.0);
    pub const TRAILING_SPACE: Dp = Dp(28.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ExtendedFabMediumTokens;
impl ExtendedFabMediumTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(80.0);
    pub const ICON_LABEL_SPACE: Dp = Dp(16.0);
    pub const ICON_SIZE: Dp = Dp(28.0);
    pub const LEADING_SPACE: Dp = Dp(26.0);
    pub const TRAILING_SPACE: Dp = Dp(26.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ExtendedFabPrimaryTokens;
impl ExtendedFabPrimaryTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const LOWERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const LOWERED_FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const LOWERED_HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const LOWERED_PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ExtendedFabSmallTokens;
impl ExtendedFabSmallTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const LEADING_SPACE: Dp = Dp(16.0);
    pub const TRAILING_SPACE: Dp = Dp(16.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FabBaselineTokens;
impl FabBaselineTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const CONTAINER_WIDTH: Dp = Dp(56.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FabLargeTokens;
impl FabLargeTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(96.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const CONTAINER_WIDTH: Dp = Dp(96.0);
    pub const ICON_SIZE: Dp = Dp(32.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FabMediumTokens;
impl FabMediumTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(80.0);
    pub const CONTAINER_WIDTH: Dp = Dp(80.0);
    pub const ICON_SIZE: Dp = Dp(28.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FabMenuBaselineTokens;
impl FabMenuBaselineTokens {
    pub const CLOSE_BUTTON_BETWEEN_SPACE: Dp = Dp(8.0);
    pub const CLOSE_BUTTON_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CLOSE_BUTTON_CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CLOSE_BUTTON_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CLOSE_BUTTON_CONTAINER_WIDTH: Dp = Dp(56.0);
    pub const CLOSE_BUTTON_ICON_SIZE: Dp = Dp(20.0);
    pub const LIST_ITEM_BETWEEN_SPACE: Dp = Dp(4.0);
    pub const LIST_ITEM_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const LIST_ITEM_CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const LIST_ITEM_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const LIST_ITEM_ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const LIST_ITEM_ICON_SIZE: Dp = Dp(24.0);
    pub const LIST_ITEM_LEADING_SPACE: Dp = Dp(24.0);
    pub const LIST_ITEM_TRAILING_SPACE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FabPrimaryContainerTokens;
impl FabPrimaryContainerTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const FOCUSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const HOVERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FabSecondaryContainerTokens;
impl FabSecondaryContainerTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const FOCUSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const HOVERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FabSmallTokens;
impl FabSmallTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const CONTAINER_WIDTH: Dp = Dp(40.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FilledAutocompleteTokens;
impl FilledAutocompleteTokens {
    pub const MENU_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const MENU_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const MENU_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const TEXT_FIELD_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const TEXT_FIELD_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const TEXT_FIELD_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL_TOP;
    pub const TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_CONTAINER_OPACITY: f32 = 0.04;
    pub const FIELD_DISABLED_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_INPUT_TEXT_OPACITY: f32 = 0.38;
    pub const FIELD_DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const FIELD_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_ERROR_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_ERROR_FOCUS_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_ERROR_FOCUS_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_HOVER_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const FIELD_ERROR_HOVER_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const TEXT_FIELD_ERROR_HOVER_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_ERROR_HOVER_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const FIELD_ERROR_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_ERROR_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_FOCUS_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_FOCUS_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(2.0);
    pub const FIELD_FOCUS_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_FOCUS_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_FOCUS_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_HOVER_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const FIELD_HOVER_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_HOVER_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_INPUT_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const FIELD_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const TEXT_FIELD_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_LEADING_ICON_SIZE: Dp = Dp(20.0);
    pub const FIELD_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
    pub const TEXT_FIELD_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_TRAILING_ICON_SIZE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FilledButtonTokens;
impl FilledButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const FOCUSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const HOVERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const UNSELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const UNSELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FilledCardTokens;
impl FilledCardTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_VARIANT;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FilledIconButtonTokens;
impl FilledIconButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OPACITY: f32 = 0.38;
    pub const FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const UNSELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const UNSELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FilledTextFieldTokens;
impl FilledTextFieldTokens {
    pub const ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const CARET_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL_TOP;
    pub const DISABLED_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const DISABLED_ACTIVE_INDICATOR_OPACITY: f32 = 0.38;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.04;
    pub const DISABLED_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_INPUT_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_OPACITY: f32 = 0.38;
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_SUPPORTING_OPACITY: f32 = 0.38;
    pub const DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ERROR_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_FOCUS_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_FOCUS_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_HOVER_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_HOVER_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_HOVER_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_HOVER_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FOCUS_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(2.0);
    pub const FOCUS_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FOCUS_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUS_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const HOVER_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INPUT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const INPUT_PLACEHOLDER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INPUT_PREFIX_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INPUT_SUFFIX_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LEADING_ICON_SIZE: Dp = Dp(24.0);
    pub const SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SUPPORTING_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
    pub const TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TRAILING_ICON_SIZE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FilledTonalButtonTokens;
impl FilledTonalButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.12;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ICON_SIZE: Dp = Dp(18.0);
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FilledTonalIconButtonTokens;
impl FilledTonalIconButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OPACITY: f32 = 0.38;
    pub const FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const SELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const UNSELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const UNSELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FilterChipTokens;
impl FilterChipTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const ELEVATED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ELEVATED_DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const ELEVATED_DISABLED_CONTAINER_OPACITY: f32 = 0.12;
    pub const ELEVATED_FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const ELEVATED_PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const ELEVATED_UNSELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const FLAT_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_DISABLED_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FLAT_DISABLED_SELECTED_CONTAINER_OPACITY: f32 = 0.12;
    pub const FLAT_DISABLED_UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FLAT_DISABLED_UNSELECTED_OUTLINE_OPACITY: f32 = 0.12;
    pub const FLAT_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const FLAT_SELECTED_FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_SELECTED_HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const FLAT_SELECTED_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const FLAT_SELECTED_PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_UNSELECTED_FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_UNSELECTED_FOCUS_OUTLINE_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FLAT_UNSELECTED_HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FLAT_UNSELECTED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FLAT_UNSELECTED_PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const SELECTED_DRAGGED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_DRAGGED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ICON_SIZE: Dp = Dp(18.0);
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const SELECTED_DRAGGED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_DRAGGED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const SELECTED_DRAGGED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_DRAGGED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FloatingToolbarTokens;
impl FloatingToolbarTokens {
    pub const CONTAINER_BETWEEN_SPACE: Dp = Dp(4.0);
    pub const CONTAINER_EXTERNAL_PADDING: Dp = Dp(16.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(64.0);
    pub const CONTAINER_LEADING_SPACE: Dp = Dp(8.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_TRAILING_SPACE: Dp = Dp(8.0);
    pub const STANDARD_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const VIBRANT_BUTTON_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const VIBRANT_BUTTON_SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const VIBRANT_BUTTON_SELECTED_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const VIBRANT_BUTTON_UNSELECTED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const VIBRANT_BUTTON_UNSELECTED_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const VIBRANT_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct StandardIconButtonTokens;
impl StandardIconButtonTokens {
    pub const DISABLED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OPACITY: f32 = 0.38;
    pub const FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct InputChipTokens;
impl InputChipTokens {
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DISABLED_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_SELECTED_CONTAINER_OPACITY: f32 = 0.12;
    pub const DISABLED_UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_UNSELECTED_OUTLINE_OPACITY: f32 = 0.12;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const SELECTED_DRAGGED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_DRAGGED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const AVATAR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const AVATAR_SIZE: Dp = Dp(24.0);
    pub const DISABLED_AVATAR_OPACITY: f32 = 0.38;
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const LEADING_ICON_SIZE: Dp = Dp(18.0);
    pub const SELECTED_DRAGGED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_DRAGGED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const SELECTED_DRAGGED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const TRAILING_ICON_SIZE: Dp = Dp(18.0);
    pub const UNSELECTED_DRAGGED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct LargeIconButtonTokens;
impl LargeIconButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(96.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const ICON_SIZE: Dp = Dp(32.0);
    pub const NARROW_LEADING_SPACE: Dp = Dp(16.0);
    pub const NARROW_TRAILING_SPACE: Dp = Dp(16.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const UNIFORM_LEADING_SPACE: Dp = Dp(32.0);
    pub const UNIFORM_TRAILING_SPACE: Dp = Dp(32.0);
    pub const WIDE_LEADING_SPACE: Dp = Dp(48.0);
    pub const WIDE_TRAILING_SPACE: Dp = Dp(48.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct LinearProgressIndicatorTokens;
impl LinearProgressIndicatorTokens {
    pub const ACTIVE_THICKNESS: Dp = Dp(4.0);
    pub const ACTIVE_WAVE_AMPLITUDE: Dp = Dp(3.0);
    pub const ACTIVE_WAVE_WAVELENGTH: Dp = Dp(40.0);
    pub const HEIGHT: Dp = Dp(4.0);
    pub const INDETERMINATE_ACTIVE_WAVE_WAVELENGTH: Dp = Dp(20.0);
    pub const STOP_SIZE: Dp = Dp(4.0);
    pub const STOP_TRAILING_SPACE: Dp = Dp(0.0);
    pub const TRACK_ACTIVE_SPACE: Dp = Dp(4.0);
    pub const TRACK_THICKNESS: Dp = Dp(4.0);
    pub const WAVE_HEIGHT: Dp = Dp(10.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ListTokens;
impl ListTokens {
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const DIVIDER_BOTTOM_SPACE: Dp = Dp(0.0);
    pub const DIVIDER_LEADING_SPACE: Dp = Dp(16.0);
    pub const DIVIDER_TOP_SPACE: Dp = Dp(0.0);
    pub const DIVIDER_TRAILING_SPACE: Dp = Dp(16.0);
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const ITEM_BETWEEN_SPACE: Dp = Dp(12.0);
    pub const ITEM_BOTTOM_SPACE: Dp = Dp(10.0);
    pub const ITEM_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const ITEM_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const ITEM_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const ITEM_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const ITEM_DISABLED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const ITEM_DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_OVERLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_OVERLINE_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_STATE_LAYER_OPACITY: f32 = 0.1;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const ITEM_DRAGGED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_DRAGGED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DRAGGED_LEADING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_DRAGGED_TRAILING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_FOCUS_LEADING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_FOCUS_TRAILING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_FOCUSED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_HOVER_LEADING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_HOVER_TRAILING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_HOVERED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const ITEM_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const ITEM_LARGE_LEADING_VIDEO_HEIGHT: Dp = Dp(64.0);
    pub const ITEM_LARGE_LEADING_VIDEO_WIDTH: Dp = Dp(114.0);
    pub const ITEM_LEADING_AVATAR_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const ITEM_LEADING_AVATAR_LABEL_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const ITEM_LEADING_AVATAR_LABEL_FONT: TypographyToken = TypographyKeyTokens::TITLE_MEDIUM;
    pub const ITEM_LEADING_AVATAR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ITEM_LEADING_AVATAR_SIZE: Dp = Dp(40.0);
    pub const ITEM_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_LEADING_ICON_EXPRESSIVE_SIZE: Dp = Dp(20.0);
    pub const ITEM_LEADING_ICON_SIZE: Dp = Dp(24.0);
    pub const ITEM_LEADING_IMAGE_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const ITEM_LEADING_IMAGE_HEIGHT: Dp = Dp(56.0);
    pub const ITEM_LEADING_IMAGE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const ITEM_LEADING_IMAGE_WIDTH: Dp = Dp(56.0);
    pub const ITEM_LEADING_SPACE: Dp = Dp(16.0);
    pub const ITEM_LEADING_VIDEO_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const ITEM_LEADING_VIDEO_WIDTH: Dp = Dp(100.0);
    pub const ITEM_ONE_LINE_CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const ITEM_OVERLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_OVERLINE_FONT: TypographyToken = TypographyKeyTokens::LABEL_SMALL;
    pub const ITEM_PRESSED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_PRESSED_LEADING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_PRESSED_TRAILING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_SEGMENTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const ITEM_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_OVERLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_OVERLINE_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_STATE_LAYER_OPACITY: f32 = 0.1;
    pub const ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DRAGGED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_DRAGGED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_DRAGGED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DRAGGED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_FOCUSED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_HOVERED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_OVERLINE_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SMALL_LEADING_VIDEO_HEIGHT: Dp = Dp(56.0);
    pub const ITEM_SMALL_LEADING_VIDEO_WIDTH: Dp = Dp(100.0);
    pub const ITEM_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_MEDIUM;
    pub const ITEM_THREE_LINE_CONTAINER_HEIGHT: Dp = Dp(88.0);
    pub const ITEM_TOP_SPACE: Dp = Dp(10.0);
    pub const ITEM_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_TRAILING_ICON_EXPRESSIVE_SIZE: Dp = Dp(20.0);
    pub const ITEM_TRAILING_ICON_SIZE: Dp = Dp(24.0);
    pub const ITEM_TRAILING_SPACE: Dp = Dp(16.0);
    pub const ITEM_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_TRAILING_SUPPORTING_TEXT_FONT: TypographyToken =
        TypographyKeyTokens::LABEL_SMALL;
    pub const ITEM_TWO_LINE_CONTAINER_HEIGHT: Dp = Dp(72.0);
    pub const ITEM_UNSELECTED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const SEGMENTED_GAP: Dp = Dp(2.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct LoadingIndicatorTokens;
impl LoadingIndicatorTokens {
    pub const ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_SIZE: Dp = Dp(38.0);
    pub const CONTAINED_ACTIVE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const CONTAINED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const CONTAINER_HEIGHT: Dp = Dp(48.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_WIDTH: Dp = Dp(48.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct MediumIconButtonTokens;
impl MediumIconButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const DEFAULT_LEADING_SPACE: Dp = Dp(16.0);
    pub const DEFAULT_TRAILING_SPACE: Dp = Dp(16.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const NARROW_LEADING_SPACE: Dp = Dp(12.0);
    pub const NARROW_TRAILING_SPACE: Dp = Dp(12.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const WIDE_LEADING_SPACE: Dp = Dp(24.0);
    pub const WIDE_TRAILING_SPACE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct MenuTokens;
impl MenuTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const LIST_ITEM_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const LIST_ITEM_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const LIST_ITEM_SELECTED_LEADING_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const MENU_LIST_ITEM_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct MotionSchemeKeyTokens;
impl MotionSchemeKeyTokens {
    pub const DEFAULT_SPATIAL: MotionSchemeToken = MotionSchemeToken::DefaultSpatial;
    pub const FAST_SPATIAL: MotionSchemeToken = MotionSchemeToken::FastSpatial;
    pub const SLOW_SPATIAL: MotionSchemeToken = MotionSchemeToken::SlowSpatial;
    pub const DEFAULT_EFFECTS: MotionSchemeToken = MotionSchemeToken::DefaultEffects;
    pub const FAST_EFFECTS: MotionSchemeToken = MotionSchemeToken::FastEffects;
    pub const SLOW_EFFECTS: MotionSchemeToken = MotionSchemeToken::SlowEffects;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct MotionTokens;
impl MotionTokens {
    pub const DURATION_EXTRA_LONG1: f64 = 700.0;
    pub const DURATION_EXTRA_LONG2: f64 = 800.0;
    pub const DURATION_EXTRA_LONG3: f64 = 900.0;
    pub const DURATION_EXTRA_LONG4: f64 = 1000.0;
    pub const DURATION_LONG1: f64 = 450.0;
    pub const DURATION_LONG2: f64 = 500.0;
    pub const DURATION_LONG3: f64 = 550.0;
    pub const DURATION_LONG4: f64 = 600.0;
    pub const DURATION_MEDIUM1: f64 = 250.0;
    pub const DURATION_MEDIUM2: f64 = 300.0;
    pub const DURATION_MEDIUM3: f64 = 350.0;
    pub const DURATION_MEDIUM4: f64 = 400.0;
    pub const DURATION_SHORT1: f64 = 50.0;
    pub const DURATION_SHORT2: f64 = 100.0;
    pub const DURATION_SHORT3: f64 = 150.0;
    pub const DURATION_SHORT4: f64 = 200.0;
    pub const EASING_EMPHASIZED_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.2,
        y1: 0.0,
        x2: 0.0,
        y2: 1.0,
    };
    pub const EASING_EMPHASIZED_ACCELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.3,
        y1: 0.0,
        x2: 0.8,
        y2: 0.15,
    };
    pub const EASING_EMPHASIZED_DECELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.05,
        y1: 0.7,
        x2: 0.1,
        y2: 1.0,
    };
    pub const EASING_LEGACY_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.4,
        y1: 0.0,
        x2: 0.2,
        y2: 1.0,
    };
    pub const EASING_LEGACY_ACCELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.4,
        y1: 0.0,
        x2: 1.0,
        y2: 1.0,
    };
    pub const EASING_LEGACY_DECELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.0,
        y1: 0.0,
        x2: 0.2,
        y2: 1.0,
    };
    pub const EASING_LINEAR_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.0,
        y1: 0.0,
        x2: 1.0,
        y2: 1.0,
    };
    pub const EASING_STANDARD_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.2,
        y1: 0.0,
        x2: 0.0,
        y2: 1.0,
    };
    pub const EASING_STANDARD_ACCELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.3,
        y1: 0.0,
        x2: 1.0,
        y2: 1.0,
    };
    pub const EASING_STANDARD_DECELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.0,
        y1: 0.0,
        x2: 0.0,
        y2: 1.0,
    };
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationBarHorizontalItemTokens;
impl NavigationBarHorizontalItemTokens {
    pub const ACTIVE_INDICATOR_HEIGHT: Dp = Dp(40.0);
    pub const ACTIVE_INDICATOR_LEADING_SPACE: Dp = Dp(16.0);
    pub const ACTIVE_INDICATOR_TRAILING_SPACE: Dp = Dp(16.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationBarTokens;
impl NavigationBarTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const CONTAINER_HEIGHT: Dp = Dp(64.0);
    pub const ITEM_ACTIVE_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_INDICATOR_ICON_LABEL_SPACE: Dp = Dp(4.0);
    pub const ITEM_ACTIVE_INDICATOR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ITEM_ACTIVE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const ITEM_BETWEEN_SPACE: Dp = Dp(0.0);
    pub const ITEM_INACTIVE_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_INACTIVE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const NAV_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const TALL_CONTAINER_HEIGHT: Dp = Dp(80.0);
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_MEDIUM;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationBarVerticalItemTokens;
impl NavigationBarVerticalItemTokens {
    pub const ACTIVE_INDICATOR_HEIGHT: Dp = Dp(32.0);
    pub const ACTIVE_INDICATOR_WIDTH: Dp = Dp(56.0);
    pub const CONTAINER_BETWEEN_SPACE: Dp = Dp(6.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationDrawerTokens;
impl NavigationDrawerTokens {
    pub const ACTIVE_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ACTIVE_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ACTIVE_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ACTIVE_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ACTIVE_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const ACTIVE_INDICATOR_HEIGHT: Dp = Dp(56.0);
    pub const ACTIVE_INDICATOR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ACTIVE_INDICATOR_WIDTH: Dp = Dp(336.0);
    pub const ACTIVE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ACTIVE_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ACTIVE_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const BOTTOM_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE_TOP;
    pub const CONTAINER_HEIGHT_PERCENT: f32 = 100.0;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE_END;
    pub const CONTAINER_WIDTH: Dp = Dp(360.0);
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HEADLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADLINE_FONT: TypographyToken = TypographyKeyTokens::TITLE_SMALL;
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const INACTIVE_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INACTIVE_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INACTIVE_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INACTIVE_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INACTIVE_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INACTIVE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INACTIVE_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INACTIVE_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const LARGE_BADGE_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LARGE_BADGE_LABEL_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const MODAL_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const MODAL_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const STANDARD_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const STANDARD_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationRailBaselineItemTokens;
impl NavigationRailBaselineItemTokens {
    pub const ACTIVE_INDICATOR_ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const ACTIVE_INDICATOR_LEADING_SPACE: Dp = Dp(16.0);
    pub const ACTIVE_INDICATOR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ACTIVE_INDICATOR_TRAILING_SPACE: Dp = Dp(16.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(64.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const CONTAINER_VERTICAL_SPACE: Dp = Dp(6.0);
    pub const HEADER_SPACE_MINIMUM: Dp = Dp(40.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationRailCollapsedTokens;
impl NavigationRailCollapsedTokens {
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const CONTAINER_WIDTH: Dp = Dp(96.0);
    pub const ITEM_VERTICAL_SPACE: Dp = Dp(4.0);
    pub const TOP_SPACE: Dp = Dp(44.0);
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const NARROW_CONTAINER_WIDTH: Dp = Dp(80.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationRailColorTokens;
impl NavigationRailColorTokens {
    pub const ITEM_ACTIVE_FOCUSED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_HOVERED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_ICON: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_INDICATOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_LABEL_TEXT: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const ITEM_ACTIVE_PRESSED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_INACTIVE_FOCUSED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_INACTIVE_HOVERED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_INACTIVE_ICON: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_INACTIVE_LABEL_TEXT: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_INACTIVE_PRESSED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationRailExpandedTokens;
impl NavigationRailExpandedTokens {
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const CONTAINER_WIDTH_MAXIMUM: Dp = Dp(360.0);
    pub const CONTAINER_WIDTH_MINIMUM: Dp = Dp(220.0);
    pub const MODAL_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const MODAL_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const TOP_SPACE: Dp = Dp(44.0);
    pub const MODAL_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationRailHorizontalItemTokens;
impl NavigationRailHorizontalItemTokens {
    pub const ACTIVE_INDICATOR_HEIGHT: Dp = Dp(56.0);
    pub const FULL_WIDTH_LEADING_SPACE: Dp = Dp(16.0);
    pub const FULL_WIDTH_TRAILING_SPACE: Dp = Dp(16.0);
    pub const ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const LEADING_SPACE: Dp = Dp(16.0);
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationRailVerticalItemTokens;
impl NavigationRailVerticalItemTokens {
    pub const ACTIVE_INDICATOR_HEIGHT: Dp = Dp(32.0);
    pub const ACTIVE_INDICATOR_WIDTH: Dp = Dp(56.0);
    pub const ICON_LABEL_SPACE: Dp = Dp(4.0);
    pub const LEADING_SPACE: Dp = Dp(16.0);
    pub const TRAILING_SPACE: Dp = Dp(16.0);
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_MEDIUM;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct OutlinedAutocompleteTokens;
impl OutlinedAutocompleteTokens {
    pub const MENU_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const MENU_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const MENU_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const TEXT_FIELD_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const TEXT_FIELD_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const FIELD_DISABLED_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_INPUT_TEXT_OPACITY: f32 = 0.38;
    pub const FIELD_DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_OUTLINE_OPACITY: f32 = 0.12;
    pub const TEXT_FIELD_DISABLED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FIELD_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_ERROR_FOCUS_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_ERROR_FOCUS_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_ERROR_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_ERROR_FOCUS_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_ERROR_HOVER_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const TEXT_FIELD_ERROR_HOVER_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_ERROR_HOVER_OUTLINE_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const FIELD_ERROR_HOVER_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const FIELD_ERROR_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_ERROR_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_ERROR_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_FOCUS_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_FOCUS_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_FOCUS_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const FIELD_FOCUS_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_HOVER_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_HOVER_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FIELD_HOVER_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_INPUT_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const FIELD_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const TEXT_FIELD_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_LEADING_ICON_SIZE: Dp = Dp(24.0);
    pub const TEXT_FIELD_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const TEXT_FIELD_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FIELD_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
    pub const TEXT_FIELD_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_TRAILING_ICON_SIZE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct OutlinedButtonTokens;
impl OutlinedButtonTokens {
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVERED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PRESSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_SURFACE;
    pub const SELECTED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const SELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const UNSELECTED_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct OutlinedCardTokens;
impl OutlinedCardTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const DISABLED_OUTLINE_OPACITY: f32 = 0.12;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const DRAGGED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const PRESSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct OutlinedIconButtonTokens;
impl OutlinedIconButtonTokens {
    pub const DISABLED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OPACITY: f32 = 0.38;
    pub const DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_SURFACE;
    pub const SELECTED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const SELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const SELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const UNSELECTED_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct OutlinedSegmentedButtonTokens;
impl OutlinedSegmentedButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OUTLINE_OPACITY: f32 = 0.12;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const UNSELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ICON_SIZE: Dp = Dp(18.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct OutlinedTextFieldTokens;
impl OutlinedTextFieldTokens {
    pub const CARET_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const DISABLED_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_INPUT_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_OPACITY: f32 = 0.38;
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OUTLINE_OPACITY: f32 = 0.12;
    pub const DISABLED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const DISABLED_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_SUPPORTING_OPACITY: f32 = 0.38;
    pub const DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ERROR_FOCUS_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_FOCUS_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_HOVER_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_HOVER_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_HOVER_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FOCUS_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FOCUS_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const FOCUS_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const HOVER_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INPUT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const INPUT_PLACEHOLDER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INPUT_PREFIX_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INPUT_SUFFIX_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LEADING_ICON_SIZE: Dp = Dp(24.0);
    pub const OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SUPPORTING_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
    pub const TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TRAILING_ICON_SIZE: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct PaletteTokens;
impl PaletteTokens {
    pub const BLACK: ColorValue = ColorValue(0x000000);
    pub const ERROR0: ColorValue = ColorValue(0x000000);
    pub const ERROR10: ColorValue = ColorValue(0x410e0b);
    pub const ERROR100: ColorValue = ColorValue(0xffffff);
    pub const ERROR20: ColorValue = ColorValue(0x601410);
    pub const ERROR30: ColorValue = ColorValue(0x8c1d18);
    pub const ERROR40: ColorValue = ColorValue(0xb3261e);
    pub const ERROR50: ColorValue = ColorValue(0xdc362e);
    pub const ERROR60: ColorValue = ColorValue(0xe46962);
    pub const ERROR70: ColorValue = ColorValue(0xec928e);
    pub const ERROR80: ColorValue = ColorValue(0xf2b8b5);
    pub const ERROR90: ColorValue = ColorValue(0xf9dedc);
    pub const ERROR95: ColorValue = ColorValue(0xfceeee);
    pub const ERROR99: ColorValue = ColorValue(0xfffbf9);
    pub const NEUTRAL0: ColorValue = ColorValue(0x000000);
    pub const NEUTRAL10: ColorValue = ColorValue(0x1d1b20);
    pub const NEUTRAL100: ColorValue = ColorValue(0xffffff);
    pub const NEUTRAL12: ColorValue = ColorValue(0x211f26);
    pub const NEUTRAL17: ColorValue = ColorValue(0x2b2930);
    pub const NEUTRAL20: ColorValue = ColorValue(0x322f35);
    pub const NEUTRAL22: ColorValue = ColorValue(0x36343b);
    pub const NEUTRAL24: ColorValue = ColorValue(0x3b383e);
    pub const NEUTRAL30: ColorValue = ColorValue(0x48464c);
    pub const NEUTRAL4: ColorValue = ColorValue(0x0f0d13);
    pub const NEUTRAL40: ColorValue = ColorValue(0x605d64);
    pub const NEUTRAL50: ColorValue = ColorValue(0x79767d);
    pub const NEUTRAL6: ColorValue = ColorValue(0x141218);
    pub const NEUTRAL60: ColorValue = ColorValue(0x938f96);
    pub const NEUTRAL70: ColorValue = ColorValue(0xaea9b1);
    pub const NEUTRAL80: ColorValue = ColorValue(0xcac5cd);
    pub const NEUTRAL87: ColorValue = ColorValue(0xded8e1);
    pub const NEUTRAL90: ColorValue = ColorValue(0xe6e0e9);
    pub const NEUTRAL92: ColorValue = ColorValue(0xece6f0);
    pub const NEUTRAL94: ColorValue = ColorValue(0xf3edf7);
    pub const NEUTRAL95: ColorValue = ColorValue(0xf5eff7);
    pub const NEUTRAL96: ColorValue = ColorValue(0xf7f2fa);
    pub const NEUTRAL98: ColorValue = ColorValue(0xfef7ff);
    pub const NEUTRAL99: ColorValue = ColorValue(0xfffbff);
    pub const NEUTRAL_VARIANT0: ColorValue = ColorValue(0x000000);
    pub const NEUTRAL_VARIANT10: ColorValue = ColorValue(0x1d1a22);
    pub const NEUTRAL_VARIANT100: ColorValue = ColorValue(0xffffff);
    pub const NEUTRAL_VARIANT20: ColorValue = ColorValue(0x322f37);
    pub const NEUTRAL_VARIANT30: ColorValue = ColorValue(0x49454f);
    pub const NEUTRAL_VARIANT40: ColorValue = ColorValue(0x605d66);
    pub const NEUTRAL_VARIANT50: ColorValue = ColorValue(0x79747e);
    pub const NEUTRAL_VARIANT60: ColorValue = ColorValue(0x938f99);
    pub const NEUTRAL_VARIANT70: ColorValue = ColorValue(0xaea9b4);
    pub const NEUTRAL_VARIANT80: ColorValue = ColorValue(0xcac4d0);
    pub const NEUTRAL_VARIANT90: ColorValue = ColorValue(0xe7e0ec);
    pub const NEUTRAL_VARIANT95: ColorValue = ColorValue(0xf5eefa);
    pub const NEUTRAL_VARIANT99: ColorValue = ColorValue(0xfffbfe);
    pub const PRIMARY0: ColorValue = ColorValue(0x000000);
    pub const PRIMARY10: ColorValue = ColorValue(0x21005d);
    pub const PRIMARY100: ColorValue = ColorValue(0xffffff);
    pub const PRIMARY20: ColorValue = ColorValue(0x381e72);
    pub const PRIMARY30: ColorValue = ColorValue(0x4f378b);
    pub const PRIMARY40: ColorValue = ColorValue(0x6750a4);
    pub const PRIMARY50: ColorValue = ColorValue(0x7f67be);
    pub const PRIMARY60: ColorValue = ColorValue(0x9a82db);
    pub const PRIMARY70: ColorValue = ColorValue(0xb69df8);
    pub const PRIMARY80: ColorValue = ColorValue(0xd0bcff);
    pub const PRIMARY90: ColorValue = ColorValue(0xeaddff);
    pub const PRIMARY95: ColorValue = ColorValue(0xf6edff);
    pub const PRIMARY99: ColorValue = ColorValue(0xfffbfe);
    pub const SECONDARY0: ColorValue = ColorValue(0x000000);
    pub const SECONDARY10: ColorValue = ColorValue(0x1d192b);
    pub const SECONDARY100: ColorValue = ColorValue(0xffffff);
    pub const SECONDARY20: ColorValue = ColorValue(0x332d41);
    pub const SECONDARY30: ColorValue = ColorValue(0x4a4458);
    pub const SECONDARY40: ColorValue = ColorValue(0x625b71);
    pub const SECONDARY50: ColorValue = ColorValue(0x7a7289);
    pub const SECONDARY60: ColorValue = ColorValue(0x958da5);
    pub const SECONDARY70: ColorValue = ColorValue(0xb0a7c0);
    pub const SECONDARY80: ColorValue = ColorValue(0xccc2dc);
    pub const SECONDARY90: ColorValue = ColorValue(0xe8def8);
    pub const SECONDARY95: ColorValue = ColorValue(0xf6edff);
    pub const SECONDARY99: ColorValue = ColorValue(0xfffbfe);
    pub const TERTIARY0: ColorValue = ColorValue(0x000000);
    pub const TERTIARY10: ColorValue = ColorValue(0x31111d);
    pub const TERTIARY100: ColorValue = ColorValue(0xffffff);
    pub const TERTIARY20: ColorValue = ColorValue(0x492532);
    pub const TERTIARY30: ColorValue = ColorValue(0x633b48);
    pub const TERTIARY40: ColorValue = ColorValue(0x7d5260);
    pub const TERTIARY50: ColorValue = ColorValue(0x986977);
    pub const TERTIARY60: ColorValue = ColorValue(0xb58392);
    pub const TERTIARY70: ColorValue = ColorValue(0xd29dac);
    pub const TERTIARY80: ColorValue = ColorValue(0xefb8c8);
    pub const TERTIARY90: ColorValue = ColorValue(0xffd8e4);
    pub const TERTIARY95: ColorValue = ColorValue(0xffecf1);
    pub const TERTIARY99: ColorValue = ColorValue(0xfffbfa);
    pub const WHITE: ColorValue = ColorValue(0xffffff);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct PlainTooltipTokens;
impl PlainTooltipTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_SURFACE;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct PrimaryNavigationTabTokens;
impl PrimaryNavigationTabTokens {
    pub const ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_INDICATOR_HEIGHT: Dp = Dp(3.0);
    pub const ACTIVE_INDICATOR_SHAPE: ShapeValue = ShapeValue::rounded(3.0);
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_HEIGHT: Dp = Dp(48.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const ACTIVE_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_AND_LABEL_TEXT_CONTAINER_HEIGHT: Dp = Dp(64.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const INACTIVE_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INACTIVE_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INACTIVE_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INACTIVE_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ACTIVE_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const INACTIVE_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INACTIVE_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INACTIVE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INACTIVE_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::TITLE_SMALL;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ProgressIndicatorTokens;
impl ProgressIndicatorTokens {
    pub const ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const STOP_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const STOP_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const TRACK_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct RadioButtonTokens;
impl RadioButtonTokens {
    pub const DISABLED_SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_SELECTED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_UNSELECTED_ICON_OPACITY: f32 = 0.38;
    pub const ICON_SIZE: Dp = Dp(20.0);
    pub const SELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const STATE_LAYER_SIZE: Dp = Dp(40.0);
    pub const UNSELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ReorderListTokens;
impl ReorderListTokens {
    pub const ITEM_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ITEM_DROP_ZONE_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const ITEM_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_OVERLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct RevealListTokens;
impl RevealListTokens {
    pub const ITEM_ACTION_BUTTON_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const ITEM_ACTION_ICON_BUTTON_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ITEM_BUTTON_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const ITEM_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_ICON_BUTTON_ACTION_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_ICON_BUTTON_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const ITEM_ICON_BUTTON_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ITEM_SEGMENTED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct RichTooltipTokens;
impl RichTooltipTokens {
    pub const ACTION_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTION_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTION_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTION_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const ACTION_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const SUBHEAD_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SUBHEAD_FONT: TypographyToken = TypographyKeyTokens::TITLE_SMALL;
    pub const SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_MEDIUM;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ScrimTokens;
impl ScrimTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SCRIM;
    pub const CONTAINER_OPACITY: f32 = 0.32;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SearchBarTokens;
impl SearchBarTokens {
    pub const AVATAR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const AVATAR_SIZE: Dp = Dp(30.0);
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGH;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HOVER_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INPUT_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const PRESSED_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SearchViewTokens;
impl SearchViewTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGH;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const DIVIDER_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const DOCKED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const DOCKED_HEADER_CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const FULL_SCREEN_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const FULL_SCREEN_HEADER_CONTAINER_HEIGHT: Dp = Dp(72.0);
    pub const HEADER_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HEADER_INPUT_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const HEADER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HEADER_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADER_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const HEADER_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SecondaryNavigationTabTokens;
impl SecondaryNavigationTabTokens {
    pub const ACTIVE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_HEIGHT: Dp = Dp(48.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const DIVIDER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_VARIANT;
    pub const DIVIDER_HEIGHT: Dp = Dp(1.0);
    pub const FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INACTIVE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::TITLE_SMALL;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ACTIVE_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const INACTIVE_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SegmentedMenuTokens;
impl SegmentedMenuTokens {
    pub const ACTIVE_CONTAINER_SHAPE: Dp = Dp(24.0);
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const GROUP_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const GROUP_PADDING: Dp = Dp(4.0);
    pub const GROUP_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const HORIZONTAL_CONTAINER_BOTTOM_SPACE: Dp = Dp(8.0);
    pub const HORIZONTAL_CONTAINER_TOP_SPACE: Dp = Dp(8.0);
    pub const HORIZONTAL_ICON_ONLY_ITEM_BOTTOM_SPACE: Dp = Dp(16.0);
    pub const HORIZONTAL_ICON_ONLY_ITEM_LEADING_SPACE: Dp = Dp(16.0);
    pub const HORIZONTAL_ICON_ONLY_ITEM_SELECTED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const HORIZONTAL_ICON_ONLY_ITEM_TOP_SPACE: Dp = Dp(16.0);
    pub const HORIZONTAL_ICON_ONLY_ITEM_TRAILING_SPACE: Dp = Dp(16.0);
    pub const HORIZONTAL_ICON_ONLY_SEGMENTED_GAP: Dp = Dp(4.0);
    pub const HORIZONTAL_ITEM_BETWEEN_SPACE: Dp = Dp(12.0);
    pub const HORIZONTAL_ITEM_BOTTOM_SPACE: Dp = Dp(6.0);
    pub const HORIZONTAL_ITEM_FOCUSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const HORIZONTAL_ITEM_HOVERED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const HORIZONTAL_ITEM_LEADING_SPACE: Dp = Dp(12.0);
    pub const HORIZONTAL_ITEM_PRESSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const HORIZONTAL_ITEM_SELECTED_FOCUSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const HORIZONTAL_ITEM_SELECTED_HOVERED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const HORIZONTAL_ITEM_SELECTED_PRESSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const HORIZONTAL_ITEM_TOP_SPACE: Dp = Dp(6.0);
    pub const HORIZONTAL_ITEM_TRAILING_SPACE: Dp = Dp(12.0);
    pub const HORIZONTAL_SEGMENTED_GAP: Dp = Dp(2.0);
    pub const INACTIVE_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const ITEM: Dp = Dp(44.0);
    pub const ITEM_BETWEEN_SPACE: Dp = Dp(12.0);
    pub const ITEM_BOTTOM_SPACE: Dp = Dp(8.0);
    pub const ITEM_FIRST_CHILD_INNER_CORNER_CORNER_SIZE: ShapeToken =
        ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const ITEM_FIRST_CHILD_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const ITEM_FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const ITEM_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const ITEM_LAST_CHILD_INNER_CORNER_CORNER_SIZE: ShapeToken =
        ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const ITEM_LAST_CHILD_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const ITEM_LEADING_ICON_SIZE: Dp = Dp(20.0);
    pub const ITEM_LEADING_SPACE: Dp = Dp(16.0);
    pub const ITEM_SELECTED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const ITEM_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const ITEM_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_MEDIUM;
    pub const ITEM_TOP_SPACE: Dp = Dp(8.0);
    pub const ITEM_TRAILING_ICON_SIZE: Dp = Dp(20.0);
    pub const ITEM_TRAILING_SPACE: Dp = Dp(16.0);
    pub const ITEM_TRAILING_SUPPORTING_TEXT_FONT: TypographyToken =
        TypographyKeyTokens::LABEL_SMALL;
    pub const SEGMENTED_GAP: Dp = Dp(2.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ShapeKeyTokens;
impl ShapeKeyTokens {
    pub const CORNER_EXTRA_EXTRA_LARGE: ShapeToken = ShapeToken::CornerExtraExtraLarge;
    pub const CORNER_EXTRA_LARGE: ShapeToken = ShapeToken::CornerExtraLarge;
    pub const CORNER_EXTRA_LARGE_INCREASED: ShapeToken = ShapeToken::CornerExtraLargeIncreased;
    pub const CORNER_EXTRA_LARGE_TOP: ShapeToken = ShapeToken::CornerExtraLargeTop;
    pub const CORNER_EXTRA_SMALL: ShapeToken = ShapeToken::CornerExtraSmall;
    pub const CORNER_EXTRA_SMALL_TOP: ShapeToken = ShapeToken::CornerExtraSmallTop;
    pub const CORNER_FULL: ShapeToken = ShapeToken::CornerFull;
    pub const CORNER_LARGE: ShapeToken = ShapeToken::CornerLarge;
    pub const CORNER_LARGE_END: ShapeToken = ShapeToken::CornerLargeEnd;
    pub const CORNER_LARGE_INCREASED: ShapeToken = ShapeToken::CornerLargeIncreased;
    pub const CORNER_LARGE_START: ShapeToken = ShapeToken::CornerLargeStart;
    pub const CORNER_LARGE_TOP: ShapeToken = ShapeToken::CornerLargeTop;
    pub const CORNER_MEDIUM: ShapeToken = ShapeToken::CornerMedium;
    pub const CORNER_NONE: ShapeToken = ShapeToken::CornerNone;
    pub const CORNER_SMALL: ShapeToken = ShapeToken::CornerSmall;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ShapeTokens;
impl ShapeTokens {
    pub const CORNER_EXTRA_EXTRA_LARGE: ShapeValue = ShapeValue::rounded(48.0);
    pub const CORNER_EXTRA_LARGE: ShapeValue = ShapeValue::rounded(28.0);
    pub const CORNER_EXTRA_LARGE_INCREASED: ShapeValue = ShapeValue::rounded(32.0);
    pub const CORNER_EXTRA_LARGE_TOP: ShapeValue = ShapeValue::Rounded {
        top_start: Dp(28.0),
        top_end: Dp(28.0),
        bottom_end: Dp(0.0),
        bottom_start: Dp(0.0),
    };
    pub const CORNER_EXTRA_SMALL: ShapeValue = ShapeValue::rounded(4.0);
    pub const CORNER_EXTRA_SMALL_TOP: ShapeValue = ShapeValue::Rounded {
        top_start: Dp(4.0),
        top_end: Dp(4.0),
        bottom_end: Dp(0.0),
        bottom_start: Dp(0.0),
    };
    pub const CORNER_FULL: ShapeValue = ShapeValue::Full;
    pub const CORNER_LARGE: ShapeValue = ShapeValue::rounded(16.0);
    pub const CORNER_LARGE_END: ShapeValue = ShapeValue::Rounded {
        top_start: Dp(0.0),
        top_end: Dp(16.0),
        bottom_end: Dp(16.0),
        bottom_start: Dp(0.0),
    };
    pub const CORNER_LARGE_INCREASED: ShapeValue = ShapeValue::rounded(20.0);
    pub const CORNER_LARGE_START: ShapeValue = ShapeValue::Rounded {
        top_start: Dp(16.0),
        top_end: Dp(0.0),
        bottom_end: Dp(0.0),
        bottom_start: Dp(16.0),
    };
    pub const CORNER_LARGE_TOP: ShapeValue = ShapeValue::Rounded {
        top_start: Dp(16.0),
        top_end: Dp(16.0),
        bottom_end: Dp(0.0),
        bottom_start: Dp(0.0),
    };
    pub const CORNER_MEDIUM: ShapeValue = ShapeValue::rounded(12.0);
    pub const CORNER_NONE: ShapeValue = ShapeValue::rounded(0.0);
    pub const CORNER_SMALL: ShapeValue = ShapeValue::rounded(8.0);
    pub const CORNER_VALUE_EXTRA_EXTRA_LARGE: Dp = Dp(48.0);
    pub const CORNER_VALUE_EXTRA_LARGE: Dp = Dp(28.0);
    pub const CORNER_VALUE_EXTRA_LARGE_INCREASED: Dp = Dp(32.0);
    pub const CORNER_VALUE_EXTRA_SMALL: Dp = Dp(4.0);
    pub const CORNER_VALUE_LARGE: Dp = Dp(16.0);
    pub const CORNER_VALUE_LARGE_INCREASED: Dp = Dp(20.0);
    pub const CORNER_VALUE_MEDIUM: Dp = Dp(12.0);
    pub const CORNER_VALUE_NONE: Dp = Dp(0.0);
    pub const CORNER_VALUE_SMALL: Dp = Dp(8.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SheetBottomTokens;
impl SheetBottomTokens {
    pub const DOCKED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const DOCKED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE_TOP;
    pub const DOCKED_DRAG_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DOCKED_DRAG_HANDLE_HEIGHT: Dp = Dp(4.0);
    pub const DOCKED_DRAG_HANDLE_WIDTH: Dp = Dp(32.0);
    pub const DOCKED_MINIMIZED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const DOCKED_MODAL_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const DOCKED_STANDARD_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SliderTokens;
impl SliderTokens {
    pub const ACTIVE_CONTAINER_OPACITY: f32 = 1.0;
    pub const ACTIVE_HANDLE_HEIGHT: Dp = Dp(44.0);
    pub const ACTIVE_HANDLE_LEADING_SPACE: Dp = Dp(6.0);
    pub const ACTIVE_HANDLE_PADDING: Dp = Dp(6.0);
    pub const ACTIVE_HANDLE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ACTIVE_HANDLE_TRAILING_SPACE: Dp = Dp(6.0);
    pub const ACTIVE_HANDLE_WIDTH: Dp = Dp(4.0);
    pub const ACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_TRACK_HEIGHT: Dp = Dp(16.0);
    pub const ACTIVE_TRACK_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ACTIVE_TRACK_SHAPE_LEADING: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const DISABLED_ACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_ACTIVE_TRACK_OPACITY: f32 = 0.38;
    pub const DISABLED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_HANDLE_OPACITY: f32 = 0.38;
    pub const DISABLED_HANDLE_WIDTH: Dp = Dp(4.0);
    pub const DISABLED_INACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_INACTIVE_TRACK_OPACITY: f32 = 0.12;
    pub const DISABLED_STOP_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FOCUS_ACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_HANDLE_WIDTH: Dp = Dp(2.0);
    pub const FOCUS_INACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const FOCUS_STOP_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HANDLE_HEIGHT: Dp = Dp(44.0);
    pub const HANDLE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const HANDLE_WIDTH: Dp = Dp(4.0);
    pub const HOVER_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HOVER_HANDLE_WIDTH: Dp = Dp(4.0);
    pub const HOVER_STOP_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const INACTIVE_CONTAINER_OPACITY: f32 = 1.0;
    pub const INACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const INACTIVE_TRACK_HEIGHT: Dp = Dp(16.0);
    pub const INACTIVE_TRACK_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const LABEL_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const PRESSED_ACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const PRESSED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const PRESSED_HANDLE_WIDTH: Dp = Dp(2.0);
    pub const PRESSED_INACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const PRESSED_STOP_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SLIDER_ACTIVE_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const STOP_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const STOP_INDICATOR_COLOR_SELECTED: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const STOP_INDICATOR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const STOP_INDICATOR_SIZE: Dp = Dp(4.0);
    pub const STOP_INDICATOR_TRAILING_SPACE: Dp = Dp(6.0);
    pub const VALUE_INDICATOR_ACTIVE_BOTTOM_SPACE: Dp = Dp(12.0);
    pub const VALUE_INDICATOR_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_SURFACE;
    pub const VALUE_INDICATOR_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const VALUE_INDICATOR_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SmallIconButtonTokens;
impl SmallIconButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DEFAULT_LEADING_SPACE: Dp = Dp(8.0);
    pub const DEFAULT_TRAILING_SPACE: Dp = Dp(8.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const NARROW_LEADING_SPACE: Dp = Dp(4.0);
    pub const NARROW_TRAILING_SPACE: Dp = Dp(4.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const WIDE_LEADING_SPACE: Dp = Dp(14.0);
    pub const WIDE_TRAILING_SPACE: Dp = Dp(14.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SnackbarTokens;
impl SnackbarTokens {
    pub const ACTION_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_PRIMARY;
    pub const ACTION_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_PRIMARY;
    pub const ACTION_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_PRIMARY;
    pub const ACTION_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const ACTION_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_PRIMARY;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_SURFACE;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_MEDIUM;
    pub const SINGLE_LINE_CONTAINER_HEIGHT: Dp = Dp(48.0);
    pub const TWO_LINES_CONTAINER_HEIGHT: Dp = Dp(68.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SplitButtonLargeTokens;
impl SplitButtonLargeTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(96.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_SMALL;
    pub const INNER_HOVERED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_LARGE_INCREASED;
    pub const INNER_PRESSED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_LARGE_INCREASED;
    pub const LEADING_BUTTON_LEADING_SPACE: Dp = Dp(48.0);
    pub const LEADING_BUTTON_TRAILING_SPACE: Dp = Dp(48.0);
    pub const TRAILING_ICON_SIZE: Dp = Dp(38.0);
    pub const TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_BUTTON_LEADING_SPACE: Dp = Dp(29.0);
    pub const TRAILING_BUTTON_TRAILING_SPACE: Dp = Dp(29.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SplitButtonMediumTokens;
impl SplitButtonMediumTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_EXTRA_SMALL;
    pub const INNER_HOVERED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_MEDIUM;
    pub const INNER_PRESSED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_MEDIUM;
    pub const LEADING_BUTTON_LEADING_SPACE: Dp = Dp(24.0);
    pub const LEADING_BUTTON_TRAILING_SPACE: Dp = Dp(24.0);
    pub const TRAILING_ICON_SIZE: Dp = Dp(26.0);
    pub const TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_BUTTON_LEADING_SPACE: Dp = Dp(15.0);
    pub const TRAILING_BUTTON_TRAILING_SPACE: Dp = Dp(15.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SplitButtonSmallTokens;
impl SplitButtonSmallTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_EXTRA_SMALL;
    pub const INNER_HOVERED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_MEDIUM;
    pub const INNER_PRESSED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_MEDIUM;
    pub const LEADING_BUTTON_LEADING_SPACE: Dp = Dp(16.0);
    pub const LEADING_BUTTON_TRAILING_SPACE: Dp = Dp(12.0);
    pub const TRAILING_ICON_SIZE: Dp = Dp(22.0);
    pub const TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_BUTTON_LEADING_SPACE: Dp = Dp(13.0);
    pub const TRAILING_BUTTON_TRAILING_SPACE: Dp = Dp(13.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SplitButtonXLargeTokens;
impl SplitButtonXLargeTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(136.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_MEDIUM;
    pub const INNER_HOVERED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_LARGE_INCREASED;
    pub const INNER_PRESSED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_LARGE_INCREASED;
    pub const LEADING_BUTTON_LEADING_SPACE: Dp = Dp(64.0);
    pub const LEADING_BUTTON_TRAILING_SPACE: Dp = Dp(64.0);
    pub const TRAILING_ICON_SIZE: Dp = Dp(50.0);
    pub const TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_BUTTON_LEADING_SPACE: Dp = Dp(43.0);
    pub const TRAILING_BUTTON_TRAILING_SPACE: Dp = Dp(43.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SplitButtonXSmallTokens;
impl SplitButtonXSmallTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_EXTRA_SMALL;
    pub const INNER_HOVERED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_SMALL;
    pub const INNER_PRESSED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_SMALL;
    pub const LEADING_BUTTON_LEADING_SPACE: Dp = Dp(12.0);
    pub const LEADING_BUTTON_TRAILING_SPACE: Dp = Dp(10.0);
    pub const OUTER_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_ICON_SIZE: Dp = Dp(22.0);
    pub const TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_BUTTON_LEADING_SPACE: Dp = Dp(13.0);
    pub const TRAILING_BUTTON_TRAILING_SPACE: Dp = Dp(13.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct StandardMenuTokens;
impl StandardMenuTokens {
    pub const BUTTON_DISABLED_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const BUTTON_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const BUTTON_SELECTED_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const DISABLED_BUTTON_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ICON_BUTTON_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const ICON_BUTTON_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ITEM_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const ITEM_DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_FOCUSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_FOCUSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_HOVERED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_HOVERED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_PRESSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_DISABLED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUSED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUSED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_HOVERED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_HOVERED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_HOVERED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_HOVERED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct StandardMotionTokens;
impl StandardMotionTokens {
    pub const SPRING_DEFAULT_SPATIAL_DAMPING: f32 = 0.9;
    pub const SPRING_DEFAULT_SPATIAL_STIFFNESS: f32 = 700.0;
    pub const SPRING_DEFAULT_EFFECTS_DAMPING: f32 = 1.0;
    pub const SPRING_DEFAULT_EFFECTS_STIFFNESS: f32 = 1600.0;
    pub const SPRING_FAST_SPATIAL_DAMPING: f32 = 0.9;
    pub const SPRING_FAST_SPATIAL_STIFFNESS: f32 = 1400.0;
    pub const SPRING_FAST_EFFECTS_DAMPING: f32 = 1.0;
    pub const SPRING_FAST_EFFECTS_STIFFNESS: f32 = 3800.0;
    pub const SPRING_SLOW_SPATIAL_DAMPING: f32 = 0.9;
    pub const SPRING_SLOW_SPATIAL_STIFFNESS: f32 = 300.0;
    pub const SPRING_SLOW_EFFECTS_DAMPING: f32 = 1.0;
    pub const SPRING_SLOW_EFFECTS_STIFFNESS: f32 = 800.0;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct StateTokens;
impl StateTokens {
    pub const DRAGGED_STATE_LAYER_OPACITY: f32 = 0.16;
    pub const FOCUS_STATE_LAYER_OPACITY: f32 = 0.1;
    pub const HOVER_STATE_LAYER_OPACITY: f32 = 0.08;
    pub const PRESSED_STATE_LAYER_OPACITY: f32 = 0.1;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SuggestionChipTokens;
impl SuggestionChipTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const DRAGGED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ELEVATED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const ELEVATED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ELEVATED_DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const ELEVATED_DISABLED_CONTAINER_OPACITY: f32 = 0.12;
    pub const ELEVATED_FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const ELEVATED_PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const FLAT_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FLAT_DISABLED_OUTLINE_OPACITY: f32 = 0.12;
    pub const FLAT_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FLAT_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FLAT_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const DRAGGED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const LEADING_ICON_SIZE: Dp = Dp(18.0);
    pub const PRESSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SwitchTokens;
impl SwitchTokens {
    pub const DISABLED_SELECTED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const DISABLED_SELECTED_HANDLE_OPACITY: f32 = 1.0;
    pub const DISABLED_SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_SELECTED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_SELECTED_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRACK_OPACITY: f32 = 0.12;
    pub const DISABLED_UNSELECTED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_UNSELECTED_HANDLE_OPACITY: f32 = 0.38;
    pub const DISABLED_UNSELECTED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const DISABLED_UNSELECTED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_UNSELECTED_TRACK_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const DISABLED_UNSELECTED_TRACK_OUTLINE_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HANDLE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const PRESSED_HANDLE_HEIGHT: Dp = Dp(28.0);
    pub const PRESSED_HANDLE_WIDTH: Dp = Dp(28.0);
    pub const SELECTED_FOCUS_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const SELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const SELECTED_FOCUS_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HANDLE_HEIGHT: Dp = Dp(24.0);
    pub const SELECTED_HANDLE_WIDTH: Dp = Dp(24.0);
    pub const SELECTED_HOVER_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const SELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const SELECTED_HOVER_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const SELECTED_ICON_SIZE: Dp = Dp(16.0);
    pub const SELECTED_PRESSED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const SELECTED_PRESSED_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const STATE_LAYER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const STATE_LAYER_SIZE: Dp = Dp(40.0);
    pub const TRACK_HEIGHT: Dp = Dp(32.0);
    pub const TRACK_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const TRACK_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const TRACK_WIDTH: Dp = Dp(52.0);
    pub const UNSELECTED_FOCUS_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_FOCUS_TRACK_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_FOCUS_TRACK_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const UNSELECTED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const UNSELECTED_HANDLE_HEIGHT: Dp = Dp(16.0);
    pub const UNSELECTED_HANDLE_WIDTH: Dp = Dp(16.0);
    pub const UNSELECTED_HOVER_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVER_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_HOVER_TRACK_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_HOVER_TRACK_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_ICON_SIZE: Dp = Dp(16.0);
    pub const UNSELECTED_PRESSED_HANDLE_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_PRESSED_TRACK_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_PRESSED_TRACK_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const UNSELECTED_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_TRACK_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const ICON_HANDLE_HEIGHT: Dp = Dp(24.0);
    pub const ICON_HANDLE_WIDTH: Dp = Dp(24.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TextButtonTokens;
impl TextButtonTokens {
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LABEL_OPACITY: f32 = 0.38;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUSED_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVERED_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PRESSED_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TimeInputTokens;
impl TimeInputTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGH;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HEADLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADLINE_FONT: TypographyToken = TypographyKeyTokens::LABEL_MEDIUM;
    pub const PERIOD_SELECTOR_CONTAINER_HEIGHT: Dp = Dp(72.0);
    pub const PERIOD_SELECTOR_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const PERIOD_SELECTOR_CONTAINER_WIDTH: Dp = Dp(52.0);
    pub const PERIOD_SELECTOR_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::TITLE_MEDIUM;
    pub const PERIOD_SELECTOR_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const PERIOD_SELECTOR_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PERIOD_SELECTOR_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_UNSELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PERIOD_SELECTOR_UNSELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PERIOD_SELECTOR_UNSELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PERIOD_SELECTOR_UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TIME_FIELD_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const TIME_FIELD_CONTAINER_HEIGHT: Dp = Dp(72.0);
    pub const TIME_FIELD_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const TIME_FIELD_CONTAINER_WIDTH: Dp = Dp(96.0);
    pub const TIME_FIELD_FOCUS_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const TIME_FIELD_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const TIME_FIELD_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TIME_FIELD_FOCUS_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const TIME_FIELD_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_FIELD_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_FIELD_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::DISPLAY_MEDIUM;
    pub const TIME_FIELD_SEPARATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_FIELD_SEPARATOR_FONT: TypographyToken = TypographyKeyTokens::DISPLAY_LARGE;
    pub const TIME_FIELD_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TIME_FIELD_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TimePickerTokens;
impl TimePickerTokens {
    pub const CLOCK_DIAL_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const CLOCK_DIAL_CONTAINER_SIZE: Dp = Dp(256.0);
    pub const CLOCK_DIAL_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const CLOCK_DIAL_SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::PRIMARY;
    pub const CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_SIZE: Dp = Dp(8.0);
    pub const CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::PRIMARY;
    pub const CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_SIZE: Dp = Dp(48.0);
    pub const CLOCK_DIAL_SELECTOR_TRACK_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CLOCK_DIAL_SELECTOR_TRACK_CONTAINER_WIDTH: Dp = Dp(2.0);
    pub const CLOCK_DIAL_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CLOCK_DIAL_UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGH;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const HEADLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADLINE_FONT: TypographyToken = TypographyKeyTokens::LABEL_MEDIUM;
    pub const PERIOD_SELECTOR_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const PERIOD_SELECTOR_HORIZONTAL_CONTAINER_HEIGHT: Dp = Dp(38.0);
    pub const PERIOD_SELECTOR_HORIZONTAL_CONTAINER_WIDTH: Dp = Dp(216.0);
    pub const PERIOD_SELECTOR_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::TITLE_MEDIUM;
    pub const PERIOD_SELECTOR_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const PERIOD_SELECTOR_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PERIOD_SELECTOR_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_UNSELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PERIOD_SELECTOR_UNSELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PERIOD_SELECTOR_UNSELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PERIOD_SELECTOR_UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PERIOD_SELECTOR_VERTICAL_CONTAINER_HEIGHT: Dp = Dp(80.0);
    pub const PERIOD_SELECTOR_VERTICAL_CONTAINER_WIDTH: Dp = Dp(52.0);
    pub const TIME_SELECTOR24_H_VERTICAL_CONTAINER_WIDTH: Dp = Dp(114.0);
    pub const TIME_SELECTOR_CONTAINER_HEIGHT: Dp = Dp(80.0);
    pub const TIME_SELECTOR_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const TIME_SELECTOR_CONTAINER_WIDTH: Dp = Dp(96.0);
    pub const TIME_SELECTOR_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::DISPLAY_LARGE;
    pub const TIME_SELECTOR_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const TIME_SELECTOR_SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const TIME_SELECTOR_SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const TIME_SELECTOR_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const TIME_SELECTOR_SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const TIME_SELECTOR_SEPARATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_SELECTOR_SEPARATOR_FONT: TypographyToken = TypographyKeyTokens::DISPLAY_LARGE;
    pub const TIME_SELECTOR_UNSELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const TIME_SELECTOR_UNSELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_SELECTOR_UNSELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_SELECTOR_UNSELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_SELECTOR_UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TonalButtonTokens;
impl TonalButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const FOCUSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const HOVERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const SELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const UNSELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const UNSELECTED_FOCUSED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_HOVERED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TypefaceTokens;
impl TypefaceTokens {
    pub const BRAND: FontFamilyToken = FontFamilyToken::SansSerif;
    pub const PLAIN: FontFamilyToken = FontFamilyToken::SansSerif;
    pub const WEIGHT_BOLD: FontWeight = FontWeight::BOLD;
    pub const WEIGHT_MEDIUM: FontWeight = FontWeight::MEDIUM;
    pub const WEIGHT_REGULAR: FontWeight = FontWeight::NORMAL;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TypeScaleTokens;
impl TypeScaleTokens {
    pub const BODY_LARGE_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_LARGE_LINE_HEIGHT: Sp = Sp(24.0);
    pub const BODY_LARGE_SIZE: Sp = Sp(16.0);
    pub const BODY_LARGE_TRACKING: Sp = Sp(0.5);
    pub const BODY_LARGE_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const BODY_MEDIUM_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_MEDIUM_LINE_HEIGHT: Sp = Sp(20.0);
    pub const BODY_MEDIUM_SIZE: Sp = Sp(14.0);
    pub const BODY_MEDIUM_TRACKING: Sp = Sp(0.2);
    pub const BODY_MEDIUM_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const BODY_SMALL_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_SMALL_LINE_HEIGHT: Sp = Sp(16.0);
    pub const BODY_SMALL_SIZE: Sp = Sp(12.0);
    pub const BODY_SMALL_TRACKING: Sp = Sp(0.4);
    pub const BODY_SMALL_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const DISPLAY_LARGE_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_LARGE_LINE_HEIGHT: Sp = Sp(64.0);
    pub const DISPLAY_LARGE_SIZE: Sp = Sp(57.0);
    pub const DISPLAY_LARGE_TRACKING: Sp = Sp(-0.2);
    pub const DISPLAY_LARGE_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const DISPLAY_MEDIUM_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_MEDIUM_LINE_HEIGHT: Sp = Sp(52.0);
    pub const DISPLAY_MEDIUM_SIZE: Sp = Sp(45.0);
    pub const DISPLAY_MEDIUM_TRACKING: Sp = Sp(0.0);
    pub const DISPLAY_MEDIUM_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const DISPLAY_SMALL_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_SMALL_LINE_HEIGHT: Sp = Sp(44.0);
    pub const DISPLAY_SMALL_SIZE: Sp = Sp(36.0);
    pub const DISPLAY_SMALL_TRACKING: Sp = Sp(0.0);
    pub const DISPLAY_SMALL_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const HEADLINE_LARGE_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_LARGE_LINE_HEIGHT: Sp = Sp(40.0);
    pub const HEADLINE_LARGE_SIZE: Sp = Sp(32.0);
    pub const HEADLINE_LARGE_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_LARGE_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const HEADLINE_MEDIUM_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_MEDIUM_LINE_HEIGHT: Sp = Sp(36.0);
    pub const HEADLINE_MEDIUM_SIZE: Sp = Sp(28.0);
    pub const HEADLINE_MEDIUM_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_MEDIUM_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const HEADLINE_SMALL_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_SMALL_LINE_HEIGHT: Sp = Sp(32.0);
    pub const HEADLINE_SMALL_SIZE: Sp = Sp(24.0);
    pub const HEADLINE_SMALL_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_SMALL_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const LABEL_LARGE_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_LARGE_LINE_HEIGHT: Sp = Sp(20.0);
    pub const LABEL_LARGE_SIZE: Sp = Sp(14.0);
    pub const LABEL_LARGE_TRACKING: Sp = Sp(0.1);
    pub const LABEL_LARGE_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const LABEL_MEDIUM_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_MEDIUM_LINE_HEIGHT: Sp = Sp(16.0);
    pub const LABEL_MEDIUM_SIZE: Sp = Sp(12.0);
    pub const LABEL_MEDIUM_TRACKING: Sp = Sp(0.5);
    pub const LABEL_MEDIUM_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const LABEL_SMALL_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_SMALL_LINE_HEIGHT: Sp = Sp(16.0);
    pub const LABEL_SMALL_SIZE: Sp = Sp(11.0);
    pub const LABEL_SMALL_TRACKING: Sp = Sp(0.5);
    pub const LABEL_SMALL_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const TITLE_LARGE_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const TITLE_LARGE_LINE_HEIGHT: Sp = Sp(28.0);
    pub const TITLE_LARGE_SIZE: Sp = Sp(22.0);
    pub const TITLE_LARGE_TRACKING: Sp = Sp(0.0);
    pub const TITLE_LARGE_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const TITLE_MEDIUM_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const TITLE_MEDIUM_LINE_HEIGHT: Sp = Sp(24.0);
    pub const TITLE_MEDIUM_SIZE: Sp = Sp(16.0);
    pub const TITLE_MEDIUM_TRACKING: Sp = Sp(0.2);
    pub const TITLE_MEDIUM_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const TITLE_SMALL_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const TITLE_SMALL_LINE_HEIGHT: Sp = Sp(20.0);
    pub const TITLE_SMALL_SIZE: Sp = Sp(14.0);
    pub const TITLE_SMALL_TRACKING: Sp = Sp(0.1);
    pub const TITLE_SMALL_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const BODY_LARGE_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_LARGE_EMPHASIZED_LINE_HEIGHT: Sp = Sp(24.0);
    pub const BODY_LARGE_EMPHASIZED_SIZE: Sp = Sp(16.0);
    pub const BODY_LARGE_EMPHASIZED_TRACKING: Sp = Sp(0.15);
    pub const BODY_LARGE_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const BODY_MEDIUM_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_MEDIUM_EMPHASIZED_LINE_HEIGHT: Sp = Sp(20.0);
    pub const BODY_MEDIUM_EMPHASIZED_SIZE: Sp = Sp(14.0);
    pub const BODY_MEDIUM_EMPHASIZED_TRACKING: Sp = Sp(0.25);
    pub const BODY_MEDIUM_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const BODY_SMALL_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_SMALL_EMPHASIZED_LINE_HEIGHT: Sp = Sp(16.0);
    pub const BODY_SMALL_EMPHASIZED_SIZE: Sp = Sp(12.0);
    pub const BODY_SMALL_EMPHASIZED_TRACKING: Sp = Sp(0.4);
    pub const BODY_SMALL_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const DISPLAY_LARGE_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_LARGE_EMPHASIZED_LINE_HEIGHT: Sp = Sp(64.0);
    pub const DISPLAY_LARGE_EMPHASIZED_SIZE: Sp = Sp(57.0);
    pub const DISPLAY_LARGE_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const DISPLAY_LARGE_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const DISPLAY_MEDIUM_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_MEDIUM_EMPHASIZED_LINE_HEIGHT: Sp = Sp(52.0);
    pub const DISPLAY_MEDIUM_EMPHASIZED_SIZE: Sp = Sp(45.0);
    pub const DISPLAY_MEDIUM_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const DISPLAY_MEDIUM_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const DISPLAY_SMALL_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_SMALL_EMPHASIZED_LINE_HEIGHT: Sp = Sp(44.0);
    pub const DISPLAY_SMALL_EMPHASIZED_SIZE: Sp = Sp(36.0);
    pub const DISPLAY_SMALL_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const DISPLAY_SMALL_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const HEADLINE_LARGE_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_LARGE_EMPHASIZED_LINE_HEIGHT: Sp = Sp(40.0);
    pub const HEADLINE_LARGE_EMPHASIZED_SIZE: Sp = Sp(32.0);
    pub const HEADLINE_LARGE_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_LARGE_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const HEADLINE_MEDIUM_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_MEDIUM_EMPHASIZED_LINE_HEIGHT: Sp = Sp(36.0);
    pub const HEADLINE_MEDIUM_EMPHASIZED_SIZE: Sp = Sp(28.0);
    pub const HEADLINE_MEDIUM_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_MEDIUM_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const HEADLINE_SMALL_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_SMALL_EMPHASIZED_LINE_HEIGHT: Sp = Sp(32.0);
    pub const HEADLINE_SMALL_EMPHASIZED_SIZE: Sp = Sp(24.0);
    pub const HEADLINE_SMALL_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_SMALL_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const LABEL_LARGE_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_LARGE_EMPHASIZED_LINE_HEIGHT: Sp = Sp(20.0);
    pub const LABEL_LARGE_EMPHASIZED_SIZE: Sp = Sp(14.0);
    pub const LABEL_LARGE_EMPHASIZED_TRACKING: Sp = Sp(0.1);
    pub const LABEL_LARGE_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_BOLD;
    pub const LABEL_MEDIUM_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_MEDIUM_EMPHASIZED_LINE_HEIGHT: Sp = Sp(16.0);
    pub const LABEL_MEDIUM_EMPHASIZED_SIZE: Sp = Sp(12.0);
    pub const LABEL_MEDIUM_EMPHASIZED_TRACKING: Sp = Sp(0.5);
    pub const LABEL_MEDIUM_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_BOLD;
    pub const LABEL_SMALL_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_SMALL_EMPHASIZED_LINE_HEIGHT: Sp = Sp(16.0);
    pub const LABEL_SMALL_EMPHASIZED_SIZE: Sp = Sp(11.0);
    pub const LABEL_SMALL_EMPHASIZED_TRACKING: Sp = Sp(0.5);
    pub const LABEL_SMALL_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_BOLD;
    pub const TITLE_LARGE_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const TITLE_LARGE_EMPHASIZED_LINE_HEIGHT: Sp = Sp(28.0);
    pub const TITLE_LARGE_EMPHASIZED_SIZE: Sp = Sp(22.0);
    pub const TITLE_LARGE_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const TITLE_LARGE_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const TITLE_MEDIUM_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const TITLE_MEDIUM_EMPHASIZED_LINE_HEIGHT: Sp = Sp(24.0);
    pub const TITLE_MEDIUM_EMPHASIZED_SIZE: Sp = Sp(16.0);
    pub const TITLE_MEDIUM_EMPHASIZED_TRACKING: Sp = Sp(0.15);
    pub const TITLE_MEDIUM_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_BOLD;
    pub const TITLE_SMALL_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const TITLE_SMALL_EMPHASIZED_LINE_HEIGHT: Sp = Sp(20.0);
    pub const TITLE_SMALL_EMPHASIZED_SIZE: Sp = Sp(14.0);
    pub const TITLE_SMALL_EMPHASIZED_TRACKING: Sp = Sp(0.1);
    pub const TITLE_SMALL_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_BOLD;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TypographyKeyTokens;
impl TypographyKeyTokens {
    pub const BODY_LARGE: TypographyToken = TypographyToken::BodyLarge;
    pub const BODY_MEDIUM: TypographyToken = TypographyToken::BodyMedium;
    pub const BODY_SMALL: TypographyToken = TypographyToken::BodySmall;
    pub const DISPLAY_LARGE: TypographyToken = TypographyToken::DisplayLarge;
    pub const DISPLAY_MEDIUM: TypographyToken = TypographyToken::DisplayMedium;
    pub const DISPLAY_SMALL: TypographyToken = TypographyToken::DisplaySmall;
    pub const HEADLINE_LARGE: TypographyToken = TypographyToken::HeadlineLarge;
    pub const HEADLINE_MEDIUM: TypographyToken = TypographyToken::HeadlineMedium;
    pub const HEADLINE_SMALL: TypographyToken = TypographyToken::HeadlineSmall;
    pub const LABEL_LARGE: TypographyToken = TypographyToken::LabelLarge;
    pub const LABEL_MEDIUM: TypographyToken = TypographyToken::LabelMedium;
    pub const LABEL_SMALL: TypographyToken = TypographyToken::LabelSmall;
    pub const TITLE_LARGE: TypographyToken = TypographyToken::TitleLarge;
    pub const TITLE_MEDIUM: TypographyToken = TypographyToken::TitleMedium;
    pub const TITLE_SMALL: TypographyToken = TypographyToken::TitleSmall;
    pub const BODY_LARGE_EMPHASIZED: TypographyToken = TypographyToken::BodyLargeEmphasized;
    pub const BODY_MEDIUM_EMPHASIZED: TypographyToken = TypographyToken::BodyMediumEmphasized;
    pub const BODY_SMALL_EMPHASIZED: TypographyToken = TypographyToken::BodySmallEmphasized;
    pub const DISPLAY_LARGE_EMPHASIZED: TypographyToken = TypographyToken::DisplayLargeEmphasized;
    pub const DISPLAY_MEDIUM_EMPHASIZED: TypographyToken = TypographyToken::DisplayMediumEmphasized;
    pub const DISPLAY_SMALL_EMPHASIZED: TypographyToken = TypographyToken::DisplaySmallEmphasized;
    pub const HEADLINE_LARGE_EMPHASIZED: TypographyToken = TypographyToken::HeadlineLargeEmphasized;
    pub const HEADLINE_MEDIUM_EMPHASIZED: TypographyToken =
        TypographyToken::HeadlineMediumEmphasized;
    pub const HEADLINE_SMALL_EMPHASIZED: TypographyToken = TypographyToken::HeadlineSmallEmphasized;
    pub const LABEL_LARGE_EMPHASIZED: TypographyToken = TypographyToken::LabelLargeEmphasized;
    pub const LABEL_MEDIUM_EMPHASIZED: TypographyToken = TypographyToken::LabelMediumEmphasized;
    pub const LABEL_SMALL_EMPHASIZED: TypographyToken = TypographyToken::LabelSmallEmphasized;
    pub const TITLE_LARGE_EMPHASIZED: TypographyToken = TypographyToken::TitleLargeEmphasized;
    pub const TITLE_MEDIUM_EMPHASIZED: TypographyToken = TypographyToken::TitleMediumEmphasized;
    pub const TITLE_SMALL_EMPHASIZED: TypographyToken = TypographyToken::TitleSmallEmphasized;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TypographyTokens;
impl TypographyTokens {
    pub const BODY_LARGE: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_LARGE_FONT,
        weight: TypeScaleTokens::BODY_LARGE_WEIGHT,
        size: TypeScaleTokens::BODY_LARGE_SIZE,
        line_height: TypeScaleTokens::BODY_LARGE_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_LARGE_TRACKING,
    };
    pub const BODY_MEDIUM: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_MEDIUM_FONT,
        weight: TypeScaleTokens::BODY_MEDIUM_WEIGHT,
        size: TypeScaleTokens::BODY_MEDIUM_SIZE,
        line_height: TypeScaleTokens::BODY_MEDIUM_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_MEDIUM_TRACKING,
    };
    pub const BODY_SMALL: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_SMALL_FONT,
        weight: TypeScaleTokens::BODY_SMALL_WEIGHT,
        size: TypeScaleTokens::BODY_SMALL_SIZE,
        line_height: TypeScaleTokens::BODY_SMALL_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_SMALL_TRACKING,
    };
    pub const DISPLAY_LARGE: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_LARGE_FONT,
        weight: TypeScaleTokens::DISPLAY_LARGE_WEIGHT,
        size: TypeScaleTokens::DISPLAY_LARGE_SIZE,
        line_height: TypeScaleTokens::DISPLAY_LARGE_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_LARGE_TRACKING,
    };
    pub const DISPLAY_MEDIUM: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_MEDIUM_FONT,
        weight: TypeScaleTokens::DISPLAY_MEDIUM_WEIGHT,
        size: TypeScaleTokens::DISPLAY_MEDIUM_SIZE,
        line_height: TypeScaleTokens::DISPLAY_MEDIUM_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_MEDIUM_TRACKING,
    };
    pub const DISPLAY_SMALL: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_SMALL_FONT,
        weight: TypeScaleTokens::DISPLAY_SMALL_WEIGHT,
        size: TypeScaleTokens::DISPLAY_SMALL_SIZE,
        line_height: TypeScaleTokens::DISPLAY_SMALL_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_SMALL_TRACKING,
    };
    pub const HEADLINE_LARGE: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_LARGE_FONT,
        weight: TypeScaleTokens::HEADLINE_LARGE_WEIGHT,
        size: TypeScaleTokens::HEADLINE_LARGE_SIZE,
        line_height: TypeScaleTokens::HEADLINE_LARGE_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_LARGE_TRACKING,
    };
    pub const HEADLINE_MEDIUM: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_MEDIUM_FONT,
        weight: TypeScaleTokens::HEADLINE_MEDIUM_WEIGHT,
        size: TypeScaleTokens::HEADLINE_MEDIUM_SIZE,
        line_height: TypeScaleTokens::HEADLINE_MEDIUM_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_MEDIUM_TRACKING,
    };
    pub const HEADLINE_SMALL: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_SMALL_FONT,
        weight: TypeScaleTokens::HEADLINE_SMALL_WEIGHT,
        size: TypeScaleTokens::HEADLINE_SMALL_SIZE,
        line_height: TypeScaleTokens::HEADLINE_SMALL_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_SMALL_TRACKING,
    };
    pub const LABEL_LARGE: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_LARGE_FONT,
        weight: TypeScaleTokens::LABEL_LARGE_WEIGHT,
        size: TypeScaleTokens::LABEL_LARGE_SIZE,
        line_height: TypeScaleTokens::LABEL_LARGE_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_LARGE_TRACKING,
    };
    pub const LABEL_MEDIUM: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_MEDIUM_FONT,
        weight: TypeScaleTokens::LABEL_MEDIUM_WEIGHT,
        size: TypeScaleTokens::LABEL_MEDIUM_SIZE,
        line_height: TypeScaleTokens::LABEL_MEDIUM_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_MEDIUM_TRACKING,
    };
    pub const LABEL_SMALL: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_SMALL_FONT,
        weight: TypeScaleTokens::LABEL_SMALL_WEIGHT,
        size: TypeScaleTokens::LABEL_SMALL_SIZE,
        line_height: TypeScaleTokens::LABEL_SMALL_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_SMALL_TRACKING,
    };
    pub const TITLE_LARGE: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_LARGE_FONT,
        weight: TypeScaleTokens::TITLE_LARGE_WEIGHT,
        size: TypeScaleTokens::TITLE_LARGE_SIZE,
        line_height: TypeScaleTokens::TITLE_LARGE_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_LARGE_TRACKING,
    };
    pub const TITLE_MEDIUM: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_MEDIUM_FONT,
        weight: TypeScaleTokens::TITLE_MEDIUM_WEIGHT,
        size: TypeScaleTokens::TITLE_MEDIUM_SIZE,
        line_height: TypeScaleTokens::TITLE_MEDIUM_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_MEDIUM_TRACKING,
    };
    pub const TITLE_SMALL: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_SMALL_FONT,
        weight: TypeScaleTokens::TITLE_SMALL_WEIGHT,
        size: TypeScaleTokens::TITLE_SMALL_SIZE,
        line_height: TypeScaleTokens::TITLE_SMALL_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_SMALL_TRACKING,
    };
    pub const BODY_LARGE_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_LARGE_EMPHASIZED_FONT,
        weight: TypeScaleTokens::BODY_LARGE_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::BODY_LARGE_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::BODY_LARGE_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_LARGE_EMPHASIZED_TRACKING,
    };
    pub const BODY_MEDIUM_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_FONT,
        weight: TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_TRACKING,
    };
    pub const BODY_SMALL_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_SMALL_EMPHASIZED_FONT,
        weight: TypeScaleTokens::BODY_SMALL_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::BODY_SMALL_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::BODY_SMALL_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_SMALL_EMPHASIZED_TRACKING,
    };
    pub const DISPLAY_LARGE_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_FONT,
        weight: TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_TRACKING,
    };
    pub const DISPLAY_MEDIUM_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_FONT,
        weight: TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_TRACKING,
    };
    pub const DISPLAY_SMALL_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_FONT,
        weight: TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_TRACKING,
    };
    pub const HEADLINE_LARGE_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_FONT,
        weight: TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_TRACKING,
    };
    pub const HEADLINE_MEDIUM_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_FONT,
        weight: TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_TRACKING,
    };
    pub const HEADLINE_SMALL_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_FONT,
        weight: TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_TRACKING,
    };
    pub const LABEL_LARGE_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_LARGE_EMPHASIZED_FONT,
        weight: TypeScaleTokens::LABEL_LARGE_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::LABEL_LARGE_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::LABEL_LARGE_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_LARGE_EMPHASIZED_TRACKING,
    };
    pub const LABEL_MEDIUM_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_FONT,
        weight: TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_TRACKING,
    };
    pub const LABEL_SMALL_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_SMALL_EMPHASIZED_FONT,
        weight: TypeScaleTokens::LABEL_SMALL_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::LABEL_SMALL_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::LABEL_SMALL_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_SMALL_EMPHASIZED_TRACKING,
    };
    pub const TITLE_LARGE_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_LARGE_EMPHASIZED_FONT,
        weight: TypeScaleTokens::TITLE_LARGE_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::TITLE_LARGE_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::TITLE_LARGE_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_LARGE_EMPHASIZED_TRACKING,
    };
    pub const TITLE_MEDIUM_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_FONT,
        weight: TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_TRACKING,
    };
    pub const TITLE_SMALL_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_SMALL_EMPHASIZED_FONT,
        weight: TypeScaleTokens::TITLE_SMALL_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::TITLE_SMALL_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::TITLE_SMALL_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_SMALL_EMPHASIZED_TRACKING,
    };
}
#[derive(Clone, Copy, Debug, Default)]
pub struct VibrantMenuTokens;
impl VibrantMenuTokens {
    pub const BUTTON_DISABLED_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const BUTTON_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const BUTTON_SELECTED_DISABLED_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY;
    pub const BUTTON_SELECTED_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ICON_BUTTON_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ICON_BUTTON_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_FOCUSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_FOCUSED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_FOCUSED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_FOCUSED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_HOVERED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_HOVERED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_HOVERED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_HOVERED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_PRESSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_PRESSED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_PRESSED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_PRESSED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct XLargeIconButtonTokens;
impl XLargeIconButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(136.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const DEFAULT_LEADING_SPACE: Dp = Dp(48.0);
    pub const DEFAULT_TRAILING_SPACE: Dp = Dp(48.0);
    pub const ICON_SIZE: Dp = Dp(40.0);
    pub const NARROW_LEADING_SPACE: Dp = Dp(32.0);
    pub const NARROW_TRAILING_SPACE: Dp = Dp(32.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(3.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const WIDE_LEADING_SPACE: Dp = Dp(72.0);
    pub const WIDE_TRAILING_SPACE: Dp = Dp(72.0);
}
#[derive(Clone, Copy, Debug, Default)]
pub struct XSmallIconButtonTokens;
impl XSmallIconButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DEFAULT_LEADING_SPACE: Dp = Dp(6.0);
    pub const DEFAULT_TRAILING_SPACE: Dp = Dp(6.0);
    pub const ICON_SIZE: Dp = Dp(20.0);
    pub const NARROW_LEADING_SPACE: Dp = Dp(4.0);
    pub const NARROW_TRAILING_SPACE: Dp = Dp(4.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const WIDE_LEADING_SPACE: Dp = Dp(10.0);
    pub const WIDE_TRAILING_SPACE: Dp = Dp(10.0);
}
pub static ALL_TOKENS: &[TokenEntry] = &[
    TokenEntry {
        group: "AppBarLargeFlexibleTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(AppBarLargeFlexibleTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "AppBarLargeFlexibleTokens",
        name: "SubtitleFont",
        value: TokenValue::TypographyRole(AppBarLargeFlexibleTokens::SUBTITLE_FONT),
    },
    TokenEntry {
        group: "AppBarLargeFlexibleTokens",
        name: "TitleFont",
        value: TokenValue::TypographyRole(AppBarLargeFlexibleTokens::TITLE_FONT),
    },
    TokenEntry {
        group: "AppBarLargeFlexibleTokens",
        name: "LargeContainerHeight",
        value: TokenValue::Dp(AppBarLargeFlexibleTokens::LARGE_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "AppBarLargeTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(AppBarLargeTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "AppBarLargeTokens",
        name: "TitleFont",
        value: TokenValue::TypographyRole(AppBarLargeTokens::TITLE_FONT),
    },
    TokenEntry {
        group: "AppBarMediumFlexibleTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(AppBarMediumFlexibleTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "AppBarMediumFlexibleTokens",
        name: "SubtitleFont",
        value: TokenValue::TypographyRole(AppBarMediumFlexibleTokens::SUBTITLE_FONT),
    },
    TokenEntry {
        group: "AppBarMediumFlexibleTokens",
        name: "TitleFont",
        value: TokenValue::TypographyRole(AppBarMediumFlexibleTokens::TITLE_FONT),
    },
    TokenEntry {
        group: "AppBarMediumFlexibleTokens",
        name: "LargeContainerHeight",
        value: TokenValue::Dp(AppBarMediumFlexibleTokens::LARGE_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "AppBarMediumTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(AppBarMediumTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "AppBarMediumTokens",
        name: "TitleFont",
        value: TokenValue::TypographyRole(AppBarMediumTokens::TITLE_FONT),
    },
    TokenEntry {
        group: "AppBarSmallTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(AppBarSmallTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "AppBarSmallTokens",
        name: "SubtitleFont",
        value: TokenValue::TypographyRole(AppBarSmallTokens::SUBTITLE_FONT),
    },
    TokenEntry {
        group: "AppBarSmallTokens",
        name: "TitleFont",
        value: TokenValue::TypographyRole(AppBarSmallTokens::TITLE_FONT),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "AvatarSize",
        value: TokenValue::Dp(AppBarTokens::AVATAR_SIZE),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(AppBarTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(AppBarTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(AppBarTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "IconButtonSpace",
        value: TokenValue::Dp(AppBarTokens::ICON_BUTTON_SPACE),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "IconSize",
        value: TokenValue::Dp(AppBarTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "LeadingIconColor",
        value: TokenValue::ColorRole(AppBarTokens::LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(AppBarTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "OnScrollContainerColor",
        value: TokenValue::ColorRole(AppBarTokens::ON_SCROLL_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "OnScrollContainerElevation",
        value: TokenValue::Dp(AppBarTokens::ON_SCROLL_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "SubtitleColor",
        value: TokenValue::ColorRole(AppBarTokens::SUBTITLE_COLOR),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "TitleColor",
        value: TokenValue::ColorRole(AppBarTokens::TITLE_COLOR),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "TrailingIconColor",
        value: TokenValue::ColorRole(AppBarTokens::TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "AppBarTokens",
        name: "TrailingSpace",
        value: TokenValue::Dp(AppBarTokens::TRAILING_SPACE),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(AssistChipTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(AssistChipTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(AssistChipTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "DisabledLabelTextOpacity",
        value: TokenValue::Float(AssistChipTokens::DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "DraggedContainerElevation",
        value: TokenValue::Dp(AssistChipTokens::DRAGGED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "DraggedLabelTextColor",
        value: TokenValue::ColorRole(AssistChipTokens::DRAGGED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "ElevatedContainerColor",
        value: TokenValue::ColorRole(AssistChipTokens::ELEVATED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "ElevatedContainerElevation",
        value: TokenValue::Dp(AssistChipTokens::ELEVATED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "ElevatedDisabledContainerColor",
        value: TokenValue::ColorRole(AssistChipTokens::ELEVATED_DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "ElevatedDisabledContainerElevation",
        value: TokenValue::Dp(AssistChipTokens::ELEVATED_DISABLED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "ElevatedDisabledContainerOpacity",
        value: TokenValue::Float(AssistChipTokens::ELEVATED_DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "ElevatedFocusContainerElevation",
        value: TokenValue::Dp(AssistChipTokens::ELEVATED_FOCUS_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "ElevatedHoverContainerElevation",
        value: TokenValue::Dp(AssistChipTokens::ELEVATED_HOVER_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "ElevatedPressedContainerElevation",
        value: TokenValue::Dp(AssistChipTokens::ELEVATED_PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "FlatContainerElevation",
        value: TokenValue::Dp(AssistChipTokens::FLAT_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "FlatDisabledOutlineColor",
        value: TokenValue::ColorRole(AssistChipTokens::FLAT_DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "FlatDisabledOutlineOpacity",
        value: TokenValue::Float(AssistChipTokens::FLAT_DISABLED_OUTLINE_OPACITY),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "FlatFocusOutlineColor",
        value: TokenValue::ColorRole(AssistChipTokens::FLAT_FOCUS_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "FlatOutlineColor",
        value: TokenValue::ColorRole(AssistChipTokens::FLAT_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "FlatOutlineWidth",
        value: TokenValue::Dp(AssistChipTokens::FLAT_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(AssistChipTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "FocusLabelTextColor",
        value: TokenValue::ColorRole(AssistChipTokens::FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "HoverLabelTextColor",
        value: TokenValue::ColorRole(AssistChipTokens::HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "LabelTextColor",
        value: TokenValue::ColorRole(AssistChipTokens::LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(AssistChipTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "PressedLabelTextColor",
        value: TokenValue::ColorRole(AssistChipTokens::PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "DisabledIconColor",
        value: TokenValue::ColorRole(AssistChipTokens::DISABLED_ICON_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "DisabledIconOpacity",
        value: TokenValue::Float(AssistChipTokens::DISABLED_ICON_OPACITY),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "DraggedIconColor",
        value: TokenValue::ColorRole(AssistChipTokens::DRAGGED_ICON_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "FocusIconColor",
        value: TokenValue::ColorRole(AssistChipTokens::FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "HoverIconColor",
        value: TokenValue::ColorRole(AssistChipTokens::HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(AssistChipTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "IconSize",
        value: TokenValue::Dp(AssistChipTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "AssistChipTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(AssistChipTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "BadgeTokens",
        name: "Color",
        value: TokenValue::ColorRole(BadgeTokens::COLOR),
    },
    TokenEntry {
        group: "BadgeTokens",
        name: "LargeColor",
        value: TokenValue::ColorRole(BadgeTokens::LARGE_COLOR),
    },
    TokenEntry {
        group: "BadgeTokens",
        name: "LargeLabelTextColor",
        value: TokenValue::ColorRole(BadgeTokens::LARGE_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BadgeTokens",
        name: "LargeLabelTextFont",
        value: TokenValue::TypographyRole(BadgeTokens::LARGE_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "BadgeTokens",
        name: "LargeShape",
        value: TokenValue::ShapeRole(BadgeTokens::LARGE_SHAPE),
    },
    TokenEntry {
        group: "BadgeTokens",
        name: "LargeSize",
        value: TokenValue::Dp(BadgeTokens::LARGE_SIZE),
    },
    TokenEntry {
        group: "BadgeTokens",
        name: "Shape",
        value: TokenValue::ShapeRole(BadgeTokens::SHAPE),
    },
    TokenEntry {
        group: "BadgeTokens",
        name: "Size",
        value: TokenValue::Dp(BadgeTokens::SIZE),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(BaselineButtonTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(BaselineButtonTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "ContainerShapeRound",
        value: TokenValue::ShapeRole(BaselineButtonTokens::CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "ContainerShapeSquare",
        value: TokenValue::ShapeRole(BaselineButtonTokens::CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "DisabledContainerColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "DisabledContainerElevation",
        value: TokenValue::Dp(BaselineButtonTokens::DISABLED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(BaselineButtonTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "DisabledIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::DISABLED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "DisabledIconOpacity",
        value: TokenValue::Float(BaselineButtonTokens::DISABLED_ICON_OPACITY),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "DisabledLabelTextOpacity",
        value: TokenValue::Float(BaselineButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "FocusedContainerElevation",
        value: TokenValue::Dp(BaselineButtonTokens::FOCUSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "FocusedIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "FocusedLabelTextColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "HoveredContainerElevation",
        value: TokenValue::Dp(BaselineButtonTokens::HOVERED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "HoveredIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "HoveredLabelTextColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "IconLabelSpace",
        value: TokenValue::Dp(BaselineButtonTokens::ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "IconSize",
        value: TokenValue::Dp(BaselineButtonTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "LabelTextColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "LabelTextSelectedColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::LABEL_TEXT_SELECTED_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "LabelTextUnselectedColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::LABEL_TEXT_UNSELECTED_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(BaselineButtonTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "PressedContainerElevation",
        value: TokenValue::Dp(BaselineButtonTokens::PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "PressedContainerShape",
        value: TokenValue::ShapeRole(BaselineButtonTokens::PRESSED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "PressedLabelTextColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "SelectedContainerShapeRound",
        value: TokenValue::ShapeRole(BaselineButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "SelectedContainerShapeSquare",
        value: TokenValue::ShapeRole(BaselineButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "SelectedFocusedIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "SelectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "SelectedHoveredIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "SelectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "SelectedIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "SelectedPressedIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "SelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "TrailingSpace",
        value: TokenValue::Dp(BaselineButtonTokens::TRAILING_SPACE),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "UnselectedContainerColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "UnselectedFocusedIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "UnselectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "UnselectedHoveredIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "UnselectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "UnselectedIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "UnselectedPressedIconColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "BaselineButtonTokens",
        name: "UnselectedPressedLabelTextColor",
        value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "BottomAppBarTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(BottomAppBarTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "BottomAppBarTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(BottomAppBarTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "BottomAppBarTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(BottomAppBarTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "BottomAppBarTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(BottomAppBarTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ButtonGroupSmallTokens",
        name: "BetweenSpace",
        value: TokenValue::Dp(ButtonGroupSmallTokens::BETWEEN_SPACE),
    },
    TokenEntry {
        group: "ButtonGroupSmallTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(ButtonGroupSmallTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ButtonLargeTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(ButtonLargeTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ButtonLargeTokens",
        name: "ContainerShapeRound",
        value: TokenValue::ShapeRole(ButtonLargeTokens::CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "ButtonLargeTokens",
        name: "ContainerShapeSquare",
        value: TokenValue::ShapeRole(ButtonLargeTokens::CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "ButtonLargeTokens",
        name: "IconLabelSpace",
        value: TokenValue::Dp(ButtonLargeTokens::ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "ButtonLargeTokens",
        name: "IconSize",
        value: TokenValue::Dp(ButtonLargeTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "ButtonLargeTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(ButtonLargeTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "ButtonLargeTokens",
        name: "OutlinedOutlineWidth",
        value: TokenValue::Dp(ButtonLargeTokens::OUTLINED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "ButtonLargeTokens",
        name: "PressedContainerShape",
        value: TokenValue::ShapeRole(ButtonLargeTokens::PRESSED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ButtonLargeTokens",
        name: "SelectedContainerShapeRound",
        value: TokenValue::ShapeRole(ButtonLargeTokens::SELECTED_CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "ButtonLargeTokens",
        name: "SelectedContainerShapeSquare",
        value: TokenValue::ShapeRole(ButtonLargeTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "ButtonLargeTokens",
        name: "TrailingSpace",
        value: TokenValue::Dp(ButtonLargeTokens::TRAILING_SPACE),
    },
    TokenEntry {
        group: "ButtonMediumTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(ButtonMediumTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ButtonMediumTokens",
        name: "ContainerShapeRound",
        value: TokenValue::ShapeRole(ButtonMediumTokens::CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "ButtonMediumTokens",
        name: "ContainerShapeSquare",
        value: TokenValue::ShapeRole(ButtonMediumTokens::CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "ButtonMediumTokens",
        name: "IconLabelSpace",
        value: TokenValue::Dp(ButtonMediumTokens::ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "ButtonMediumTokens",
        name: "IconSize",
        value: TokenValue::Dp(ButtonMediumTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "ButtonMediumTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(ButtonMediumTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "ButtonMediumTokens",
        name: "OutlinedOutlineWidth",
        value: TokenValue::Dp(ButtonMediumTokens::OUTLINED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "ButtonMediumTokens",
        name: "PressedContainerShape",
        value: TokenValue::ShapeRole(ButtonMediumTokens::PRESSED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ButtonMediumTokens",
        name: "SelectedContainerShapeRound",
        value: TokenValue::ShapeRole(ButtonMediumTokens::SELECTED_CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "ButtonMediumTokens",
        name: "SelectedContainerShapeSquare",
        value: TokenValue::ShapeRole(ButtonMediumTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "ButtonMediumTokens",
        name: "TrailingSpace",
        value: TokenValue::Dp(ButtonMediumTokens::TRAILING_SPACE),
    },
    TokenEntry {
        group: "ButtonSmallTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(ButtonSmallTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ButtonSmallTokens",
        name: "ContainerShapeRound",
        value: TokenValue::ShapeRole(ButtonSmallTokens::CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "ButtonSmallTokens",
        name: "ContainerShapeSquare",
        value: TokenValue::ShapeRole(ButtonSmallTokens::CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "ButtonSmallTokens",
        name: "IconLabelSpace",
        value: TokenValue::Dp(ButtonSmallTokens::ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "ButtonSmallTokens",
        name: "IconSize",
        value: TokenValue::Dp(ButtonSmallTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "ButtonSmallTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(ButtonSmallTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "ButtonSmallTokens",
        name: "OutlinedOutlineWidth",
        value: TokenValue::Dp(ButtonSmallTokens::OUTLINED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "ButtonSmallTokens",
        name: "PressedContainerShape",
        value: TokenValue::ShapeRole(ButtonSmallTokens::PRESSED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ButtonSmallTokens",
        name: "SelectedContainerShapeRound",
        value: TokenValue::ShapeRole(ButtonSmallTokens::SELECTED_CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "ButtonSmallTokens",
        name: "SelectedContainerShapeSquare",
        value: TokenValue::ShapeRole(ButtonSmallTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "ButtonSmallTokens",
        name: "TrailingSpace",
        value: TokenValue::Dp(ButtonSmallTokens::TRAILING_SPACE),
    },
    TokenEntry {
        group: "ButtonXLargeTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(ButtonXLargeTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ButtonXLargeTokens",
        name: "ContainerShapeRound",
        value: TokenValue::ShapeRole(ButtonXLargeTokens::CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "ButtonXLargeTokens",
        name: "ContainerShapeSquare",
        value: TokenValue::ShapeRole(ButtonXLargeTokens::CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "ButtonXLargeTokens",
        name: "IconLabelSpace",
        value: TokenValue::Dp(ButtonXLargeTokens::ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "ButtonXLargeTokens",
        name: "IconSize",
        value: TokenValue::Dp(ButtonXLargeTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "ButtonXLargeTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(ButtonXLargeTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "ButtonXLargeTokens",
        name: "OutlinedOutlineWidth",
        value: TokenValue::Dp(ButtonXLargeTokens::OUTLINED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "ButtonXLargeTokens",
        name: "PressedContainerShape",
        value: TokenValue::ShapeRole(ButtonXLargeTokens::PRESSED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ButtonXLargeTokens",
        name: "SelectedContainerShapeRound",
        value: TokenValue::ShapeRole(ButtonXLargeTokens::SELECTED_CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "ButtonXLargeTokens",
        name: "SelectedContainerShapeSquare",
        value: TokenValue::ShapeRole(ButtonXLargeTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "ButtonXLargeTokens",
        name: "TrailingSpace",
        value: TokenValue::Dp(ButtonXLargeTokens::TRAILING_SPACE),
    },
    TokenEntry {
        group: "ButtonXSmallTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(ButtonXSmallTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ButtonXSmallTokens",
        name: "ContainerShapeRound",
        value: TokenValue::ShapeRole(ButtonXSmallTokens::CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "ButtonXSmallTokens",
        name: "ContainerShapeSquare",
        value: TokenValue::ShapeRole(ButtonXSmallTokens::CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "ButtonXSmallTokens",
        name: "IconLabelSpace",
        value: TokenValue::Dp(ButtonXSmallTokens::ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "ButtonXSmallTokens",
        name: "IconSize",
        value: TokenValue::Dp(ButtonXSmallTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "ButtonXSmallTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(ButtonXSmallTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "ButtonXSmallTokens",
        name: "OutlinedOutlineWidth",
        value: TokenValue::Dp(ButtonXSmallTokens::OUTLINED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "ButtonXSmallTokens",
        name: "PressedContainerShape",
        value: TokenValue::ShapeRole(ButtonXSmallTokens::PRESSED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ButtonXSmallTokens",
        name: "SelectedContainerShapeRound",
        value: TokenValue::ShapeRole(ButtonXSmallTokens::SELECTED_CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "ButtonXSmallTokens",
        name: "SelectedContainerShapeSquare",
        value: TokenValue::ShapeRole(ButtonXSmallTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "ButtonXSmallTokens",
        name: "TrailingSpace",
        value: TokenValue::Dp(ButtonXSmallTokens::TRAILING_SPACE),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "ContainerShape",
        value: TokenValue::Shape(CheckboxTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "ContainerSize",
        value: TokenValue::Dp(CheckboxTokens::CONTAINER_SIZE),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(CheckboxTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "IconSize",
        value: TokenValue::Dp(CheckboxTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedDisabledContainerColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedDisabledContainerOpacity",
        value: TokenValue::Float(CheckboxTokens::SELECTED_DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedDisabledContainerOutlineWidth",
        value: TokenValue::Dp(CheckboxTokens::SELECTED_DISABLED_CONTAINER_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedDisabledIconColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_DISABLED_ICON_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedErrorContainerColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedErrorFocusContainerColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_FOCUS_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedErrorFocusIconColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedErrorHoverContainerColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_HOVER_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedErrorHoverIconColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedErrorIconColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_ICON_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedErrorPressedContainerColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_PRESSED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedErrorPressedIconColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedFocusContainerColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_FOCUS_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedFocusIconColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedFocusOutlineWidth",
        value: TokenValue::Dp(CheckboxTokens::SELECTED_FOCUS_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedHoverContainerColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_HOVER_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedHoverIconColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedHoverOutlineWidth",
        value: TokenValue::Dp(CheckboxTokens::SELECTED_HOVER_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedIconColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedOutlineWidth",
        value: TokenValue::Dp(CheckboxTokens::SELECTED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedPressedContainerColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_PRESSED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedPressedIconColor",
        value: TokenValue::ColorRole(CheckboxTokens::SELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "SelectedPressedOutlineWidth",
        value: TokenValue::Dp(CheckboxTokens::SELECTED_PRESSED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "StateLayerShape",
        value: TokenValue::ShapeRole(CheckboxTokens::STATE_LAYER_SHAPE),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "StateLayerSize",
        value: TokenValue::Dp(CheckboxTokens::STATE_LAYER_SIZE),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedDisabledContainerOpacity",
        value: TokenValue::Float(CheckboxTokens::UNSELECTED_DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedDisabledOutlineColor",
        value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedDisabledOutlineWidth",
        value: TokenValue::Dp(CheckboxTokens::UNSELECTED_DISABLED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedErrorFocusOutlineColor",
        value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_ERROR_FOCUS_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedErrorHoverOutlineColor",
        value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_ERROR_HOVER_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedErrorOutlineColor",
        value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_ERROR_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedErrorPressedOutlineColor",
        value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_ERROR_PRESSED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedFocusOutlineColor",
        value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_FOCUS_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedFocusOutlineWidth",
        value: TokenValue::Dp(CheckboxTokens::UNSELECTED_FOCUS_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedHoverOutlineColor",
        value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_HOVER_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedHoverOutlineWidth",
        value: TokenValue::Dp(CheckboxTokens::UNSELECTED_HOVER_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedOutlineColor",
        value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedOutlineWidth",
        value: TokenValue::Dp(CheckboxTokens::UNSELECTED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedPressedOutlineColor",
        value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_PRESSED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "CheckboxTokens",
        name: "UnselectedPressedOutlineWidth",
        value: TokenValue::Dp(CheckboxTokens::UNSELECTED_PRESSED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "AvatarShape",
        value: TokenValue::ShapeRole(ChipsTokens::AVATAR_SHAPE),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "AvatarSize",
        value: TokenValue::Dp(ChipsTokens::AVATAR_SIZE),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(ChipsTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(ChipsTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "DisabledLeadingIconColor",
        value: TokenValue::ColorRole(ChipsTokens::DISABLED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "DisabledTrailingIconColor",
        value: TokenValue::ColorRole(ChipsTokens::DISABLED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "DraggedContainerElevation",
        value: TokenValue::Dp(ChipsTokens::DRAGGED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "FocusedIndicatorColor",
        value: TokenValue::ColorRole(ChipsTokens::FOCUSED_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "Height",
        value: TokenValue::Dp(ChipsTokens::HEIGHT),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "LabelText",
        value: TokenValue::TypographyRole(ChipsTokens::LABEL_TEXT),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "LeadingIconSize",
        value: TokenValue::Dp(ChipsTokens::LEADING_ICON_SIZE),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "PressedShape",
        value: TokenValue::ShapeRole(ChipsTokens::PRESSED_SHAPE),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(ChipsTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "SelectedDisabledContainerColor",
        value: TokenValue::ColorRole(ChipsTokens::SELECTED_DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "SelectedDisabledContainerOpacity",
        value: TokenValue::Float(ChipsTokens::SELECTED_DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "SelectedLabelTextColor",
        value: TokenValue::ColorRole(ChipsTokens::SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "SelectedLeadingIconColor",
        value: TokenValue::ColorRole(ChipsTokens::SELECTED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "SelectedOutlineWidth",
        value: TokenValue::Dp(ChipsTokens::SELECTED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "SelectedShape",
        value: TokenValue::ShapeRole(ChipsTokens::SELECTED_SHAPE),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "SelectedTrailingIconColor",
        value: TokenValue::ColorRole(ChipsTokens::SELECTED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "TrailingIconSize",
        value: TokenValue::Dp(ChipsTokens::TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "UnselectedDisabledOutlineColor",
        value: TokenValue::ColorRole(ChipsTokens::UNSELECTED_DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "UnselectedDisabledOutlineOpacity",
        value: TokenValue::Float(ChipsTokens::UNSELECTED_DISABLED_OUTLINE_OPACITY),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "UnselectedLabelTextColor",
        value: TokenValue::ColorRole(ChipsTokens::UNSELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "UnselectedLeadingIconColor",
        value: TokenValue::ColorRole(ChipsTokens::UNSELECTED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "UnselectedOutlineColor",
        value: TokenValue::ColorRole(ChipsTokens::UNSELECTED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "UnselectedOutlineWidth",
        value: TokenValue::Dp(ChipsTokens::UNSELECTED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "UnselectedShape",
        value: TokenValue::ShapeRole(ChipsTokens::UNSELECTED_SHAPE),
    },
    TokenEntry {
        group: "ChipsTokens",
        name: "UnselectedTrailingIconColor",
        value: TokenValue::ColorRole(ChipsTokens::UNSELECTED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "CircularProgressIndicatorTokens",
        name: "ActiveThickness",
        value: TokenValue::Dp(CircularProgressIndicatorTokens::ACTIVE_THICKNESS),
    },
    TokenEntry {
        group: "CircularProgressIndicatorTokens",
        name: "ActiveWaveAmplitude",
        value: TokenValue::Dp(CircularProgressIndicatorTokens::ACTIVE_WAVE_AMPLITUDE),
    },
    TokenEntry {
        group: "CircularProgressIndicatorTokens",
        name: "ActiveWaveWavelength",
        value: TokenValue::Dp(CircularProgressIndicatorTokens::ACTIVE_WAVE_WAVELENGTH),
    },
    TokenEntry {
        group: "CircularProgressIndicatorTokens",
        name: "Size",
        value: TokenValue::Dp(CircularProgressIndicatorTokens::SIZE),
    },
    TokenEntry {
        group: "CircularProgressIndicatorTokens",
        name: "TrackActiveSpace",
        value: TokenValue::Dp(CircularProgressIndicatorTokens::TRACK_ACTIVE_SPACE),
    },
    TokenEntry {
        group: "CircularProgressIndicatorTokens",
        name: "TrackThickness",
        value: TokenValue::Dp(CircularProgressIndicatorTokens::TRACK_THICKNESS),
    },
    TokenEntry {
        group: "CircularProgressIndicatorTokens",
        name: "WaveSize",
        value: TokenValue::Dp(CircularProgressIndicatorTokens::WAVE_SIZE),
    },
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
    TokenEntry {
        group: "ConnectedButtonGroupSmallTokens",
        name: "BetweenSpace",
        value: TokenValue::Dp(ConnectedButtonGroupSmallTokens::BETWEEN_SPACE),
    },
    TokenEntry {
        group: "ConnectedButtonGroupSmallTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(ConnectedButtonGroupSmallTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ConnectedButtonGroupSmallTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(ConnectedButtonGroupSmallTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ConnectedButtonGroupSmallTokens",
        name: "InnerCornerCornerSize",
        value: TokenValue::Dp(ConnectedButtonGroupSmallTokens::INNER_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "ConnectedButtonGroupSmallTokens",
        name: "PressedInnerCornerCornerSize",
        value: TokenValue::Dp(ConnectedButtonGroupSmallTokens::PRESSED_INNER_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "ConnectedButtonGroupSmallTokens",
        name: "SelectedInnerCornerCornerSizePercent",
        value: TokenValue::Float(
            ConnectedButtonGroupSmallTokens::SELECTED_INNER_CORNER_CORNER_SIZE_PERCENT,
        ),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(DateInputModalTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(DateInputModalTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(DateInputModalTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(DateInputModalTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "ContainerSurfaceTintLayerColor",
        value: TokenValue::ColorRole(DateInputModalTokens::CONTAINER_SURFACE_TINT_LAYER_COLOR),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "ContainerWidth",
        value: TokenValue::Dp(DateInputModalTokens::CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "HeaderContainerHeight",
        value: TokenValue::Dp(DateInputModalTokens::HEADER_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "HeaderContainerWidth",
        value: TokenValue::Dp(DateInputModalTokens::HEADER_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "HeaderHeadlineColor",
        value: TokenValue::ColorRole(DateInputModalTokens::HEADER_HEADLINE_COLOR),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "HeaderHeadlineFont",
        value: TokenValue::TypographyRole(DateInputModalTokens::HEADER_HEADLINE_FONT),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "HeaderSupportingTextColor",
        value: TokenValue::ColorRole(DateInputModalTokens::HEADER_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "DateInputModalTokens",
        name: "HeaderSupportingTextFont",
        value: TokenValue::TypographyRole(DateInputModalTokens::HEADER_SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(DatePickerModalTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(DatePickerModalTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(DatePickerModalTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(DatePickerModalTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "ContainerWidth",
        value: TokenValue::Dp(DatePickerModalTokens::CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateContainerHeight",
        value: TokenValue::Dp(DatePickerModalTokens::DATE_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateContainerShape",
        value: TokenValue::ShapeRole(DatePickerModalTokens::DATE_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateContainerWidth",
        value: TokenValue::Dp(DatePickerModalTokens::DATE_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateLabelTextFont",
        value: TokenValue::TypographyRole(DatePickerModalTokens::DATE_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateSelectedContainerColor",
        value: TokenValue::ColorRole(DatePickerModalTokens::DATE_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateSelectedLabelTextColor",
        value: TokenValue::ColorRole(DatePickerModalTokens::DATE_SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateStateLayerHeight",
        value: TokenValue::Dp(DatePickerModalTokens::DATE_STATE_LAYER_HEIGHT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateStateLayerShape",
        value: TokenValue::ShapeRole(DatePickerModalTokens::DATE_STATE_LAYER_SHAPE),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateStateLayerWidth",
        value: TokenValue::Dp(DatePickerModalTokens::DATE_STATE_LAYER_WIDTH),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateTodayContainerOutlineColor",
        value: TokenValue::ColorRole(DatePickerModalTokens::DATE_TODAY_CONTAINER_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateTodayContainerOutlineWidth",
        value: TokenValue::Dp(DatePickerModalTokens::DATE_TODAY_CONTAINER_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateTodayLabelTextColor",
        value: TokenValue::ColorRole(DatePickerModalTokens::DATE_TODAY_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "DateUnselectedLabelTextColor",
        value: TokenValue::ColorRole(DatePickerModalTokens::DATE_UNSELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "HeaderContainerHeight",
        value: TokenValue::Dp(DatePickerModalTokens::HEADER_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "HeaderContainerWidth",
        value: TokenValue::Dp(DatePickerModalTokens::HEADER_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "HeaderHeadlineColor",
        value: TokenValue::ColorRole(DatePickerModalTokens::HEADER_HEADLINE_COLOR),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "HeaderHeadlineFont",
        value: TokenValue::TypographyRole(DatePickerModalTokens::HEADER_HEADLINE_FONT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "HeaderSupportingTextColor",
        value: TokenValue::ColorRole(DatePickerModalTokens::HEADER_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "HeaderSupportingTextFont",
        value: TokenValue::TypographyRole(DatePickerModalTokens::HEADER_SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "RangeSelectionActiveIndicatorContainerColor",
        value: TokenValue::ColorRole(
            DatePickerModalTokens::RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_COLOR,
        ),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "RangeSelectionActiveIndicatorContainerHeight",
        value: TokenValue::Dp(
            DatePickerModalTokens::RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_HEIGHT,
        ),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "RangeSelectionActiveIndicatorContainerShape",
        value: TokenValue::ShapeRole(
            DatePickerModalTokens::RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_SHAPE,
        ),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "RangeSelectionContainerElevation",
        value: TokenValue::Dp(DatePickerModalTokens::RANGE_SELECTION_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "RangeSelectionContainerShape",
        value: TokenValue::ShapeRole(DatePickerModalTokens::RANGE_SELECTION_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "SelectionDateInRangeLabelTextColor",
        value: TokenValue::ColorRole(
            DatePickerModalTokens::SELECTION_DATE_IN_RANGE_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "RangeSelectionHeaderContainerHeight",
        value: TokenValue::Dp(DatePickerModalTokens::RANGE_SELECTION_HEADER_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "RangeSelectionHeaderHeadlineFont",
        value: TokenValue::TypographyRole(
            DatePickerModalTokens::RANGE_SELECTION_HEADER_HEADLINE_FONT,
        ),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "RangeSelectionMonthSubheadColor",
        value: TokenValue::ColorRole(DatePickerModalTokens::RANGE_SELECTION_MONTH_SUBHEAD_COLOR),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "RangeSelectionMonthSubheadFont",
        value: TokenValue::TypographyRole(
            DatePickerModalTokens::RANGE_SELECTION_MONTH_SUBHEAD_FONT,
        ),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "WeekdaysLabelTextColor",
        value: TokenValue::ColorRole(DatePickerModalTokens::WEEKDAYS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "WeekdaysLabelTextFont",
        value: TokenValue::TypographyRole(DatePickerModalTokens::WEEKDAYS_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "SelectionYearContainerHeight",
        value: TokenValue::Dp(DatePickerModalTokens::SELECTION_YEAR_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "SelectionYearContainerWidth",
        value: TokenValue::Dp(DatePickerModalTokens::SELECTION_YEAR_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "SelectionYearLabelTextFont",
        value: TokenValue::TypographyRole(DatePickerModalTokens::SELECTION_YEAR_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "SelectionYearSelectedContainerColor",
        value: TokenValue::ColorRole(
            DatePickerModalTokens::SELECTION_YEAR_SELECTED_CONTAINER_COLOR,
        ),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "SelectionYearSelectedLabelTextColor",
        value: TokenValue::ColorRole(
            DatePickerModalTokens::SELECTION_YEAR_SELECTED_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "SelectionYearStateLayerHeight",
        value: TokenValue::Dp(DatePickerModalTokens::SELECTION_YEAR_STATE_LAYER_HEIGHT),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "SelectionYearStateLayerShape",
        value: TokenValue::ShapeRole(DatePickerModalTokens::SELECTION_YEAR_STATE_LAYER_SHAPE),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "SelectionYearStateLayerWidth",
        value: TokenValue::Dp(DatePickerModalTokens::SELECTION_YEAR_STATE_LAYER_WIDTH),
    },
    TokenEntry {
        group: "DatePickerModalTokens",
        name: "SelectionYearUnselectedLabelTextColor",
        value: TokenValue::ColorRole(
            DatePickerModalTokens::SELECTION_YEAR_UNSELECTED_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "ActionFocusLabelTextColor",
        value: TokenValue::ColorRole(DialogTokens::ACTION_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "ActionHoverLabelTextColor",
        value: TokenValue::ColorRole(DialogTokens::ACTION_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "ActionLabelTextColor",
        value: TokenValue::ColorRole(DialogTokens::ACTION_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "ActionLabelTextFont",
        value: TokenValue::TypographyRole(DialogTokens::ACTION_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "ActionPressedLabelTextColor",
        value: TokenValue::ColorRole(DialogTokens::ACTION_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(DialogTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(DialogTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(DialogTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "HeadlineColor",
        value: TokenValue::ColorRole(DialogTokens::HEADLINE_COLOR),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "HeadlineFont",
        value: TokenValue::TypographyRole(DialogTokens::HEADLINE_FONT),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "SupportingTextColor",
        value: TokenValue::ColorRole(DialogTokens::SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "SupportingTextFont",
        value: TokenValue::TypographyRole(DialogTokens::SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(DialogTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "DialogTokens",
        name: "IconSize",
        value: TokenValue::Dp(DialogTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "DividerTokens",
        name: "Color",
        value: TokenValue::ColorRole(DividerTokens::COLOR),
    },
    TokenEntry {
        group: "DividerTokens",
        name: "Thickness",
        value: TokenValue::Dp(DividerTokens::THICKNESS),
    },
    TokenEntry {
        group: "DockedToolbarTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(DockedToolbarTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "DockedToolbarTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(DockedToolbarTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "DockedToolbarTokens",
        name: "ContainerLeadingSpace",
        value: TokenValue::Dp(DockedToolbarTokens::CONTAINER_LEADING_SPACE),
    },
    TokenEntry {
        group: "DockedToolbarTokens",
        name: "ContainerMaxSpacing",
        value: TokenValue::Dp(DockedToolbarTokens::CONTAINER_MAX_SPACING),
    },
    TokenEntry {
        group: "DockedToolbarTokens",
        name: "ContainerMinSpacing",
        value: TokenValue::Dp(DockedToolbarTokens::CONTAINER_MIN_SPACING),
    },
    TokenEntry {
        group: "DockedToolbarTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(DockedToolbarTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "DockedToolbarTokens",
        name: "ContainerTrailingSpace",
        value: TokenValue::Dp(DockedToolbarTokens::CONTAINER_TRAILING_SPACE),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "Color",
        value: TokenValue::ColorRole(DragHandleTokens::COLOR),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "ContainerWidth",
        value: TokenValue::Dp(DragHandleTokens::CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "DraggedColor",
        value: TokenValue::ColorRole(DragHandleTokens::DRAGGED_COLOR),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "DraggedElevation",
        value: TokenValue::Dp(DragHandleTokens::DRAGGED_ELEVATION),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "DraggedHeight",
        value: TokenValue::Dp(DragHandleTokens::DRAGGED_HEIGHT),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "DraggedShape",
        value: TokenValue::ShapeRole(DragHandleTokens::DRAGGED_SHAPE),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "DraggedWidth",
        value: TokenValue::Dp(DragHandleTokens::DRAGGED_WIDTH),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "Elevation",
        value: TokenValue::Dp(DragHandleTokens::ELEVATION),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "Height",
        value: TokenValue::Dp(DragHandleTokens::HEIGHT),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "PressedColor",
        value: TokenValue::ColorRole(DragHandleTokens::PRESSED_COLOR),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "PressedElevation",
        value: TokenValue::Dp(DragHandleTokens::PRESSED_ELEVATION),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "PressedHeight",
        value: TokenValue::Dp(DragHandleTokens::PRESSED_HEIGHT),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "PressedShape",
        value: TokenValue::ShapeRole(DragHandleTokens::PRESSED_SHAPE),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "PressedWidth",
        value: TokenValue::Dp(DragHandleTokens::PRESSED_WIDTH),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "Shape",
        value: TokenValue::ShapeRole(DragHandleTokens::SHAPE),
    },
    TokenEntry {
        group: "DragHandleTokens",
        name: "Width",
        value: TokenValue::Dp(DragHandleTokens::WIDTH),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(ElevatedButtonTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "DisabledContainerColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "DisabledContainerElevation",
        value: TokenValue::Dp(ElevatedButtonTokens::DISABLED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(ElevatedButtonTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "DisabledIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::DISABLED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "DisabledIconOpacity",
        value: TokenValue::Float(ElevatedButtonTokens::DISABLED_ICON_OPACITY),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "DisabledLabelTextOpacity",
        value: TokenValue::Float(ElevatedButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "FocusedContainerElevation",
        value: TokenValue::Dp(ElevatedButtonTokens::FOCUSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "FocusedIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "FocusedLabelTextColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "HoveredContainerElevation",
        value: TokenValue::Dp(ElevatedButtonTokens::HOVERED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "HoveredIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "HoveredLabelTextColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "LabelTextColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "LabelTextSelectedColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::LABEL_TEXT_SELECTED_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "LabelTextUnselectedColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::LABEL_TEXT_UNSELECTED_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "PressedContainerElevation",
        value: TokenValue::Dp(ElevatedButtonTokens::PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "PressedLabelTextColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "SelectedFocusedIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "SelectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "SelectedHoveredIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "SelectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "SelectedIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "SelectedPressedIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "SelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "UnselectedContainerColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "UnselectedFocusedIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "UnselectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "UnselectedHoveredIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "UnselectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "UnselectedIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "UnselectedPressedIconColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedButtonTokens",
        name: "UnselectedPressedLabelTextColor",
        value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(ElevatedCardTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(ElevatedCardTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(ElevatedCardTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "DisabledContainerColor",
        value: TokenValue::ColorRole(ElevatedCardTokens::DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "DisabledContainerElevation",
        value: TokenValue::Dp(ElevatedCardTokens::DISABLED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(ElevatedCardTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "DraggedContainerElevation",
        value: TokenValue::Dp(ElevatedCardTokens::DRAGGED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "FocusContainerElevation",
        value: TokenValue::Dp(ElevatedCardTokens::FOCUS_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(ElevatedCardTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "HoverContainerElevation",
        value: TokenValue::Dp(ElevatedCardTokens::HOVER_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(ElevatedCardTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "IconSize",
        value: TokenValue::Dp(ElevatedCardTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "ElevatedCardTokens",
        name: "PressedContainerElevation",
        value: TokenValue::Dp(ElevatedCardTokens::PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ElevationTokens",
        name: "Level0",
        value: TokenValue::Dp(ElevationTokens::LEVEL0),
    },
    TokenEntry {
        group: "ElevationTokens",
        name: "Level1",
        value: TokenValue::Dp(ElevationTokens::LEVEL1),
    },
    TokenEntry {
        group: "ElevationTokens",
        name: "Level2",
        value: TokenValue::Dp(ElevationTokens::LEVEL2),
    },
    TokenEntry {
        group: "ElevationTokens",
        name: "Level3",
        value: TokenValue::Dp(ElevationTokens::LEVEL3),
    },
    TokenEntry {
        group: "ElevationTokens",
        name: "Level4",
        value: TokenValue::Dp(ElevationTokens::LEVEL4),
    },
    TokenEntry {
        group: "ElevationTokens",
        name: "Level5",
        value: TokenValue::Dp(ElevationTokens::LEVEL5),
    },
    TokenEntry {
        group: "ExpandedListTokens",
        name: "CollapsedItemTrailingIconContainerColor",
        value: TokenValue::ColorRole(
            ExpandedListTokens::COLLAPSED_ITEM_TRAILING_ICON_CONTAINER_COLOR,
        ),
    },
    TokenEntry {
        group: "ExpandedListTokens",
        name: "CollapsedItemTrailingIconIconColor",
        value: TokenValue::ColorRole(ExpandedListTokens::COLLAPSED_ITEM_TRAILING_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "ExpandedListTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(ExpandedListTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ExpandedListTokens",
        name: "ExpandedItemContainerColor",
        value: TokenValue::ColorRole(ExpandedListTokens::EXPANDED_ITEM_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ExpandedListTokens",
        name: "ExpandedItemSegmentedContainerColor",
        value: TokenValue::ColorRole(ExpandedListTokens::EXPANDED_ITEM_SEGMENTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ExpandedListTokens",
        name: "ExpandedItemTrailingIconContainerColor",
        value: TokenValue::ColorRole(
            ExpandedListTokens::EXPANDED_ITEM_TRAILING_ICON_CONTAINER_COLOR,
        ),
    },
    TokenEntry {
        group: "ExpandedListTokens",
        name: "ExpandedItemTrailingIconIconColor",
        value: TokenValue::ColorRole(ExpandedListTokens::EXPANDED_ITEM_TRAILING_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "ExpandedListTokens",
        name: "TrailingIconShape",
        value: TokenValue::ShapeRole(ExpandedListTokens::TRAILING_ICON_SHAPE),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringDefaultSpatialDamping",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_DEFAULT_SPATIAL_DAMPING),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringDefaultSpatialStiffness",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_DEFAULT_SPATIAL_STIFFNESS),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringDefaultEffectsDamping",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_DEFAULT_EFFECTS_DAMPING),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringDefaultEffectsStiffness",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_DEFAULT_EFFECTS_STIFFNESS),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringFastSpatialDamping",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_FAST_SPATIAL_DAMPING),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringFastSpatialStiffness",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_FAST_SPATIAL_STIFFNESS),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringFastEffectsDamping",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_FAST_EFFECTS_DAMPING),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringFastEffectsStiffness",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_FAST_EFFECTS_STIFFNESS),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringSlowSpatialDamping",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_SLOW_SPATIAL_DAMPING),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringSlowSpatialStiffness",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_SLOW_SPATIAL_STIFFNESS),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringSlowEffectsDamping",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_SLOW_EFFECTS_DAMPING),
    },
    TokenEntry {
        group: "ExpressiveMotionTokens",
        name: "SpringSlowEffectsStiffness",
        value: TokenValue::Float(ExpressiveMotionTokens::SPRING_SLOW_EFFECTS_STIFFNESS),
    },
    TokenEntry {
        group: "ExtendedFabLargeTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(ExtendedFabLargeTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ExtendedFabLargeTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(ExtendedFabLargeTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ExtendedFabLargeTokens",
        name: "IconLabelSpace",
        value: TokenValue::Dp(ExtendedFabLargeTokens::ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "ExtendedFabLargeTokens",
        name: "IconSize",
        value: TokenValue::Dp(ExtendedFabLargeTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "ExtendedFabLargeTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(ExtendedFabLargeTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "ExtendedFabLargeTokens",
        name: "TrailingSpace",
        value: TokenValue::Dp(ExtendedFabLargeTokens::TRAILING_SPACE),
    },
    TokenEntry {
        group: "ExtendedFabMediumTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(ExtendedFabMediumTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ExtendedFabMediumTokens",
        name: "IconLabelSpace",
        value: TokenValue::Dp(ExtendedFabMediumTokens::ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "ExtendedFabMediumTokens",
        name: "IconSize",
        value: TokenValue::Dp(ExtendedFabMediumTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "ExtendedFabMediumTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(ExtendedFabMediumTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "ExtendedFabMediumTokens",
        name: "TrailingSpace",
        value: TokenValue::Dp(ExtendedFabMediumTokens::TRAILING_SPACE),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(ExtendedFabPrimaryTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(ExtendedFabPrimaryTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(ExtendedFabPrimaryTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(ExtendedFabPrimaryTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "FocusContainerElevation",
        value: TokenValue::Dp(ExtendedFabPrimaryTokens::FOCUS_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "FocusIconColor",
        value: TokenValue::ColorRole(ExtendedFabPrimaryTokens::FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "FocusLabelTextColor",
        value: TokenValue::ColorRole(ExtendedFabPrimaryTokens::FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "HoverContainerElevation",
        value: TokenValue::Dp(ExtendedFabPrimaryTokens::HOVER_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "HoverIconColor",
        value: TokenValue::ColorRole(ExtendedFabPrimaryTokens::HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "HoverLabelTextColor",
        value: TokenValue::ColorRole(ExtendedFabPrimaryTokens::HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(ExtendedFabPrimaryTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "IconSize",
        value: TokenValue::Dp(ExtendedFabPrimaryTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "LabelTextColor",
        value: TokenValue::ColorRole(ExtendedFabPrimaryTokens::LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(ExtendedFabPrimaryTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "LoweredContainerElevation",
        value: TokenValue::Dp(ExtendedFabPrimaryTokens::LOWERED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "LoweredFocusContainerElevation",
        value: TokenValue::Dp(ExtendedFabPrimaryTokens::LOWERED_FOCUS_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "LoweredHoverContainerElevation",
        value: TokenValue::Dp(ExtendedFabPrimaryTokens::LOWERED_HOVER_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "LoweredPressedContainerElevation",
        value: TokenValue::Dp(ExtendedFabPrimaryTokens::LOWERED_PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "PressedContainerElevation",
        value: TokenValue::Dp(ExtendedFabPrimaryTokens::PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(ExtendedFabPrimaryTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "ExtendedFabPrimaryTokens",
        name: "PressedLabelTextColor",
        value: TokenValue::ColorRole(ExtendedFabPrimaryTokens::PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ExtendedFabSmallTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(ExtendedFabSmallTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ExtendedFabSmallTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(ExtendedFabSmallTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ExtendedFabSmallTokens",
        name: "IconLabelSpace",
        value: TokenValue::Dp(ExtendedFabSmallTokens::ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "ExtendedFabSmallTokens",
        name: "IconSize",
        value: TokenValue::Dp(ExtendedFabSmallTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "ExtendedFabSmallTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(ExtendedFabSmallTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "ExtendedFabSmallTokens",
        name: "TrailingSpace",
        value: TokenValue::Dp(ExtendedFabSmallTokens::TRAILING_SPACE),
    },
    TokenEntry {
        group: "FabBaselineTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(FabBaselineTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "FabBaselineTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(FabBaselineTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FabBaselineTokens",
        name: "ContainerWidth",
        value: TokenValue::Dp(FabBaselineTokens::CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "FabBaselineTokens",
        name: "IconSize",
        value: TokenValue::Dp(FabBaselineTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "FabLargeTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(FabLargeTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "FabLargeTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(FabLargeTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FabLargeTokens",
        name: "ContainerWidth",
        value: TokenValue::Dp(FabLargeTokens::CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "FabLargeTokens",
        name: "IconSize",
        value: TokenValue::Dp(FabLargeTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "FabMediumTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(FabMediumTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "FabMediumTokens",
        name: "ContainerWidth",
        value: TokenValue::Dp(FabMediumTokens::CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "FabMediumTokens",
        name: "IconSize",
        value: TokenValue::Dp(FabMediumTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "CloseButtonBetweenSpace",
        value: TokenValue::Dp(FabMenuBaselineTokens::CLOSE_BUTTON_BETWEEN_SPACE),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "CloseButtonContainerElevation",
        value: TokenValue::Dp(FabMenuBaselineTokens::CLOSE_BUTTON_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "CloseButtonContainerHeight",
        value: TokenValue::Dp(FabMenuBaselineTokens::CLOSE_BUTTON_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "CloseButtonContainerShape",
        value: TokenValue::ShapeRole(FabMenuBaselineTokens::CLOSE_BUTTON_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "CloseButtonContainerWidth",
        value: TokenValue::Dp(FabMenuBaselineTokens::CLOSE_BUTTON_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "CloseButtonIconSize",
        value: TokenValue::Dp(FabMenuBaselineTokens::CLOSE_BUTTON_ICON_SIZE),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "ListItemBetweenSpace",
        value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_BETWEEN_SPACE),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "ListItemContainerElevation",
        value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "ListItemContainerHeight",
        value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "ListItemContainerShape",
        value: TokenValue::ShapeRole(FabMenuBaselineTokens::LIST_ITEM_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "ListItemIconLabelSpace",
        value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "ListItemIconSize",
        value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_ICON_SIZE),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "ListItemLeadingSpace",
        value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_LEADING_SPACE),
    },
    TokenEntry {
        group: "FabMenuBaselineTokens",
        name: "ListItemTrailingSpace",
        value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_TRAILING_SPACE),
    },
    TokenEntry {
        group: "FabPrimaryContainerTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(FabPrimaryContainerTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FabPrimaryContainerTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(FabPrimaryContainerTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FabPrimaryContainerTokens",
        name: "FocusedContainerElevation",
        value: TokenValue::Dp(FabPrimaryContainerTokens::FOCUSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FabPrimaryContainerTokens",
        name: "FocusedIconColor",
        value: TokenValue::ColorRole(FabPrimaryContainerTokens::FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "FabPrimaryContainerTokens",
        name: "HoveredContainerElevation",
        value: TokenValue::Dp(FabPrimaryContainerTokens::HOVERED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FabPrimaryContainerTokens",
        name: "HoveredIconColor",
        value: TokenValue::ColorRole(FabPrimaryContainerTokens::HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "FabPrimaryContainerTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(FabPrimaryContainerTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "FabPrimaryContainerTokens",
        name: "PressedContainerElevation",
        value: TokenValue::Dp(FabPrimaryContainerTokens::PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FabPrimaryContainerTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(FabPrimaryContainerTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "FabSecondaryContainerTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(FabSecondaryContainerTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FabSecondaryContainerTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(FabSecondaryContainerTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FabSecondaryContainerTokens",
        name: "FocusedContainerElevation",
        value: TokenValue::Dp(FabSecondaryContainerTokens::FOCUSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FabSecondaryContainerTokens",
        name: "FocusedIconColor",
        value: TokenValue::ColorRole(FabSecondaryContainerTokens::FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "FabSecondaryContainerTokens",
        name: "HoveredContainerElevation",
        value: TokenValue::Dp(FabSecondaryContainerTokens::HOVERED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FabSecondaryContainerTokens",
        name: "HoveredIconColor",
        value: TokenValue::ColorRole(FabSecondaryContainerTokens::HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "FabSecondaryContainerTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(FabSecondaryContainerTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "FabSecondaryContainerTokens",
        name: "PressedContainerElevation",
        value: TokenValue::Dp(FabSecondaryContainerTokens::PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FabSecondaryContainerTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(FabSecondaryContainerTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "FabSmallTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(FabSmallTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "FabSmallTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(FabSmallTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FabSmallTokens",
        name: "ContainerWidth",
        value: TokenValue::Dp(FabSmallTokens::CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "FabSmallTokens",
        name: "IconSize",
        value: TokenValue::Dp(FabSmallTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "MenuContainerColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::MENU_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "MenuContainerElevation",
        value: TokenValue::Dp(FilledAutocompleteTokens::MENU_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "MenuContainerShape",
        value: TokenValue::ShapeRole(FilledAutocompleteTokens::MENU_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldActiveIndicatorColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldActiveIndicatorHeight",
        value: TokenValue::Dp(FilledAutocompleteTokens::TEXT_FIELD_ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldCaretColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_CARET_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldContainerColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldContainerShape",
        value: TokenValue::ShapeRole(FilledAutocompleteTokens::TEXT_FIELD_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldDisabledActiveIndicatorColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldDisabledActiveIndicatorHeight",
        value: TokenValue::Dp(
            FilledAutocompleteTokens::TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_HEIGHT,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldDisabledActiveIndicatorOpacity",
        value: TokenValue::Float(
            FilledAutocompleteTokens::TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_OPACITY,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldDisabledContainerColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldDisabledContainerOpacity",
        value: TokenValue::Float(FilledAutocompleteTokens::TEXT_FIELD_DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldDisabledInputTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_DISABLED_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldDisabledInputTextOpacity",
        value: TokenValue::Float(FilledAutocompleteTokens::FIELD_DISABLED_INPUT_TEXT_OPACITY),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldDisabledLabelTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldDisabledLabelTextOpacity",
        value: TokenValue::Float(FilledAutocompleteTokens::FIELD_DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldDisabledLeadingIconColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_DISABLED_LEADING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldDisabledLeadingIconOpacity",
        value: TokenValue::Float(
            FilledAutocompleteTokens::TEXT_FIELD_DISABLED_LEADING_ICON_OPACITY,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldDisabledSupportingTextColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::FIELD_DISABLED_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldDisabledSupportingTextOpacity",
        value: TokenValue::Float(FilledAutocompleteTokens::FIELD_DISABLED_SUPPORTING_TEXT_OPACITY),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldDisabledTrailingIconColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_DISABLED_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldDisabledTrailingIconOpacity",
        value: TokenValue::Float(
            FilledAutocompleteTokens::TEXT_FIELD_DISABLED_TRAILING_ICON_OPACITY,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldErrorActiveIndicatorColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_ERROR_ACTIVE_INDICATOR_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldErrorFocusActiveIndicatorColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_ACTIVE_INDICATOR_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldErrorFocusCaretColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_CARET_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldErrorFocusInputTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_ERROR_FOCUS_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldErrorFocusLabelTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_ERROR_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldErrorFocusLeadingIconColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_LEADING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldErrorFocusSupportingTextColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::FIELD_ERROR_FOCUS_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldErrorFocusTrailingIconColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldErrorHoverActiveIndicatorColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_ACTIVE_INDICATOR_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldErrorHoverInputTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_ERROR_HOVER_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldErrorHoverLabelTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_ERROR_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldErrorHoverLeadingIconColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_LEADING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldErrorHoverSupportingTextColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::FIELD_ERROR_HOVER_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldErrorHoverTrailingIconColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldErrorInputTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_ERROR_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldErrorLabelTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_ERROR_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldErrorLeadingIconColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_ERROR_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldErrorSupportingTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_ERROR_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldErrorTrailingIconColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_ERROR_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldFocusActiveIndicatorColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_FOCUS_ACTIVE_INDICATOR_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldFocusActiveIndicatorHeight",
        value: TokenValue::Dp(FilledAutocompleteTokens::TEXT_FIELD_FOCUS_ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldFocusInputTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_FOCUS_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldFocusLabelTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldFocusLeadingIconColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_FOCUS_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldFocusSupportingTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_FOCUS_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldFocusTrailingIconColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_FOCUS_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldHoverActiveIndicatorColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_HOVER_ACTIVE_INDICATOR_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldHoverActiveIndicatorHeight",
        value: TokenValue::Dp(FilledAutocompleteTokens::TEXT_FIELD_HOVER_ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldHoverInputTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_HOVER_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldHoverLabelTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldHoverLeadingIconColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_HOVER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldHoverSupportingTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_HOVER_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldHoverTrailingIconColor",
        value: TokenValue::ColorRole(
            FilledAutocompleteTokens::TEXT_FIELD_HOVER_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldInputTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldInputTextFont",
        value: TokenValue::TypographyRole(FilledAutocompleteTokens::FIELD_INPUT_TEXT_FONT),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldLabelTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldLabelTextFont",
        value: TokenValue::TypographyRole(FilledAutocompleteTokens::FIELD_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldLeadingIconColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldLeadingIconSize",
        value: TokenValue::Dp(FilledAutocompleteTokens::TEXT_FIELD_LEADING_ICON_SIZE),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldSupportingTextColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "FieldSupportingTextFont",
        value: TokenValue::TypographyRole(FilledAutocompleteTokens::FIELD_SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldTrailingIconColor",
        value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledAutocompleteTokens",
        name: "TextFieldTrailingIconSize",
        value: TokenValue::Dp(FilledAutocompleteTokens::TEXT_FIELD_TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(FilledButtonTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(FilledButtonTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "DisabledContainerColor",
        value: TokenValue::ColorRole(FilledButtonTokens::DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "DisabledContainerElevation",
        value: TokenValue::Dp(FilledButtonTokens::DISABLED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(FilledButtonTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "DisabledIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::DISABLED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "DisabledIconOpacity",
        value: TokenValue::Float(FilledButtonTokens::DISABLED_ICON_OPACITY),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(FilledButtonTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "DisabledLabelTextOpacity",
        value: TokenValue::Float(FilledButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "FocusedContainerElevation",
        value: TokenValue::Dp(FilledButtonTokens::FOCUSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "FocusedIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "FocusedLabelTextColor",
        value: TokenValue::ColorRole(FilledButtonTokens::FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "HoveredContainerElevation",
        value: TokenValue::Dp(FilledButtonTokens::HOVERED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "HoveredIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "HoveredLabelTextColor",
        value: TokenValue::ColorRole(FilledButtonTokens::HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "LabelTextColor",
        value: TokenValue::ColorRole(FilledButtonTokens::LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "LabelTextSelectedColor",
        value: TokenValue::ColorRole(FilledButtonTokens::LABEL_TEXT_SELECTED_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "LabelTextUnselectedColor",
        value: TokenValue::ColorRole(FilledButtonTokens::LABEL_TEXT_UNSELECTED_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "PressedContainerElevation",
        value: TokenValue::Dp(FilledButtonTokens::PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "PressedLabelTextColor",
        value: TokenValue::ColorRole(FilledButtonTokens::PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "SelectedFocusedIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "SelectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "SelectedHoveredIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "SelectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "SelectedIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "SelectedPressedIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "SelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "UnselectedContainerColor",
        value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "UnselectedFocusedIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "UnselectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "UnselectedHoveredIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "UnselectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "UnselectedIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "UnselectedPressedIconColor",
        value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledButtonTokens",
        name: "UnselectedPressedLabelTextColor",
        value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(FilledCardTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(FilledCardTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(FilledCardTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "DisabledContainerColor",
        value: TokenValue::ColorRole(FilledCardTokens::DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "DisabledContainerElevation",
        value: TokenValue::Dp(FilledCardTokens::DISABLED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(FilledCardTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "DraggedContainerElevation",
        value: TokenValue::Dp(FilledCardTokens::DRAGGED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "FocusContainerElevation",
        value: TokenValue::Dp(FilledCardTokens::FOCUS_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(FilledCardTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "HoverContainerElevation",
        value: TokenValue::Dp(FilledCardTokens::HOVER_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(FilledCardTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "IconSize",
        value: TokenValue::Dp(FilledCardTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "FilledCardTokens",
        name: "PressedContainerElevation",
        value: TokenValue::Dp(FilledCardTokens::PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "DisabledContainerColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(FilledIconButtonTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "DisabledColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::DISABLED_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "DisabledOpacity",
        value: TokenValue::Float(FilledIconButtonTokens::DISABLED_OPACITY),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "FocusedColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::FOCUSED_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "HoveredColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::HOVERED_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "Color",
        value: TokenValue::ColorRole(FilledIconButtonTokens::COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "PressedColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::PRESSED_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "SelectedFocusedColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::SELECTED_FOCUSED_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "SelectedHoveredColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::SELECTED_HOVERED_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "SelectedColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::SELECTED_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "SelectedPressedColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::SELECTED_PRESSED_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "UnselectedContainerColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::UNSELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "UnselectedFocusedColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::UNSELECTED_FOCUSED_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "UnselectedHoveredColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::UNSELECTED_HOVERED_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "UnselectedColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::UNSELECTED_COLOR),
    },
    TokenEntry {
        group: "FilledIconButtonTokens",
        name: "UnselectedPressedColor",
        value: TokenValue::ColorRole(FilledIconButtonTokens::UNSELECTED_PRESSED_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ActiveIndicatorColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ActiveIndicatorHeight",
        value: TokenValue::Dp(FilledTextFieldTokens::ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "CaretColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::CARET_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(FilledTextFieldTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledActiveIndicatorColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledActiveIndicatorHeight",
        value: TokenValue::Dp(FilledTextFieldTokens::DISABLED_ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledActiveIndicatorOpacity",
        value: TokenValue::Float(FilledTextFieldTokens::DISABLED_ACTIVE_INDICATOR_OPACITY),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledContainerColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(FilledTextFieldTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledInputColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_INPUT_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledInputOpacity",
        value: TokenValue::Float(FilledTextFieldTokens::DISABLED_INPUT_OPACITY),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledLabelColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_LABEL_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledLabelOpacity",
        value: TokenValue::Float(FilledTextFieldTokens::DISABLED_LABEL_OPACITY),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledLeadingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledLeadingIconOpacity",
        value: TokenValue::Float(FilledTextFieldTokens::DISABLED_LEADING_ICON_OPACITY),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledSupportingColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledSupportingOpacity",
        value: TokenValue::Float(FilledTextFieldTokens::DISABLED_SUPPORTING_OPACITY),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledTrailingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "DisabledTrailingIconOpacity",
        value: TokenValue::Float(FilledTextFieldTokens::DISABLED_TRAILING_ICON_OPACITY),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorActiveIndicatorColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorFocusActiveIndicatorColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorFocusCaretColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_CARET_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorFocusInputColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_INPUT_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorFocusLabelColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_LABEL_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorFocusLeadingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorFocusSupportingColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorFocusTrailingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorHoverActiveIndicatorColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorHoverInputColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_INPUT_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorHoverLabelColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_LABEL_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorHoverLeadingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorHoverSupportingColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorHoverTrailingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorInputColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_INPUT_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorLabelColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_LABEL_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorLeadingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorSupportingColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "ErrorTrailingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "FocusActiveIndicatorColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "FocusActiveIndicatorHeight",
        value: TokenValue::Dp(FilledTextFieldTokens::FOCUS_ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "FocusInputColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_INPUT_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "FocusLabelColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_LABEL_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "FocusLeadingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "FocusSupportingColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "FocusTrailingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "HoverActiveIndicatorColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "HoverActiveIndicatorHeight",
        value: TokenValue::Dp(FilledTextFieldTokens::HOVER_ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "HoverInputColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_INPUT_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "HoverLabelColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_LABEL_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "HoverLeadingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "HoverSupportingColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "HoverTrailingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "InputColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::INPUT_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "InputFont",
        value: TokenValue::TypographyRole(FilledTextFieldTokens::INPUT_FONT),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "InputPlaceholderColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::INPUT_PLACEHOLDER_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "InputPrefixColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::INPUT_PREFIX_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "InputSuffixColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::INPUT_SUFFIX_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "LabelColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::LABEL_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "LabelFont",
        value: TokenValue::TypographyRole(FilledTextFieldTokens::LABEL_FONT),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "LeadingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "LeadingIconSize",
        value: TokenValue::Dp(FilledTextFieldTokens::LEADING_ICON_SIZE),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "SupportingColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "SupportingFont",
        value: TokenValue::TypographyRole(FilledTextFieldTokens::SUPPORTING_FONT),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "TrailingIconColor",
        value: TokenValue::ColorRole(FilledTextFieldTokens::TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTextFieldTokens",
        name: "TrailingIconSize",
        value: TokenValue::Dp(FilledTextFieldTokens::TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(FilledTonalButtonTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(FilledTonalButtonTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(FilledTonalButtonTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "DisabledContainerColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "DisabledContainerElevation",
        value: TokenValue::Dp(FilledTonalButtonTokens::DISABLED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(FilledTonalButtonTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "DisabledLabelTextOpacity",
        value: TokenValue::Float(FilledTonalButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "FocusContainerElevation",
        value: TokenValue::Dp(FilledTonalButtonTokens::FOCUS_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "FocusLabelTextColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "HoverContainerElevation",
        value: TokenValue::Dp(FilledTonalButtonTokens::HOVER_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "HoverLabelTextColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "LabelTextColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(FilledTonalButtonTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "PressedContainerElevation",
        value: TokenValue::Dp(FilledTonalButtonTokens::PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "PressedLabelTextColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "DisabledIconColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::DISABLED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "DisabledIconOpacity",
        value: TokenValue::Float(FilledTonalButtonTokens::DISABLED_ICON_OPACITY),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "FocusIconColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "HoverIconColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "IconSize",
        value: TokenValue::Dp(FilledTonalButtonTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "FilledTonalButtonTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(FilledTonalButtonTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "DisabledContainerColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(FilledTonalIconButtonTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "DisabledColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::DISABLED_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "DisabledOpacity",
        value: TokenValue::Float(FilledTonalIconButtonTokens::DISABLED_OPACITY),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "FocusedColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::FOCUSED_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "HoveredColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::HOVERED_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "Color",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "PressedColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::PRESSED_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "SelectedFocusedColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::SELECTED_FOCUSED_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "SelectedHoveredColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::SELECTED_HOVERED_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "SelectedColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::SELECTED_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "SelectedPressedColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::SELECTED_PRESSED_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "UnselectedContainerColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::UNSELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "UnselectedFocusedColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::UNSELECTED_FOCUSED_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "UnselectedHoveredColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::UNSELECTED_HOVERED_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "UnselectedColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::UNSELECTED_COLOR),
    },
    TokenEntry {
        group: "FilledTonalIconButtonTokens",
        name: "UnselectedPressedColor",
        value: TokenValue::ColorRole(FilledTonalIconButtonTokens::UNSELECTED_PRESSED_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(FilterChipTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(FilterChipTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(FilterChipTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "DisabledLabelTextOpacity",
        value: TokenValue::Float(FilterChipTokens::DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "DraggedContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::DRAGGED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "ElevatedContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::ELEVATED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "ElevatedDisabledContainerColor",
        value: TokenValue::ColorRole(FilterChipTokens::ELEVATED_DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "ElevatedDisabledContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::ELEVATED_DISABLED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "ElevatedDisabledContainerOpacity",
        value: TokenValue::Float(FilterChipTokens::ELEVATED_DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "ElevatedFocusContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::ELEVATED_FOCUS_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "ElevatedHoverContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::ELEVATED_HOVER_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "ElevatedPressedContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::ELEVATED_PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "ElevatedSelectedContainerColor",
        value: TokenValue::ColorRole(FilterChipTokens::ELEVATED_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "ElevatedUnselectedContainerColor",
        value: TokenValue::ColorRole(FilterChipTokens::ELEVATED_UNSELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::FLAT_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatDisabledSelectedContainerColor",
        value: TokenValue::ColorRole(FilterChipTokens::FLAT_DISABLED_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatDisabledSelectedContainerOpacity",
        value: TokenValue::Float(FilterChipTokens::FLAT_DISABLED_SELECTED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatDisabledUnselectedOutlineColor",
        value: TokenValue::ColorRole(FilterChipTokens::FLAT_DISABLED_UNSELECTED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatDisabledUnselectedOutlineOpacity",
        value: TokenValue::Float(FilterChipTokens::FLAT_DISABLED_UNSELECTED_OUTLINE_OPACITY),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatSelectedContainerColor",
        value: TokenValue::ColorRole(FilterChipTokens::FLAT_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatSelectedFocusContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::FLAT_SELECTED_FOCUS_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatSelectedHoverContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::FLAT_SELECTED_HOVER_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatSelectedOutlineWidth",
        value: TokenValue::Dp(FilterChipTokens::FLAT_SELECTED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatSelectedPressedContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::FLAT_SELECTED_PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatUnselectedFocusContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::FLAT_UNSELECTED_FOCUS_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatUnselectedFocusOutlineColor",
        value: TokenValue::ColorRole(FilterChipTokens::FLAT_UNSELECTED_FOCUS_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatUnselectedHoverContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::FLAT_UNSELECTED_HOVER_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatUnselectedOutlineColor",
        value: TokenValue::ColorRole(FilterChipTokens::FLAT_UNSELECTED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatUnselectedOutlineWidth",
        value: TokenValue::Dp(FilterChipTokens::FLAT_UNSELECTED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FlatUnselectedPressedContainerElevation",
        value: TokenValue::Dp(FilterChipTokens::FLAT_UNSELECTED_PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(FilterChipTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(FilterChipTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedDraggedLabelTextColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_DRAGGED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedFocusLabelTextColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedHoverLabelTextColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedLabelTextColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedDraggedLabelTextColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_DRAGGED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedFocusLabelTextColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedHoverLabelTextColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedLabelTextColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedPressedLabelTextColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "IconSize",
        value: TokenValue::Dp(FilterChipTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "DisabledLeadingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::DISABLED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "DisabledLeadingIconOpacity",
        value: TokenValue::Float(FilterChipTokens::DISABLED_LEADING_ICON_OPACITY),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedDraggedLeadingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_DRAGGED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedFocusLeadingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_FOCUS_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedHoverLeadingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_HOVER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedLeadingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedPressedLeadingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_PRESSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedDraggedLeadingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_DRAGGED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedFocusLeadingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_FOCUS_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedHoverLeadingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_HOVER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedLeadingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedPressedLeadingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_PRESSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "DisabledTrailingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::DISABLED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "DisabledTrailingIconOpacity",
        value: TokenValue::Float(FilterChipTokens::DISABLED_TRAILING_ICON_OPACITY),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedDraggedTrailingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_DRAGGED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedFocusTrailingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_FOCUS_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedHoverTrailingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_HOVER_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedPressedTrailingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_PRESSED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "SelectedTrailingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::SELECTED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedDraggedTrailingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_DRAGGED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedFocusTrailingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_FOCUS_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedHoverTrailingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_HOVER_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedPressedTrailingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_PRESSED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FilterChipTokens",
        name: "UnselectedTrailingIconColor",
        value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "ContainerBetweenSpace",
        value: TokenValue::Dp(FloatingToolbarTokens::CONTAINER_BETWEEN_SPACE),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "ContainerExternalPadding",
        value: TokenValue::Dp(FloatingToolbarTokens::CONTAINER_EXTERNAL_PADDING),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(FloatingToolbarTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "ContainerLeadingSpace",
        value: TokenValue::Dp(FloatingToolbarTokens::CONTAINER_LEADING_SPACE),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(FloatingToolbarTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "ContainerTrailingSpace",
        value: TokenValue::Dp(FloatingToolbarTokens::CONTAINER_TRAILING_SPACE),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "StandardContainerColor",
        value: TokenValue::ColorRole(FloatingToolbarTokens::STANDARD_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "VibrantButtonSelectedContainerColor",
        value: TokenValue::ColorRole(
            FloatingToolbarTokens::VIBRANT_BUTTON_SELECTED_CONTAINER_COLOR,
        ),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "VibrantButtonSelectedIconColor",
        value: TokenValue::ColorRole(FloatingToolbarTokens::VIBRANT_BUTTON_SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "VibrantButtonSelectedTextColor",
        value: TokenValue::ColorRole(FloatingToolbarTokens::VIBRANT_BUTTON_SELECTED_TEXT_COLOR),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "VibrantButtonUnselectedIconColor",
        value: TokenValue::ColorRole(FloatingToolbarTokens::VIBRANT_BUTTON_UNSELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "VibrantButtonUnselectedTextColor",
        value: TokenValue::ColorRole(FloatingToolbarTokens::VIBRANT_BUTTON_UNSELECTED_TEXT_COLOR),
    },
    TokenEntry {
        group: "FloatingToolbarTokens",
        name: "VibrantContainerColor",
        value: TokenValue::ColorRole(FloatingToolbarTokens::VIBRANT_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "DisabledColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::DISABLED_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "DisabledOpacity",
        value: TokenValue::Float(StandardIconButtonTokens::DISABLED_OPACITY),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "FocusedColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::FOCUSED_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "HoveredColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::HOVERED_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "Color",
        value: TokenValue::ColorRole(StandardIconButtonTokens::COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "PressedColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::PRESSED_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "SelectedFocusedColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::SELECTED_FOCUSED_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "SelectedHoveredColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::SELECTED_HOVERED_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "SelectedColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::SELECTED_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "SelectedPressedColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::SELECTED_PRESSED_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "UnselectedFocusedColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::UNSELECTED_FOCUSED_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "UnselectedHoveredColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::UNSELECTED_HOVERED_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "UnselectedColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::UNSELECTED_COLOR),
    },
    TokenEntry {
        group: "StandardIconButtonTokens",
        name: "UnselectedPressedColor",
        value: TokenValue::ColorRole(StandardIconButtonTokens::UNSELECTED_PRESSED_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(InputChipTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(InputChipTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(InputChipTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(InputChipTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DisabledLabelTextOpacity",
        value: TokenValue::Float(InputChipTokens::DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DisabledSelectedContainerColor",
        value: TokenValue::ColorRole(InputChipTokens::DISABLED_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DisabledSelectedContainerOpacity",
        value: TokenValue::Float(InputChipTokens::DISABLED_SELECTED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DisabledUnselectedOutlineColor",
        value: TokenValue::ColorRole(InputChipTokens::DISABLED_UNSELECTED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DisabledUnselectedOutlineOpacity",
        value: TokenValue::Float(InputChipTokens::DISABLED_UNSELECTED_OUTLINE_OPACITY),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DraggedContainerElevation",
        value: TokenValue::Dp(InputChipTokens::DRAGGED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(InputChipTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(InputChipTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedDraggedLabelTextColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_DRAGGED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedFocusLabelTextColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedHoverLabelTextColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedLabelTextColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedOutlineWidth",
        value: TokenValue::Dp(InputChipTokens::SELECTED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedDraggedLabelTextColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_DRAGGED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedFocusLabelTextColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedFocusOutlineColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_FOCUS_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedHoverLabelTextColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedLabelTextColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedOutlineColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedOutlineWidth",
        value: TokenValue::Dp(InputChipTokens::UNSELECTED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedPressedLabelTextColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "AvatarShape",
        value: TokenValue::ShapeRole(InputChipTokens::AVATAR_SHAPE),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "AvatarSize",
        value: TokenValue::Dp(InputChipTokens::AVATAR_SIZE),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DisabledAvatarOpacity",
        value: TokenValue::Float(InputChipTokens::DISABLED_AVATAR_OPACITY),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DisabledLeadingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::DISABLED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DisabledLeadingIconOpacity",
        value: TokenValue::Float(InputChipTokens::DISABLED_LEADING_ICON_OPACITY),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "LeadingIconSize",
        value: TokenValue::Dp(InputChipTokens::LEADING_ICON_SIZE),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedDraggedLeadingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_DRAGGED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedFocusLeadingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_FOCUS_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedHoverLeadingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_HOVER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedLeadingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedPressedLeadingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_PRESSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedDraggedLeadingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_DRAGGED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedFocusLeadingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_FOCUS_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedHoverLeadingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_HOVER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedLeadingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedPressedLeadingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_PRESSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DisabledTrailingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::DISABLED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "DisabledTrailingIconOpacity",
        value: TokenValue::Float(InputChipTokens::DISABLED_TRAILING_ICON_OPACITY),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedDraggedTrailingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_DRAGGED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedFocusTrailingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_FOCUS_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedHoverTrailingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_HOVER_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedPressedTrailingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_PRESSED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "SelectedTrailingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::SELECTED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "TrailingIconSize",
        value: TokenValue::Dp(InputChipTokens::TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedDraggedTrailingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_DRAGGED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedFocusTrailingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_FOCUS_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedHoverTrailingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_HOVER_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedPressedTrailingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_PRESSED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "InputChipTokens",
        name: "UnselectedTrailingIconColor",
        value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(LargeIconButtonTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "ContainerShapeRound",
        value: TokenValue::ShapeRole(LargeIconButtonTokens::CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "ContainerShapeSquare",
        value: TokenValue::ShapeRole(LargeIconButtonTokens::CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "IconSize",
        value: TokenValue::Dp(LargeIconButtonTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "NarrowLeadingSpace",
        value: TokenValue::Dp(LargeIconButtonTokens::NARROW_LEADING_SPACE),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "NarrowTrailingSpace",
        value: TokenValue::Dp(LargeIconButtonTokens::NARROW_TRAILING_SPACE),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "OutlinedOutlineWidth",
        value: TokenValue::Dp(LargeIconButtonTokens::OUTLINED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "PressedContainerShape",
        value: TokenValue::ShapeRole(LargeIconButtonTokens::PRESSED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "SelectedContainerShapeRound",
        value: TokenValue::ShapeRole(LargeIconButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "SelectedContainerShapeSquare",
        value: TokenValue::ShapeRole(LargeIconButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "UniformLeadingSpace",
        value: TokenValue::Dp(LargeIconButtonTokens::UNIFORM_LEADING_SPACE),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "UniformTrailingSpace",
        value: TokenValue::Dp(LargeIconButtonTokens::UNIFORM_TRAILING_SPACE),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "WideLeadingSpace",
        value: TokenValue::Dp(LargeIconButtonTokens::WIDE_LEADING_SPACE),
    },
    TokenEntry {
        group: "LargeIconButtonTokens",
        name: "WideTrailingSpace",
        value: TokenValue::Dp(LargeIconButtonTokens::WIDE_TRAILING_SPACE),
    },
    TokenEntry {
        group: "LinearProgressIndicatorTokens",
        name: "ActiveThickness",
        value: TokenValue::Dp(LinearProgressIndicatorTokens::ACTIVE_THICKNESS),
    },
    TokenEntry {
        group: "LinearProgressIndicatorTokens",
        name: "ActiveWaveAmplitude",
        value: TokenValue::Dp(LinearProgressIndicatorTokens::ACTIVE_WAVE_AMPLITUDE),
    },
    TokenEntry {
        group: "LinearProgressIndicatorTokens",
        name: "ActiveWaveWavelength",
        value: TokenValue::Dp(LinearProgressIndicatorTokens::ACTIVE_WAVE_WAVELENGTH),
    },
    TokenEntry {
        group: "LinearProgressIndicatorTokens",
        name: "Height",
        value: TokenValue::Dp(LinearProgressIndicatorTokens::HEIGHT),
    },
    TokenEntry {
        group: "LinearProgressIndicatorTokens",
        name: "IndeterminateActiveWaveWavelength",
        value: TokenValue::Dp(LinearProgressIndicatorTokens::INDETERMINATE_ACTIVE_WAVE_WAVELENGTH),
    },
    TokenEntry {
        group: "LinearProgressIndicatorTokens",
        name: "StopSize",
        value: TokenValue::Dp(LinearProgressIndicatorTokens::STOP_SIZE),
    },
    TokenEntry {
        group: "LinearProgressIndicatorTokens",
        name: "StopTrailingSpace",
        value: TokenValue::Dp(LinearProgressIndicatorTokens::STOP_TRAILING_SPACE),
    },
    TokenEntry {
        group: "LinearProgressIndicatorTokens",
        name: "TrackActiveSpace",
        value: TokenValue::Dp(LinearProgressIndicatorTokens::TRACK_ACTIVE_SPACE),
    },
    TokenEntry {
        group: "LinearProgressIndicatorTokens",
        name: "TrackThickness",
        value: TokenValue::Dp(LinearProgressIndicatorTokens::TRACK_THICKNESS),
    },
    TokenEntry {
        group: "LinearProgressIndicatorTokens",
        name: "WaveHeight",
        value: TokenValue::Dp(LinearProgressIndicatorTokens::WAVE_HEIGHT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(ListTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "DividerBottomSpace",
        value: TokenValue::Dp(ListTokens::DIVIDER_BOTTOM_SPACE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "DividerLeadingSpace",
        value: TokenValue::Dp(ListTokens::DIVIDER_LEADING_SPACE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "DividerTopSpace",
        value: TokenValue::Dp(ListTokens::DIVIDER_TOP_SPACE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "DividerTrailingSpace",
        value: TokenValue::Dp(ListTokens::DIVIDER_TRAILING_SPACE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(ListTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemBetweenSpace",
        value: TokenValue::Dp(ListTokens::ITEM_BETWEEN_SPACE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemBottomSpace",
        value: TokenValue::Dp(ListTokens::ITEM_BOTTOM_SPACE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemContainerColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemContainerElevation",
        value: TokenValue::Dp(ListTokens::ITEM_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemContainerShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_DISABLED_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledLabelTextOpacity",
        value: TokenValue::Float(ListTokens::ITEM_DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledLeadingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_DISABLED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledLeadingIconOpacity",
        value: TokenValue::Float(ListTokens::ITEM_DISABLED_LEADING_ICON_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledOverlineColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_DISABLED_OVERLINE_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledOverlineOpacity",
        value: TokenValue::Float(ListTokens::ITEM_DISABLED_OVERLINE_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledStateLayerOpacity",
        value: TokenValue::Float(ListTokens::ITEM_DISABLED_STATE_LAYER_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledSupportingTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_DISABLED_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledSupportingTextOpacity",
        value: TokenValue::Float(ListTokens::ITEM_DISABLED_SUPPORTING_TEXT_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledTrailingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_DISABLED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDisabledTrailingIconOpacity",
        value: TokenValue::Float(ListTokens::ITEM_DISABLED_TRAILING_ICON_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDraggedContainerElevation",
        value: TokenValue::Dp(ListTokens::ITEM_DRAGGED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDraggedContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_DRAGGED_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDraggedLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_DRAGGED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDraggedLeadingIconIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_DRAGGED_LEADING_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemDraggedTrailingIconIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_DRAGGED_TRAILING_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemFocusLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemFocusLeadingIconIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_FOCUS_LEADING_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemFocusTrailingIconIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_FOCUS_TRAILING_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemFocusedContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_FOCUSED_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemHoverLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemHoverLeadingIconIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_HOVER_LEADING_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemHoverTrailingIconIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_HOVER_TRAILING_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemHoveredContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_HOVERED_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLabelTextFont",
        value: TokenValue::TypographyRole(ListTokens::ITEM_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLargeLeadingVideoHeight",
        value: TokenValue::Dp(ListTokens::ITEM_LARGE_LEADING_VIDEO_HEIGHT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLargeLeadingVideoWidth",
        value: TokenValue::Dp(ListTokens::ITEM_LARGE_LEADING_VIDEO_WIDTH),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingAvatarColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_LEADING_AVATAR_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingAvatarLabelColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_LEADING_AVATAR_LABEL_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingAvatarLabelFont",
        value: TokenValue::TypographyRole(ListTokens::ITEM_LEADING_AVATAR_LABEL_FONT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingAvatarShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_LEADING_AVATAR_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingAvatarSize",
        value: TokenValue::Dp(ListTokens::ITEM_LEADING_AVATAR_SIZE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingIconExpressiveSize",
        value: TokenValue::Dp(ListTokens::ITEM_LEADING_ICON_EXPRESSIVE_SIZE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingIconSize",
        value: TokenValue::Dp(ListTokens::ITEM_LEADING_ICON_SIZE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingImageExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_LEADING_IMAGE_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingImageHeight",
        value: TokenValue::Dp(ListTokens::ITEM_LEADING_IMAGE_HEIGHT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingImageShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_LEADING_IMAGE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingImageWidth",
        value: TokenValue::Dp(ListTokens::ITEM_LEADING_IMAGE_WIDTH),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingSpace",
        value: TokenValue::Dp(ListTokens::ITEM_LEADING_SPACE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingVideoShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_LEADING_VIDEO_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemLeadingVideoWidth",
        value: TokenValue::Dp(ListTokens::ITEM_LEADING_VIDEO_WIDTH),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemOneLineContainerHeight",
        value: TokenValue::Dp(ListTokens::ITEM_ONE_LINE_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemOverlineColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_OVERLINE_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemOverlineFont",
        value: TokenValue::TypographyRole(ListTokens::ITEM_OVERLINE_FONT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemPressedContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_PRESSED_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemPressedLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemPressedLeadingIconIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_PRESSED_LEADING_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemPressedTrailingIconIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_PRESSED_TRAILING_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSegmentedContainerColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SEGMENTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedContainerColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_SELECTED_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedContainerShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_SELECTED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledContainerColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_SELECTED_DISABLED_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledContainerOpacity",
        value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledLabelTextOpacity",
        value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledLeadingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledLeadingIconOpacity",
        value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledOverlineColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_OVERLINE_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledOverlineOpacity",
        value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_OVERLINE_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledStateLayerOpacity",
        value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_STATE_LAYER_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledSupportingTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledSupportingTextOpacity",
        value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledTrailingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledTrailingIconOpacity",
        value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            ListTokens::ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDisabledTrailingSupportingTextOpacity",
        value: TokenValue::Float(
            ListTokens::ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY,
        ),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDraggedContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_SELECTED_DRAGGED_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDraggedLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DRAGGED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDraggedLeadingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DRAGGED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedDraggedTrailingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DRAGGED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedFocusLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedFocusLeadingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_FOCUS_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedFocusTrailingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_FOCUS_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedFocusedContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_SELECTED_FOCUSED_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedHoverLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedHoverLeadingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_HOVER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedHoverTrailingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_HOVER_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedHoveredContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_SELECTED_HOVERED_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedLeadingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedOverlineColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_OVERLINE_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedPressedContainerExpressiveShape",
        value: TokenValue::ShapeRole(ListTokens::ITEM_SELECTED_PRESSED_CONTAINER_EXPRESSIVE_SHAPE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedPressedLeadingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_PRESSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedPressedTrailingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_PRESSED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedSupportingTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedTrailingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSelectedTrailingSupportingTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSmallLeadingVideoHeight",
        value: TokenValue::Dp(ListTokens::ITEM_SMALL_LEADING_VIDEO_HEIGHT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSmallLeadingVideoWidth",
        value: TokenValue::Dp(ListTokens::ITEM_SMALL_LEADING_VIDEO_WIDTH),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSupportingTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemSupportingTextFont",
        value: TokenValue::TypographyRole(ListTokens::ITEM_SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemThreeLineContainerHeight",
        value: TokenValue::Dp(ListTokens::ITEM_THREE_LINE_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemTopSpace",
        value: TokenValue::Dp(ListTokens::ITEM_TOP_SPACE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemTrailingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemTrailingIconExpressiveSize",
        value: TokenValue::Dp(ListTokens::ITEM_TRAILING_ICON_EXPRESSIVE_SIZE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemTrailingIconSize",
        value: TokenValue::Dp(ListTokens::ITEM_TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemTrailingSpace",
        value: TokenValue::Dp(ListTokens::ITEM_TRAILING_SPACE),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemTrailingSupportingTextColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_TRAILING_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemTrailingSupportingTextFont",
        value: TokenValue::TypographyRole(ListTokens::ITEM_TRAILING_SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemTwoLineContainerHeight",
        value: TokenValue::Dp(ListTokens::ITEM_TWO_LINE_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "ListTokens",
        name: "ItemUnselectedTrailingIconColor",
        value: TokenValue::ColorRole(ListTokens::ITEM_UNSELECTED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ListTokens",
        name: "SegmentedGap",
        value: TokenValue::Dp(ListTokens::SEGMENTED_GAP),
    },
    TokenEntry {
        group: "LoadingIndicatorTokens",
        name: "ActiveIndicatorColor",
        value: TokenValue::ColorRole(LoadingIndicatorTokens::ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "LoadingIndicatorTokens",
        name: "ActiveSize",
        value: TokenValue::Dp(LoadingIndicatorTokens::ACTIVE_SIZE),
    },
    TokenEntry {
        group: "LoadingIndicatorTokens",
        name: "ContainedActiveColor",
        value: TokenValue::ColorRole(LoadingIndicatorTokens::CONTAINED_ACTIVE_COLOR),
    },
    TokenEntry {
        group: "LoadingIndicatorTokens",
        name: "ContainedContainerColor",
        value: TokenValue::ColorRole(LoadingIndicatorTokens::CONTAINED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "LoadingIndicatorTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(LoadingIndicatorTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "LoadingIndicatorTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(LoadingIndicatorTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "LoadingIndicatorTokens",
        name: "ContainerWidth",
        value: TokenValue::Dp(LoadingIndicatorTokens::CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(MediumIconButtonTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "ContainerShapeRound",
        value: TokenValue::ShapeRole(MediumIconButtonTokens::CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "ContainerShapeSquare",
        value: TokenValue::ShapeRole(MediumIconButtonTokens::CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "DefaultLeadingSpace",
        value: TokenValue::Dp(MediumIconButtonTokens::DEFAULT_LEADING_SPACE),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "DefaultTrailingSpace",
        value: TokenValue::Dp(MediumIconButtonTokens::DEFAULT_TRAILING_SPACE),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "IconSize",
        value: TokenValue::Dp(MediumIconButtonTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "NarrowLeadingSpace",
        value: TokenValue::Dp(MediumIconButtonTokens::NARROW_LEADING_SPACE),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "NarrowTrailingSpace",
        value: TokenValue::Dp(MediumIconButtonTokens::NARROW_TRAILING_SPACE),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "OutlinedOutlineWidth",
        value: TokenValue::Dp(MediumIconButtonTokens::OUTLINED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "PressedContainerShape",
        value: TokenValue::ShapeRole(MediumIconButtonTokens::PRESSED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "SelectedContainerShapeRound",
        value: TokenValue::ShapeRole(MediumIconButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "SelectedContainerShapeSquare",
        value: TokenValue::ShapeRole(MediumIconButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "WideLeadingSpace",
        value: TokenValue::Dp(MediumIconButtonTokens::WIDE_LEADING_SPACE),
    },
    TokenEntry {
        group: "MediumIconButtonTokens",
        name: "WideTrailingSpace",
        value: TokenValue::Dp(MediumIconButtonTokens::WIDE_TRAILING_SPACE),
    },
    TokenEntry {
        group: "MenuTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(MenuTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "MenuTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(MenuTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "MenuTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(MenuTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "MenuTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(MenuTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "MenuTokens",
        name: "ListItemSelectedContainerColor",
        value: TokenValue::ColorRole(MenuTokens::LIST_ITEM_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "MenuTokens",
        name: "ListItemSelectedLabelTextColor",
        value: TokenValue::ColorRole(MenuTokens::LIST_ITEM_SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "MenuTokens",
        name: "ListItemSelectedLeadingTrailingIconColor",
        value: TokenValue::ColorRole(MenuTokens::LIST_ITEM_SELECTED_LEADING_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "MenuTokens",
        name: "MenuListItemLeadingIconColor",
        value: TokenValue::ColorRole(MenuTokens::MENU_LIST_ITEM_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "MotionSchemeKeyTokens",
        name: "DefaultSpatial",
        value: TokenValue::MotionRole(MotionSchemeKeyTokens::DEFAULT_SPATIAL),
    },
    TokenEntry {
        group: "MotionSchemeKeyTokens",
        name: "FastSpatial",
        value: TokenValue::MotionRole(MotionSchemeKeyTokens::FAST_SPATIAL),
    },
    TokenEntry {
        group: "MotionSchemeKeyTokens",
        name: "SlowSpatial",
        value: TokenValue::MotionRole(MotionSchemeKeyTokens::SLOW_SPATIAL),
    },
    TokenEntry {
        group: "MotionSchemeKeyTokens",
        name: "DefaultEffects",
        value: TokenValue::MotionRole(MotionSchemeKeyTokens::DEFAULT_EFFECTS),
    },
    TokenEntry {
        group: "MotionSchemeKeyTokens",
        name: "FastEffects",
        value: TokenValue::MotionRole(MotionSchemeKeyTokens::FAST_EFFECTS),
    },
    TokenEntry {
        group: "MotionSchemeKeyTokens",
        name: "SlowEffects",
        value: TokenValue::MotionRole(MotionSchemeKeyTokens::SLOW_EFFECTS),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationExtraLong1",
        value: TokenValue::Double(MotionTokens::DURATION_EXTRA_LONG1),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationExtraLong2",
        value: TokenValue::Double(MotionTokens::DURATION_EXTRA_LONG2),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationExtraLong3",
        value: TokenValue::Double(MotionTokens::DURATION_EXTRA_LONG3),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationExtraLong4",
        value: TokenValue::Double(MotionTokens::DURATION_EXTRA_LONG4),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationLong1",
        value: TokenValue::Double(MotionTokens::DURATION_LONG1),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationLong2",
        value: TokenValue::Double(MotionTokens::DURATION_LONG2),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationLong3",
        value: TokenValue::Double(MotionTokens::DURATION_LONG3),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationLong4",
        value: TokenValue::Double(MotionTokens::DURATION_LONG4),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationMedium1",
        value: TokenValue::Double(MotionTokens::DURATION_MEDIUM1),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationMedium2",
        value: TokenValue::Double(MotionTokens::DURATION_MEDIUM2),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationMedium3",
        value: TokenValue::Double(MotionTokens::DURATION_MEDIUM3),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationMedium4",
        value: TokenValue::Double(MotionTokens::DURATION_MEDIUM4),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationShort1",
        value: TokenValue::Double(MotionTokens::DURATION_SHORT1),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationShort2",
        value: TokenValue::Double(MotionTokens::DURATION_SHORT2),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationShort3",
        value: TokenValue::Double(MotionTokens::DURATION_SHORT3),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "DurationShort4",
        value: TokenValue::Double(MotionTokens::DURATION_SHORT4),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "EasingEmphasizedCubicBezier",
        value: TokenValue::Easing(MotionTokens::EASING_EMPHASIZED_CUBIC_BEZIER),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "EasingEmphasizedAccelerateCubicBezier",
        value: TokenValue::Easing(MotionTokens::EASING_EMPHASIZED_ACCELERATE_CUBIC_BEZIER),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "EasingEmphasizedDecelerateCubicBezier",
        value: TokenValue::Easing(MotionTokens::EASING_EMPHASIZED_DECELERATE_CUBIC_BEZIER),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "EasingLegacyCubicBezier",
        value: TokenValue::Easing(MotionTokens::EASING_LEGACY_CUBIC_BEZIER),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "EasingLegacyAccelerateCubicBezier",
        value: TokenValue::Easing(MotionTokens::EASING_LEGACY_ACCELERATE_CUBIC_BEZIER),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "EasingLegacyDecelerateCubicBezier",
        value: TokenValue::Easing(MotionTokens::EASING_LEGACY_DECELERATE_CUBIC_BEZIER),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "EasingLinearCubicBezier",
        value: TokenValue::Easing(MotionTokens::EASING_LINEAR_CUBIC_BEZIER),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "EasingStandardCubicBezier",
        value: TokenValue::Easing(MotionTokens::EASING_STANDARD_CUBIC_BEZIER),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "EasingStandardAccelerateCubicBezier",
        value: TokenValue::Easing(MotionTokens::EASING_STANDARD_ACCELERATE_CUBIC_BEZIER),
    },
    TokenEntry {
        group: "MotionTokens",
        name: "EasingStandardDecelerateCubicBezier",
        value: TokenValue::Easing(MotionTokens::EASING_STANDARD_DECELERATE_CUBIC_BEZIER),
    },
    TokenEntry {
        group: "NavigationBarHorizontalItemTokens",
        name: "ActiveIndicatorHeight",
        value: TokenValue::Dp(NavigationBarHorizontalItemTokens::ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "NavigationBarHorizontalItemTokens",
        name: "ActiveIndicatorLeadingSpace",
        value: TokenValue::Dp(NavigationBarHorizontalItemTokens::ACTIVE_INDICATOR_LEADING_SPACE),
    },
    TokenEntry {
        group: "NavigationBarHorizontalItemTokens",
        name: "ActiveIndicatorTrailingSpace",
        value: TokenValue::Dp(NavigationBarHorizontalItemTokens::ACTIVE_INDICATOR_TRAILING_SPACE),
    },
    TokenEntry {
        group: "NavigationBarHorizontalItemTokens",
        name: "IconSize",
        value: TokenValue::Dp(NavigationBarHorizontalItemTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(NavigationBarTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(NavigationBarTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(NavigationBarTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "ItemActiveIconColor",
        value: TokenValue::ColorRole(NavigationBarTokens::ITEM_ACTIVE_ICON_COLOR),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "ItemActiveIndicatorColor",
        value: TokenValue::ColorRole(NavigationBarTokens::ITEM_ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "ItemActiveIndicatorIconLabelSpace",
        value: TokenValue::Dp(NavigationBarTokens::ITEM_ACTIVE_INDICATOR_ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "ItemActiveIndicatorShape",
        value: TokenValue::ShapeRole(NavigationBarTokens::ITEM_ACTIVE_INDICATOR_SHAPE),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "ItemActiveLabelTextColor",
        value: TokenValue::ColorRole(NavigationBarTokens::ITEM_ACTIVE_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "ItemBetweenSpace",
        value: TokenValue::Dp(NavigationBarTokens::ITEM_BETWEEN_SPACE),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "ItemInactiveIconColor",
        value: TokenValue::ColorRole(NavigationBarTokens::ITEM_INACTIVE_ICON_COLOR),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "ItemInactiveLabelTextColor",
        value: TokenValue::ColorRole(NavigationBarTokens::ITEM_INACTIVE_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "NavShape",
        value: TokenValue::ShapeRole(NavigationBarTokens::NAV_SHAPE),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "TallContainerHeight",
        value: TokenValue::Dp(NavigationBarTokens::TALL_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "NavigationBarTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(NavigationBarTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "NavigationBarVerticalItemTokens",
        name: "ActiveIndicatorHeight",
        value: TokenValue::Dp(NavigationBarVerticalItemTokens::ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "NavigationBarVerticalItemTokens",
        name: "ActiveIndicatorWidth",
        value: TokenValue::Dp(NavigationBarVerticalItemTokens::ACTIVE_INDICATOR_WIDTH),
    },
    TokenEntry {
        group: "NavigationBarVerticalItemTokens",
        name: "ContainerBetweenSpace",
        value: TokenValue::Dp(NavigationBarVerticalItemTokens::CONTAINER_BETWEEN_SPACE),
    },
    TokenEntry {
        group: "NavigationBarVerticalItemTokens",
        name: "IconSize",
        value: TokenValue::Dp(NavigationBarVerticalItemTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActiveFocusIconColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::ACTIVE_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActiveFocusLabelTextColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::ACTIVE_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActiveHoverIconColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::ACTIVE_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActiveHoverLabelTextColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::ACTIVE_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActiveIconColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::ACTIVE_ICON_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActiveIndicatorColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActiveIndicatorHeight",
        value: TokenValue::Dp(NavigationDrawerTokens::ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActiveIndicatorShape",
        value: TokenValue::ShapeRole(NavigationDrawerTokens::ACTIVE_INDICATOR_SHAPE),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActiveIndicatorWidth",
        value: TokenValue::Dp(NavigationDrawerTokens::ACTIVE_INDICATOR_WIDTH),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActiveLabelTextColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::ACTIVE_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActivePressedIconColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::ACTIVE_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ActivePressedLabelTextColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::ACTIVE_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "BottomContainerShape",
        value: TokenValue::ShapeRole(NavigationDrawerTokens::BOTTOM_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ContainerHeightPercent",
        value: TokenValue::Float(NavigationDrawerTokens::CONTAINER_HEIGHT_PERCENT),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(NavigationDrawerTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ContainerWidth",
        value: TokenValue::Dp(NavigationDrawerTokens::CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "HeadlineColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::HEADLINE_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "HeadlineFont",
        value: TokenValue::TypographyRole(NavigationDrawerTokens::HEADLINE_FONT),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "IconSize",
        value: TokenValue::Dp(NavigationDrawerTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "InactiveFocusIconColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::INACTIVE_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "InactiveFocusLabelTextColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::INACTIVE_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "InactiveHoverIconColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::INACTIVE_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "InactiveHoverLabelTextColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::INACTIVE_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "InactiveIconColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::INACTIVE_ICON_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "InactiveLabelTextColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::INACTIVE_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "InactivePressedIconColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::INACTIVE_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "InactivePressedLabelTextColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::INACTIVE_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(NavigationDrawerTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "LargeBadgeLabelColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::LARGE_BADGE_LABEL_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "LargeBadgeLabelFont",
        value: TokenValue::TypographyRole(NavigationDrawerTokens::LARGE_BADGE_LABEL_FONT),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ModalContainerColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::MODAL_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "ModalContainerElevation",
        value: TokenValue::Dp(NavigationDrawerTokens::MODAL_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "StandardContainerColor",
        value: TokenValue::ColorRole(NavigationDrawerTokens::STANDARD_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "NavigationDrawerTokens",
        name: "StandardContainerElevation",
        value: TokenValue::Dp(NavigationDrawerTokens::STANDARD_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "NavigationRailBaselineItemTokens",
        name: "ActiveIndicatorIconLabelSpace",
        value: TokenValue::Dp(NavigationRailBaselineItemTokens::ACTIVE_INDICATOR_ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "NavigationRailBaselineItemTokens",
        name: "ActiveIndicatorLeadingSpace",
        value: TokenValue::Dp(NavigationRailBaselineItemTokens::ACTIVE_INDICATOR_LEADING_SPACE),
    },
    TokenEntry {
        group: "NavigationRailBaselineItemTokens",
        name: "ActiveIndicatorShape",
        value: TokenValue::ShapeRole(NavigationRailBaselineItemTokens::ACTIVE_INDICATOR_SHAPE),
    },
    TokenEntry {
        group: "NavigationRailBaselineItemTokens",
        name: "ActiveIndicatorTrailingSpace",
        value: TokenValue::Dp(NavigationRailBaselineItemTokens::ACTIVE_INDICATOR_TRAILING_SPACE),
    },
    TokenEntry {
        group: "NavigationRailBaselineItemTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(NavigationRailBaselineItemTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "NavigationRailBaselineItemTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(NavigationRailBaselineItemTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "NavigationRailBaselineItemTokens",
        name: "ContainerVerticalSpace",
        value: TokenValue::Dp(NavigationRailBaselineItemTokens::CONTAINER_VERTICAL_SPACE),
    },
    TokenEntry {
        group: "NavigationRailBaselineItemTokens",
        name: "HeaderSpaceMinimum",
        value: TokenValue::Dp(NavigationRailBaselineItemTokens::HEADER_SPACE_MINIMUM),
    },
    TokenEntry {
        group: "NavigationRailBaselineItemTokens",
        name: "IconSize",
        value: TokenValue::Dp(NavigationRailBaselineItemTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "NavigationRailCollapsedTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(NavigationRailCollapsedTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "NavigationRailCollapsedTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(NavigationRailCollapsedTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "NavigationRailCollapsedTokens",
        name: "ContainerWidth",
        value: TokenValue::Dp(NavigationRailCollapsedTokens::CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "NavigationRailCollapsedTokens",
        name: "ItemVerticalSpace",
        value: TokenValue::Dp(NavigationRailCollapsedTokens::ITEM_VERTICAL_SPACE),
    },
    TokenEntry {
        group: "NavigationRailCollapsedTokens",
        name: "TopSpace",
        value: TokenValue::Dp(NavigationRailCollapsedTokens::TOP_SPACE),
    },
    TokenEntry {
        group: "NavigationRailCollapsedTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(NavigationRailCollapsedTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "NavigationRailCollapsedTokens",
        name: "NarrowContainerWidth",
        value: TokenValue::Dp(NavigationRailCollapsedTokens::NARROW_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "NavigationRailColorTokens",
        name: "ItemActiveFocusedStateLayer",
        value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_ACTIVE_FOCUSED_STATE_LAYER),
    },
    TokenEntry {
        group: "NavigationRailColorTokens",
        name: "ItemActiveHoveredStateLayer",
        value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_ACTIVE_HOVERED_STATE_LAYER),
    },
    TokenEntry {
        group: "NavigationRailColorTokens",
        name: "ItemActiveIcon",
        value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_ACTIVE_ICON),
    },
    TokenEntry {
        group: "NavigationRailColorTokens",
        name: "ItemActiveIndicator",
        value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_ACTIVE_INDICATOR),
    },
    TokenEntry {
        group: "NavigationRailColorTokens",
        name: "ItemActiveLabelText",
        value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_ACTIVE_LABEL_TEXT),
    },
    TokenEntry {
        group: "NavigationRailColorTokens",
        name: "ItemActivePressedStateLayer",
        value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_ACTIVE_PRESSED_STATE_LAYER),
    },
    TokenEntry {
        group: "NavigationRailColorTokens",
        name: "ItemInactiveFocusedStateLayer",
        value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_INACTIVE_FOCUSED_STATE_LAYER),
    },
    TokenEntry {
        group: "NavigationRailColorTokens",
        name: "ItemInactiveHoveredStateLayer",
        value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_INACTIVE_HOVERED_STATE_LAYER),
    },
    TokenEntry {
        group: "NavigationRailColorTokens",
        name: "ItemInactiveIcon",
        value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_INACTIVE_ICON),
    },
    TokenEntry {
        group: "NavigationRailColorTokens",
        name: "ItemInactiveLabelText",
        value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_INACTIVE_LABEL_TEXT),
    },
    TokenEntry {
        group: "NavigationRailColorTokens",
        name: "ItemInactivePressedStateLayer",
        value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_INACTIVE_PRESSED_STATE_LAYER),
    },
    TokenEntry {
        group: "NavigationRailExpandedTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(NavigationRailExpandedTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "NavigationRailExpandedTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(NavigationRailExpandedTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "NavigationRailExpandedTokens",
        name: "ContainerWidthMaximum",
        value: TokenValue::Dp(NavigationRailExpandedTokens::CONTAINER_WIDTH_MAXIMUM),
    },
    TokenEntry {
        group: "NavigationRailExpandedTokens",
        name: "ContainerWidthMinimum",
        value: TokenValue::Dp(NavigationRailExpandedTokens::CONTAINER_WIDTH_MINIMUM),
    },
    TokenEntry {
        group: "NavigationRailExpandedTokens",
        name: "ModalContainerElevation",
        value: TokenValue::Dp(NavigationRailExpandedTokens::MODAL_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "NavigationRailExpandedTokens",
        name: "ModalContainerShape",
        value: TokenValue::ShapeRole(NavigationRailExpandedTokens::MODAL_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "NavigationRailExpandedTokens",
        name: "TopSpace",
        value: TokenValue::Dp(NavigationRailExpandedTokens::TOP_SPACE),
    },
    TokenEntry {
        group: "NavigationRailExpandedTokens",
        name: "ModalContainerColor",
        value: TokenValue::ColorRole(NavigationRailExpandedTokens::MODAL_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "NavigationRailHorizontalItemTokens",
        name: "ActiveIndicatorHeight",
        value: TokenValue::Dp(NavigationRailHorizontalItemTokens::ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "NavigationRailHorizontalItemTokens",
        name: "FullWidthLeadingSpace",
        value: TokenValue::Dp(NavigationRailHorizontalItemTokens::FULL_WIDTH_LEADING_SPACE),
    },
    TokenEntry {
        group: "NavigationRailHorizontalItemTokens",
        name: "FullWidthTrailingSpace",
        value: TokenValue::Dp(NavigationRailHorizontalItemTokens::FULL_WIDTH_TRAILING_SPACE),
    },
    TokenEntry {
        group: "NavigationRailHorizontalItemTokens",
        name: "IconLabelSpace",
        value: TokenValue::Dp(NavigationRailHorizontalItemTokens::ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "NavigationRailHorizontalItemTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(NavigationRailHorizontalItemTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "NavigationRailHorizontalItemTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(NavigationRailHorizontalItemTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "NavigationRailVerticalItemTokens",
        name: "ActiveIndicatorHeight",
        value: TokenValue::Dp(NavigationRailVerticalItemTokens::ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "NavigationRailVerticalItemTokens",
        name: "ActiveIndicatorWidth",
        value: TokenValue::Dp(NavigationRailVerticalItemTokens::ACTIVE_INDICATOR_WIDTH),
    },
    TokenEntry {
        group: "NavigationRailVerticalItemTokens",
        name: "IconLabelSpace",
        value: TokenValue::Dp(NavigationRailVerticalItemTokens::ICON_LABEL_SPACE),
    },
    TokenEntry {
        group: "NavigationRailVerticalItemTokens",
        name: "LeadingSpace",
        value: TokenValue::Dp(NavigationRailVerticalItemTokens::LEADING_SPACE),
    },
    TokenEntry {
        group: "NavigationRailVerticalItemTokens",
        name: "TrailingSpace",
        value: TokenValue::Dp(NavigationRailVerticalItemTokens::TRAILING_SPACE),
    },
    TokenEntry {
        group: "NavigationRailVerticalItemTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(NavigationRailVerticalItemTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "MenuContainerColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::MENU_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "MenuContainerElevation",
        value: TokenValue::Dp(OutlinedAutocompleteTokens::MENU_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "MenuContainerShape",
        value: TokenValue::ShapeRole(OutlinedAutocompleteTokens::MENU_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldCaretColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_CARET_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldContainerColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldContainerShape",
        value: TokenValue::ShapeRole(OutlinedAutocompleteTokens::TEXT_FIELD_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldDisabledInputTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_DISABLED_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldDisabledInputTextOpacity",
        value: TokenValue::Float(OutlinedAutocompleteTokens::FIELD_DISABLED_INPUT_TEXT_OPACITY),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldDisabledLabelTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldDisabledLabelTextOpacity",
        value: TokenValue::Float(OutlinedAutocompleteTokens::FIELD_DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldDisabledLeadingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_LEADING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldDisabledLeadingIconOpacity",
        value: TokenValue::Float(
            OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_LEADING_ICON_OPACITY,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldDisabledOutlineColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldDisabledOutlineOpacity",
        value: TokenValue::Float(OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_OUTLINE_OPACITY),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldDisabledOutlineWidth",
        value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldDisabledSupportingTextColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::FIELD_DISABLED_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldDisabledSupportingTextOpacity",
        value: TokenValue::Float(
            OutlinedAutocompleteTokens::FIELD_DISABLED_SUPPORTING_TEXT_OPACITY,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldDisabledTrailingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldDisabledTrailingIconOpacity",
        value: TokenValue::Float(
            OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_TRAILING_ICON_OPACITY,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldErrorFocusCaretColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_CARET_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldErrorFocusInputTextColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::FIELD_ERROR_FOCUS_INPUT_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldErrorFocusLabelTextColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::FIELD_ERROR_FOCUS_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldErrorFocusLeadingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_LEADING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldErrorFocusOutlineColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_OUTLINE_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldErrorFocusSupportingTextColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::FIELD_ERROR_FOCUS_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldErrorFocusTrailingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldErrorHoverInputTextColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::FIELD_ERROR_HOVER_INPUT_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldErrorHoverLabelTextColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::FIELD_ERROR_HOVER_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldErrorHoverLeadingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_LEADING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldErrorHoverOutlineColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_OUTLINE_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldErrorHoverSupportingTextColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::FIELD_ERROR_HOVER_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldErrorHoverTrailingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldErrorInputTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_ERROR_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldErrorLabelTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_ERROR_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldErrorLeadingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_LEADING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldErrorOutlineColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldErrorSupportingTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_ERROR_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldErrorTrailingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldFocusInputTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_FOCUS_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldFocusLabelTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldFocusLeadingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_FOCUS_LEADING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldFocusOutlineColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_FOCUS_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldFocusOutlineWidth",
        value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_FOCUS_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldFocusSupportingTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_FOCUS_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldFocusTrailingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_FOCUS_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldHoverInputTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_HOVER_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldHoverLabelTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldHoverLeadingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_HOVER_LEADING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldHoverOutlineColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_HOVER_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldHoverOutlineWidth",
        value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_HOVER_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldHoverSupportingTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_HOVER_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldHoverTrailingIconColor",
        value: TokenValue::ColorRole(
            OutlinedAutocompleteTokens::TEXT_FIELD_HOVER_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldInputTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldInputTextFont",
        value: TokenValue::TypographyRole(OutlinedAutocompleteTokens::FIELD_INPUT_TEXT_FONT),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldLabelTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldLabelTextFont",
        value: TokenValue::TypographyRole(OutlinedAutocompleteTokens::FIELD_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldLeadingIconColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldLeadingIconSize",
        value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_LEADING_ICON_SIZE),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldOutlineColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldOutlineWidth",
        value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldSupportingTextColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "FieldSupportingTextFont",
        value: TokenValue::TypographyRole(OutlinedAutocompleteTokens::FIELD_SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldTrailingIconColor",
        value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedAutocompleteTokens",
        name: "TextFieldTrailingIconSize",
        value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(OutlinedButtonTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "DisabledIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::DISABLED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "DisabledIconOpacity",
        value: TokenValue::Float(OutlinedButtonTokens::DISABLED_ICON_OPACITY),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "DisabledLabelTextOpacity",
        value: TokenValue::Float(OutlinedButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "DisabledOutlineColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "FocusedIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "FocusedLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "FocusedOutlineColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::FOCUSED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "HoveredIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "HoveredLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "HoveredOutlineColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::HOVERED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "LabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "OutlineColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "PressedLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "PressedOutlineColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::PRESSED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "SelectedDisabledContainerColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "SelectedFocusedIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "SelectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "SelectedHoveredIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "SelectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "SelectedIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "SelectedLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "SelectedPressedIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "SelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedDisabledOutlineColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedFocusedIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedFocusedOutlineColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_FOCUSED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedHoveredIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedHoveredOutlineColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_HOVERED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedPressedIconColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedPressedLabelTextColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedButtonTokens",
        name: "UnselectedPressedOutlineColor",
        value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_PRESSED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(OutlinedCardTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(OutlinedCardTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(OutlinedCardTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "DisabledContainerElevation",
        value: TokenValue::Dp(OutlinedCardTokens::DISABLED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "DisabledOutlineColor",
        value: TokenValue::ColorRole(OutlinedCardTokens::DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "DisabledOutlineOpacity",
        value: TokenValue::Float(OutlinedCardTokens::DISABLED_OUTLINE_OPACITY),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "DraggedContainerElevation",
        value: TokenValue::Dp(OutlinedCardTokens::DRAGGED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "DraggedOutlineColor",
        value: TokenValue::ColorRole(OutlinedCardTokens::DRAGGED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "FocusContainerElevation",
        value: TokenValue::Dp(OutlinedCardTokens::FOCUS_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "FocusOutlineColor",
        value: TokenValue::ColorRole(OutlinedCardTokens::FOCUS_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "HoverContainerElevation",
        value: TokenValue::Dp(OutlinedCardTokens::HOVER_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "HoverOutlineColor",
        value: TokenValue::ColorRole(OutlinedCardTokens::HOVER_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(OutlinedCardTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "IconSize",
        value: TokenValue::Dp(OutlinedCardTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "OutlineColor",
        value: TokenValue::ColorRole(OutlinedCardTokens::OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "OutlineWidth",
        value: TokenValue::Dp(OutlinedCardTokens::OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "PressedContainerElevation",
        value: TokenValue::Dp(OutlinedCardTokens::PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "OutlinedCardTokens",
        name: "PressedOutlineColor",
        value: TokenValue::ColorRole(OutlinedCardTokens::PRESSED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "DisabledColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::DISABLED_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "DisabledOpacity",
        value: TokenValue::Float(OutlinedIconButtonTokens::DISABLED_OPACITY),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "DisabledOutlineColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "FocusedColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::FOCUSED_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "HoveredColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::HOVERED_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "Color",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "OutlineColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "PressedColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::PRESSED_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "SelectedDisabledContainerColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::SELECTED_DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "SelectedDisabledContainerOpacity",
        value: TokenValue::Float(OutlinedIconButtonTokens::SELECTED_DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "SelectedFocusedColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::SELECTED_FOCUSED_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "SelectedHoveredColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::SELECTED_HOVERED_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "SelectedColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::SELECTED_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "SelectedPressedColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::SELECTED_PRESSED_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "UnselectedDisabledOutlineColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::UNSELECTED_DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "UnselectedFocusedColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::UNSELECTED_FOCUSED_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "UnselectedHoveredColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::UNSELECTED_HOVERED_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "UnselectedColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::UNSELECTED_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "UnselectedOutlineColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::UNSELECTED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedIconButtonTokens",
        name: "UnselectedPressedColor",
        value: TokenValue::ColorRole(OutlinedIconButtonTokens::UNSELECTED_PRESSED_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(OutlinedSegmentedButtonTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "DisabledIconColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::DISABLED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "DisabledIconOpacity",
        value: TokenValue::Float(OutlinedSegmentedButtonTokens::DISABLED_ICON_OPACITY),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "DisabledLabelTextOpacity",
        value: TokenValue::Float(OutlinedSegmentedButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "DisabledOutlineColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "DisabledOutlineOpacity",
        value: TokenValue::Float(OutlinedSegmentedButtonTokens::DISABLED_OUTLINE_OPACITY),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(OutlinedSegmentedButtonTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "OutlineColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "OutlineWidth",
        value: TokenValue::Dp(OutlinedSegmentedButtonTokens::OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "SelectedFocusIconColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::SELECTED_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "SelectedFocusLabelTextColor",
        value: TokenValue::ColorRole(
            OutlinedSegmentedButtonTokens::SELECTED_FOCUS_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "SelectedHoverIconColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::SELECTED_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "SelectedHoverLabelTextColor",
        value: TokenValue::ColorRole(
            OutlinedSegmentedButtonTokens::SELECTED_HOVER_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "SelectedLabelTextColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "SelectedPressedIconColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::SELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "SelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(
            OutlinedSegmentedButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "SelectedIconColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "Shape",
        value: TokenValue::ShapeRole(OutlinedSegmentedButtonTokens::SHAPE),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "UnselectedFocusIconColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::UNSELECTED_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "UnselectedFocusLabelTextColor",
        value: TokenValue::ColorRole(
            OutlinedSegmentedButtonTokens::UNSELECTED_FOCUS_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "UnselectedHoverIconColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::UNSELECTED_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "UnselectedHoverLabelTextColor",
        value: TokenValue::ColorRole(
            OutlinedSegmentedButtonTokens::UNSELECTED_HOVER_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "UnselectedLabelTextColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::UNSELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "UnselectedPressedIconColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "UnselectedPressedLabelTextColor",
        value: TokenValue::ColorRole(
            OutlinedSegmentedButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "UnselectedIconColor",
        value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::UNSELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedSegmentedButtonTokens",
        name: "IconSize",
        value: TokenValue::Dp(OutlinedSegmentedButtonTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "CaretColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::CARET_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(OutlinedTextFieldTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(OutlinedTextFieldTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledInputColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_INPUT_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledInputOpacity",
        value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_INPUT_OPACITY),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledLabelColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_LABEL_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledLabelOpacity",
        value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_LABEL_OPACITY),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledLeadingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledLeadingIconOpacity",
        value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_LEADING_ICON_OPACITY),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledOutlineColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledOutlineOpacity",
        value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_OUTLINE_OPACITY),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledOutlineWidth",
        value: TokenValue::Dp(OutlinedTextFieldTokens::DISABLED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledSupportingColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledSupportingOpacity",
        value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_SUPPORTING_OPACITY),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledTrailingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "DisabledTrailingIconOpacity",
        value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_TRAILING_ICON_OPACITY),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorFocusCaretColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_CARET_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorFocusInputColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_INPUT_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorFocusLabelColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_LABEL_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorFocusLeadingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorFocusOutlineColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorFocusSupportingColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorFocusTrailingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorHoverInputColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_INPUT_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorHoverLabelColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_LABEL_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorHoverLeadingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorHoverOutlineColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorHoverSupportingColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorHoverTrailingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorInputColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_INPUT_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorLabelColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_LABEL_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorLeadingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorOutlineColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorSupportingColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "ErrorTrailingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "FocusInputColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_INPUT_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "FocusLabelColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_LABEL_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "FocusLeadingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "FocusOutlineColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "FocusOutlineWidth",
        value: TokenValue::Dp(OutlinedTextFieldTokens::FOCUS_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "FocusSupportingColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "FocusTrailingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "HoverInputColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_INPUT_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "HoverLabelColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_LABEL_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "HoverLeadingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "HoverOutlineColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "HoverOutlineWidth",
        value: TokenValue::Dp(OutlinedTextFieldTokens::HOVER_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "HoverSupportingColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "HoverTrailingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "InputColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::INPUT_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "InputFont",
        value: TokenValue::TypographyRole(OutlinedTextFieldTokens::INPUT_FONT),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "InputPlaceholderColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::INPUT_PLACEHOLDER_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "InputPrefixColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::INPUT_PREFIX_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "InputSuffixColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::INPUT_SUFFIX_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "LabelColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::LABEL_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "LabelFont",
        value: TokenValue::TypographyRole(OutlinedTextFieldTokens::LABEL_FONT),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "LeadingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "LeadingIconSize",
        value: TokenValue::Dp(OutlinedTextFieldTokens::LEADING_ICON_SIZE),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "OutlineColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::OUTLINE_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "OutlineWidth",
        value: TokenValue::Dp(OutlinedTextFieldTokens::OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "SupportingColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::SUPPORTING_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "SupportingFont",
        value: TokenValue::TypographyRole(OutlinedTextFieldTokens::SUPPORTING_FONT),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "TrailingIconColor",
        value: TokenValue::ColorRole(OutlinedTextFieldTokens::TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "OutlinedTextFieldTokens",
        name: "TrailingIconSize",
        value: TokenValue::Dp(OutlinedTextFieldTokens::TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Black",
        value: TokenValue::Color(PaletteTokens::BLACK),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error0",
        value: TokenValue::Color(PaletteTokens::ERROR0),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error10",
        value: TokenValue::Color(PaletteTokens::ERROR10),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error100",
        value: TokenValue::Color(PaletteTokens::ERROR100),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error20",
        value: TokenValue::Color(PaletteTokens::ERROR20),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error30",
        value: TokenValue::Color(PaletteTokens::ERROR30),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error40",
        value: TokenValue::Color(PaletteTokens::ERROR40),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error50",
        value: TokenValue::Color(PaletteTokens::ERROR50),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error60",
        value: TokenValue::Color(PaletteTokens::ERROR60),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error70",
        value: TokenValue::Color(PaletteTokens::ERROR70),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error80",
        value: TokenValue::Color(PaletteTokens::ERROR80),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error90",
        value: TokenValue::Color(PaletteTokens::ERROR90),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error95",
        value: TokenValue::Color(PaletteTokens::ERROR95),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Error99",
        value: TokenValue::Color(PaletteTokens::ERROR99),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral0",
        value: TokenValue::Color(PaletteTokens::NEUTRAL0),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral10",
        value: TokenValue::Color(PaletteTokens::NEUTRAL10),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral100",
        value: TokenValue::Color(PaletteTokens::NEUTRAL100),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral12",
        value: TokenValue::Color(PaletteTokens::NEUTRAL12),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral17",
        value: TokenValue::Color(PaletteTokens::NEUTRAL17),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral20",
        value: TokenValue::Color(PaletteTokens::NEUTRAL20),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral22",
        value: TokenValue::Color(PaletteTokens::NEUTRAL22),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral24",
        value: TokenValue::Color(PaletteTokens::NEUTRAL24),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral30",
        value: TokenValue::Color(PaletteTokens::NEUTRAL30),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral4",
        value: TokenValue::Color(PaletteTokens::NEUTRAL4),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral40",
        value: TokenValue::Color(PaletteTokens::NEUTRAL40),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral50",
        value: TokenValue::Color(PaletteTokens::NEUTRAL50),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral6",
        value: TokenValue::Color(PaletteTokens::NEUTRAL6),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral60",
        value: TokenValue::Color(PaletteTokens::NEUTRAL60),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral70",
        value: TokenValue::Color(PaletteTokens::NEUTRAL70),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral80",
        value: TokenValue::Color(PaletteTokens::NEUTRAL80),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral87",
        value: TokenValue::Color(PaletteTokens::NEUTRAL87),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral90",
        value: TokenValue::Color(PaletteTokens::NEUTRAL90),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral92",
        value: TokenValue::Color(PaletteTokens::NEUTRAL92),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral94",
        value: TokenValue::Color(PaletteTokens::NEUTRAL94),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral95",
        value: TokenValue::Color(PaletteTokens::NEUTRAL95),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral96",
        value: TokenValue::Color(PaletteTokens::NEUTRAL96),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral98",
        value: TokenValue::Color(PaletteTokens::NEUTRAL98),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Neutral99",
        value: TokenValue::Color(PaletteTokens::NEUTRAL99),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant0",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT0),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant10",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT10),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant100",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT100),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant20",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT20),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant30",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT30),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant40",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT40),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant50",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT50),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant60",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT60),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant70",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT70),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant80",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT80),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant90",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT90),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant95",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT95),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "NeutralVariant99",
        value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT99),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary0",
        value: TokenValue::Color(PaletteTokens::PRIMARY0),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary10",
        value: TokenValue::Color(PaletteTokens::PRIMARY10),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary100",
        value: TokenValue::Color(PaletteTokens::PRIMARY100),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary20",
        value: TokenValue::Color(PaletteTokens::PRIMARY20),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary30",
        value: TokenValue::Color(PaletteTokens::PRIMARY30),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary40",
        value: TokenValue::Color(PaletteTokens::PRIMARY40),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary50",
        value: TokenValue::Color(PaletteTokens::PRIMARY50),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary60",
        value: TokenValue::Color(PaletteTokens::PRIMARY60),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary70",
        value: TokenValue::Color(PaletteTokens::PRIMARY70),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary80",
        value: TokenValue::Color(PaletteTokens::PRIMARY80),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary90",
        value: TokenValue::Color(PaletteTokens::PRIMARY90),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary95",
        value: TokenValue::Color(PaletteTokens::PRIMARY95),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Primary99",
        value: TokenValue::Color(PaletteTokens::PRIMARY99),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary0",
        value: TokenValue::Color(PaletteTokens::SECONDARY0),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary10",
        value: TokenValue::Color(PaletteTokens::SECONDARY10),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary100",
        value: TokenValue::Color(PaletteTokens::SECONDARY100),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary20",
        value: TokenValue::Color(PaletteTokens::SECONDARY20),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary30",
        value: TokenValue::Color(PaletteTokens::SECONDARY30),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary40",
        value: TokenValue::Color(PaletteTokens::SECONDARY40),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary50",
        value: TokenValue::Color(PaletteTokens::SECONDARY50),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary60",
        value: TokenValue::Color(PaletteTokens::SECONDARY60),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary70",
        value: TokenValue::Color(PaletteTokens::SECONDARY70),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary80",
        value: TokenValue::Color(PaletteTokens::SECONDARY80),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary90",
        value: TokenValue::Color(PaletteTokens::SECONDARY90),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary95",
        value: TokenValue::Color(PaletteTokens::SECONDARY95),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Secondary99",
        value: TokenValue::Color(PaletteTokens::SECONDARY99),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary0",
        value: TokenValue::Color(PaletteTokens::TERTIARY0),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary10",
        value: TokenValue::Color(PaletteTokens::TERTIARY10),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary100",
        value: TokenValue::Color(PaletteTokens::TERTIARY100),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary20",
        value: TokenValue::Color(PaletteTokens::TERTIARY20),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary30",
        value: TokenValue::Color(PaletteTokens::TERTIARY30),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary40",
        value: TokenValue::Color(PaletteTokens::TERTIARY40),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary50",
        value: TokenValue::Color(PaletteTokens::TERTIARY50),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary60",
        value: TokenValue::Color(PaletteTokens::TERTIARY60),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary70",
        value: TokenValue::Color(PaletteTokens::TERTIARY70),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary80",
        value: TokenValue::Color(PaletteTokens::TERTIARY80),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary90",
        value: TokenValue::Color(PaletteTokens::TERTIARY90),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary95",
        value: TokenValue::Color(PaletteTokens::TERTIARY95),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "Tertiary99",
        value: TokenValue::Color(PaletteTokens::TERTIARY99),
    },
    TokenEntry {
        group: "PaletteTokens",
        name: "White",
        value: TokenValue::Color(PaletteTokens::WHITE),
    },
    TokenEntry {
        group: "PlainTooltipTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(PlainTooltipTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "PlainTooltipTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(PlainTooltipTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "PlainTooltipTokens",
        name: "SupportingTextColor",
        value: TokenValue::ColorRole(PlainTooltipTokens::SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "PlainTooltipTokens",
        name: "SupportingTextFont",
        value: TokenValue::TypographyRole(PlainTooltipTokens::SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ActiveIndicatorColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ActiveIndicatorHeight",
        value: TokenValue::Dp(PrimaryNavigationTabTokens::ACTIVE_INDICATOR_HEIGHT),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ActiveIndicatorShape",
        value: TokenValue::Shape(PrimaryNavigationTabTokens::ACTIVE_INDICATOR_SHAPE),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(PrimaryNavigationTabTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(PrimaryNavigationTabTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(PrimaryNavigationTabTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ActiveFocusIconColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::ACTIVE_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ActiveHoverIconColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::ACTIVE_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ActiveIconColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::ACTIVE_ICON_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ActivePressedIconColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::ACTIVE_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "IconAndLabelTextContainerHeight",
        value: TokenValue::Dp(PrimaryNavigationTabTokens::ICON_AND_LABEL_TEXT_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "IconSize",
        value: TokenValue::Dp(PrimaryNavigationTabTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "InactiveFocusIconColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::INACTIVE_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "InactiveHoverIconColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::INACTIVE_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "InactiveIconColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::INACTIVE_ICON_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "InactivePressedIconColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::INACTIVE_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ActiveFocusLabelTextColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::ACTIVE_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ActiveHoverLabelTextColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::ACTIVE_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ActiveLabelTextColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::ACTIVE_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "ActivePressedLabelTextColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::ACTIVE_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "InactiveFocusLabelTextColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::INACTIVE_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "InactiveHoverLabelTextColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::INACTIVE_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "InactiveLabelTextColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::INACTIVE_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "InactivePressedLabelTextColor",
        value: TokenValue::ColorRole(PrimaryNavigationTabTokens::INACTIVE_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "PrimaryNavigationTabTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(PrimaryNavigationTabTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "ProgressIndicatorTokens",
        name: "ActiveIndicatorColor",
        value: TokenValue::ColorRole(ProgressIndicatorTokens::ACTIVE_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "ProgressIndicatorTokens",
        name: "ActiveShape",
        value: TokenValue::ShapeRole(ProgressIndicatorTokens::ACTIVE_SHAPE),
    },
    TokenEntry {
        group: "ProgressIndicatorTokens",
        name: "StopColor",
        value: TokenValue::ColorRole(ProgressIndicatorTokens::STOP_COLOR),
    },
    TokenEntry {
        group: "ProgressIndicatorTokens",
        name: "StopShape",
        value: TokenValue::ShapeRole(ProgressIndicatorTokens::STOP_SHAPE),
    },
    TokenEntry {
        group: "ProgressIndicatorTokens",
        name: "TrackColor",
        value: TokenValue::ColorRole(ProgressIndicatorTokens::TRACK_COLOR),
    },
    TokenEntry {
        group: "ProgressIndicatorTokens",
        name: "TrackShape",
        value: TokenValue::ShapeRole(ProgressIndicatorTokens::TRACK_SHAPE),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "DisabledSelectedIconColor",
        value: TokenValue::ColorRole(RadioButtonTokens::DISABLED_SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "DisabledSelectedIconOpacity",
        value: TokenValue::Float(RadioButtonTokens::DISABLED_SELECTED_ICON_OPACITY),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "DisabledUnselectedIconColor",
        value: TokenValue::ColorRole(RadioButtonTokens::DISABLED_UNSELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "DisabledUnselectedIconOpacity",
        value: TokenValue::Float(RadioButtonTokens::DISABLED_UNSELECTED_ICON_OPACITY),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "IconSize",
        value: TokenValue::Dp(RadioButtonTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "SelectedFocusIconColor",
        value: TokenValue::ColorRole(RadioButtonTokens::SELECTED_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "SelectedHoverIconColor",
        value: TokenValue::ColorRole(RadioButtonTokens::SELECTED_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "SelectedIconColor",
        value: TokenValue::ColorRole(RadioButtonTokens::SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "SelectedPressedIconColor",
        value: TokenValue::ColorRole(RadioButtonTokens::SELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "StateLayerSize",
        value: TokenValue::Dp(RadioButtonTokens::STATE_LAYER_SIZE),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "UnselectedFocusIconColor",
        value: TokenValue::ColorRole(RadioButtonTokens::UNSELECTED_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "UnselectedHoverIconColor",
        value: TokenValue::ColorRole(RadioButtonTokens::UNSELECTED_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "UnselectedIconColor",
        value: TokenValue::ColorRole(RadioButtonTokens::UNSELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "RadioButtonTokens",
        name: "UnselectedPressedIconColor",
        value: TokenValue::ColorRole(RadioButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "ReorderListTokens",
        name: "ItemContainerColor",
        value: TokenValue::ColorRole(ReorderListTokens::ITEM_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ReorderListTokens",
        name: "ItemDropZoneColor",
        value: TokenValue::ColorRole(ReorderListTokens::ITEM_DROP_ZONE_COLOR),
    },
    TokenEntry {
        group: "ReorderListTokens",
        name: "ItemLabelTextColor",
        value: TokenValue::ColorRole(ReorderListTokens::ITEM_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "ReorderListTokens",
        name: "ItemLeadingIconColor",
        value: TokenValue::ColorRole(ReorderListTokens::ITEM_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "ReorderListTokens",
        name: "ItemOverlineColor",
        value: TokenValue::ColorRole(ReorderListTokens::ITEM_OVERLINE_COLOR),
    },
    TokenEntry {
        group: "ReorderListTokens",
        name: "ItemShape",
        value: TokenValue::ShapeRole(ReorderListTokens::ITEM_SHAPE),
    },
    TokenEntry {
        group: "ReorderListTokens",
        name: "ItemSupportingTextColor",
        value: TokenValue::ColorRole(ReorderListTokens::ITEM_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "ReorderListTokens",
        name: "ItemTrailingIconColor",
        value: TokenValue::ColorRole(ReorderListTokens::ITEM_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "ReorderListTokens",
        name: "ItemTrailingSupportingTextColor",
        value: TokenValue::ColorRole(ReorderListTokens::ITEM_TRAILING_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "RevealListTokens",
        name: "ItemActionButtonIconIconColor",
        value: TokenValue::ColorRole(RevealListTokens::ITEM_ACTION_BUTTON_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "RevealListTokens",
        name: "ItemActionIconButtonContainerColor",
        value: TokenValue::ColorRole(RevealListTokens::ITEM_ACTION_ICON_BUTTON_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "RevealListTokens",
        name: "ItemButtonIconIconColor",
        value: TokenValue::ColorRole(RevealListTokens::ITEM_BUTTON_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "RevealListTokens",
        name: "ItemContainerColor",
        value: TokenValue::ColorRole(RevealListTokens::ITEM_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "RevealListTokens",
        name: "ItemContainerShape",
        value: TokenValue::ShapeRole(RevealListTokens::ITEM_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "RevealListTokens",
        name: "ItemIconButtonActionContainerShape",
        value: TokenValue::ShapeRole(RevealListTokens::ITEM_ICON_BUTTON_ACTION_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "RevealListTokens",
        name: "ItemIconButtonContainerColor",
        value: TokenValue::ColorRole(RevealListTokens::ITEM_ICON_BUTTON_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "RevealListTokens",
        name: "ItemIconButtonContainerShape",
        value: TokenValue::ShapeRole(RevealListTokens::ITEM_ICON_BUTTON_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "RevealListTokens",
        name: "ItemSegmentedContainerShape",
        value: TokenValue::ShapeRole(RevealListTokens::ITEM_SEGMENTED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "ActionFocusLabelTextColor",
        value: TokenValue::ColorRole(RichTooltipTokens::ACTION_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "ActionHoverLabelTextColor",
        value: TokenValue::ColorRole(RichTooltipTokens::ACTION_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "ActionLabelTextColor",
        value: TokenValue::ColorRole(RichTooltipTokens::ACTION_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "ActionLabelTextFont",
        value: TokenValue::TypographyRole(RichTooltipTokens::ACTION_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "ActionPressedLabelTextColor",
        value: TokenValue::ColorRole(RichTooltipTokens::ACTION_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(RichTooltipTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(RichTooltipTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(RichTooltipTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "SubheadColor",
        value: TokenValue::ColorRole(RichTooltipTokens::SUBHEAD_COLOR),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "SubheadFont",
        value: TokenValue::TypographyRole(RichTooltipTokens::SUBHEAD_FONT),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "SupportingTextColor",
        value: TokenValue::ColorRole(RichTooltipTokens::SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "RichTooltipTokens",
        name: "SupportingTextFont",
        value: TokenValue::TypographyRole(RichTooltipTokens::SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "ScrimTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(ScrimTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "ScrimTokens",
        name: "ContainerOpacity",
        value: TokenValue::Float(ScrimTokens::CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "AvatarShape",
        value: TokenValue::ShapeRole(SearchBarTokens::AVATAR_SHAPE),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "AvatarSize",
        value: TokenValue::Dp(SearchBarTokens::AVATAR_SIZE),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(SearchBarTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(SearchBarTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(SearchBarTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(SearchBarTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(SearchBarTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "HoverSupportingTextColor",
        value: TokenValue::ColorRole(SearchBarTokens::HOVER_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "InputTextColor",
        value: TokenValue::ColorRole(SearchBarTokens::INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "InputTextFont",
        value: TokenValue::TypographyRole(SearchBarTokens::INPUT_TEXT_FONT),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "LeadingIconColor",
        value: TokenValue::ColorRole(SearchBarTokens::LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "PressedSupportingTextColor",
        value: TokenValue::ColorRole(SearchBarTokens::PRESSED_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "SupportingTextColor",
        value: TokenValue::ColorRole(SearchBarTokens::SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "SupportingTextFont",
        value: TokenValue::TypographyRole(SearchBarTokens::SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "SearchBarTokens",
        name: "TrailingIconColor",
        value: TokenValue::ColorRole(SearchBarTokens::TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(SearchViewTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(SearchViewTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "DividerColor",
        value: TokenValue::ColorRole(SearchViewTokens::DIVIDER_COLOR),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "DockedContainerShape",
        value: TokenValue::ShapeRole(SearchViewTokens::DOCKED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "DockedHeaderContainerHeight",
        value: TokenValue::Dp(SearchViewTokens::DOCKED_HEADER_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "FullScreenContainerShape",
        value: TokenValue::ShapeRole(SearchViewTokens::FULL_SCREEN_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "FullScreenHeaderContainerHeight",
        value: TokenValue::Dp(SearchViewTokens::FULL_SCREEN_HEADER_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "HeaderInputTextColor",
        value: TokenValue::ColorRole(SearchViewTokens::HEADER_INPUT_TEXT_COLOR),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "HeaderInputTextFont",
        value: TokenValue::TypographyRole(SearchViewTokens::HEADER_INPUT_TEXT_FONT),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "HeaderLeadingIconColor",
        value: TokenValue::ColorRole(SearchViewTokens::HEADER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "HeaderSupportingTextColor",
        value: TokenValue::ColorRole(SearchViewTokens::HEADER_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "HeaderSupportingTextFont",
        value: TokenValue::TypographyRole(SearchViewTokens::HEADER_SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "SearchViewTokens",
        name: "HeaderTrailingIconColor",
        value: TokenValue::ColorRole(SearchViewTokens::HEADER_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "ActiveLabelTextColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::ACTIVE_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(SecondaryNavigationTabTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(SecondaryNavigationTabTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(SecondaryNavigationTabTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "DividerColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::DIVIDER_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "DividerHeight",
        value: TokenValue::Dp(SecondaryNavigationTabTokens::DIVIDER_HEIGHT),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "FocusLabelTextColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "HoverLabelTextColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "InactiveLabelTextColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::INACTIVE_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(SecondaryNavigationTabTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "PressedLabelTextColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "ActiveIconColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::ACTIVE_ICON_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "FocusIconColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "HoverIconColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "IconSize",
        value: TokenValue::Dp(SecondaryNavigationTabTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "InactiveIconColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::INACTIVE_ICON_COLOR),
    },
    TokenEntry {
        group: "SecondaryNavigationTabTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(SecondaryNavigationTabTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ActiveContainerShape",
        value: TokenValue::Dp(SegmentedMenuTokens::ACTIVE_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(SegmentedMenuTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "GroupContainerColor",
        value: TokenValue::ColorRole(SegmentedMenuTokens::GROUP_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "GroupPadding",
        value: TokenValue::Dp(SegmentedMenuTokens::GROUP_PADDING),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "GroupShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::GROUP_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalContainerBottomSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_CONTAINER_BOTTOM_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalContainerTopSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_CONTAINER_TOP_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalIconOnlyItemBottomSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_ITEM_BOTTOM_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalIconOnlyItemLeadingSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_ITEM_LEADING_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalIconOnlyItemSelectedShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_ITEM_SELECTED_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalIconOnlyItemTopSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_ITEM_TOP_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalIconOnlyItemTrailingSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_ITEM_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalIconOnlySegmentedGap",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_SEGMENTED_GAP),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalItemBetweenSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ITEM_BETWEEN_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalItemBottomSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ITEM_BOTTOM_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalItemFocusedShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::HORIZONTAL_ITEM_FOCUSED_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalItemHoveredShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::HORIZONTAL_ITEM_HOVERED_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalItemLeadingSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ITEM_LEADING_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalItemPressedShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::HORIZONTAL_ITEM_PRESSED_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalItemSelectedFocusedShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::HORIZONTAL_ITEM_SELECTED_FOCUSED_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalItemSelectedHoveredShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::HORIZONTAL_ITEM_SELECTED_HOVERED_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalItemSelectedPressedShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::HORIZONTAL_ITEM_SELECTED_PRESSED_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalItemTopSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ITEM_TOP_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalItemTrailingSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ITEM_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "HorizontalSegmentedGap",
        value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_SEGMENTED_GAP),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "InactiveContainerShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::INACTIVE_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "Item",
        value: TokenValue::Dp(SegmentedMenuTokens::ITEM),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemBetweenSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::ITEM_BETWEEN_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemBottomSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::ITEM_BOTTOM_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemFirstChildInnerCornerCornerSize",
        value: TokenValue::ShapeRole(
            SegmentedMenuTokens::ITEM_FIRST_CHILD_INNER_CORNER_CORNER_SIZE,
        ),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemFirstChildShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::ITEM_FIRST_CHILD_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemFocusIndicatorColor",
        value: TokenValue::ColorRole(SegmentedMenuTokens::ITEM_FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemLabelTextFont",
        value: TokenValue::TypographyRole(SegmentedMenuTokens::ITEM_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemLastChildInnerCornerCornerSize",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::ITEM_LAST_CHILD_INNER_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemLastChildShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::ITEM_LAST_CHILD_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemLeadingIconSize",
        value: TokenValue::Dp(SegmentedMenuTokens::ITEM_LEADING_ICON_SIZE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemLeadingSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::ITEM_LEADING_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemSelectedShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::ITEM_SELECTED_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemShape",
        value: TokenValue::ShapeRole(SegmentedMenuTokens::ITEM_SHAPE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemSupportingTextFont",
        value: TokenValue::TypographyRole(SegmentedMenuTokens::ITEM_SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemTopSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::ITEM_TOP_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemTrailingIconSize",
        value: TokenValue::Dp(SegmentedMenuTokens::ITEM_TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemTrailingSpace",
        value: TokenValue::Dp(SegmentedMenuTokens::ITEM_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "ItemTrailingSupportingTextFont",
        value: TokenValue::TypographyRole(SegmentedMenuTokens::ITEM_TRAILING_SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "SegmentedMenuTokens",
        name: "SegmentedGap",
        value: TokenValue::Dp(SegmentedMenuTokens::SEGMENTED_GAP),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerExtraExtraLarge",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_EXTRA_LARGE),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerExtraLarge",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_LARGE),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerExtraLargeIncreased",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_LARGE_INCREASED),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerExtraLargeTop",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_LARGE_TOP),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerExtraSmall",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_SMALL),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerExtraSmallTop",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_SMALL_TOP),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerFull",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_FULL),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerLarge",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_LARGE),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerLargeEnd",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_LARGE_END),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerLargeIncreased",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_LARGE_INCREASED),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerLargeStart",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_LARGE_START),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerLargeTop",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_LARGE_TOP),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerMedium",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_MEDIUM),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerNone",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_NONE),
    },
    TokenEntry {
        group: "ShapeKeyTokens",
        name: "CornerSmall",
        value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_SMALL),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerExtraExtraLarge",
        value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_EXTRA_LARGE),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerExtraLarge",
        value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_LARGE),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerExtraLargeIncreased",
        value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_LARGE_INCREASED),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerExtraLargeTop",
        value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_LARGE_TOP),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerExtraSmall",
        value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_SMALL),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerExtraSmallTop",
        value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_SMALL_TOP),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerFull",
        value: TokenValue::Shape(ShapeTokens::CORNER_FULL),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerLarge",
        value: TokenValue::Shape(ShapeTokens::CORNER_LARGE),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerLargeEnd",
        value: TokenValue::Shape(ShapeTokens::CORNER_LARGE_END),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerLargeIncreased",
        value: TokenValue::Shape(ShapeTokens::CORNER_LARGE_INCREASED),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerLargeStart",
        value: TokenValue::Shape(ShapeTokens::CORNER_LARGE_START),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerLargeTop",
        value: TokenValue::Shape(ShapeTokens::CORNER_LARGE_TOP),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerMedium",
        value: TokenValue::Shape(ShapeTokens::CORNER_MEDIUM),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerNone",
        value: TokenValue::Shape(ShapeTokens::CORNER_NONE),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerSmall",
        value: TokenValue::Shape(ShapeTokens::CORNER_SMALL),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerValueExtraExtraLarge",
        value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_EXTRA_EXTRA_LARGE),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerValueExtraLarge",
        value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_EXTRA_LARGE),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerValueExtraLargeIncreased",
        value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_EXTRA_LARGE_INCREASED),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerValueExtraSmall",
        value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_EXTRA_SMALL),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerValueLarge",
        value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_LARGE),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerValueLargeIncreased",
        value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_LARGE_INCREASED),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerValueMedium",
        value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_MEDIUM),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerValueNone",
        value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_NONE),
    },
    TokenEntry {
        group: "ShapeTokens",
        name: "CornerValueSmall",
        value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_SMALL),
    },
    TokenEntry {
        group: "SheetBottomTokens",
        name: "DockedContainerColor",
        value: TokenValue::ColorRole(SheetBottomTokens::DOCKED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "SheetBottomTokens",
        name: "DockedContainerShape",
        value: TokenValue::ShapeRole(SheetBottomTokens::DOCKED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SheetBottomTokens",
        name: "DockedDragHandleColor",
        value: TokenValue::ColorRole(SheetBottomTokens::DOCKED_DRAG_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SheetBottomTokens",
        name: "DockedDragHandleHeight",
        value: TokenValue::Dp(SheetBottomTokens::DOCKED_DRAG_HANDLE_HEIGHT),
    },
    TokenEntry {
        group: "SheetBottomTokens",
        name: "DockedDragHandleWidth",
        value: TokenValue::Dp(SheetBottomTokens::DOCKED_DRAG_HANDLE_WIDTH),
    },
    TokenEntry {
        group: "SheetBottomTokens",
        name: "DockedMinimizedContainerShape",
        value: TokenValue::ShapeRole(SheetBottomTokens::DOCKED_MINIMIZED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SheetBottomTokens",
        name: "DockedModalContainerElevation",
        value: TokenValue::Dp(SheetBottomTokens::DOCKED_MODAL_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SheetBottomTokens",
        name: "DockedStandardContainerElevation",
        value: TokenValue::Dp(SheetBottomTokens::DOCKED_STANDARD_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SheetBottomTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(SheetBottomTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ActiveContainerOpacity",
        value: TokenValue::Float(SliderTokens::ACTIVE_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ActiveHandleHeight",
        value: TokenValue::Dp(SliderTokens::ACTIVE_HANDLE_HEIGHT),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ActiveHandleLeadingSpace",
        value: TokenValue::Dp(SliderTokens::ACTIVE_HANDLE_LEADING_SPACE),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ActiveHandlePadding",
        value: TokenValue::Dp(SliderTokens::ACTIVE_HANDLE_PADDING),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ActiveHandleShape",
        value: TokenValue::ShapeRole(SliderTokens::ACTIVE_HANDLE_SHAPE),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ActiveHandleTrailingSpace",
        value: TokenValue::Dp(SliderTokens::ACTIVE_HANDLE_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ActiveHandleWidth",
        value: TokenValue::Dp(SliderTokens::ACTIVE_HANDLE_WIDTH),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ActiveTrackColor",
        value: TokenValue::ColorRole(SliderTokens::ACTIVE_TRACK_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ActiveTrackHeight",
        value: TokenValue::Dp(SliderTokens::ACTIVE_TRACK_HEIGHT),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ActiveTrackShape",
        value: TokenValue::ShapeRole(SliderTokens::ACTIVE_TRACK_SHAPE),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ActiveTrackShapeLeading",
        value: TokenValue::ShapeRole(SliderTokens::ACTIVE_TRACK_SHAPE_LEADING),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "DisabledActiveTrackColor",
        value: TokenValue::ColorRole(SliderTokens::DISABLED_ACTIVE_TRACK_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "DisabledActiveTrackOpacity",
        value: TokenValue::Float(SliderTokens::DISABLED_ACTIVE_TRACK_OPACITY),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "DisabledHandleColor",
        value: TokenValue::ColorRole(SliderTokens::DISABLED_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "DisabledHandleOpacity",
        value: TokenValue::Float(SliderTokens::DISABLED_HANDLE_OPACITY),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "DisabledHandleWidth",
        value: TokenValue::Dp(SliderTokens::DISABLED_HANDLE_WIDTH),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "DisabledInactiveTrackColor",
        value: TokenValue::ColorRole(SliderTokens::DISABLED_INACTIVE_TRACK_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "DisabledInactiveTrackOpacity",
        value: TokenValue::Float(SliderTokens::DISABLED_INACTIVE_TRACK_OPACITY),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "DisabledStopColor",
        value: TokenValue::ColorRole(SliderTokens::DISABLED_STOP_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "FocusActiveTrackColor",
        value: TokenValue::ColorRole(SliderTokens::FOCUS_ACTIVE_TRACK_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "FocusHandleWidth",
        value: TokenValue::Dp(SliderTokens::FOCUS_HANDLE_WIDTH),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "FocusInactiveTrackColor",
        value: TokenValue::ColorRole(SliderTokens::FOCUS_INACTIVE_TRACK_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "FocusStopColor",
        value: TokenValue::ColorRole(SliderTokens::FOCUS_STOP_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "HandleColor",
        value: TokenValue::ColorRole(SliderTokens::HANDLE_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "HandleHeight",
        value: TokenValue::Dp(SliderTokens::HANDLE_HEIGHT),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "HandleShape",
        value: TokenValue::ShapeRole(SliderTokens::HANDLE_SHAPE),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "HandleWidth",
        value: TokenValue::Dp(SliderTokens::HANDLE_WIDTH),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "HoverHandleColor",
        value: TokenValue::ColorRole(SliderTokens::HOVER_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "HoverHandleWidth",
        value: TokenValue::Dp(SliderTokens::HOVER_HANDLE_WIDTH),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "HoverStopColor",
        value: TokenValue::ColorRole(SliderTokens::HOVER_STOP_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "InactiveContainerOpacity",
        value: TokenValue::Float(SliderTokens::INACTIVE_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "InactiveTrackColor",
        value: TokenValue::ColorRole(SliderTokens::INACTIVE_TRACK_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "InactiveTrackHeight",
        value: TokenValue::Dp(SliderTokens::INACTIVE_TRACK_HEIGHT),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "InactiveTrackShape",
        value: TokenValue::ShapeRole(SliderTokens::INACTIVE_TRACK_SHAPE),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "LabelContainerColor",
        value: TokenValue::ColorRole(SliderTokens::LABEL_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "LabelTextColor",
        value: TokenValue::ColorRole(SliderTokens::LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "PressedActiveTrackColor",
        value: TokenValue::ColorRole(SliderTokens::PRESSED_ACTIVE_TRACK_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "PressedHandleColor",
        value: TokenValue::ColorRole(SliderTokens::PRESSED_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "PressedHandleWidth",
        value: TokenValue::Dp(SliderTokens::PRESSED_HANDLE_WIDTH),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "PressedInactiveTrackColor",
        value: TokenValue::ColorRole(SliderTokens::PRESSED_INACTIVE_TRACK_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "PressedStopColor",
        value: TokenValue::ColorRole(SliderTokens::PRESSED_STOP_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "SliderActiveHandleColor",
        value: TokenValue::ColorRole(SliderTokens::SLIDER_ACTIVE_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "StopIndicatorColor",
        value: TokenValue::ColorRole(SliderTokens::STOP_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "StopIndicatorColorSelected",
        value: TokenValue::ColorRole(SliderTokens::STOP_INDICATOR_COLOR_SELECTED),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "StopIndicatorShape",
        value: TokenValue::ShapeRole(SliderTokens::STOP_INDICATOR_SHAPE),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "StopIndicatorSize",
        value: TokenValue::Dp(SliderTokens::STOP_INDICATOR_SIZE),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "StopIndicatorTrailingSpace",
        value: TokenValue::Dp(SliderTokens::STOP_INDICATOR_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ValueIndicatorActiveBottomSpace",
        value: TokenValue::Dp(SliderTokens::VALUE_INDICATOR_ACTIVE_BOTTOM_SPACE),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ValueIndicatorContainerColor",
        value: TokenValue::ColorRole(SliderTokens::VALUE_INDICATOR_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ValueIndicatorLabelTextColor",
        value: TokenValue::ColorRole(SliderTokens::VALUE_INDICATOR_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SliderTokens",
        name: "ValueIndicatorLabelTextFont",
        value: TokenValue::TypographyRole(SliderTokens::VALUE_INDICATOR_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(SmallIconButtonTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "ContainerShapeRound",
        value: TokenValue::ShapeRole(SmallIconButtonTokens::CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "ContainerShapeSquare",
        value: TokenValue::ShapeRole(SmallIconButtonTokens::CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "DefaultLeadingSpace",
        value: TokenValue::Dp(SmallIconButtonTokens::DEFAULT_LEADING_SPACE),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "DefaultTrailingSpace",
        value: TokenValue::Dp(SmallIconButtonTokens::DEFAULT_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "IconSize",
        value: TokenValue::Dp(SmallIconButtonTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "NarrowLeadingSpace",
        value: TokenValue::Dp(SmallIconButtonTokens::NARROW_LEADING_SPACE),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "NarrowTrailingSpace",
        value: TokenValue::Dp(SmallIconButtonTokens::NARROW_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "OutlinedOutlineWidth",
        value: TokenValue::Dp(SmallIconButtonTokens::OUTLINED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "PressedContainerShape",
        value: TokenValue::ShapeRole(SmallIconButtonTokens::PRESSED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "SelectedContainerShapeRound",
        value: TokenValue::ShapeRole(SmallIconButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "SelectedContainerShapeSquare",
        value: TokenValue::ShapeRole(SmallIconButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "WideLeadingSpace",
        value: TokenValue::Dp(SmallIconButtonTokens::WIDE_LEADING_SPACE),
    },
    TokenEntry {
        group: "SmallIconButtonTokens",
        name: "WideTrailingSpace",
        value: TokenValue::Dp(SmallIconButtonTokens::WIDE_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "ActionFocusLabelTextColor",
        value: TokenValue::ColorRole(SnackbarTokens::ACTION_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "ActionHoverLabelTextColor",
        value: TokenValue::ColorRole(SnackbarTokens::ACTION_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "ActionLabelTextColor",
        value: TokenValue::ColorRole(SnackbarTokens::ACTION_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "ActionLabelTextFont",
        value: TokenValue::TypographyRole(SnackbarTokens::ACTION_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "ActionPressedLabelTextColor",
        value: TokenValue::ColorRole(SnackbarTokens::ACTION_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(SnackbarTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(SnackbarTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(SnackbarTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(SnackbarTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "FocusIconColor",
        value: TokenValue::ColorRole(SnackbarTokens::FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "HoverIconColor",
        value: TokenValue::ColorRole(SnackbarTokens::HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(SnackbarTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "IconSize",
        value: TokenValue::Dp(SnackbarTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "SupportingTextColor",
        value: TokenValue::ColorRole(SnackbarTokens::SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "SupportingTextFont",
        value: TokenValue::TypographyRole(SnackbarTokens::SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "SingleLineContainerHeight",
        value: TokenValue::Dp(SnackbarTokens::SINGLE_LINE_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SnackbarTokens",
        name: "TwoLinesContainerHeight",
        value: TokenValue::Dp(SnackbarTokens::TWO_LINES_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "BetweenSpace",
        value: TokenValue::Dp(SplitButtonLargeTokens::BETWEEN_SPACE),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(SplitButtonLargeTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(SplitButtonLargeTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "InnerCornerCornerSize",
        value: TokenValue::Dp(SplitButtonLargeTokens::INNER_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "InnerHoveredCornerCornerSize",
        value: TokenValue::Dp(SplitButtonLargeTokens::INNER_HOVERED_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "InnerPressedCornerCornerSize",
        value: TokenValue::Dp(SplitButtonLargeTokens::INNER_PRESSED_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "LeadingButtonLeadingSpace",
        value: TokenValue::Dp(SplitButtonLargeTokens::LEADING_BUTTON_LEADING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "LeadingButtonTrailingSpace",
        value: TokenValue::Dp(SplitButtonLargeTokens::LEADING_BUTTON_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "TrailingIconSize",
        value: TokenValue::Dp(SplitButtonLargeTokens::TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "TrailingInnerSelectedCornerCornerSizePercent",
        value: TokenValue::Float(
            SplitButtonLargeTokens::TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT,
        ),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "TrailingButtonLeadingSpace",
        value: TokenValue::Dp(SplitButtonLargeTokens::TRAILING_BUTTON_LEADING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonLargeTokens",
        name: "TrailingButtonTrailingSpace",
        value: TokenValue::Dp(SplitButtonLargeTokens::TRAILING_BUTTON_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "BetweenSpace",
        value: TokenValue::Dp(SplitButtonMediumTokens::BETWEEN_SPACE),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(SplitButtonMediumTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(SplitButtonMediumTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "InnerCornerCornerSize",
        value: TokenValue::Dp(SplitButtonMediumTokens::INNER_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "InnerHoveredCornerCornerSize",
        value: TokenValue::Dp(SplitButtonMediumTokens::INNER_HOVERED_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "InnerPressedCornerCornerSize",
        value: TokenValue::Dp(SplitButtonMediumTokens::INNER_PRESSED_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "LeadingButtonLeadingSpace",
        value: TokenValue::Dp(SplitButtonMediumTokens::LEADING_BUTTON_LEADING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "LeadingButtonTrailingSpace",
        value: TokenValue::Dp(SplitButtonMediumTokens::LEADING_BUTTON_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "TrailingIconSize",
        value: TokenValue::Dp(SplitButtonMediumTokens::TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "TrailingInnerSelectedCornerCornerSizePercent",
        value: TokenValue::Float(
            SplitButtonMediumTokens::TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT,
        ),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "TrailingButtonLeadingSpace",
        value: TokenValue::Dp(SplitButtonMediumTokens::TRAILING_BUTTON_LEADING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonMediumTokens",
        name: "TrailingButtonTrailingSpace",
        value: TokenValue::Dp(SplitButtonMediumTokens::TRAILING_BUTTON_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "BetweenSpace",
        value: TokenValue::Dp(SplitButtonSmallTokens::BETWEEN_SPACE),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(SplitButtonSmallTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(SplitButtonSmallTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "InnerCornerCornerSize",
        value: TokenValue::Dp(SplitButtonSmallTokens::INNER_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "InnerHoveredCornerCornerSize",
        value: TokenValue::Dp(SplitButtonSmallTokens::INNER_HOVERED_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "InnerPressedCornerCornerSize",
        value: TokenValue::Dp(SplitButtonSmallTokens::INNER_PRESSED_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "LeadingButtonLeadingSpace",
        value: TokenValue::Dp(SplitButtonSmallTokens::LEADING_BUTTON_LEADING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "LeadingButtonTrailingSpace",
        value: TokenValue::Dp(SplitButtonSmallTokens::LEADING_BUTTON_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "TrailingIconSize",
        value: TokenValue::Dp(SplitButtonSmallTokens::TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "TrailingInnerSelectedCornerCornerSizePercent",
        value: TokenValue::Float(
            SplitButtonSmallTokens::TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT,
        ),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "TrailingButtonLeadingSpace",
        value: TokenValue::Dp(SplitButtonSmallTokens::TRAILING_BUTTON_LEADING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonSmallTokens",
        name: "TrailingButtonTrailingSpace",
        value: TokenValue::Dp(SplitButtonSmallTokens::TRAILING_BUTTON_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "BetweenSpace",
        value: TokenValue::Dp(SplitButtonXLargeTokens::BETWEEN_SPACE),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(SplitButtonXLargeTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(SplitButtonXLargeTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "InnerCornerCornerSize",
        value: TokenValue::Dp(SplitButtonXLargeTokens::INNER_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "InnerHoveredCornerCornerSize",
        value: TokenValue::Dp(SplitButtonXLargeTokens::INNER_HOVERED_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "InnerPressedCornerCornerSize",
        value: TokenValue::Dp(SplitButtonXLargeTokens::INNER_PRESSED_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "LeadingButtonLeadingSpace",
        value: TokenValue::Dp(SplitButtonXLargeTokens::LEADING_BUTTON_LEADING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "LeadingButtonTrailingSpace",
        value: TokenValue::Dp(SplitButtonXLargeTokens::LEADING_BUTTON_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "TrailingIconSize",
        value: TokenValue::Dp(SplitButtonXLargeTokens::TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "TrailingInnerSelectedCornerCornerSizePercent",
        value: TokenValue::Float(
            SplitButtonXLargeTokens::TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT,
        ),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "TrailingButtonLeadingSpace",
        value: TokenValue::Dp(SplitButtonXLargeTokens::TRAILING_BUTTON_LEADING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonXLargeTokens",
        name: "TrailingButtonTrailingSpace",
        value: TokenValue::Dp(SplitButtonXLargeTokens::TRAILING_BUTTON_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "BetweenSpace",
        value: TokenValue::Dp(SplitButtonXSmallTokens::BETWEEN_SPACE),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(SplitButtonXSmallTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(SplitButtonXSmallTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "InnerCornerCornerSize",
        value: TokenValue::Dp(SplitButtonXSmallTokens::INNER_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "InnerHoveredCornerCornerSize",
        value: TokenValue::Dp(SplitButtonXSmallTokens::INNER_HOVERED_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "InnerPressedCornerCornerSize",
        value: TokenValue::Dp(SplitButtonXSmallTokens::INNER_PRESSED_CORNER_CORNER_SIZE),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "LeadingButtonLeadingSpace",
        value: TokenValue::Dp(SplitButtonXSmallTokens::LEADING_BUTTON_LEADING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "LeadingButtonTrailingSpace",
        value: TokenValue::Dp(SplitButtonXSmallTokens::LEADING_BUTTON_TRAILING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "OuterCornerCornerSizePercent",
        value: TokenValue::Float(SplitButtonXSmallTokens::OUTER_CORNER_CORNER_SIZE_PERCENT),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "TrailingIconSize",
        value: TokenValue::Dp(SplitButtonXSmallTokens::TRAILING_ICON_SIZE),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "TrailingInnerSelectedCornerCornerSizePercent",
        value: TokenValue::Float(
            SplitButtonXSmallTokens::TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT,
        ),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "TrailingButtonLeadingSpace",
        value: TokenValue::Dp(SplitButtonXSmallTokens::TRAILING_BUTTON_LEADING_SPACE),
    },
    TokenEntry {
        group: "SplitButtonXSmallTokens",
        name: "TrailingButtonTrailingSpace",
        value: TokenValue::Dp(SplitButtonXSmallTokens::TRAILING_BUTTON_TRAILING_SPACE),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ButtonDisabledIconIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::BUTTON_DISABLED_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ButtonIconIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::BUTTON_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ButtonSelectedIconIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::BUTTON_SELECTED_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(StandardMenuTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "DisabledButtonIconIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::DISABLED_BUTTON_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "IconButtonContainerColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ICON_BUTTON_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "IconButtonSelectedContainerColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ICON_BUTTON_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemContainerColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemDisabledLabelTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemDisabledLabelTextOpacity",
        value: TokenValue::Float(StandardMenuTokens::ITEM_DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemDisabledLeadingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_DISABLED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemDisabledLeadingIconOpacity",
        value: TokenValue::Float(StandardMenuTokens::ITEM_DISABLED_LEADING_ICON_OPACITY),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemDisabledSupportingTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_DISABLED_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemDisabledSupportingTextOpacity",
        value: TokenValue::Float(StandardMenuTokens::ITEM_DISABLED_SUPPORTING_TEXT_OPACITY),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemDisabledTrailingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_DISABLED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemDisabledTrailingIconOpacity",
        value: TokenValue::Float(StandardMenuTokens::ITEM_DISABLED_TRAILING_ICON_OPACITY),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemDisabledTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            StandardMenuTokens::ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemDisabledTrailingSupportingTextOpacity",
        value: TokenValue::Float(
            StandardMenuTokens::ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY,
        ),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemFocusedLabelTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemFocusedLeadingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_FOCUSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemFocusedTrailingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_FOCUSED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemHoveredLabelTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemHoveredLeadingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_HOVERED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemHoveredTrailingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_HOVERED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemLabelTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemLeadingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemPressedLabelTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemPressedLeadingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_PRESSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemPressedTrailingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_PRESSED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedContainerColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedDisabledContainerColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedDisabledContainerOpacity",
        value: TokenValue::Float(StandardMenuTokens::ITEM_SELECTED_DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedDisabledLabelTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedDisabledLabelTextOpacity",
        value: TokenValue::Float(StandardMenuTokens::ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedDisabledLeadingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_DISABLED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedDisabledLeadingIconOpacity",
        value: TokenValue::Float(StandardMenuTokens::ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedDisabledTrailingIconColor",
        value: TokenValue::ColorRole(
            StandardMenuTokens::ITEM_SELECTED_DISABLED_TRAILING_ICON_COLOR,
        ),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedDisabledTrailingIconOpacity",
        value: TokenValue::Float(StandardMenuTokens::ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedDisabledTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            StandardMenuTokens::ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedFocusedLeadingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_FOCUSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedFocusedSupportingTextColor",
        value: TokenValue::ColorRole(
            StandardMenuTokens::ITEM_SELECTED_FOCUSED_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedFocusedTrailingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_FOCUSED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedFocusedTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            StandardMenuTokens::ITEM_SELECTED_FOCUSED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedHoveredLeadingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_HOVERED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedHoveredSupportingTextColor",
        value: TokenValue::ColorRole(
            StandardMenuTokens::ITEM_SELECTED_HOVERED_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedHoveredTrailingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_HOVERED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedHoveredTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            StandardMenuTokens::ITEM_SELECTED_HOVERED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedLabelTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedLeadingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedPressedLeadingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_PRESSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedPressedSupportingTextColor",
        value: TokenValue::ColorRole(
            StandardMenuTokens::ITEM_SELECTED_PRESSED_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedPressedTrailingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_PRESSED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedPressedTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            StandardMenuTokens::ITEM_SELECTED_PRESSED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedSupportingTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedTrailingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSelectedTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            StandardMenuTokens::ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemSupportingTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemTrailingIconColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "StandardMenuTokens",
        name: "ItemTrailingSupportingTextColor",
        value: TokenValue::ColorRole(StandardMenuTokens::ITEM_TRAILING_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringDefaultSpatialDamping",
        value: TokenValue::Float(StandardMotionTokens::SPRING_DEFAULT_SPATIAL_DAMPING),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringDefaultSpatialStiffness",
        value: TokenValue::Float(StandardMotionTokens::SPRING_DEFAULT_SPATIAL_STIFFNESS),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringDefaultEffectsDamping",
        value: TokenValue::Float(StandardMotionTokens::SPRING_DEFAULT_EFFECTS_DAMPING),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringDefaultEffectsStiffness",
        value: TokenValue::Float(StandardMotionTokens::SPRING_DEFAULT_EFFECTS_STIFFNESS),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringFastSpatialDamping",
        value: TokenValue::Float(StandardMotionTokens::SPRING_FAST_SPATIAL_DAMPING),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringFastSpatialStiffness",
        value: TokenValue::Float(StandardMotionTokens::SPRING_FAST_SPATIAL_STIFFNESS),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringFastEffectsDamping",
        value: TokenValue::Float(StandardMotionTokens::SPRING_FAST_EFFECTS_DAMPING),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringFastEffectsStiffness",
        value: TokenValue::Float(StandardMotionTokens::SPRING_FAST_EFFECTS_STIFFNESS),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringSlowSpatialDamping",
        value: TokenValue::Float(StandardMotionTokens::SPRING_SLOW_SPATIAL_DAMPING),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringSlowSpatialStiffness",
        value: TokenValue::Float(StandardMotionTokens::SPRING_SLOW_SPATIAL_STIFFNESS),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringSlowEffectsDamping",
        value: TokenValue::Float(StandardMotionTokens::SPRING_SLOW_EFFECTS_DAMPING),
    },
    TokenEntry {
        group: "StandardMotionTokens",
        name: "SpringSlowEffectsStiffness",
        value: TokenValue::Float(StandardMotionTokens::SPRING_SLOW_EFFECTS_STIFFNESS),
    },
    TokenEntry {
        group: "StateTokens",
        name: "DraggedStateLayerOpacity",
        value: TokenValue::Float(StateTokens::DRAGGED_STATE_LAYER_OPACITY),
    },
    TokenEntry {
        group: "StateTokens",
        name: "FocusStateLayerOpacity",
        value: TokenValue::Float(StateTokens::FOCUS_STATE_LAYER_OPACITY),
    },
    TokenEntry {
        group: "StateTokens",
        name: "HoverStateLayerOpacity",
        value: TokenValue::Float(StateTokens::HOVER_STATE_LAYER_OPACITY),
    },
    TokenEntry {
        group: "StateTokens",
        name: "PressedStateLayerOpacity",
        value: TokenValue::Float(StateTokens::PRESSED_STATE_LAYER_OPACITY),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(SuggestionChipTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(SuggestionChipTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "DisabledLabelTextOpacity",
        value: TokenValue::Float(SuggestionChipTokens::DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "DraggedContainerElevation",
        value: TokenValue::Dp(SuggestionChipTokens::DRAGGED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "DraggedLabelTextColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::DRAGGED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "ElevatedContainerColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::ELEVATED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "ElevatedContainerElevation",
        value: TokenValue::Dp(SuggestionChipTokens::ELEVATED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "ElevatedDisabledContainerColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::ELEVATED_DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "ElevatedDisabledContainerElevation",
        value: TokenValue::Dp(SuggestionChipTokens::ELEVATED_DISABLED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "ElevatedDisabledContainerOpacity",
        value: TokenValue::Float(SuggestionChipTokens::ELEVATED_DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "ElevatedFocusContainerElevation",
        value: TokenValue::Dp(SuggestionChipTokens::ELEVATED_FOCUS_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "ElevatedHoverContainerElevation",
        value: TokenValue::Dp(SuggestionChipTokens::ELEVATED_HOVER_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "ElevatedPressedContainerElevation",
        value: TokenValue::Dp(SuggestionChipTokens::ELEVATED_PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "FlatContainerElevation",
        value: TokenValue::Dp(SuggestionChipTokens::FLAT_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "FlatDisabledOutlineColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::FLAT_DISABLED_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "FlatDisabledOutlineOpacity",
        value: TokenValue::Float(SuggestionChipTokens::FLAT_DISABLED_OUTLINE_OPACITY),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "FlatFocusOutlineColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::FLAT_FOCUS_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "FlatOutlineColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::FLAT_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "FlatOutlineWidth",
        value: TokenValue::Dp(SuggestionChipTokens::FLAT_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "FocusLabelTextColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "HoverLabelTextColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "LabelTextColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "LabelTextFont",
        value: TokenValue::TypographyRole(SuggestionChipTokens::LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "PressedLabelTextColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "DisabledLeadingIconColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::DISABLED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "DisabledLeadingIconOpacity",
        value: TokenValue::Float(SuggestionChipTokens::DISABLED_LEADING_ICON_OPACITY),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "DraggedLeadingIconColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::DRAGGED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "FocusLeadingIconColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::FOCUS_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "HoverLeadingIconColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::HOVER_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "LeadingIconColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "LeadingIconSize",
        value: TokenValue::Dp(SuggestionChipTokens::LEADING_ICON_SIZE),
    },
    TokenEntry {
        group: "SuggestionChipTokens",
        name: "PressedLeadingIconColor",
        value: TokenValue::ColorRole(SuggestionChipTokens::PRESSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledSelectedHandleColor",
        value: TokenValue::ColorRole(SwitchTokens::DISABLED_SELECTED_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledSelectedHandleOpacity",
        value: TokenValue::Float(SwitchTokens::DISABLED_SELECTED_HANDLE_OPACITY),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledSelectedIconColor",
        value: TokenValue::ColorRole(SwitchTokens::DISABLED_SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledSelectedIconOpacity",
        value: TokenValue::Float(SwitchTokens::DISABLED_SELECTED_ICON_OPACITY),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledSelectedTrackColor",
        value: TokenValue::ColorRole(SwitchTokens::DISABLED_SELECTED_TRACK_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledTrackOpacity",
        value: TokenValue::Float(SwitchTokens::DISABLED_TRACK_OPACITY),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledUnselectedHandleColor",
        value: TokenValue::ColorRole(SwitchTokens::DISABLED_UNSELECTED_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledUnselectedHandleOpacity",
        value: TokenValue::Float(SwitchTokens::DISABLED_UNSELECTED_HANDLE_OPACITY),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledUnselectedIconColor",
        value: TokenValue::ColorRole(SwitchTokens::DISABLED_UNSELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledUnselectedIconOpacity",
        value: TokenValue::Float(SwitchTokens::DISABLED_UNSELECTED_ICON_OPACITY),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledUnselectedTrackColor",
        value: TokenValue::ColorRole(SwitchTokens::DISABLED_UNSELECTED_TRACK_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "DisabledUnselectedTrackOutlineColor",
        value: TokenValue::ColorRole(SwitchTokens::DISABLED_UNSELECTED_TRACK_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(SwitchTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "HandleShape",
        value: TokenValue::ShapeRole(SwitchTokens::HANDLE_SHAPE),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "PressedHandleHeight",
        value: TokenValue::Dp(SwitchTokens::PRESSED_HANDLE_HEIGHT),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "PressedHandleWidth",
        value: TokenValue::Dp(SwitchTokens::PRESSED_HANDLE_WIDTH),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedFocusHandleColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_FOCUS_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedFocusIconColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedFocusTrackColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_FOCUS_TRACK_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedHandleColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedHandleHeight",
        value: TokenValue::Dp(SwitchTokens::SELECTED_HANDLE_HEIGHT),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedHandleWidth",
        value: TokenValue::Dp(SwitchTokens::SELECTED_HANDLE_WIDTH),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedHoverHandleColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_HOVER_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedHoverIconColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedHoverTrackColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_HOVER_TRACK_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedIconColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedIconSize",
        value: TokenValue::Dp(SwitchTokens::SELECTED_ICON_SIZE),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedPressedHandleColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_PRESSED_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedPressedIconColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedPressedTrackColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_PRESSED_TRACK_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "SelectedTrackColor",
        value: TokenValue::ColorRole(SwitchTokens::SELECTED_TRACK_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "StateLayerShape",
        value: TokenValue::ShapeRole(SwitchTokens::STATE_LAYER_SHAPE),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "StateLayerSize",
        value: TokenValue::Dp(SwitchTokens::STATE_LAYER_SIZE),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "TrackHeight",
        value: TokenValue::Dp(SwitchTokens::TRACK_HEIGHT),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "TrackOutlineWidth",
        value: TokenValue::Dp(SwitchTokens::TRACK_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "TrackShape",
        value: TokenValue::ShapeRole(SwitchTokens::TRACK_SHAPE),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "TrackWidth",
        value: TokenValue::Dp(SwitchTokens::TRACK_WIDTH),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedFocusHandleColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_FOCUS_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedFocusIconColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_FOCUS_ICON_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedFocusTrackColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_FOCUS_TRACK_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedFocusTrackOutlineColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_FOCUS_TRACK_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedHandleColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedHandleHeight",
        value: TokenValue::Dp(SwitchTokens::UNSELECTED_HANDLE_HEIGHT),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedHandleWidth",
        value: TokenValue::Dp(SwitchTokens::UNSELECTED_HANDLE_WIDTH),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedHoverHandleColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_HOVER_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedHoverIconColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_HOVER_ICON_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedHoverTrackColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_HOVER_TRACK_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedHoverTrackOutlineColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_HOVER_TRACK_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedIconColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedIconSize",
        value: TokenValue::Dp(SwitchTokens::UNSELECTED_ICON_SIZE),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedPressedHandleColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_PRESSED_HANDLE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedPressedIconColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedPressedTrackColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_PRESSED_TRACK_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedPressedTrackOutlineColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_PRESSED_TRACK_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedTrackColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_TRACK_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "UnselectedTrackOutlineColor",
        value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_TRACK_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "IconHandleHeight",
        value: TokenValue::Dp(SwitchTokens::ICON_HANDLE_HEIGHT),
    },
    TokenEntry {
        group: "SwitchTokens",
        name: "IconHandleWidth",
        value: TokenValue::Dp(SwitchTokens::ICON_HANDLE_WIDTH),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "DisabledContainerColor",
        value: TokenValue::ColorRole(TextButtonTokens::DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(TextButtonTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "DisabledIconColor",
        value: TokenValue::ColorRole(TextButtonTokens::DISABLED_ICON_COLOR),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "DisabledIconOpacity",
        value: TokenValue::Float(TextButtonTokens::DISABLED_ICON_OPACITY),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "DisabledLabelColor",
        value: TokenValue::ColorRole(TextButtonTokens::DISABLED_LABEL_COLOR),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "DisabledLabelOpacity",
        value: TokenValue::Float(TextButtonTokens::DISABLED_LABEL_OPACITY),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "FocusedIconColor",
        value: TokenValue::ColorRole(TextButtonTokens::FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "FocusedLabelColor",
        value: TokenValue::ColorRole(TextButtonTokens::FOCUSED_LABEL_COLOR),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "HoveredIconColor",
        value: TokenValue::ColorRole(TextButtonTokens::HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "HoveredLabelColor",
        value: TokenValue::ColorRole(TextButtonTokens::HOVERED_LABEL_COLOR),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(TextButtonTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "LabelColor",
        value: TokenValue::ColorRole(TextButtonTokens::LABEL_COLOR),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(TextButtonTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "TextButtonTokens",
        name: "PressedLabelColor",
        value: TokenValue::ColorRole(TextButtonTokens::PRESSED_LABEL_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(TimeInputTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(TimeInputTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(TimeInputTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "FocusIndicatorColor",
        value: TokenValue::ColorRole(TimeInputTokens::FOCUS_INDICATOR_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "HeadlineColor",
        value: TokenValue::ColorRole(TimeInputTokens::HEADLINE_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "HeadlineFont",
        value: TokenValue::TypographyRole(TimeInputTokens::HEADLINE_FONT),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorContainerHeight",
        value: TokenValue::Dp(TimeInputTokens::PERIOD_SELECTOR_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorContainerShape",
        value: TokenValue::ShapeRole(TimeInputTokens::PERIOD_SELECTOR_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorContainerWidth",
        value: TokenValue::Dp(TimeInputTokens::PERIOD_SELECTOR_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorLabelTextFont",
        value: TokenValue::TypographyRole(TimeInputTokens::PERIOD_SELECTOR_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorOutlineColor",
        value: TokenValue::ColorRole(TimeInputTokens::PERIOD_SELECTOR_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorOutlineWidth",
        value: TokenValue::Dp(TimeInputTokens::PERIOD_SELECTOR_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorSelectedContainerColor",
        value: TokenValue::ColorRole(TimeInputTokens::PERIOD_SELECTOR_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorSelectedFocusLabelTextColor",
        value: TokenValue::ColorRole(
            TimeInputTokens::PERIOD_SELECTOR_SELECTED_FOCUS_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorSelectedHoverLabelTextColor",
        value: TokenValue::ColorRole(
            TimeInputTokens::PERIOD_SELECTOR_SELECTED_HOVER_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorSelectedLabelTextColor",
        value: TokenValue::ColorRole(TimeInputTokens::PERIOD_SELECTOR_SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorSelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(
            TimeInputTokens::PERIOD_SELECTOR_SELECTED_PRESSED_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorUnselectedFocusLabelTextColor",
        value: TokenValue::ColorRole(
            TimeInputTokens::PERIOD_SELECTOR_UNSELECTED_FOCUS_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorUnselectedHoverLabelTextColor",
        value: TokenValue::ColorRole(
            TimeInputTokens::PERIOD_SELECTOR_UNSELECTED_HOVER_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorUnselectedLabelTextColor",
        value: TokenValue::ColorRole(TimeInputTokens::PERIOD_SELECTOR_UNSELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "PeriodSelectorUnselectedPressedLabelTextColor",
        value: TokenValue::ColorRole(
            TimeInputTokens::PERIOD_SELECTOR_UNSELECTED_PRESSED_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldContainerColor",
        value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldContainerHeight",
        value: TokenValue::Dp(TimeInputTokens::TIME_FIELD_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldContainerShape",
        value: TokenValue::ShapeRole(TimeInputTokens::TIME_FIELD_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldContainerWidth",
        value: TokenValue::Dp(TimeInputTokens::TIME_FIELD_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldFocusContainerColor",
        value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_FOCUS_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldFocusLabelTextColor",
        value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_FOCUS_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldFocusOutlineColor",
        value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_FOCUS_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldFocusOutlineWidth",
        value: TokenValue::Dp(TimeInputTokens::TIME_FIELD_FOCUS_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldHoverLabelTextColor",
        value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_HOVER_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldLabelTextColor",
        value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldLabelTextFont",
        value: TokenValue::TypographyRole(TimeInputTokens::TIME_FIELD_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldSeparatorColor",
        value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_SEPARATOR_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldSeparatorFont",
        value: TokenValue::TypographyRole(TimeInputTokens::TIME_FIELD_SEPARATOR_FONT),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldSupportingTextColor",
        value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimeInputTokens",
        name: "TimeFieldSupportingTextFont",
        value: TokenValue::TypographyRole(TimeInputTokens::TIME_FIELD_SUPPORTING_TEXT_FONT),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialColor",
        value: TokenValue::ColorRole(TimePickerTokens::CLOCK_DIAL_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialContainerSize",
        value: TokenValue::Dp(TimePickerTokens::CLOCK_DIAL_CONTAINER_SIZE),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialLabelTextFont",
        value: TokenValue::TypographyRole(TimePickerTokens::CLOCK_DIAL_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialSelectedLabelTextColor",
        value: TokenValue::ColorRole(TimePickerTokens::CLOCK_DIAL_SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialSelectorCenterContainerColor",
        value: TokenValue::ColorRole(TimePickerTokens::CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialSelectorCenterContainerShape",
        value: TokenValue::ShapeRole(TimePickerTokens::CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialSelectorCenterContainerSize",
        value: TokenValue::Dp(TimePickerTokens::CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_SIZE),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialSelectorHandleContainerColor",
        value: TokenValue::ColorRole(TimePickerTokens::CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialSelectorHandleContainerShape",
        value: TokenValue::ShapeRole(TimePickerTokens::CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialSelectorHandleContainerSize",
        value: TokenValue::Dp(TimePickerTokens::CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_SIZE),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialSelectorTrackContainerColor",
        value: TokenValue::ColorRole(TimePickerTokens::CLOCK_DIAL_SELECTOR_TRACK_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialSelectorTrackContainerWidth",
        value: TokenValue::Dp(TimePickerTokens::CLOCK_DIAL_SELECTOR_TRACK_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialShape",
        value: TokenValue::ShapeRole(TimePickerTokens::CLOCK_DIAL_SHAPE),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ClockDialUnselectedLabelTextColor",
        value: TokenValue::ColorRole(TimePickerTokens::CLOCK_DIAL_UNSELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(TimePickerTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(TimePickerTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "ContainerShape",
        value: TokenValue::ShapeRole(TimePickerTokens::CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "HeadlineColor",
        value: TokenValue::ColorRole(TimePickerTokens::HEADLINE_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "HeadlineFont",
        value: TokenValue::TypographyRole(TimePickerTokens::HEADLINE_FONT),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorContainerShape",
        value: TokenValue::ShapeRole(TimePickerTokens::PERIOD_SELECTOR_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorHorizontalContainerHeight",
        value: TokenValue::Dp(TimePickerTokens::PERIOD_SELECTOR_HORIZONTAL_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorHorizontalContainerWidth",
        value: TokenValue::Dp(TimePickerTokens::PERIOD_SELECTOR_HORIZONTAL_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorLabelTextFont",
        value: TokenValue::TypographyRole(TimePickerTokens::PERIOD_SELECTOR_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorOutlineColor",
        value: TokenValue::ColorRole(TimePickerTokens::PERIOD_SELECTOR_OUTLINE_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorOutlineWidth",
        value: TokenValue::Dp(TimePickerTokens::PERIOD_SELECTOR_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorSelectedContainerColor",
        value: TokenValue::ColorRole(TimePickerTokens::PERIOD_SELECTOR_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorSelectedFocusLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::PERIOD_SELECTOR_SELECTED_FOCUS_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorSelectedHoverLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::PERIOD_SELECTOR_SELECTED_HOVER_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorSelectedLabelTextColor",
        value: TokenValue::ColorRole(TimePickerTokens::PERIOD_SELECTOR_SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorSelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::PERIOD_SELECTOR_SELECTED_PRESSED_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorUnselectedFocusLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::PERIOD_SELECTOR_UNSELECTED_FOCUS_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorUnselectedHoverLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::PERIOD_SELECTOR_UNSELECTED_HOVER_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorUnselectedLabelTextColor",
        value: TokenValue::ColorRole(TimePickerTokens::PERIOD_SELECTOR_UNSELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorUnselectedPressedLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::PERIOD_SELECTOR_UNSELECTED_PRESSED_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorVerticalContainerHeight",
        value: TokenValue::Dp(TimePickerTokens::PERIOD_SELECTOR_VERTICAL_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "PeriodSelectorVerticalContainerWidth",
        value: TokenValue::Dp(TimePickerTokens::PERIOD_SELECTOR_VERTICAL_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelector24HVerticalContainerWidth",
        value: TokenValue::Dp(TimePickerTokens::TIME_SELECTOR24_H_VERTICAL_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorContainerHeight",
        value: TokenValue::Dp(TimePickerTokens::TIME_SELECTOR_CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorContainerShape",
        value: TokenValue::ShapeRole(TimePickerTokens::TIME_SELECTOR_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorContainerWidth",
        value: TokenValue::Dp(TimePickerTokens::TIME_SELECTOR_CONTAINER_WIDTH),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorLabelTextFont",
        value: TokenValue::TypographyRole(TimePickerTokens::TIME_SELECTOR_LABEL_TEXT_FONT),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorSelectedContainerColor",
        value: TokenValue::ColorRole(TimePickerTokens::TIME_SELECTOR_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorSelectedFocusLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::TIME_SELECTOR_SELECTED_FOCUS_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorSelectedHoverLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::TIME_SELECTOR_SELECTED_HOVER_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorSelectedLabelTextColor",
        value: TokenValue::ColorRole(TimePickerTokens::TIME_SELECTOR_SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorSelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::TIME_SELECTOR_SELECTED_PRESSED_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorSeparatorColor",
        value: TokenValue::ColorRole(TimePickerTokens::TIME_SELECTOR_SEPARATOR_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorSeparatorFont",
        value: TokenValue::TypographyRole(TimePickerTokens::TIME_SELECTOR_SEPARATOR_FONT),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorUnselectedContainerColor",
        value: TokenValue::ColorRole(TimePickerTokens::TIME_SELECTOR_UNSELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorUnselectedFocusLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::TIME_SELECTOR_UNSELECTED_FOCUS_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorUnselectedHoverLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::TIME_SELECTOR_UNSELECTED_HOVER_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorUnselectedLabelTextColor",
        value: TokenValue::ColorRole(TimePickerTokens::TIME_SELECTOR_UNSELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TimePickerTokens",
        name: "TimeSelectorUnselectedPressedLabelTextColor",
        value: TokenValue::ColorRole(
            TimePickerTokens::TIME_SELECTOR_UNSELECTED_PRESSED_LABEL_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(TonalButtonTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "ContainerElevation",
        value: TokenValue::Dp(TonalButtonTokens::CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "DisabledContainerColor",
        value: TokenValue::ColorRole(TonalButtonTokens::DISABLED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "DisabledContainerElevation",
        value: TokenValue::Dp(TonalButtonTokens::DISABLED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "DisabledContainerOpacity",
        value: TokenValue::Float(TonalButtonTokens::DISABLED_CONTAINER_OPACITY),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "DisabledIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::DISABLED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "DisabledIconOpacity",
        value: TokenValue::Float(TonalButtonTokens::DISABLED_ICON_OPACITY),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "DisabledLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "DisabledLabelTextOpacity",
        value: TokenValue::Float(TonalButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "FocusedContainerElevation",
        value: TokenValue::Dp(TonalButtonTokens::FOCUSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "FocusedIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "FocusedLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "HoveredContainerElevation",
        value: TokenValue::Dp(TonalButtonTokens::HOVERED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "HoveredIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "HoveredLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "IconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "LabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "PressedContainerElevation",
        value: TokenValue::Dp(TonalButtonTokens::PRESSED_CONTAINER_ELEVATION),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "PressedIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "PressedLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "SelectedContainerColor",
        value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "SelectedFocusedIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "SelectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "SelectedHoveredIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "SelectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "SelectedIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "SelectedLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "SelectedPressedIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "SelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "UnselectedContainerColor",
        value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "UnselectedFocusedIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_FOCUSED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "UnselectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "UnselectedHoveredIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_HOVERED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "UnselectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "UnselectedIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "UnselectedLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "UnselectedPressedIconColor",
        value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
    },
    TokenEntry {
        group: "TonalButtonTokens",
        name: "UnselectedPressedLabelTextColor",
        value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "TypefaceTokens",
        name: "Brand",
        value: TokenValue::FontFamily(TypefaceTokens::BRAND),
    },
    TokenEntry {
        group: "TypefaceTokens",
        name: "Plain",
        value: TokenValue::FontFamily(TypefaceTokens::PLAIN),
    },
    TokenEntry {
        group: "TypefaceTokens",
        name: "WeightBold",
        value: TokenValue::FontWeight(TypefaceTokens::WEIGHT_BOLD),
    },
    TokenEntry {
        group: "TypefaceTokens",
        name: "WeightMedium",
        value: TokenValue::FontWeight(TypefaceTokens::WEIGHT_MEDIUM),
    },
    TokenEntry {
        group: "TypefaceTokens",
        name: "WeightRegular",
        value: TokenValue::FontWeight(TypefaceTokens::WEIGHT_REGULAR),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyLargeFont",
        value: TokenValue::FontFamily(TypeScaleTokens::BODY_LARGE_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyLargeLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyLargeSize",
        value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyLargeTracking",
        value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyLargeWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::BODY_LARGE_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyMediumFont",
        value: TokenValue::FontFamily(TypeScaleTokens::BODY_MEDIUM_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyMediumLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyMediumSize",
        value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyMediumTracking",
        value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyMediumWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::BODY_MEDIUM_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodySmallFont",
        value: TokenValue::FontFamily(TypeScaleTokens::BODY_SMALL_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodySmallLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodySmallSize",
        value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodySmallTracking",
        value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodySmallWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::BODY_SMALL_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayLargeFont",
        value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_LARGE_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayLargeLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayLargeSize",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayLargeTracking",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayLargeWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_LARGE_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayMediumFont",
        value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_MEDIUM_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayMediumLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayMediumSize",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayMediumTracking",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayMediumWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_MEDIUM_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplaySmallFont",
        value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_SMALL_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplaySmallLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplaySmallSize",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplaySmallTracking",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplaySmallWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_SMALL_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineLargeFont",
        value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_LARGE_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineLargeLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineLargeSize",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineLargeTracking",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineLargeWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_LARGE_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineMediumFont",
        value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_MEDIUM_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineMediumLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineMediumSize",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineMediumTracking",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineMediumWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_MEDIUM_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineSmallFont",
        value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_SMALL_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineSmallLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineSmallSize",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineSmallTracking",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineSmallWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_SMALL_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelLargeFont",
        value: TokenValue::FontFamily(TypeScaleTokens::LABEL_LARGE_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelLargeLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelLargeSize",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelLargeTracking",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelLargeWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::LABEL_LARGE_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelMediumFont",
        value: TokenValue::FontFamily(TypeScaleTokens::LABEL_MEDIUM_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelMediumLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelMediumSize",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelMediumTracking",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelMediumWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::LABEL_MEDIUM_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelSmallFont",
        value: TokenValue::FontFamily(TypeScaleTokens::LABEL_SMALL_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelSmallLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelSmallSize",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelSmallTracking",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelSmallWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::LABEL_SMALL_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleLargeFont",
        value: TokenValue::FontFamily(TypeScaleTokens::TITLE_LARGE_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleLargeLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleLargeSize",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleLargeTracking",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleLargeWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::TITLE_LARGE_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleMediumFont",
        value: TokenValue::FontFamily(TypeScaleTokens::TITLE_MEDIUM_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleMediumLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleMediumSize",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleMediumTracking",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleMediumWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::TITLE_MEDIUM_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleSmallFont",
        value: TokenValue::FontFamily(TypeScaleTokens::TITLE_SMALL_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleSmallLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleSmallSize",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleSmallTracking",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleSmallWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::TITLE_SMALL_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyLargeEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::BODY_LARGE_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyLargeEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyLargeEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyLargeEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyLargeEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::BODY_LARGE_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyMediumEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyMediumEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyMediumEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyMediumEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodyMediumEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodySmallEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::BODY_SMALL_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodySmallEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodySmallEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodySmallEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "BodySmallEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::BODY_SMALL_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayLargeEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayLargeEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayLargeEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayLargeEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayLargeEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayMediumEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayMediumEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayMediumEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayMediumEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplayMediumEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplaySmallEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplaySmallEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplaySmallEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplaySmallEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "DisplaySmallEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineLargeEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineLargeEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineLargeEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineLargeEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineLargeEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineMediumEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineMediumEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineMediumEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineMediumEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineMediumEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineSmallEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineSmallEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineSmallEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineSmallEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "HeadlineSmallEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelLargeEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::LABEL_LARGE_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelLargeEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelLargeEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelLargeEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelLargeEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::LABEL_LARGE_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelMediumEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelMediumEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelMediumEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelMediumEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelMediumEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelSmallEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::LABEL_SMALL_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelSmallEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelSmallEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelSmallEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "LabelSmallEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::LABEL_SMALL_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleLargeEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::TITLE_LARGE_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleLargeEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleLargeEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleLargeEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleLargeEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::TITLE_LARGE_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleMediumEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleMediumEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleMediumEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleMediumEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleMediumEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleSmallEmphasizedFont",
        value: TokenValue::FontFamily(TypeScaleTokens::TITLE_SMALL_EMPHASIZED_FONT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleSmallEmphasizedLineHeight",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_EMPHASIZED_LINE_HEIGHT),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleSmallEmphasizedSize",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_EMPHASIZED_SIZE),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleSmallEmphasizedTracking",
        value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_EMPHASIZED_TRACKING),
    },
    TokenEntry {
        group: "TypeScaleTokens",
        name: "TitleSmallEmphasizedWeight",
        value: TokenValue::FontWeight(TypeScaleTokens::TITLE_SMALL_EMPHASIZED_WEIGHT),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "BodyLarge",
        value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_LARGE),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "BodyMedium",
        value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_MEDIUM),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "BodySmall",
        value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_SMALL),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "DisplayLarge",
        value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_LARGE),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "DisplayMedium",
        value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_MEDIUM),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "DisplaySmall",
        value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_SMALL),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "HeadlineLarge",
        value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_LARGE),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "HeadlineMedium",
        value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_MEDIUM),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "HeadlineSmall",
        value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_SMALL),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "LabelLarge",
        value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_LARGE),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "LabelMedium",
        value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_MEDIUM),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "LabelSmall",
        value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_SMALL),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "TitleLarge",
        value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_LARGE),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "TitleMedium",
        value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_MEDIUM),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "TitleSmall",
        value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_SMALL),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "BodyLargeEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_LARGE_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "BodyMediumEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_MEDIUM_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "BodySmallEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_SMALL_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "DisplayLargeEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_LARGE_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "DisplayMediumEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_MEDIUM_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "DisplaySmallEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_SMALL_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "HeadlineLargeEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_LARGE_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "HeadlineMediumEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_MEDIUM_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "HeadlineSmallEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_SMALL_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "LabelLargeEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_LARGE_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "LabelMediumEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_MEDIUM_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "LabelSmallEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_SMALL_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "TitleLargeEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_LARGE_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "TitleMediumEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_MEDIUM_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyKeyTokens",
        name: "TitleSmallEmphasized",
        value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_SMALL_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "BodyLarge",
        value: TokenValue::TextStyle(TypographyTokens::BODY_LARGE),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "BodyMedium",
        value: TokenValue::TextStyle(TypographyTokens::BODY_MEDIUM),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "BodySmall",
        value: TokenValue::TextStyle(TypographyTokens::BODY_SMALL),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "DisplayLarge",
        value: TokenValue::TextStyle(TypographyTokens::DISPLAY_LARGE),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "DisplayMedium",
        value: TokenValue::TextStyle(TypographyTokens::DISPLAY_MEDIUM),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "DisplaySmall",
        value: TokenValue::TextStyle(TypographyTokens::DISPLAY_SMALL),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "HeadlineLarge",
        value: TokenValue::TextStyle(TypographyTokens::HEADLINE_LARGE),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "HeadlineMedium",
        value: TokenValue::TextStyle(TypographyTokens::HEADLINE_MEDIUM),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "HeadlineSmall",
        value: TokenValue::TextStyle(TypographyTokens::HEADLINE_SMALL),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "LabelLarge",
        value: TokenValue::TextStyle(TypographyTokens::LABEL_LARGE),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "LabelMedium",
        value: TokenValue::TextStyle(TypographyTokens::LABEL_MEDIUM),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "LabelSmall",
        value: TokenValue::TextStyle(TypographyTokens::LABEL_SMALL),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "TitleLarge",
        value: TokenValue::TextStyle(TypographyTokens::TITLE_LARGE),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "TitleMedium",
        value: TokenValue::TextStyle(TypographyTokens::TITLE_MEDIUM),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "TitleSmall",
        value: TokenValue::TextStyle(TypographyTokens::TITLE_SMALL),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "BodyLargeEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::BODY_LARGE_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "BodyMediumEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::BODY_MEDIUM_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "BodySmallEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::BODY_SMALL_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "DisplayLargeEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::DISPLAY_LARGE_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "DisplayMediumEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::DISPLAY_MEDIUM_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "DisplaySmallEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::DISPLAY_SMALL_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "HeadlineLargeEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::HEADLINE_LARGE_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "HeadlineMediumEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::HEADLINE_MEDIUM_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "HeadlineSmallEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::HEADLINE_SMALL_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "LabelLargeEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::LABEL_LARGE_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "LabelMediumEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::LABEL_MEDIUM_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "LabelSmallEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::LABEL_SMALL_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "TitleLargeEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::TITLE_LARGE_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "TitleMediumEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::TITLE_MEDIUM_EMPHASIZED),
    },
    TokenEntry {
        group: "TypographyTokens",
        name: "TitleSmallEmphasized",
        value: TokenValue::TextStyle(TypographyTokens::TITLE_SMALL_EMPHASIZED),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ButtonDisabledIconIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::BUTTON_DISABLED_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ButtonIconIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::BUTTON_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ButtonSelectedDisabledIconIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::BUTTON_SELECTED_DISABLED_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ButtonSelectedIconIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::BUTTON_SELECTED_ICON_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ContainerColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::CONTAINER_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "IconButtonContainerColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ICON_BUTTON_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "IconButtonSelectedContainerColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ICON_BUTTON_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemDisabledLabelTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_DISABLED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemDisabledLabelTextOpacity",
        value: TokenValue::Float(VibrantMenuTokens::ITEM_DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemDisabledLeadingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_DISABLED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemDisabledLeadingIconOpacity",
        value: TokenValue::Float(VibrantMenuTokens::ITEM_DISABLED_LEADING_ICON_OPACITY),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemDisabledSupportingTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_DISABLED_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemDisabledSupportingTextOpacity",
        value: TokenValue::Float(VibrantMenuTokens::ITEM_DISABLED_SUPPORTING_TEXT_OPACITY),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemDisabledTrailingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_DISABLED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemDisabledTrailingIconOpacity",
        value: TokenValue::Float(VibrantMenuTokens::ITEM_DISABLED_TRAILING_ICON_OPACITY),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemDisabledTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            VibrantMenuTokens::ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemDisabledTrailingSupportingTextOpacity",
        value: TokenValue::Float(VibrantMenuTokens::ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemFocusedLabelTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemFocusedLeadingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_FOCUSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemFocusedSupportingTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_FOCUSED_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemFocusedTrailingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_FOCUSED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemFocusedTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            VibrantMenuTokens::ITEM_FOCUSED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemHoveredLabelTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemHoveredLeadingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_HOVERED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemHoveredSupportingTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_HOVERED_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemHoveredTrailingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_HOVERED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemHoveredTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            VibrantMenuTokens::ITEM_HOVERED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemLabelTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemLeadingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemPressedLabelTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemPressedLeadingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_PRESSED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemPressedSupportingTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_PRESSED_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemPressedTrailingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_PRESSED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemPressedTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            VibrantMenuTokens::ITEM_PRESSED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedContainerColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_CONTAINER_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedDisabledLabelTextOpacity",
        value: TokenValue::Float(VibrantMenuTokens::ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedDisabledLeadingIconOpacity",
        value: TokenValue::Float(VibrantMenuTokens::ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedDisabledSupportingTextOpacity",
        value: TokenValue::Float(VibrantMenuTokens::ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_OPACITY),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedDisabledTrailingIconOpacity",
        value: TokenValue::Float(VibrantMenuTokens::ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedDisabledTrailingSupportingTextOpacity",
        value: TokenValue::Float(
            VibrantMenuTokens::ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY,
        ),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedFocusedLabelTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_FOCUSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedHoveredLabelTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_HOVERED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedLabelTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedLeadingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_LEADING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedPressedLabelTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedSupportingTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedTrailingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSelectedTrailingSupportingTextColor",
        value: TokenValue::ColorRole(
            VibrantMenuTokens::ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR,
        ),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemSupportingTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemTrailingIconColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_TRAILING_ICON_COLOR),
    },
    TokenEntry {
        group: "VibrantMenuTokens",
        name: "ItemTrailingSupportingTextColor",
        value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_TRAILING_SUPPORTING_TEXT_COLOR),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(XLargeIconButtonTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "ContainerShapeRound",
        value: TokenValue::ShapeRole(XLargeIconButtonTokens::CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "ContainerShapeSquare",
        value: TokenValue::ShapeRole(XLargeIconButtonTokens::CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "DefaultLeadingSpace",
        value: TokenValue::Dp(XLargeIconButtonTokens::DEFAULT_LEADING_SPACE),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "DefaultTrailingSpace",
        value: TokenValue::Dp(XLargeIconButtonTokens::DEFAULT_TRAILING_SPACE),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "IconSize",
        value: TokenValue::Dp(XLargeIconButtonTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "NarrowLeadingSpace",
        value: TokenValue::Dp(XLargeIconButtonTokens::NARROW_LEADING_SPACE),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "NarrowTrailingSpace",
        value: TokenValue::Dp(XLargeIconButtonTokens::NARROW_TRAILING_SPACE),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "OutlinedOutlineWidth",
        value: TokenValue::Dp(XLargeIconButtonTokens::OUTLINED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "PressedContainerShape",
        value: TokenValue::ShapeRole(XLargeIconButtonTokens::PRESSED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "SelectedContainerShapeRound",
        value: TokenValue::ShapeRole(XLargeIconButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "SelectedContainerShapeSquare",
        value: TokenValue::ShapeRole(XLargeIconButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "WideLeadingSpace",
        value: TokenValue::Dp(XLargeIconButtonTokens::WIDE_LEADING_SPACE),
    },
    TokenEntry {
        group: "XLargeIconButtonTokens",
        name: "WideTrailingSpace",
        value: TokenValue::Dp(XLargeIconButtonTokens::WIDE_TRAILING_SPACE),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "ContainerHeight",
        value: TokenValue::Dp(XSmallIconButtonTokens::CONTAINER_HEIGHT),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "ContainerShapeRound",
        value: TokenValue::ShapeRole(XSmallIconButtonTokens::CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "ContainerShapeSquare",
        value: TokenValue::ShapeRole(XSmallIconButtonTokens::CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "DefaultLeadingSpace",
        value: TokenValue::Dp(XSmallIconButtonTokens::DEFAULT_LEADING_SPACE),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "DefaultTrailingSpace",
        value: TokenValue::Dp(XSmallIconButtonTokens::DEFAULT_TRAILING_SPACE),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "IconSize",
        value: TokenValue::Dp(XSmallIconButtonTokens::ICON_SIZE),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "NarrowLeadingSpace",
        value: TokenValue::Dp(XSmallIconButtonTokens::NARROW_LEADING_SPACE),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "NarrowTrailingSpace",
        value: TokenValue::Dp(XSmallIconButtonTokens::NARROW_TRAILING_SPACE),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "OutlinedOutlineWidth",
        value: TokenValue::Dp(XSmallIconButtonTokens::OUTLINED_OUTLINE_WIDTH),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "PressedContainerShape",
        value: TokenValue::ShapeRole(XSmallIconButtonTokens::PRESSED_CONTAINER_SHAPE),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "SelectedContainerShapeRound",
        value: TokenValue::ShapeRole(XSmallIconButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "SelectedContainerShapeSquare",
        value: TokenValue::ShapeRole(XSmallIconButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "WideLeadingSpace",
        value: TokenValue::Dp(XSmallIconButtonTokens::WIDE_LEADING_SPACE),
    },
    TokenEntry {
        group: "XSmallIconButtonTokens",
        name: "WideTrailingSpace",
        value: TokenValue::Dp(XSmallIconButtonTokens::WIDE_TRAILING_SPACE),
    },
];
impl MotionTokens {
    pub const EXTRA_LONG1: Duration = Duration::from_millis(700);
    pub const EXTRA_LONG2: Duration = Duration::from_millis(800);
    pub const EXTRA_LONG3: Duration = Duration::from_millis(900);
    pub const EXTRA_LONG4: Duration = Duration::from_millis(1000);
    pub const LONG1: Duration = Duration::from_millis(450);
    pub const LONG2: Duration = Duration::from_millis(500);
    pub const LONG3: Duration = Duration::from_millis(550);
    pub const LONG4: Duration = Duration::from_millis(600);
    pub const MEDIUM1: Duration = Duration::from_millis(250);
    pub const MEDIUM2: Duration = Duration::from_millis(300);
    pub const MEDIUM3: Duration = Duration::from_millis(350);
    pub const MEDIUM4: Duration = Duration::from_millis(400);
    pub const SHORT1: Duration = Duration::from_millis(50);
    pub const SHORT2: Duration = Duration::from_millis(100);
    pub const SHORT3: Duration = Duration::from_millis(150);
    pub const SHORT4: Duration = Duration::from_millis(200);
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
impl TypeScale {
    pub fn androidx() -> Self {
        Self {
            body_large: TypographyTokens::BODY_LARGE.type_style(1.0),
            body_medium: TypographyTokens::BODY_MEDIUM.type_style(1.0),
            body_small: TypographyTokens::BODY_SMALL.type_style(1.0),
            display_large: TypographyTokens::DISPLAY_LARGE.type_style(1.0),
            display_medium: TypographyTokens::DISPLAY_MEDIUM.type_style(1.0),
            display_small: TypographyTokens::DISPLAY_SMALL.type_style(1.0),
            headline_large: TypographyTokens::HEADLINE_LARGE.type_style(1.0),
            headline_medium: TypographyTokens::HEADLINE_MEDIUM.type_style(1.0),
            headline_small: TypographyTokens::HEADLINE_SMALL.type_style(1.0),
            label_large: TypographyTokens::LABEL_LARGE.type_style(1.0),
            label_medium: TypographyTokens::LABEL_MEDIUM.type_style(1.0),
            label_small: TypographyTokens::LABEL_SMALL.type_style(1.0),
            title_large: TypographyTokens::TITLE_LARGE.type_style(1.0),
            title_medium: TypographyTokens::TITLE_MEDIUM.type_style(1.0),
            title_small: TypographyTokens::TITLE_SMALL.type_style(1.0),
            body_large_emphasized: TypographyTokens::BODY_LARGE_EMPHASIZED.type_style(1.0),
            body_medium_emphasized: TypographyTokens::BODY_MEDIUM_EMPHASIZED.type_style(1.0),
            body_small_emphasized: TypographyTokens::BODY_SMALL_EMPHASIZED.type_style(1.0),
            display_large_emphasized: TypographyTokens::DISPLAY_LARGE_EMPHASIZED.type_style(1.0),
            display_medium_emphasized: TypographyTokens::DISPLAY_MEDIUM_EMPHASIZED.type_style(1.0),
            display_small_emphasized: TypographyTokens::DISPLAY_SMALL_EMPHASIZED.type_style(1.0),
            headline_large_emphasized: TypographyTokens::HEADLINE_LARGE_EMPHASIZED.type_style(1.0),
            headline_medium_emphasized: TypographyTokens::HEADLINE_MEDIUM_EMPHASIZED
                .type_style(1.0),
            headline_small_emphasized: TypographyTokens::HEADLINE_SMALL_EMPHASIZED.type_style(1.0),
            label_large_emphasized: TypographyTokens::LABEL_LARGE_EMPHASIZED.type_style(1.0),
            label_medium_emphasized: TypographyTokens::LABEL_MEDIUM_EMPHASIZED.type_style(1.0),
            label_small_emphasized: TypographyTokens::LABEL_SMALL_EMPHASIZED.type_style(1.0),
            title_large_emphasized: TypographyTokens::TITLE_LARGE_EMPHASIZED.type_style(1.0),
            title_medium_emphasized: TypographyTokens::TITLE_MEDIUM_EMPHASIZED.type_style(1.0),
            title_small_emphasized: TypographyTokens::TITLE_SMALL_EMPHASIZED.type_style(1.0),
        }
    }
}
impl TypographyToken {
    pub fn resolve(self, tokens: &TokenSet) -> TypeStyle {
        match self {
            Self::BodyLarge => tokens.typography.body_large,
            Self::BodyMedium => tokens.typography.body_medium,
            Self::BodySmall => tokens.typography.body_small,
            Self::DisplayLarge => tokens.typography.display_large,
            Self::DisplayMedium => tokens.typography.display_medium,
            Self::DisplaySmall => tokens.typography.display_small,
            Self::HeadlineLarge => tokens.typography.headline_large,
            Self::HeadlineMedium => tokens.typography.headline_medium,
            Self::HeadlineSmall => tokens.typography.headline_small,
            Self::LabelLarge => tokens.typography.label_large,
            Self::LabelMedium => tokens.typography.label_medium,
            Self::LabelSmall => tokens.typography.label_small,
            Self::TitleLarge => tokens.typography.title_large,
            Self::TitleMedium => tokens.typography.title_medium,
            Self::TitleSmall => tokens.typography.title_small,
            Self::BodyLargeEmphasized => tokens.typography.body_large_emphasized,
            Self::BodyMediumEmphasized => tokens.typography.body_medium_emphasized,
            Self::BodySmallEmphasized => tokens.typography.body_small_emphasized,
            Self::DisplayLargeEmphasized => tokens.typography.display_large_emphasized,
            Self::DisplayMediumEmphasized => tokens.typography.display_medium_emphasized,
            Self::DisplaySmallEmphasized => tokens.typography.display_small_emphasized,
            Self::HeadlineLargeEmphasized => tokens.typography.headline_large_emphasized,
            Self::HeadlineMediumEmphasized => tokens.typography.headline_medium_emphasized,
            Self::HeadlineSmallEmphasized => tokens.typography.headline_small_emphasized,
            Self::LabelLargeEmphasized => tokens.typography.label_large_emphasized,
            Self::LabelMediumEmphasized => tokens.typography.label_medium_emphasized,
            Self::LabelSmallEmphasized => tokens.typography.label_small_emphasized,
            Self::TitleLargeEmphasized => tokens.typography.title_large_emphasized,
            Self::TitleMediumEmphasized => tokens.typography.title_medium_emphasized,
            Self::TitleSmallEmphasized => tokens.typography.title_small_emphasized,
        }
    }
}
impl MotionSchemeToken {
    pub fn resolve(self, tokens: &TokenSet) -> MotionSpec {
        *tokens.motion.spec(match self {
            Self::DefaultSpatial => MotionRole::DefaultSpatial,
            Self::FastSpatial => MotionRole::FastSpatial,
            Self::SlowSpatial => MotionRole::SlowSpatial,
            Self::DefaultEffects => MotionRole::DefaultEffects,
            Self::FastEffects => MotionRole::FastEffects,
            Self::SlowEffects => MotionRole::SlowEffects,
        })
    }
}
impl ShapeToken {
    pub fn resolve(self, tokens: &TokenSet) -> ShapeValue {
        match self {
            Self::CornerExtraExtraLarge => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_extra_large)),
                top_end: Dp(f32::from(tokens.shapes.extra_extra_large)),
                bottom_end: Dp(f32::from(tokens.shapes.extra_extra_large)),
                bottom_start: Dp(f32::from(tokens.shapes.extra_extra_large)),
            },
            Self::CornerExtraLarge => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_large)),
                top_end: Dp(f32::from(tokens.shapes.extra_large)),
                bottom_end: Dp(f32::from(tokens.shapes.extra_large)),
                bottom_start: Dp(f32::from(tokens.shapes.extra_large)),
            },
            Self::CornerExtraLargeIncreased => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_large_increased)),
                top_end: Dp(f32::from(tokens.shapes.extra_large_increased)),
                bottom_end: Dp(f32::from(tokens.shapes.extra_large_increased)),
                bottom_start: Dp(f32::from(tokens.shapes.extra_large_increased)),
            },
            Self::CornerExtraLargeTop => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_large)),
                top_end: Dp(f32::from(tokens.shapes.extra_large)),
                bottom_end: Dp(0.0),
                bottom_start: Dp(0.0),
            },
            Self::CornerExtraSmall => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_small)),
                top_end: Dp(f32::from(tokens.shapes.extra_small)),
                bottom_end: Dp(f32::from(tokens.shapes.extra_small)),
                bottom_start: Dp(f32::from(tokens.shapes.extra_small)),
            },
            Self::CornerExtraSmallTop => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_small)),
                top_end: Dp(f32::from(tokens.shapes.extra_small)),
                bottom_end: Dp(0.0),
                bottom_start: Dp(0.0),
            },
            Self::CornerFull => ShapeValue::Full,
            Self::CornerLarge => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.large)),
                top_end: Dp(f32::from(tokens.shapes.large)),
                bottom_end: Dp(f32::from(tokens.shapes.large)),
                bottom_start: Dp(f32::from(tokens.shapes.large)),
            },
            Self::CornerLargeEnd => ShapeValue::Rounded {
                top_start: Dp(0.0),
                top_end: Dp(f32::from(tokens.shapes.large)),
                bottom_end: Dp(f32::from(tokens.shapes.large)),
                bottom_start: Dp(0.0),
            },
            Self::CornerLargeIncreased => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.large_increased)),
                top_end: Dp(f32::from(tokens.shapes.large_increased)),
                bottom_end: Dp(f32::from(tokens.shapes.large_increased)),
                bottom_start: Dp(f32::from(tokens.shapes.large_increased)),
            },
            Self::CornerLargeStart => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.large)),
                top_end: Dp(0.0),
                bottom_end: Dp(0.0),
                bottom_start: Dp(f32::from(tokens.shapes.large)),
            },
            Self::CornerLargeTop => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.large)),
                top_end: Dp(f32::from(tokens.shapes.large)),
                bottom_end: Dp(0.0),
                bottom_start: Dp(0.0),
            },
            Self::CornerMedium => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.medium)),
                top_end: Dp(f32::from(tokens.shapes.medium)),
                bottom_end: Dp(f32::from(tokens.shapes.medium)),
                bottom_start: Dp(f32::from(tokens.shapes.medium)),
            },
            Self::CornerNone => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.none)),
                top_end: Dp(f32::from(tokens.shapes.none)),
                bottom_end: Dp(f32::from(tokens.shapes.none)),
                bottom_start: Dp(f32::from(tokens.shapes.none)),
            },
            Self::CornerSmall => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.small)),
                top_end: Dp(f32::from(tokens.shapes.small)),
                bottom_end: Dp(f32::from(tokens.shapes.small)),
                bottom_start: Dp(f32::from(tokens.shapes.small)),
            },
        }
    }
}
