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
pub struct OutlinedTextFieldTokens;
impl OutlinedTextFieldTokens {
    pub const CARET_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const DISABLED_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_INPUT_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_OPACITY: f32 = 0.38;
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OUTLINE_OPACITY: f32 = 0.12;
    pub const DISABLED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const DISABLED_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_SUPPORTING_OPACITY: f32 = 0.38;
    pub const DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ERROR_FOCUS_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_FOCUS_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_HOVER_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_HOVER_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_HOVER_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const ERROR_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ERROR_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ERROR_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const ERROR_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FOCUS_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FOCUS_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const FOCUS_SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_INPUT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_LABEL_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_OUTLINE_WIDTH: Dp = Dp(1.0);
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
    pub const OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const SUPPORTING_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SUPPORTING_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
    pub const TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TRAILING_ICON_SIZE: Dp = Dp(24.0);
}
impl OutlinedTextFieldTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "CaretColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::CARET_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(OutlinedTextFieldTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(OutlinedTextFieldTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledInputColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_INPUT_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledInputOpacity",
            value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_INPUT_OPACITY),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledLabelColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_LABEL_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledLabelOpacity",
            value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_LABEL_OPACITY),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledLeadingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledLeadingIconOpacity",
            value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_LEADING_ICON_OPACITY),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledOutlineColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledOutlineOpacity",
            value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_OUTLINE_OPACITY),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledOutlineWidth",
            value: TokenValue::Dp(OutlinedTextFieldTokens::DISABLED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledSupportingColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledSupportingOpacity",
            value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_SUPPORTING_OPACITY),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledTrailingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::DISABLED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "DisabledTrailingIconOpacity",
            value: TokenValue::Float(OutlinedTextFieldTokens::DISABLED_TRAILING_ICON_OPACITY),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorFocusCaretColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_CARET_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorFocusInputColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_INPUT_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorFocusLabelColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_LABEL_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorFocusLeadingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorFocusOutlineColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorFocusSupportingColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorFocusTrailingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_FOCUS_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorHoverInputColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_INPUT_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorHoverLabelColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_LABEL_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorHoverLeadingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorHoverOutlineColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorHoverSupportingColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorHoverTrailingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_HOVER_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorInputColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_INPUT_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorLabelColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_LABEL_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorLeadingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorOutlineColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorSupportingColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "ErrorTrailingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::ERROR_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "FocusInputColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_INPUT_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "FocusLabelColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_LABEL_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "FocusLeadingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "FocusOutlineColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "FocusOutlineWidth",
            value: TokenValue::Dp(OutlinedTextFieldTokens::FOCUS_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "FocusSupportingColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "FocusTrailingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::FOCUS_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "HoverInputColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_INPUT_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "HoverLabelColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_LABEL_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "HoverLeadingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "HoverOutlineColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "HoverOutlineWidth",
            value: TokenValue::Dp(OutlinedTextFieldTokens::HOVER_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "HoverSupportingColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "HoverTrailingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::HOVER_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "InputColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::INPUT_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "InputFont",
            value: TokenValue::TypographyRole(OutlinedTextFieldTokens::INPUT_FONT),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "InputPlaceholderColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::INPUT_PLACEHOLDER_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "InputPrefixColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::INPUT_PREFIX_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "InputSuffixColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::INPUT_SUFFIX_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "LabelColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::LABEL_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "LabelFont",
            value: TokenValue::TypographyRole(OutlinedTextFieldTokens::LABEL_FONT),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "LeadingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "LeadingIconSize",
            value: TokenValue::Dp(OutlinedTextFieldTokens::LEADING_ICON_SIZE),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "OutlineColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "OutlineWidth",
            value: TokenValue::Dp(OutlinedTextFieldTokens::OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "SupportingColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::SUPPORTING_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "SupportingFont",
            value: TokenValue::TypographyRole(OutlinedTextFieldTokens::SUPPORTING_FONT),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "TrailingIconColor",
            value: TokenValue::ColorRole(OutlinedTextFieldTokens::TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedTextFieldTokens",
            name: "TrailingIconSize",
            value: TokenValue::Dp(OutlinedTextFieldTokens::TRAILING_ICON_SIZE),
        },
    ];
}
