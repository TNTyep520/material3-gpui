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
    ColorSchemeKeyTokens, ColorToken, Dp, ElevationTokens, ShapeKeyTokens, ShapeToken, TokenEntry,
    TokenValue, TypographyKeyTokens, TypographyToken,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct OutlinedAutocompleteTokens;
impl OutlinedAutocompleteTokens {
    pub const MENU_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const MENU_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const MENU_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const TEXT_FIELD_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const TEXT_FIELD_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const FIELD_DISABLED_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_INPUT_TEXT_OPACITY: f32 = 0.38;
    pub const FIELD_DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_OUTLINE_OPACITY: f32 = 0.12;
    pub const TEXT_FIELD_DISABLED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FIELD_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_ERROR_FOCUS_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_ERROR_FOCUS_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_ERROR_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_ERROR_FOCUS_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_ERROR_HOVER_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const TEXT_FIELD_ERROR_HOVER_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_ERROR_HOVER_OUTLINE_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const FIELD_ERROR_HOVER_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const FIELD_ERROR_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_ERROR_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_ERROR_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_FOCUS_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_FOCUS_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_FOCUS_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const FIELD_FOCUS_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_HOVER_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_HOVER_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FIELD_HOVER_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_INPUT_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const FIELD_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const TEXT_FIELD_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_LEADING_ICON_SIZE: Dp = Dp(24.0);
    pub const TEXT_FIELD_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const TEXT_FIELD_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FIELD_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
    pub const TEXT_FIELD_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_TRAILING_ICON_SIZE: Dp = Dp(24.0);
}
impl OutlinedAutocompleteTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "MenuContainerColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::MENU_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "MenuContainerElevation",
            value: TokenValue::Dp(OutlinedAutocompleteTokens::MENU_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "MenuContainerShape",
            value: TokenValue::ShapeRole(OutlinedAutocompleteTokens::MENU_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldCaretColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_CARET_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldContainerColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldContainerShape",
            value: TokenValue::ShapeRole(OutlinedAutocompleteTokens::TEXT_FIELD_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldDisabledInputTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_DISABLED_INPUT_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldDisabledInputTextOpacity",
            value: TokenValue::Float(OutlinedAutocompleteTokens::FIELD_DISABLED_INPUT_TEXT_OPACITY),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldDisabledLabelTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_DISABLED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldDisabledLabelTextOpacity",
            value: TokenValue::Float(OutlinedAutocompleteTokens::FIELD_DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldDisabledLeadingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldDisabledLeadingIconOpacity",
            value: TokenValue::Float(
                OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_LEADING_ICON_OPACITY,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldDisabledOutlineColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_OUTLINE_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldDisabledOutlineOpacity",
            value: TokenValue::Float(
                OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_OUTLINE_OPACITY,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldDisabledOutlineWidth",
            value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldDisabledSupportingTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_DISABLED_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldDisabledSupportingTextOpacity",
            value: TokenValue::Float(
                OutlinedAutocompleteTokens::FIELD_DISABLED_SUPPORTING_TEXT_OPACITY,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldDisabledTrailingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldDisabledTrailingIconOpacity",
            value: TokenValue::Float(
                OutlinedAutocompleteTokens::TEXT_FIELD_DISABLED_TRAILING_ICON_OPACITY,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldErrorFocusCaretColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_CARET_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldErrorFocusInputTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_ERROR_FOCUS_INPUT_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldErrorFocusLabelTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_ERROR_FOCUS_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldErrorFocusLeadingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldErrorFocusOutlineColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_OUTLINE_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldErrorFocusSupportingTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_ERROR_FOCUS_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldErrorFocusTrailingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldErrorHoverInputTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_ERROR_HOVER_INPUT_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldErrorHoverLabelTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_ERROR_HOVER_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldErrorHoverLeadingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldErrorHoverOutlineColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_OUTLINE_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldErrorHoverSupportingTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_ERROR_HOVER_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldErrorHoverTrailingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldErrorInputTextColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_ERROR_INPUT_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldErrorLabelTextColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_ERROR_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldErrorLeadingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldErrorOutlineColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_OUTLINE_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldErrorSupportingTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_ERROR_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldErrorTrailingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_ERROR_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldFocusInputTextColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_FOCUS_INPUT_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldFocusLabelTextColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_FOCUS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldFocusLeadingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_FOCUS_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldFocusOutlineColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_FOCUS_OUTLINE_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldFocusOutlineWidth",
            value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_FOCUS_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldFocusSupportingTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_FOCUS_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldFocusTrailingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_FOCUS_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldHoverInputTextColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_HOVER_INPUT_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldHoverLabelTextColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_HOVER_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldHoverLeadingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_HOVER_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldHoverOutlineColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_HOVER_OUTLINE_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldHoverOutlineWidth",
            value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_HOVER_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldHoverSupportingTextColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::FIELD_HOVER_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldHoverTrailingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_HOVER_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldInputTextColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_INPUT_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldInputTextFont",
            value: TokenValue::TypographyRole(OutlinedAutocompleteTokens::FIELD_INPUT_TEXT_FONT),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldLabelTextColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldLabelTextFont",
            value: TokenValue::TypographyRole(OutlinedAutocompleteTokens::FIELD_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldLeadingIconColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldLeadingIconSize",
            value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_LEADING_ICON_SIZE),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldOutlineColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::TEXT_FIELD_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldOutlineWidth",
            value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldSupportingTextColor",
            value: TokenValue::ColorRole(OutlinedAutocompleteTokens::FIELD_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "FieldSupportingTextFont",
            value: TokenValue::TypographyRole(
                OutlinedAutocompleteTokens::FIELD_SUPPORTING_TEXT_FONT,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldTrailingIconColor",
            value: TokenValue::ColorRole(
                OutlinedAutocompleteTokens::TEXT_FIELD_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedAutocompleteTokens",
            name: "TextFieldTrailingIconSize",
            value: TokenValue::Dp(OutlinedAutocompleteTokens::TEXT_FIELD_TRAILING_ICON_SIZE),
        },
    ];
}
