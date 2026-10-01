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
pub struct DockedToolbarTokens;
impl DockedToolbarTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const CONTAINER_HEIGHT: Dp = Dp(64.0);
    pub const CONTAINER_LEADING_SPACE: Dp = Dp(16.0);
    pub const CONTAINER_MAX_SPACING: Dp = Dp(32.0);
    pub const CONTAINER_MIN_SPACING: Dp = Dp(4.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const CONTAINER_TRAILING_SPACE: Dp = Dp(16.0);
}
impl DockedToolbarTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "DockedToolbarTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(DockedToolbarTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "DockedToolbarTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(DockedToolbarTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "DockedToolbarTokens",
            name: "ContainerLeadingSpace",
            value: TokenValue::Dp(DockedToolbarTokens::CONTAINER_LEADING_SPACE),
        },
        TokenEntry {
            group: "DockedToolbarTokens",
            name: "ContainerMaxSpacing",
            value: TokenValue::Dp(DockedToolbarTokens::CONTAINER_MAX_SPACING),
        },
        TokenEntry {
            group: "DockedToolbarTokens",
            name: "ContainerMinSpacing",
            value: TokenValue::Dp(DockedToolbarTokens::CONTAINER_MIN_SPACING),
        },
        TokenEntry {
            group: "DockedToolbarTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(DockedToolbarTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "DockedToolbarTokens",
            name: "ContainerTrailingSpace",
            value: TokenValue::Dp(DockedToolbarTokens::CONTAINER_TRAILING_SPACE),
        },
    ];
}
