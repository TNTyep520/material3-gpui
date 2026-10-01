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
pub struct NavigationRailCollapsedTokens;
impl NavigationRailCollapsedTokens {
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const CONTAINER_WIDTH: Dp = Dp(96.0);
    pub const ITEM_VERTICAL_SPACE: Dp = Dp(4.0);
    pub const TOP_SPACE: Dp = Dp(44.0);
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const NARROW_CONTAINER_WIDTH: Dp = Dp(80.0);
}
impl NavigationRailCollapsedTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "NavigationRailCollapsedTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(NavigationRailCollapsedTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "NavigationRailCollapsedTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(NavigationRailCollapsedTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "NavigationRailCollapsedTokens",
            name: "ContainerWidth",
            value: TokenValue::Dp(NavigationRailCollapsedTokens::CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "NavigationRailCollapsedTokens",
            name: "ItemVerticalSpace",
            value: TokenValue::Dp(NavigationRailCollapsedTokens::ITEM_VERTICAL_SPACE),
        },
        TokenEntry {
            group: "NavigationRailCollapsedTokens",
            name: "TopSpace",
            value: TokenValue::Dp(NavigationRailCollapsedTokens::TOP_SPACE),
        },
        TokenEntry {
            group: "NavigationRailCollapsedTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(NavigationRailCollapsedTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "NavigationRailCollapsedTokens",
            name: "NarrowContainerWidth",
            value: TokenValue::Dp(NavigationRailCollapsedTokens::NARROW_CONTAINER_WIDTH),
        },
    ];
}
