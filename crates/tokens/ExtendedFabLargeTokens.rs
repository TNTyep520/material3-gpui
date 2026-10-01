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

use super::{Dp, ShapeKeyTokens, ShapeToken, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct ExtendedFabLargeTokens;
impl ExtendedFabLargeTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(96.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const ICON_LABEL_SPACE: Dp = Dp(20.0);
    pub const ICON_SIZE: Dp = Dp(32.0);
    pub const LEADING_SPACE: Dp = Dp(28.0);
    pub const TRAILING_SPACE: Dp = Dp(28.0);
}
impl ExtendedFabLargeTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ExtendedFabLargeTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(ExtendedFabLargeTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ExtendedFabLargeTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(ExtendedFabLargeTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ExtendedFabLargeTokens",
            name: "IconLabelSpace",
            value: TokenValue::Dp(ExtendedFabLargeTokens::ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "ExtendedFabLargeTokens",
            name: "IconSize",
            value: TokenValue::Dp(ExtendedFabLargeTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "ExtendedFabLargeTokens",
            name: "LeadingSpace",
            value: TokenValue::Dp(ExtendedFabLargeTokens::LEADING_SPACE),
        },
        TokenEntry {
            group: "ExtendedFabLargeTokens",
            name: "TrailingSpace",
            value: TokenValue::Dp(ExtendedFabLargeTokens::TRAILING_SPACE),
        },
    ];
}
