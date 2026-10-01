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

use super::{
    ColorSchemeKeyTokens, ColorToken, Dp, ElevationTokens, ShapeKeyTokens, ShapeToken, TokenEntry,
    TokenValue, TypographyKeyTokens, TypographyToken,
};

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
impl ExtendedFabPrimaryTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
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
    ];
}
