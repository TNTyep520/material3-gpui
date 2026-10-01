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
pub struct ExtendedFabSmallTokens;
impl ExtendedFabSmallTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const LEADING_SPACE: Dp = Dp(16.0);
    pub const TRAILING_SPACE: Dp = Dp(16.0);
}
impl ExtendedFabSmallTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ExtendedFabSmallTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(ExtendedFabSmallTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ExtendedFabSmallTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(ExtendedFabSmallTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ExtendedFabSmallTokens",
            name: "IconLabelSpace",
            value: TokenValue::Dp(ExtendedFabSmallTokens::ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "ExtendedFabSmallTokens",
            name: "IconSize",
            value: TokenValue::Dp(ExtendedFabSmallTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "ExtendedFabSmallTokens",
            name: "LeadingSpace",
            value: TokenValue::Dp(ExtendedFabSmallTokens::LEADING_SPACE),
        },
        TokenEntry {
            group: "ExtendedFabSmallTokens",
            name: "TrailingSpace",
            value: TokenValue::Dp(ExtendedFabSmallTokens::TRAILING_SPACE),
        },
    ];
}
