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

use super::{ColorValue, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct PaletteTokens;
impl PaletteTokens {
    pub const BLACK: ColorValue = ColorValue(0x000000);
    pub const ERROR0: ColorValue = ColorValue(0x000000);
    pub const ERROR10: ColorValue = ColorValue(0x410e0b);
    pub const ERROR100: ColorValue = ColorValue(0xffffff);
    pub const ERROR20: ColorValue = ColorValue(0x601410);
    pub const ERROR30: ColorValue = ColorValue(0x8c1d18);
    pub const ERROR40: ColorValue = ColorValue(0xb3261e);
    pub const ERROR50: ColorValue = ColorValue(0xdc362e);
    pub const ERROR60: ColorValue = ColorValue(0xe46962);
    pub const ERROR70: ColorValue = ColorValue(0xec928e);
    pub const ERROR80: ColorValue = ColorValue(0xf2b8b5);
    pub const ERROR90: ColorValue = ColorValue(0xf9dedc);
    pub const ERROR95: ColorValue = ColorValue(0xfceeee);
    pub const ERROR99: ColorValue = ColorValue(0xfffbf9);
    pub const NEUTRAL0: ColorValue = ColorValue(0x000000);
    pub const NEUTRAL10: ColorValue = ColorValue(0x1d1b20);
    pub const NEUTRAL100: ColorValue = ColorValue(0xffffff);
    pub const NEUTRAL12: ColorValue = ColorValue(0x211f26);
    pub const NEUTRAL17: ColorValue = ColorValue(0x2b2930);
    pub const NEUTRAL20: ColorValue = ColorValue(0x322f35);
    pub const NEUTRAL22: ColorValue = ColorValue(0x36343b);
    pub const NEUTRAL24: ColorValue = ColorValue(0x3b383e);
    pub const NEUTRAL30: ColorValue = ColorValue(0x48464c);
    pub const NEUTRAL4: ColorValue = ColorValue(0x0f0d13);
    pub const NEUTRAL40: ColorValue = ColorValue(0x605d64);
    pub const NEUTRAL50: ColorValue = ColorValue(0x79767d);
    pub const NEUTRAL6: ColorValue = ColorValue(0x141218);
    pub const NEUTRAL60: ColorValue = ColorValue(0x938f96);
    pub const NEUTRAL70: ColorValue = ColorValue(0xaea9b1);
    pub const NEUTRAL80: ColorValue = ColorValue(0xcac5cd);
    pub const NEUTRAL87: ColorValue = ColorValue(0xded8e1);
    pub const NEUTRAL90: ColorValue = ColorValue(0xe6e0e9);
    pub const NEUTRAL92: ColorValue = ColorValue(0xece6f0);
    pub const NEUTRAL94: ColorValue = ColorValue(0xf3edf7);
    pub const NEUTRAL95: ColorValue = ColorValue(0xf5eff7);
    pub const NEUTRAL96: ColorValue = ColorValue(0xf7f2fa);
    pub const NEUTRAL98: ColorValue = ColorValue(0xfef7ff);
    pub const NEUTRAL99: ColorValue = ColorValue(0xfffbff);
    pub const NEUTRAL_VARIANT0: ColorValue = ColorValue(0x000000);
    pub const NEUTRAL_VARIANT10: ColorValue = ColorValue(0x1d1a22);
    pub const NEUTRAL_VARIANT100: ColorValue = ColorValue(0xffffff);
    pub const NEUTRAL_VARIANT20: ColorValue = ColorValue(0x322f37);
    pub const NEUTRAL_VARIANT30: ColorValue = ColorValue(0x49454f);
    pub const NEUTRAL_VARIANT40: ColorValue = ColorValue(0x605d66);
    pub const NEUTRAL_VARIANT50: ColorValue = ColorValue(0x79747e);
    pub const NEUTRAL_VARIANT60: ColorValue = ColorValue(0x938f99);
    pub const NEUTRAL_VARIANT70: ColorValue = ColorValue(0xaea9b4);
    pub const NEUTRAL_VARIANT80: ColorValue = ColorValue(0xcac4d0);
    pub const NEUTRAL_VARIANT90: ColorValue = ColorValue(0xe7e0ec);
    pub const NEUTRAL_VARIANT95: ColorValue = ColorValue(0xf5eefa);
    pub const NEUTRAL_VARIANT99: ColorValue = ColorValue(0xfffbfe);
    pub const PRIMARY0: ColorValue = ColorValue(0x000000);
    pub const PRIMARY10: ColorValue = ColorValue(0x21005d);
    pub const PRIMARY100: ColorValue = ColorValue(0xffffff);
    pub const PRIMARY20: ColorValue = ColorValue(0x381e72);
    pub const PRIMARY30: ColorValue = ColorValue(0x4f378b);
    pub const PRIMARY40: ColorValue = ColorValue(0x6750a4);
    pub const PRIMARY50: ColorValue = ColorValue(0x7f67be);
    pub const PRIMARY60: ColorValue = ColorValue(0x9a82db);
    pub const PRIMARY70: ColorValue = ColorValue(0xb69df8);
    pub const PRIMARY80: ColorValue = ColorValue(0xd0bcff);
    pub const PRIMARY90: ColorValue = ColorValue(0xeaddff);
    pub const PRIMARY95: ColorValue = ColorValue(0xf6edff);
    pub const PRIMARY99: ColorValue = ColorValue(0xfffbfe);
    pub const SECONDARY0: ColorValue = ColorValue(0x000000);
    pub const SECONDARY10: ColorValue = ColorValue(0x1d192b);
    pub const SECONDARY100: ColorValue = ColorValue(0xffffff);
    pub const SECONDARY20: ColorValue = ColorValue(0x332d41);
    pub const SECONDARY30: ColorValue = ColorValue(0x4a4458);
    pub const SECONDARY40: ColorValue = ColorValue(0x625b71);
    pub const SECONDARY50: ColorValue = ColorValue(0x7a7289);
    pub const SECONDARY60: ColorValue = ColorValue(0x958da5);
    pub const SECONDARY70: ColorValue = ColorValue(0xb0a7c0);
    pub const SECONDARY80: ColorValue = ColorValue(0xccc2dc);
    pub const SECONDARY90: ColorValue = ColorValue(0xe8def8);
    pub const SECONDARY95: ColorValue = ColorValue(0xf6edff);
    pub const SECONDARY99: ColorValue = ColorValue(0xfffbfe);
    pub const TERTIARY0: ColorValue = ColorValue(0x000000);
    pub const TERTIARY10: ColorValue = ColorValue(0x31111d);
    pub const TERTIARY100: ColorValue = ColorValue(0xffffff);
    pub const TERTIARY20: ColorValue = ColorValue(0x492532);
    pub const TERTIARY30: ColorValue = ColorValue(0x633b48);
    pub const TERTIARY40: ColorValue = ColorValue(0x7d5260);
    pub const TERTIARY50: ColorValue = ColorValue(0x986977);
    pub const TERTIARY60: ColorValue = ColorValue(0xb58392);
    pub const TERTIARY70: ColorValue = ColorValue(0xd29dac);
    pub const TERTIARY80: ColorValue = ColorValue(0xefb8c8);
    pub const TERTIARY90: ColorValue = ColorValue(0xffd8e4);
    pub const TERTIARY95: ColorValue = ColorValue(0xffecf1);
    pub const TERTIARY99: ColorValue = ColorValue(0xfffbfa);
    pub const WHITE: ColorValue = ColorValue(0xffffff);
}
impl PaletteTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "PaletteTokens",
            name: "Black",
            value: TokenValue::Color(PaletteTokens::BLACK),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error0",
            value: TokenValue::Color(PaletteTokens::ERROR0),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error10",
            value: TokenValue::Color(PaletteTokens::ERROR10),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error100",
            value: TokenValue::Color(PaletteTokens::ERROR100),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error20",
            value: TokenValue::Color(PaletteTokens::ERROR20),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error30",
            value: TokenValue::Color(PaletteTokens::ERROR30),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error40",
            value: TokenValue::Color(PaletteTokens::ERROR40),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error50",
            value: TokenValue::Color(PaletteTokens::ERROR50),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error60",
            value: TokenValue::Color(PaletteTokens::ERROR60),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error70",
            value: TokenValue::Color(PaletteTokens::ERROR70),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error80",
            value: TokenValue::Color(PaletteTokens::ERROR80),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error90",
            value: TokenValue::Color(PaletteTokens::ERROR90),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error95",
            value: TokenValue::Color(PaletteTokens::ERROR95),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Error99",
            value: TokenValue::Color(PaletteTokens::ERROR99),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral0",
            value: TokenValue::Color(PaletteTokens::NEUTRAL0),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral10",
            value: TokenValue::Color(PaletteTokens::NEUTRAL10),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral100",
            value: TokenValue::Color(PaletteTokens::NEUTRAL100),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral12",
            value: TokenValue::Color(PaletteTokens::NEUTRAL12),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral17",
            value: TokenValue::Color(PaletteTokens::NEUTRAL17),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral20",
            value: TokenValue::Color(PaletteTokens::NEUTRAL20),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral22",
            value: TokenValue::Color(PaletteTokens::NEUTRAL22),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral24",
            value: TokenValue::Color(PaletteTokens::NEUTRAL24),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral30",
            value: TokenValue::Color(PaletteTokens::NEUTRAL30),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral4",
            value: TokenValue::Color(PaletteTokens::NEUTRAL4),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral40",
            value: TokenValue::Color(PaletteTokens::NEUTRAL40),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral50",
            value: TokenValue::Color(PaletteTokens::NEUTRAL50),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral6",
            value: TokenValue::Color(PaletteTokens::NEUTRAL6),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral60",
            value: TokenValue::Color(PaletteTokens::NEUTRAL60),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral70",
            value: TokenValue::Color(PaletteTokens::NEUTRAL70),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral80",
            value: TokenValue::Color(PaletteTokens::NEUTRAL80),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral87",
            value: TokenValue::Color(PaletteTokens::NEUTRAL87),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral90",
            value: TokenValue::Color(PaletteTokens::NEUTRAL90),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral92",
            value: TokenValue::Color(PaletteTokens::NEUTRAL92),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral94",
            value: TokenValue::Color(PaletteTokens::NEUTRAL94),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral95",
            value: TokenValue::Color(PaletteTokens::NEUTRAL95),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral96",
            value: TokenValue::Color(PaletteTokens::NEUTRAL96),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral98",
            value: TokenValue::Color(PaletteTokens::NEUTRAL98),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Neutral99",
            value: TokenValue::Color(PaletteTokens::NEUTRAL99),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant0",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT0),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant10",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT10),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant100",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT100),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant20",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT20),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant30",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT30),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant40",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT40),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant50",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT50),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant60",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT60),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant70",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT70),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant80",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT80),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant90",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT90),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant95",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT95),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "NeutralVariant99",
            value: TokenValue::Color(PaletteTokens::NEUTRAL_VARIANT99),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary0",
            value: TokenValue::Color(PaletteTokens::PRIMARY0),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary10",
            value: TokenValue::Color(PaletteTokens::PRIMARY10),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary100",
            value: TokenValue::Color(PaletteTokens::PRIMARY100),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary20",
            value: TokenValue::Color(PaletteTokens::PRIMARY20),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary30",
            value: TokenValue::Color(PaletteTokens::PRIMARY30),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary40",
            value: TokenValue::Color(PaletteTokens::PRIMARY40),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary50",
            value: TokenValue::Color(PaletteTokens::PRIMARY50),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary60",
            value: TokenValue::Color(PaletteTokens::PRIMARY60),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary70",
            value: TokenValue::Color(PaletteTokens::PRIMARY70),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary80",
            value: TokenValue::Color(PaletteTokens::PRIMARY80),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary90",
            value: TokenValue::Color(PaletteTokens::PRIMARY90),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary95",
            value: TokenValue::Color(PaletteTokens::PRIMARY95),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Primary99",
            value: TokenValue::Color(PaletteTokens::PRIMARY99),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary0",
            value: TokenValue::Color(PaletteTokens::SECONDARY0),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary10",
            value: TokenValue::Color(PaletteTokens::SECONDARY10),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary100",
            value: TokenValue::Color(PaletteTokens::SECONDARY100),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary20",
            value: TokenValue::Color(PaletteTokens::SECONDARY20),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary30",
            value: TokenValue::Color(PaletteTokens::SECONDARY30),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary40",
            value: TokenValue::Color(PaletteTokens::SECONDARY40),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary50",
            value: TokenValue::Color(PaletteTokens::SECONDARY50),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary60",
            value: TokenValue::Color(PaletteTokens::SECONDARY60),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary70",
            value: TokenValue::Color(PaletteTokens::SECONDARY70),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary80",
            value: TokenValue::Color(PaletteTokens::SECONDARY80),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary90",
            value: TokenValue::Color(PaletteTokens::SECONDARY90),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary95",
            value: TokenValue::Color(PaletteTokens::SECONDARY95),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Secondary99",
            value: TokenValue::Color(PaletteTokens::SECONDARY99),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary0",
            value: TokenValue::Color(PaletteTokens::TERTIARY0),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary10",
            value: TokenValue::Color(PaletteTokens::TERTIARY10),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary100",
            value: TokenValue::Color(PaletteTokens::TERTIARY100),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary20",
            value: TokenValue::Color(PaletteTokens::TERTIARY20),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary30",
            value: TokenValue::Color(PaletteTokens::TERTIARY30),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary40",
            value: TokenValue::Color(PaletteTokens::TERTIARY40),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary50",
            value: TokenValue::Color(PaletteTokens::TERTIARY50),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary60",
            value: TokenValue::Color(PaletteTokens::TERTIARY60),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary70",
            value: TokenValue::Color(PaletteTokens::TERTIARY70),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary80",
            value: TokenValue::Color(PaletteTokens::TERTIARY80),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary90",
            value: TokenValue::Color(PaletteTokens::TERTIARY90),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary95",
            value: TokenValue::Color(PaletteTokens::TERTIARY95),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "Tertiary99",
            value: TokenValue::Color(PaletteTokens::TERTIARY99),
        },
        TokenEntry {
            group: "PaletteTokens",
            name: "White",
            value: TokenValue::Color(PaletteTokens::WHITE),
        },
    ];
}
