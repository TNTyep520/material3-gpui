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
    ColorSchemeKeyTokens, ColorToken, Dp, ShapeKeyTokens, ShapeToken, TokenEntry, TokenValue,
};

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
impl FloatingToolbarTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
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
            value: TokenValue::ColorRole(
                FloatingToolbarTokens::VIBRANT_BUTTON_UNSELECTED_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FloatingToolbarTokens",
            name: "VibrantButtonUnselectedTextColor",
            value: TokenValue::ColorRole(
                FloatingToolbarTokens::VIBRANT_BUTTON_UNSELECTED_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "FloatingToolbarTokens",
            name: "VibrantContainerColor",
            value: TokenValue::ColorRole(FloatingToolbarTokens::VIBRANT_CONTAINER_COLOR),
        },
    ];
}
