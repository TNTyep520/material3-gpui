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

use super::{FontFamilyToken, TokenEntry, TokenValue};
use gpui::FontWeight;

#[derive(Clone, Copy, Debug, Default)]
pub struct TypefaceTokens;
impl TypefaceTokens {
    pub const BRAND: FontFamilyToken = FontFamilyToken::SansSerif;
    pub const PLAIN: FontFamilyToken = FontFamilyToken::SansSerif;
    pub const WEIGHT_BOLD: FontWeight = FontWeight::BOLD;
    pub const WEIGHT_MEDIUM: FontWeight = FontWeight::MEDIUM;
    pub const WEIGHT_REGULAR: FontWeight = FontWeight::NORMAL;
}
impl TypefaceTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "TypefaceTokens",
            name: "Brand",
            value: TokenValue::FontFamily(TypefaceTokens::BRAND),
        },
        TokenEntry {
            group: "TypefaceTokens",
            name: "Plain",
            value: TokenValue::FontFamily(TypefaceTokens::PLAIN),
        },
        TokenEntry {
            group: "TypefaceTokens",
            name: "WeightBold",
            value: TokenValue::FontWeight(TypefaceTokens::WEIGHT_BOLD),
        },
        TokenEntry {
            group: "TypefaceTokens",
            name: "WeightMedium",
            value: TokenValue::FontWeight(TypefaceTokens::WEIGHT_MEDIUM),
        },
        TokenEntry {
            group: "TypefaceTokens",
            name: "WeightRegular",
            value: TokenValue::FontWeight(TypefaceTokens::WEIGHT_REGULAR),
        },
    ];
}
