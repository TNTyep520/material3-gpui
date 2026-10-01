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
impl SecondaryNavigationTabTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
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
    ];
}
