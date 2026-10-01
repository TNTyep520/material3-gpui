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
pub struct FilledAutocompleteTokens;
impl FilledAutocompleteTokens {
    pub const MENU_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const MENU_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const MENU_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const TEXT_FIELD_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const TEXT_FIELD_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const TEXT_FIELD_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL_TOP;
    pub const TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_CONTAINER_OPACITY: f32 = 0.04;
    pub const FIELD_DISABLED_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_INPUT_TEXT_OPACITY: f32 = 0.38;
    pub const FIELD_DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const FIELD_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_DISABLED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const TEXT_FIELD_ERROR_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_CARET_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const FIELD_ERROR_FOCUS_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_ERROR_FOCUS_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_FOCUS_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_HOVER_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const FIELD_ERROR_HOVER_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const TEXT_FIELD_ERROR_HOVER_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_ERROR_HOVER_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_ERROR_CONTAINER;
    pub const FIELD_ERROR_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_ERROR_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_ERROR_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_ERROR_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const TEXT_FIELD_FOCUS_ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_FOCUS_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(2.0);
    pub const FIELD_FOCUS_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TEXT_FIELD_FOCUS_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_FOCUS_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_ACTIVE_INDICATOR_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TEXT_FIELD_HOVER_ACTIVE_INDICATOR_HEIGHT: Dp = Dp(1.0);
    pub const FIELD_HOVER_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_HOVER_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_INPUT_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FIELD_INPUT_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const FIELD_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const TEXT_FIELD_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_LEADING_ICON_SIZE: Dp = Dp(20.0);
    pub const FIELD_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FIELD_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
    pub const TEXT_FIELD_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TEXT_FIELD_TRAILING_ICON_SIZE: Dp = Dp(24.0);
}
impl FilledAutocompleteTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "MenuContainerColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::MENU_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "MenuContainerElevation",
            value: TokenValue::Dp(FilledAutocompleteTokens::MENU_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "MenuContainerShape",
            value: TokenValue::ShapeRole(FilledAutocompleteTokens::MENU_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldActiveIndicatorColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_ACTIVE_INDICATOR_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldActiveIndicatorHeight",
            value: TokenValue::Dp(FilledAutocompleteTokens::TEXT_FIELD_ACTIVE_INDICATOR_HEIGHT),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldCaretColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_CARET_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldContainerColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldContainerShape",
            value: TokenValue::ShapeRole(FilledAutocompleteTokens::TEXT_FIELD_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldDisabledActiveIndicatorColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldDisabledActiveIndicatorHeight",
            value: TokenValue::Dp(
                FilledAutocompleteTokens::TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_HEIGHT,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldDisabledActiveIndicatorOpacity",
            value: TokenValue::Float(
                FilledAutocompleteTokens::TEXT_FIELD_DISABLED_ACTIVE_INDICATOR_OPACITY,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldDisabledContainerColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_DISABLED_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldDisabledContainerOpacity",
            value: TokenValue::Float(
                FilledAutocompleteTokens::TEXT_FIELD_DISABLED_CONTAINER_OPACITY,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldDisabledInputTextColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_DISABLED_INPUT_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldDisabledInputTextOpacity",
            value: TokenValue::Float(FilledAutocompleteTokens::FIELD_DISABLED_INPUT_TEXT_OPACITY),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldDisabledLabelTextColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldDisabledLabelTextOpacity",
            value: TokenValue::Float(FilledAutocompleteTokens::FIELD_DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldDisabledLeadingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_DISABLED_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldDisabledLeadingIconOpacity",
            value: TokenValue::Float(
                FilledAutocompleteTokens::TEXT_FIELD_DISABLED_LEADING_ICON_OPACITY,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldDisabledSupportingTextColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::FIELD_DISABLED_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldDisabledSupportingTextOpacity",
            value: TokenValue::Float(
                FilledAutocompleteTokens::FIELD_DISABLED_SUPPORTING_TEXT_OPACITY,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldDisabledTrailingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_DISABLED_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldDisabledTrailingIconOpacity",
            value: TokenValue::Float(
                FilledAutocompleteTokens::TEXT_FIELD_DISABLED_TRAILING_ICON_OPACITY,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldErrorActiveIndicatorColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_ERROR_ACTIVE_INDICATOR_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldErrorFocusActiveIndicatorColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_ACTIVE_INDICATOR_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldErrorFocusCaretColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_CARET_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldErrorFocusInputTextColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::FIELD_ERROR_FOCUS_INPUT_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldErrorFocusLabelTextColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::FIELD_ERROR_FOCUS_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldErrorFocusLeadingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldErrorFocusSupportingTextColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::FIELD_ERROR_FOCUS_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldErrorFocusTrailingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_ERROR_FOCUS_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldErrorHoverActiveIndicatorColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_ACTIVE_INDICATOR_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldErrorHoverInputTextColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::FIELD_ERROR_HOVER_INPUT_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldErrorHoverLabelTextColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::FIELD_ERROR_HOVER_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldErrorHoverLeadingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldErrorHoverSupportingTextColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::FIELD_ERROR_HOVER_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldErrorHoverTrailingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_ERROR_HOVER_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldErrorInputTextColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_ERROR_INPUT_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldErrorLabelTextColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_ERROR_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldErrorLeadingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_ERROR_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldErrorSupportingTextColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::FIELD_ERROR_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldErrorTrailingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_ERROR_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldFocusActiveIndicatorColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_FOCUS_ACTIVE_INDICATOR_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldFocusActiveIndicatorHeight",
            value: TokenValue::Dp(
                FilledAutocompleteTokens::TEXT_FIELD_FOCUS_ACTIVE_INDICATOR_HEIGHT,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldFocusInputTextColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_FOCUS_INPUT_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldFocusLabelTextColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_FOCUS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldFocusLeadingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_FOCUS_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldFocusSupportingTextColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::FIELD_FOCUS_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldFocusTrailingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_FOCUS_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldHoverActiveIndicatorColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_HOVER_ACTIVE_INDICATOR_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldHoverActiveIndicatorHeight",
            value: TokenValue::Dp(
                FilledAutocompleteTokens::TEXT_FIELD_HOVER_ACTIVE_INDICATOR_HEIGHT,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldHoverInputTextColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_HOVER_INPUT_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldHoverLabelTextColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_HOVER_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldHoverLeadingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_HOVER_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldHoverSupportingTextColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::FIELD_HOVER_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldHoverTrailingIconColor",
            value: TokenValue::ColorRole(
                FilledAutocompleteTokens::TEXT_FIELD_HOVER_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldInputTextColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_INPUT_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldInputTextFont",
            value: TokenValue::TypographyRole(FilledAutocompleteTokens::FIELD_INPUT_TEXT_FONT),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldLabelTextColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldLabelTextFont",
            value: TokenValue::TypographyRole(FilledAutocompleteTokens::FIELD_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldLeadingIconColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldLeadingIconSize",
            value: TokenValue::Dp(FilledAutocompleteTokens::TEXT_FIELD_LEADING_ICON_SIZE),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldSupportingTextColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::FIELD_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "FieldSupportingTextFont",
            value: TokenValue::TypographyRole(FilledAutocompleteTokens::FIELD_SUPPORTING_TEXT_FONT),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldTrailingIconColor",
            value: TokenValue::ColorRole(FilledAutocompleteTokens::TEXT_FIELD_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledAutocompleteTokens",
            name: "TextFieldTrailingIconSize",
            value: TokenValue::Dp(FilledAutocompleteTokens::TEXT_FIELD_TRAILING_ICON_SIZE),
        },
    ];
}
