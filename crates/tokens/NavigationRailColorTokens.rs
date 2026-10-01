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

use super::{ColorSchemeKeyTokens, ColorToken, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationRailColorTokens;
impl NavigationRailColorTokens {
    pub const ITEM_ACTIVE_FOCUSED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_HOVERED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_ICON: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_INDICATOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const ITEM_ACTIVE_LABEL_TEXT: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const ITEM_ACTIVE_PRESSED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_INACTIVE_FOCUSED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_INACTIVE_HOVERED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_INACTIVE_ICON: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_INACTIVE_LABEL_TEXT: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_INACTIVE_PRESSED_STATE_LAYER: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
}
impl NavigationRailColorTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "NavigationRailColorTokens",
            name: "ItemActiveFocusedStateLayer",
            value: TokenValue::ColorRole(
                NavigationRailColorTokens::ITEM_ACTIVE_FOCUSED_STATE_LAYER,
            ),
        },
        TokenEntry {
            group: "NavigationRailColorTokens",
            name: "ItemActiveHoveredStateLayer",
            value: TokenValue::ColorRole(
                NavigationRailColorTokens::ITEM_ACTIVE_HOVERED_STATE_LAYER,
            ),
        },
        TokenEntry {
            group: "NavigationRailColorTokens",
            name: "ItemActiveIcon",
            value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_ACTIVE_ICON),
        },
        TokenEntry {
            group: "NavigationRailColorTokens",
            name: "ItemActiveIndicator",
            value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_ACTIVE_INDICATOR),
        },
        TokenEntry {
            group: "NavigationRailColorTokens",
            name: "ItemActiveLabelText",
            value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_ACTIVE_LABEL_TEXT),
        },
        TokenEntry {
            group: "NavigationRailColorTokens",
            name: "ItemActivePressedStateLayer",
            value: TokenValue::ColorRole(
                NavigationRailColorTokens::ITEM_ACTIVE_PRESSED_STATE_LAYER,
            ),
        },
        TokenEntry {
            group: "NavigationRailColorTokens",
            name: "ItemInactiveFocusedStateLayer",
            value: TokenValue::ColorRole(
                NavigationRailColorTokens::ITEM_INACTIVE_FOCUSED_STATE_LAYER,
            ),
        },
        TokenEntry {
            group: "NavigationRailColorTokens",
            name: "ItemInactiveHoveredStateLayer",
            value: TokenValue::ColorRole(
                NavigationRailColorTokens::ITEM_INACTIVE_HOVERED_STATE_LAYER,
            ),
        },
        TokenEntry {
            group: "NavigationRailColorTokens",
            name: "ItemInactiveIcon",
            value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_INACTIVE_ICON),
        },
        TokenEntry {
            group: "NavigationRailColorTokens",
            name: "ItemInactiveLabelText",
            value: TokenValue::ColorRole(NavigationRailColorTokens::ITEM_INACTIVE_LABEL_TEXT),
        },
        TokenEntry {
            group: "NavigationRailColorTokens",
            name: "ItemInactivePressedStateLayer",
            value: TokenValue::ColorRole(
                NavigationRailColorTokens::ITEM_INACTIVE_PRESSED_STATE_LAYER,
            ),
        },
    ];
}
