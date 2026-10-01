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
pub struct NavigationBarTokens;
impl NavigationBarTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const CONTAINER_HEIGHT: Dp = Dp(64.0);
    pub const ITEM_ACTIVE_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_INDICATOR_ICON_LABEL_SPACE: Dp = Dp(4.0);
    pub const ITEM_ACTIVE_INDICATOR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ITEM_ACTIVE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const ITEM_BETWEEN_SPACE: Dp = Dp(0.0);
    pub const ITEM_INACTIVE_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_INACTIVE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const NAV_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const TALL_CONTAINER_HEIGHT: Dp = Dp(80.0);
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_MEDIUM;
}
impl NavigationBarTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "NavigationBarTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(NavigationBarTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(NavigationBarTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(NavigationBarTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "ItemActiveIconColor",
            value: TokenValue::ColorRole(NavigationBarTokens::ITEM_ACTIVE_ICON_COLOR),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "ItemActiveIndicatorColor",
            value: TokenValue::ColorRole(NavigationBarTokens::ITEM_ACTIVE_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "ItemActiveIndicatorIconLabelSpace",
            value: TokenValue::Dp(NavigationBarTokens::ITEM_ACTIVE_INDICATOR_ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "ItemActiveIndicatorShape",
            value: TokenValue::ShapeRole(NavigationBarTokens::ITEM_ACTIVE_INDICATOR_SHAPE),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "ItemActiveLabelTextColor",
            value: TokenValue::ColorRole(NavigationBarTokens::ITEM_ACTIVE_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "ItemBetweenSpace",
            value: TokenValue::Dp(NavigationBarTokens::ITEM_BETWEEN_SPACE),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "ItemInactiveIconColor",
            value: TokenValue::ColorRole(NavigationBarTokens::ITEM_INACTIVE_ICON_COLOR),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "ItemInactiveLabelTextColor",
            value: TokenValue::ColorRole(NavigationBarTokens::ITEM_INACTIVE_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "NavShape",
            value: TokenValue::ShapeRole(NavigationBarTokens::NAV_SHAPE),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "TallContainerHeight",
            value: TokenValue::Dp(NavigationBarTokens::TALL_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "NavigationBarTokens",
            name: "LabelTextFont",
            value: TokenValue::TypographyRole(NavigationBarTokens::LABEL_TEXT_FONT),
        },
    ];
}
