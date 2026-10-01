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
    TokenValue,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct BottomAppBarTokens;
impl BottomAppBarTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const CONTAINER_HEIGHT: Dp = Dp(80.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
}
impl BottomAppBarTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "BottomAppBarTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(BottomAppBarTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "BottomAppBarTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(BottomAppBarTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "BottomAppBarTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(BottomAppBarTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "BottomAppBarTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(BottomAppBarTokens::CONTAINER_SHAPE),
        },
    ];
}
