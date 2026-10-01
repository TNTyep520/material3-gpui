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
impl RichTooltipTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
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
    ];
}
