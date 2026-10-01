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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/tokens/M3ComponentTokens.java
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tokens.kt

use crate::tokens::{
    BaselineButtonTokens, ButtonSmallTokens, FilledTextFieldTokens, ShapeTokens,
    SliderTokens as AndroidxSliderTokens, SnackbarTokens as AndroidxSnackbarTokens,
    SwitchTokens as AndroidxSwitchTokens,
};

#[derive(Clone, Copy, Debug)]
pub struct ButtonTokens {
    pub height: f32,

    pub icon_size: f32,

    pub icon_gap: f32,

    pub horizontal_padding_with_icon: f32,

    pub horizontal_padding: f32,

    pub text_horizontal_padding: f32,

    pub pressed_scale: f64,
}

impl Default for ButtonTokens {
    fn default() -> Self {
        Self {
            height: 40.,
            icon_size: 18.,
            icon_gap: 8.,
            horizontal_padding_with_icon: 16.,
            horizontal_padding: 24.,
            text_horizontal_padding: 12.,
            pressed_scale: 0.98,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TextFieldTokens {
    pub min_height: f32,

    pub horizontal_padding: f32,

    pub top_padding: f32,

    pub bottom_padding: f32,

    pub indicator_height: f32,

    pub focused_indicator_height: f32,

    pub supporting_gap: f32,

    pub icon_size: f32,
}

impl Default for TextFieldTokens {
    fn default() -> Self {
        Self {
            min_height: 56.,
            horizontal_padding: 16.,
            top_padding: 8.,
            bottom_padding: 8.,
            indicator_height: 1.,
            focused_indicator_height: 2.,
            supporting_gap: 4.,
            icon_size: 20.,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SwitchTokens {
    pub track_width: f32,

    pub track_height: f32,

    pub thumb_size: f32,

    pub unselected_thumb_size: f32,

    pub icon_size: f32,
}

impl Default for SwitchTokens {
    fn default() -> Self {
        Self {
            track_width: 52.,
            track_height: 32.,
            thumb_size: 24.,
            unselected_thumb_size: 16.,
            icon_size: 16.,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SliderTokens {
    pub track_height: f32,

    pub handle_width: f32,

    pub handle_height: f32,

    pub tick_size: f32,
}

impl Default for SliderTokens {
    fn default() -> Self {
        Self {
            track_height: 16.,
            handle_width: 4.,
            handle_height: 44.,
            tick_size: 4.,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SnackbarTokens {
    pub min_height: f32,

    pub horizontal_padding: f32,

    pub vertical_padding: f32,

    pub bottom_offset: f32,

    pub action_gap: f32,
}

impl Default for SnackbarTokens {
    fn default() -> Self {
        Self {
            min_height: 48.,
            horizontal_padding: 16.,
            vertical_padding: 14.,
            bottom_offset: 16.,
            action_gap: 8.,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MenuTokens {
    pub item_height: f32,

    pub vertical_padding: f32,

    pub item_horizontal_padding: f32,

    pub corner_radius: f32,

    pub anchor_gap: f32,
}

impl Default for MenuTokens {
    fn default() -> Self {
        Self {
            item_height: 48.,
            vertical_padding: 8.,
            item_horizontal_padding: 12.,
            corner_radius: 12.,
            anchor_gap: 4.,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TooltipTokens {
    pub height: f32,

    pub horizontal_padding: f32,

    pub anchor_gap: f32,

    pub show_delay_ms: u64,

    pub hide_delay_ms: u64,
}

impl Default for TooltipTokens {
    fn default() -> Self {
        Self {
            height: 24.,
            horizontal_padding: 8.,
            anchor_gap: 6.,
            show_delay_ms: 500,
            hide_delay_ms: 200,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ComponentTokens {
    pub button: ButtonTokens,

    pub text_field: TextFieldTokens,

    pub switch: SwitchTokens,

    pub slider: SliderTokens,

    pub snackbar: SnackbarTokens,

    pub menu: MenuTokens,

    pub tooltip: TooltipTokens,
}

impl ComponentTokens {
    pub fn androidx() -> Self {
        Self {
            button: ButtonTokens {
                height: BaselineButtonTokens::CONTAINER_HEIGHT.0,
                icon_size: BaselineButtonTokens::ICON_SIZE.0,
                icon_gap: BaselineButtonTokens::ICON_LABEL_SPACE.0,
                horizontal_padding: BaselineButtonTokens::LEADING_SPACE.0,
                horizontal_padding_with_icon: ButtonSmallTokens::LEADING_SPACE.0,
                ..ButtonTokens::default()
            },
            text_field: TextFieldTokens {
                indicator_height: FilledTextFieldTokens::ACTIVE_INDICATOR_HEIGHT.0,
                focused_indicator_height: FilledTextFieldTokens::FOCUS_ACTIVE_INDICATOR_HEIGHT.0,
                icon_size: FilledTextFieldTokens::LEADING_ICON_SIZE.0,
                ..TextFieldTokens::default()
            },
            switch: SwitchTokens {
                track_width: AndroidxSwitchTokens::TRACK_WIDTH.0,
                track_height: AndroidxSwitchTokens::TRACK_HEIGHT.0,
                thumb_size: AndroidxSwitchTokens::SELECTED_HANDLE_WIDTH.0,
                unselected_thumb_size: AndroidxSwitchTokens::UNSELECTED_HANDLE_WIDTH.0,
                icon_size: AndroidxSwitchTokens::SELECTED_ICON_SIZE.0,
            },
            slider: SliderTokens {
                track_height: AndroidxSliderTokens::ACTIVE_TRACK_HEIGHT.0,
                handle_width: AndroidxSliderTokens::HANDLE_WIDTH.0,
                handle_height: AndroidxSliderTokens::HANDLE_HEIGHT.0,
                tick_size: AndroidxSliderTokens::STOP_INDICATOR_SIZE.0,
            },
            snackbar: SnackbarTokens {
                min_height: AndroidxSnackbarTokens::SINGLE_LINE_CONTAINER_HEIGHT.0,
                ..SnackbarTokens::default()
            },
            menu: MenuTokens {
                corner_radius: ShapeTokens::CORNER_VALUE_EXTRA_SMALL.0,
                ..MenuTokens::default()
            },
            tooltip: TooltipTokens::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_material_specs() {
        let t = ComponentTokens::default();
        assert_eq!(t.button.height, 40.);
        assert_eq!(t.button.pressed_scale, 0.98);
        assert_eq!(t.text_field.min_height, 56.);
        assert_eq!(t.switch.track_width, 52.);
        assert_eq!(t.snackbar.min_height, 48.);
    }
}
