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
impl SnackbarTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
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
    ];
}
