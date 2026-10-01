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
impl DialogTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
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
    ];
}
