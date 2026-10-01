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
impl FilledTonalButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
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
    ];
}
