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
impl NavigationDrawerTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
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
    ];
}
