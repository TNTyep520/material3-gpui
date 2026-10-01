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

use super::{Dp, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct ExtendedFabMediumTokens;
impl ExtendedFabMediumTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(80.0);
    pub const ICON_LABEL_SPACE: Dp = Dp(16.0);
    pub const ICON_SIZE: Dp = Dp(28.0);
    pub const LEADING_SPACE: Dp = Dp(26.0);
    pub const TRAILING_SPACE: Dp = Dp(26.0);
}
impl ExtendedFabMediumTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ExtendedFabMediumTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(ExtendedFabMediumTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ExtendedFabMediumTokens",
            name: "IconLabelSpace",
            value: TokenValue::Dp(ExtendedFabMediumTokens::ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "ExtendedFabMediumTokens",
            name: "IconSize",
            value: TokenValue::Dp(ExtendedFabMediumTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "ExtendedFabMediumTokens",
            name: "LeadingSpace",
            value: TokenValue::Dp(ExtendedFabMediumTokens::LEADING_SPACE),
        },
        TokenEntry {
            group: "ExtendedFabMediumTokens",
            name: "TrailingSpace",
            value: TokenValue::Dp(ExtendedFabMediumTokens::TRAILING_SPACE),
        },
    ];
}
