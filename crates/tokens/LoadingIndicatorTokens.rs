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
    ColorSchemeKeyTokens, ColorToken, Dp, ShapeKeyTokens, ShapeToken, TokenEntry, TokenValue,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct LoadingIndicatorTokens;
impl LoadingIndicatorTokens {
    pub const ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_SIZE: Dp = Dp(38.0);
    pub const CONTAINED_ACTIVE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const CONTAINED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const CONTAINER_HEIGHT: Dp = Dp(48.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_WIDTH: Dp = Dp(48.0);
}
impl LoadingIndicatorTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "LoadingIndicatorTokens",
            name: "ActiveIndicatorColor",
            value: TokenValue::ColorRole(LoadingIndicatorTokens::ACTIVE_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "LoadingIndicatorTokens",
            name: "ActiveSize",
            value: TokenValue::Dp(LoadingIndicatorTokens::ACTIVE_SIZE),
        },
        TokenEntry {
            group: "LoadingIndicatorTokens",
            name: "ContainedActiveColor",
            value: TokenValue::ColorRole(LoadingIndicatorTokens::CONTAINED_ACTIVE_COLOR),
        },
        TokenEntry {
            group: "LoadingIndicatorTokens",
            name: "ContainedContainerColor",
            value: TokenValue::ColorRole(LoadingIndicatorTokens::CONTAINED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "LoadingIndicatorTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(LoadingIndicatorTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "LoadingIndicatorTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(LoadingIndicatorTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "LoadingIndicatorTokens",
            name: "ContainerWidth",
            value: TokenValue::Dp(LoadingIndicatorTokens::CONTAINER_WIDTH),
        },
    ];
}
