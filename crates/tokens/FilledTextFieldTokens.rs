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
    TypographyKeyTokens, TypographyToken,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct FilledTextFieldTokens;
impl FilledTextFieldTokens {
    pub const ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const CARET_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL_TOP;
    pub const DISABLED_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const DISABLED_ACTIVE_INDICATOR_OPACITY: f32 = 0.38;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.04;
    pub const DISABLED_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_INPUT_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_OPACITY: f32 = 0.38;
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_SUPPORTING_OPACITY: f32 = 0.38;
    pub const DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ERROR_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_FOCUS_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_FOCUS_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_HOVER_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_HOVER_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_HOVER_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_HOVER_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FOCUS_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(2.0);
    pub const FOCUS_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FOCUS_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUS_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const HOVER_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const INPUT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const INPUT_PLACEHOLDER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INPUT_PREFIX_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const INPUT_SUFFIX_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LEADING_ICON_SIZE: Dp = Dp(24.0);
    pub const SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SUPPORTING_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
    pub const TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TRAILING_ICON_SIZE: Dp = Dp(24.0);
}
impl FilledTextFieldTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ActiveIndicatorColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ACTIVE_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ActiveIndicatorHeight",
            value: TokenValue::Dp(FilledTextFieldTokens::ACTIVE_INDICATOR_HEIGHT),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "CaretColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::CARET_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(FilledTextFieldTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledActiveIndicatorColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_ACTIVE_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledActiveIndicatorHeight",
            value: TokenValue::Dp(FilledTextFieldTokens::DISABLED_ACTIVE_INDICATOR_HEIGHT),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledActiveIndicatorOpacity",
            value: TokenValue::Float(FilledTextFieldTokens::DISABLED_ACTIVE_INDICATOR_OPACITY),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledContainerColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledContainerOpacity",
            value: TokenValue::Float(FilledTextFieldTokens::DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledInputColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_INPUT_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledInputOpacity",
            value: TokenValue::Float(FilledTextFieldTokens::DISABLED_INPUT_OPACITY),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledLabelColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_LABEL_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledLabelOpacity",
            value: TokenValue::Float(FilledTextFieldTokens::DISABLED_LABEL_OPACITY),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledLeadingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledLeadingIconOpacity",
            value: TokenValue::Float(FilledTextFieldTokens::DISABLED_LEADING_ICON_OPACITY),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledSupportingColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledSupportingOpacity",
            value: TokenValue::Float(FilledTextFieldTokens::DISABLED_SUPPORTING_OPACITY),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledTrailingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::DISABLED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "DisabledTrailingIconOpacity",
            value: TokenValue::Float(FilledTextFieldTokens::DISABLED_TRAILING_ICON_OPACITY),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorActiveIndicatorColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_ACTIVE_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorFocusActiveIndicatorColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_ACTIVE_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorFocusCaretColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_CARET_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorFocusInputColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_INPUT_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorFocusLabelColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_LABEL_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorFocusLeadingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorFocusSupportingColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorFocusTrailingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_FOCUS_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorHoverActiveIndicatorColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_ACTIVE_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorHoverInputColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_INPUT_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorHoverLabelColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_LABEL_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorHoverLeadingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorHoverSupportingColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorHoverTrailingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_HOVER_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorInputColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_INPUT_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorLabelColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_LABEL_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorLeadingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorSupportingColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "ErrorTrailingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::ERROR_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "FocusActiveIndicatorColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_ACTIVE_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "FocusActiveIndicatorHeight",
            value: TokenValue::Dp(FilledTextFieldTokens::FOCUS_ACTIVE_INDICATOR_HEIGHT),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "FocusInputColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_INPUT_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "FocusLabelColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_LABEL_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "FocusLeadingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "FocusSupportingColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "FocusTrailingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::FOCUS_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "HoverActiveIndicatorColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_ACTIVE_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "HoverActiveIndicatorHeight",
            value: TokenValue::Dp(FilledTextFieldTokens::HOVER_ACTIVE_INDICATOR_HEIGHT),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "HoverInputColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_INPUT_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "HoverLabelColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_LABEL_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "HoverLeadingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "HoverSupportingColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "HoverTrailingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::HOVER_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "InputColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::INPUT_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "InputFont",
            value: TokenValue::TypographyRole(FilledTextFieldTokens::INPUT_FONT),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "InputPlaceholderColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::INPUT_PLACEHOLDER_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "InputPrefixColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::INPUT_PREFIX_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "InputSuffixColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::INPUT_SUFFIX_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "LabelColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::LABEL_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "LabelFont",
            value: TokenValue::TypographyRole(FilledTextFieldTokens::LABEL_FONT),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "LeadingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "LeadingIconSize",
            value: TokenValue::Dp(FilledTextFieldTokens::LEADING_ICON_SIZE),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "SupportingColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "SupportingFont",
            value: TokenValue::TypographyRole(FilledTextFieldTokens::SUPPORTING_FONT),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "TrailingIconColor",
            value: TokenValue::ColorRole(FilledTextFieldTokens::TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledTextFieldTokens",
            name: "TrailingIconSize",
            value: TokenValue::Dp(FilledTextFieldTokens::TRAILING_ICON_SIZE),
        },
    ];
}
