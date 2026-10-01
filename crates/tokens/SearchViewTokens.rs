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
impl SearchViewTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
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
    ];
}
