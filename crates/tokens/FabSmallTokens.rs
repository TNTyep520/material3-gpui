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
pub struct FabSmallTokens;
impl FabSmallTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const CONTAINER_WIDTH: Dp = Dp(40.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
}
impl FabSmallTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FabSmallTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(FabSmallTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "FabSmallTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(FabSmallTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "FabSmallTokens",
            name: "ContainerWidth",
            value: TokenValue::Dp(FabSmallTokens::CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "FabSmallTokens",
            name: "IconSize",
            value: TokenValue::Dp(FabSmallTokens::ICON_SIZE),
        },
    ];
}
