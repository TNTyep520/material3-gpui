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
    TokenValue,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct AppBarTokens;
impl AppBarTokens {
    pub const AVATAR_SIZE: Dp = Dp(32.0);
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const ICON_BUTTON_SPACE: Dp = Dp(0.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const LEADING_SPACE: Dp = Dp(4.0);
    pub const ON_SCROLL_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const ON_SCROLL_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const SUBTITLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TITLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TRAILING_SPACE: Dp = Dp(4.0);
}
impl AppBarTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "AppBarTokens",
            name: "AvatarSize",
            value: TokenValue::Dp(AppBarTokens::AVATAR_SIZE),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(AppBarTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(AppBarTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(AppBarTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "IconButtonSpace",
            value: TokenValue::Dp(AppBarTokens::ICON_BUTTON_SPACE),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "IconSize",
            value: TokenValue::Dp(AppBarTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "LeadingIconColor",
            value: TokenValue::ColorRole(AppBarTokens::LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "LeadingSpace",
            value: TokenValue::Dp(AppBarTokens::LEADING_SPACE),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "OnScrollContainerColor",
            value: TokenValue::ColorRole(AppBarTokens::ON_SCROLL_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "OnScrollContainerElevation",
            value: TokenValue::Dp(AppBarTokens::ON_SCROLL_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "SubtitleColor",
            value: TokenValue::ColorRole(AppBarTokens::SUBTITLE_COLOR),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "TitleColor",
            value: TokenValue::ColorRole(AppBarTokens::TITLE_COLOR),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "TrailingIconColor",
            value: TokenValue::ColorRole(AppBarTokens::TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "AppBarTokens",
            name: "TrailingSpace",
            value: TokenValue::Dp(AppBarTokens::TRAILING_SPACE),
        },
    ];
}
