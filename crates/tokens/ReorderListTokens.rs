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
pub struct ReorderListTokens;
impl ReorderListTokens {
    pub const ITEM_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ITEM_DROP_ZONE_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const ITEM_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_OVERLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
}
impl ReorderListTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ReorderListTokens",
            name: "ItemContainerColor",
            value: TokenValue::ColorRole(ReorderListTokens::ITEM_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ReorderListTokens",
            name: "ItemDropZoneColor",
            value: TokenValue::ColorRole(ReorderListTokens::ITEM_DROP_ZONE_COLOR),
        },
        TokenEntry {
            group: "ReorderListTokens",
            name: "ItemLabelTextColor",
            value: TokenValue::ColorRole(ReorderListTokens::ITEM_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ReorderListTokens",
            name: "ItemLeadingIconColor",
            value: TokenValue::ColorRole(ReorderListTokens::ITEM_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ReorderListTokens",
            name: "ItemOverlineColor",
            value: TokenValue::ColorRole(ReorderListTokens::ITEM_OVERLINE_COLOR),
        },
        TokenEntry {
            group: "ReorderListTokens",
            name: "ItemShape",
            value: TokenValue::ShapeRole(ReorderListTokens::ITEM_SHAPE),
        },
        TokenEntry {
            group: "ReorderListTokens",
            name: "ItemSupportingTextColor",
            value: TokenValue::ColorRole(ReorderListTokens::ITEM_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "ReorderListTokens",
            name: "ItemTrailingIconColor",
            value: TokenValue::ColorRole(ReorderListTokens::ITEM_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ReorderListTokens",
            name: "ItemTrailingSupportingTextColor",
            value: TokenValue::ColorRole(ReorderListTokens::ITEM_TRAILING_SUPPORTING_TEXT_COLOR),
        },
    ];
}
