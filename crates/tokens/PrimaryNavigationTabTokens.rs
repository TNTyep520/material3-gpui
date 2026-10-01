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
    ColorSchemeKeyTokens, ColorToken, Dp, ElevationTokens, ShapeKeyTokens, ShapeToken, ShapeValue,
    TokenEntry, TokenValue, TypographyKeyTokens, TypographyToken,
};

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
impl PrimaryNavigationTabTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
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
            value: TokenValue::ColorRole(
                PrimaryNavigationTabTokens::ACTIVE_PRESSED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "PrimaryNavigationTabTokens",
            name: "InactiveFocusLabelTextColor",
            value: TokenValue::ColorRole(
                PrimaryNavigationTabTokens::INACTIVE_FOCUS_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "PrimaryNavigationTabTokens",
            name: "InactiveHoverLabelTextColor",
            value: TokenValue::ColorRole(
                PrimaryNavigationTabTokens::INACTIVE_HOVER_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "PrimaryNavigationTabTokens",
            name: "InactiveLabelTextColor",
            value: TokenValue::ColorRole(PrimaryNavigationTabTokens::INACTIVE_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "PrimaryNavigationTabTokens",
            name: "InactivePressedLabelTextColor",
            value: TokenValue::ColorRole(
                PrimaryNavigationTabTokens::INACTIVE_PRESSED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "PrimaryNavigationTabTokens",
            name: "LabelTextFont",
            value: TokenValue::TypographyRole(PrimaryNavigationTabTokens::LABEL_TEXT_FONT),
        },
    ];
}
