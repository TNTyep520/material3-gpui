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

use super::{TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct StateTokens;
impl StateTokens {
    pub const DRAGGED_STATE_LAYER_OPACITY: f32 = 0.16;
    pub const FOCUS_STATE_LAYER_OPACITY: f32 = 0.1;
    pub const HOVER_STATE_LAYER_OPACITY: f32 = 0.08;
    pub const PRESSED_STATE_LAYER_OPACITY: f32 = 0.1;
}
impl StateTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "StateTokens",
            name: "DraggedStateLayerOpacity",
            value: TokenValue::Float(StateTokens::DRAGGED_STATE_LAYER_OPACITY),
        },
        TokenEntry {
            group: "StateTokens",
            name: "FocusStateLayerOpacity",
            value: TokenValue::Float(StateTokens::FOCUS_STATE_LAYER_OPACITY),
        },
        TokenEntry {
            group: "StateTokens",
            name: "HoverStateLayerOpacity",
            value: TokenValue::Float(StateTokens::HOVER_STATE_LAYER_OPACITY),
        },
        TokenEntry {
            group: "StateTokens",
            name: "PressedStateLayerOpacity",
            value: TokenValue::Float(StateTokens::PRESSED_STATE_LAYER_OPACITY),
        },
    ];
}
