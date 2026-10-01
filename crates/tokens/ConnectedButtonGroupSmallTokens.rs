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

use super::{Dp, ShapeKeyTokens, ShapeToken, ShapeTokens, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectedButtonGroupSmallTokens;
impl ConnectedButtonGroupSmallTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_SMALL;
    pub const PRESSED_INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_EXTRA_SMALL;
    pub const SELECTED_INNER_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
}
impl ConnectedButtonGroupSmallTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ConnectedButtonGroupSmallTokens",
            name: "BetweenSpace",
            value: TokenValue::Dp(ConnectedButtonGroupSmallTokens::BETWEEN_SPACE),
        },
        TokenEntry {
            group: "ConnectedButtonGroupSmallTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(ConnectedButtonGroupSmallTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ConnectedButtonGroupSmallTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(ConnectedButtonGroupSmallTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ConnectedButtonGroupSmallTokens",
            name: "InnerCornerCornerSize",
            value: TokenValue::Dp(ConnectedButtonGroupSmallTokens::INNER_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "ConnectedButtonGroupSmallTokens",
            name: "PressedInnerCornerCornerSize",
            value: TokenValue::Dp(
                ConnectedButtonGroupSmallTokens::PRESSED_INNER_CORNER_CORNER_SIZE,
            ),
        },
        TokenEntry {
            group: "ConnectedButtonGroupSmallTokens",
            name: "SelectedInnerCornerCornerSizePercent",
            value: TokenValue::Float(
                ConnectedButtonGroupSmallTokens::SELECTED_INNER_CORNER_CORNER_SIZE_PERCENT,
            ),
        },
    ];
}
