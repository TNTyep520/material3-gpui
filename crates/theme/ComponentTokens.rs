//! MD3 组件令牌（Component Tokens）——首期子集。
//!
//! 对应 [m3fx](https://github.com/Glavo/m3fx) 的 `M3ComponentTokens`
//! （Apache-2.0，© 2026 Glavo）。m3fx 覆盖全部控件；本首期子集覆盖
//! 交互最密集的组件（按钮/文本输入/开关/滑块/弹出层族），
//! 其余组件的几何常量暂留在组件内部，后续期逐步令牌化。
//!
//! 数值来源：material-web / m3fx 基线规格；Expressive profile 暂与
//! 基线相同（Expressive 的差异主要通过 shape / motion 令牌传导）。

/// 按钮组件令牌。
#[derive(Clone, Copy, Debug)]
pub struct ButtonTokens {
    /// 容器高度。
    pub height: f32,
    /// 图标尺寸。
    pub icon_size: f32,
    /// 图标与文字间距。
    pub icon_gap: f32,
    /// 水平内边距（含图标时）。
    pub horizontal_padding_with_icon: f32,
    /// 水平内边距（纯文字时）。
    pub horizontal_padding: f32,
    /// Text 变体水平内边距。
    pub text_horizontal_padding: f32,
    /// 按压缩放比例（m3fx `PRESSED_SCALE`）。
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

/// 文本输入组件令牌（TextField / TextInputLayout）。
#[derive(Clone, Copy, Debug)]
pub struct TextFieldTokens {
    /// 输入区最小高度。
    pub min_height: f32,
    /// 水平内边距。
    pub horizontal_padding: f32,
    /// 顶部内边距（标签浮动后）。
    pub top_padding: f32,
    /// 底部内边距。
    pub bottom_padding: f32,
    /// 底部指示条（active indicator）高度。
    pub indicator_height: f32,
    /// 聚焦时指示条高度。
    pub focused_indicator_height: f32,
    /// 支撑文本（helper/error）与输入区的间距。
    pub supporting_gap: f32,
    /// 前后图标尺寸。
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

/// 开关（Switch）组件令牌。
#[derive(Clone, Copy, Debug)]
pub struct SwitchTokens {
    /// 轨道宽度。
    pub track_width: f32,
    /// 轨道高度。
    pub track_height: f32,
    /// 滑块（thumb）直径（选中态）。
    pub thumb_size: f32,
    /// 滑块直径（未选中态）。
    pub unselected_thumb_size: f32,
    /// 图标尺寸（选中态滑块内）。
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

/// 滑块（Slider）组件令牌。
#[derive(Clone, Copy, Debug)]
pub struct SliderTokens {
    /// 轨道高度。
    pub track_height: f32,
    /// 活动手柄（handle）宽度。
    pub handle_width: f32,
    /// 手柄高度。
    pub handle_height: f32,
    /// 刻度点直径。
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

/// Snackbar 组件令牌。
#[derive(Clone, Copy, Debug)]
pub struct SnackbarTokens {
    /// 最小高度。
    pub min_height: f32,
    /// 水平内边距。
    pub horizontal_padding: f32,
    /// 垂直内边距。
    pub vertical_padding: f32,
    /// 与窗口底边的间距。
    pub bottom_offset: f32,
    /// 动作文本与消息的间距。
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

/// 菜单（Menu）组件令牌。
#[derive(Clone, Copy, Debug)]
pub struct MenuTokens {
    /// 菜单项高度。
    pub item_height: f32,
    /// 菜单容器内边距（垂直）。
    pub vertical_padding: f32,
    /// 菜单项水平内边距。
    pub item_horizontal_padding: f32,
    /// 菜单圆角档位对应 px（使用 Shapes::medium 的建议值）。
    pub corner_radius: f32,
    /// 与触发者的锚定间距。
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

/// 工具提示（Tooltip）组件令牌。
#[derive(Clone, Copy, Debug)]
pub struct TooltipTokens {
    /// 容器高度。
    pub height: f32,
    /// 水平内边距。
    pub horizontal_padding: f32,
    /// 与锚定组件的间距。
    pub anchor_gap: f32,
    /// 显示延迟。
    pub show_delay_ms: u64,
    /// 隐藏延迟。
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

/// 组件令牌集合（首期子集）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ComponentTokens {
    /// 按钮令牌。
    pub button: ButtonTokens,
    /// 文本输入令牌。
    pub text_field: TextFieldTokens,
    /// 开关令牌。
    pub switch: SwitchTokens,
    /// 滑块令牌。
    pub slider: SliderTokens,
    /// Snackbar 令牌。
    pub snackbar: SnackbarTokens,
    /// 菜单令牌。
    pub menu: MenuTokens,
    /// 工具提示令牌。
    pub tooltip: TooltipTokens,
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
