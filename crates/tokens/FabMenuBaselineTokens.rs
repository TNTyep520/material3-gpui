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

use super::{Dp, ElevationTokens, ShapeKeyTokens, ShapeToken, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct FabMenuBaselineTokens;
impl FabMenuBaselineTokens {
    pub const CLOSE_BUTTON_BETWEEN_SPACE: Dp = Dp(8.0);
    pub const CLOSE_BUTTON_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CLOSE_BUTTON_CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CLOSE_BUTTON_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CLOSE_BUTTON_CONTAINER_WIDTH: Dp = Dp(56.0);
    pub const CLOSE_BUTTON_ICON_SIZE: Dp = Dp(20.0);
    pub const LIST_ITEM_BETWEEN_SPACE: Dp = Dp(4.0);
    pub const LIST_ITEM_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const LIST_ITEM_CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const LIST_ITEM_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const LIST_ITEM_ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const LIST_ITEM_ICON_SIZE: Dp = Dp(24.0);
    pub const LIST_ITEM_LEADING_SPACE: Dp = Dp(24.0);
    pub const LIST_ITEM_TRAILING_SPACE: Dp = Dp(24.0);
}
impl FabMenuBaselineTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "CloseButtonBetweenSpace",
            value: TokenValue::Dp(FabMenuBaselineTokens::CLOSE_BUTTON_BETWEEN_SPACE),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "CloseButtonContainerElevation",
            value: TokenValue::Dp(FabMenuBaselineTokens::CLOSE_BUTTON_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "CloseButtonContainerHeight",
            value: TokenValue::Dp(FabMenuBaselineTokens::CLOSE_BUTTON_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "CloseButtonContainerShape",
            value: TokenValue::ShapeRole(FabMenuBaselineTokens::CLOSE_BUTTON_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "CloseButtonContainerWidth",
            value: TokenValue::Dp(FabMenuBaselineTokens::CLOSE_BUTTON_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "CloseButtonIconSize",
            value: TokenValue::Dp(FabMenuBaselineTokens::CLOSE_BUTTON_ICON_SIZE),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "ListItemBetweenSpace",
            value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_BETWEEN_SPACE),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "ListItemContainerElevation",
            value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "ListItemContainerHeight",
            value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "ListItemContainerShape",
            value: TokenValue::ShapeRole(FabMenuBaselineTokens::LIST_ITEM_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "ListItemIconLabelSpace",
            value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "ListItemIconSize",
            value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_ICON_SIZE),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "ListItemLeadingSpace",
            value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_LEADING_SPACE),
        },
        TokenEntry {
            group: "FabMenuBaselineTokens",
            name: "ListItemTrailingSpace",
            value: TokenValue::Dp(FabMenuBaselineTokens::LIST_ITEM_TRAILING_SPACE),
        },
    ];
}
