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

use super::{Dp, TokenEntry, TokenValue, TypographyKeyTokens, TypographyToken};

#[derive(Clone, Copy, Debug, Default)]
pub struct AppBarLargeTokens;
impl AppBarLargeTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(152.0);
    pub const TITLE_FONT: TypographyToken = TypographyKeyTokens::HEADLINE_MEDIUM;
}
impl AppBarLargeTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "AppBarLargeTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(AppBarLargeTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "AppBarLargeTokens",
            name: "TitleFont",
            value: TokenValue::TypographyRole(AppBarLargeTokens::TITLE_FONT),
        },
    ];
}
