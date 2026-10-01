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

use super::{ColorSchemeKeyTokens, ColorToken, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct ScrimTokens;
impl ScrimTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SCRIM;
    pub const CONTAINER_OPACITY: f32 = 0.32;
}
impl ScrimTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ScrimTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(ScrimTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ScrimTokens",
            name: "ContainerOpacity",
            value: TokenValue::Float(ScrimTokens::CONTAINER_OPACITY),
        },
    ];
}
