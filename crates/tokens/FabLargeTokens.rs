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
pub struct FabLargeTokens;
impl FabLargeTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(96.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const CONTAINER_WIDTH: Dp = Dp(96.0);
    pub const ICON_SIZE: Dp = Dp(32.0);
}
impl FabLargeTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FabLargeTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(FabLargeTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "FabLargeTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(FabLargeTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "FabLargeTokens",
            name: "ContainerWidth",
            value: TokenValue::Dp(FabLargeTokens::CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "FabLargeTokens",
            name: "IconSize",
            value: TokenValue::Dp(FabLargeTokens::ICON_SIZE),
        },
    ];
}
