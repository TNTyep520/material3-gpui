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

use super::{ColorSchemeKeyTokens, ColorToken, ShapeKeyTokens, ShapeToken, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct ExpandedListTokens;
impl ExpandedListTokens {
    pub const COLLAPSED_ITEM_TRAILING_ICON_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE;
    pub const COLLAPSED_ITEM_TRAILING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const EXPANDED_ITEM_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const EXPANDED_ITEM_SEGMENTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const EXPANDED_ITEM_TRAILING_ICON_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const EXPANDED_ITEM_TRAILING_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TRAILING_ICON_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
}
impl ExpandedListTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ExpandedListTokens",
            name: "CollapsedItemTrailingIconContainerColor",
            value: TokenValue::ColorRole(
                ExpandedListTokens::COLLAPSED_ITEM_TRAILING_ICON_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "ExpandedListTokens",
            name: "CollapsedItemTrailingIconIconColor",
            value: TokenValue::ColorRole(
                ExpandedListTokens::COLLAPSED_ITEM_TRAILING_ICON_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "ExpandedListTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(ExpandedListTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ExpandedListTokens",
            name: "ExpandedItemContainerColor",
            value: TokenValue::ColorRole(ExpandedListTokens::EXPANDED_ITEM_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ExpandedListTokens",
            name: "ExpandedItemSegmentedContainerColor",
            value: TokenValue::ColorRole(
                ExpandedListTokens::EXPANDED_ITEM_SEGMENTED_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "ExpandedListTokens",
            name: "ExpandedItemTrailingIconContainerColor",
            value: TokenValue::ColorRole(
                ExpandedListTokens::EXPANDED_ITEM_TRAILING_ICON_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "ExpandedListTokens",
            name: "ExpandedItemTrailingIconIconColor",
            value: TokenValue::ColorRole(
                ExpandedListTokens::EXPANDED_ITEM_TRAILING_ICON_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "ExpandedListTokens",
            name: "TrailingIconShape",
            value: TokenValue::ShapeRole(ExpandedListTokens::TRAILING_ICON_SHAPE),
        },
    ];
}
