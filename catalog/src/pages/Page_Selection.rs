//! Selection 页：比较选择控件的可交互和禁用状态。

use gpui::{App, AppContext as _, Context, Entity, IntoElement, Render, WeakEntity, Window};
use material3_gpui::prelude::*;

use super::{LogErr as _, gallery, showcase_group, specimen};

/// Selection controls 页视图。
pub struct SelectionPage {
    segmented_single: Entity<SegmentedButtonRowState>,
    segmented_multiple: Entity<SegmentedButtonRowState>,
    segmented_disabled: Entity<SegmentedButtonRowState>,
    cb_a: Entity<CheckboxState>,
    cb_disabled: Entity<CheckboxState>,
    radios: Vec<Entity<RadioState>>,
    sw_dicts: Entity<SwitchState>,
    sw_sync: Entity<SwitchState>,
    sw_quiet: Entity<SwitchState>,
    sw_disabled_on: Entity<SwitchState>,
    sw_disabled_icon_on: Entity<SwitchState>,
}

impl SelectionPage {
    /// 创建页面及其初始组件状态。
    pub fn new(cx: &mut App) -> Entity<Self> {
        // 设置行开关:翻转即弹出 Snackbar 反馈
        let sw_dicts = Switch::new("sw-dicts")
            .checked(true)
            .on_change(move |checked, window, cx| {
                let message = if checked {
                    "Spell-check on"
                } else {
                    "Spell-check off"
                };
                show_snackbar(window, cx, Snackbar::new(message), None);
            })
            .build(cx);
        let sw_sync = Switch::new("sw-sync").build(cx);
        let sw_quiet = Switch::new("sw-quiet")
            .checked(true)
            .with_check_icon(true)
            .build(cx);
        let sw_disabled_on = Switch::new("sw-dis-on")
            .disabled(true)
            .checked(true)
            .build(cx);
        let sw_disabled_icon_on = Switch::new("sw-dis-icon-on")
            .with_check_icon(true)
            .checked(true)
            .disabled(true)
            .build(cx);

        // 复选框:收件箱规则两行
        let cb_a = Checkbox::new("cb-a").checked(true).build(cx);
        let cb_disabled = Checkbox::new("cb-dis")
            .checked(true)
            .disabled(true)
            .build(cx);

        // 分段控件
        let segmented_single = SegmentedButtonRow::new("segmented-single")
            .buttons([
                SegmentedButton::new("Day").selected(true),
                SegmentedButton::new("Week"),
                SegmentedButton::new("Month"),
            ])
            .build(cx);
        let segmented_multiple = SegmentedButtonRow::new("segmented-multiple")
            .selection_mode(SegmentedButtonSelectionMode::Multiple)
            .buttons([
                SegmentedButton::new("Walk")
                    .icon(IconName::Custom("directions_walk"))
                    .selected(true),
                SegmentedButton::new("Bike").icon(IconName::Custom("directions_bike")),
                SegmentedButton::new("Drive")
                    .icon(IconName::Custom("directions_car"))
                    .selected(true),
            ])
            .build(cx);
        let segmented_disabled = SegmentedButtonRow::new("segmented-disabled")
            .disabled(true)
            .buttons([
                SegmentedButton::new("Day").selected(true),
                SegmentedButton::new("Week"),
                SegmentedButton::new("Month"),
            ])
            .build(cx);

        // 电台方案:第一档默认选中,第三档禁用;单选组内互斥
        cx.new(|cx| {
            let weak: WeakEntity<Self> = cx.entity().downgrade();
            let radios: Vec<Entity<RadioState>> = (0..3usize)
                .map(|ix| {
                    let weak = weak.clone();
                    RadioButton::new(("radio-plan", ix))
                        .selected(ix == 0)
                        .disabled(ix == 2)
                        .on_select(move |window, cx| {
                            weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                for (j, other) in page.radios.iter().enumerate() {
                                    if j != ix {
                                        other.update(
                                            cx,
                                            |radio: &mut RadioState,
                                             radio_cx: &mut Context<RadioState>| {
                                                radio.set_selected(false, window, radio_cx);
                                            },
                                        );
                                    }
                                }
                            })
                            .log_err();
                        })
                        .build(cx)
                })
                .collect();

            Self {
                segmented_single,
                segmented_multiple,
                segmented_disabled,
                cb_a,
                cb_disabled,
                radios,
                sw_dicts,
                sw_sync,
                sw_quiet,
                sw_disabled_on,
                sw_disabled_icon_on,
            }
        })
    }
}

impl Render for SelectionPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        gallery([
            showcase_group(
                cx,
                "Switches",
                [
                    specimen(cx, "Selected", self.sw_dicts.clone()),
                    specimen(cx, "Unselected", self.sw_sync.clone()),
                    specimen(cx, "With icon", self.sw_quiet.clone()),
                    specimen(cx, "Disabled", self.sw_disabled_on.clone()),
                    specimen(cx, "Disabled · icon", self.sw_disabled_icon_on.clone()),
                ],
            ),
            showcase_group(
                cx,
                "Checkboxes",
                [
                    specimen(cx, "Enabled", self.cb_a.clone()),
                    specimen(cx, "Disabled", self.cb_disabled.clone()),
                ],
            ),
            showcase_group(
                cx,
                "Radio buttons",
                ["Selected", "Unselected", "Disabled"]
                    .into_iter()
                    .zip(self.radios.iter())
                    .map(|(label, radio)| specimen(cx, label, radio.clone())),
            ),
            showcase_group(
                cx,
                "Segmented buttons",
                [
                    self.segmented_single.clone().into_any_element(),
                    self.segmented_multiple.clone().into_any_element(),
                    self.segmented_disabled.clone().into_any_element(),
                ],
            ),
        ])
    }
}
