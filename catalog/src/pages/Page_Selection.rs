//! Selection controls 页(对齐 m3fx demo 的 `SwitchesDemoPage` 画廊风格)。

use gpui::{AnyElement, App, Entity, Hsla, IntoElement, Styled, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Selection controls 页视图。
pub struct SelectionPage {
    segmented_single: Entity<SegmentedButtonRowState>,
    segmented_multiple: Entity<SegmentedButtonRowState>,
    segmented_disabled: Entity<SegmentedButtonRowState>,
    pub cb_a: Entity<CheckboxState>,
    pub cb_b: Entity<CheckboxState>,
    pub cb_disabled: Entity<CheckboxState>,
    pub radios: Vec<Entity<RadioState>>,
    pub sw_on: Entity<SwitchState>,
    pub sw_off: Entity<SwitchState>,
    pub sw_icon_off: Entity<SwitchState>,
    pub sw_icon_on: Entity<SwitchState>,
    pub sw_disabled_off: Entity<SwitchState>,
    pub sw_disabled_on: Entity<SwitchState>,
    pub sw_disabled_icon_off: Entity<SwitchState>,
    pub sw_disabled_icon_on: Entity<SwitchState>,
}

impl SelectionPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let cb_a = Checkbox::new("cb-a").checked(true).build(cx);
        let cb_b = Checkbox::new("cb-b").build(cx);
        let cb_disabled = Checkbox::new("cb-dis")
            .checked(true)
            .disabled(true)
            .build(cx);
        let radios = (0usize..3)
            .map(|ix| {
                RadioButton::new(("radio", ix))
                    .selected(ix == 0)
                    .disabled(ix == 2)
                    .build(cx)
            })
            .collect();

        // 对齐 m3fx SwitchesDemoPage:Interactive / Check Icon / Disabled 三组
        let sw_on = Switch::new("sw-on").checked(true).build(cx);
        let sw_off = Switch::new("sw-off").build(cx);
        let sw_icon_off = Switch::new("sw-icon-off").with_check_icon(true).build(cx);
        let sw_icon_on = Switch::new("sw-icon-on")
            .with_check_icon(true)
            .checked(true)
            .build(cx);
        let sw_disabled_off = Switch::new("sw-dis-off").disabled(true).build(cx);
        let sw_disabled_on = Switch::new("sw-dis-on")
            .disabled(true)
            .checked(true)
            .build(cx);
        let sw_disabled_icon_off = Switch::new("sw-dis-icon-off")
            .with_check_icon(true)
            .disabled(true)
            .build(cx);
        let sw_disabled_icon_on = Switch::new("sw-dis-icon-on")
            .with_check_icon(true)
            .checked(true)
            .disabled(true)
            .build(cx);

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
        cx.new(|_| Self {
            segmented_single,
            segmented_multiple,
            segmented_disabled,
            cb_a,
            cb_b,
            cb_disabled,
            radios,
            sw_on,
            sw_off,
            sw_icon_off,
            sw_icon_on,
            sw_disabled_off,
            sw_disabled_on,
            sw_disabled_icon_off,
            sw_disabled_icon_on,
        })
    }
}

/// 行组合:文字标签 + 开关(m3fx `M3Switch("On")` 的形态)。
fn switch_row(label: &'static str, sw: Entity<SwitchState>, label_color: Hsla) -> AnyElement {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(
            div()
                .text_size(px(14.))
                .text_color(label_color)
                .child(label),
        )
        .child(sw)
        .into_any_element()
}

impl Render for SelectionPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let on_surface = cx.theme().colors().on_surface;
        let label_disabled = on_surface.opacity(0.38);

        gallery([
            showcase_group(
                cx,
                "Segmented buttons",
                [
                    self.segmented_single.clone().into_any_element(),
                    self.segmented_multiple.clone().into_any_element(),
                    self.segmented_disabled.clone().into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Interactive States",
                [
                    switch_row("On", self.sw_on.clone(), on_surface),
                    switch_row("Off", self.sw_off.clone(), on_surface),
                ],
            ),
            showcase_group(
                cx,
                "Check Icon",
                [
                    switch_row("Icon off", self.sw_icon_off.clone(), on_surface),
                    switch_row("Icon on", self.sw_icon_on.clone(), on_surface),
                ],
            ),
            showcase_group(
                cx,
                "Disabled States",
                [
                    switch_row("Disabled off", self.sw_disabled_off.clone(), label_disabled),
                    switch_row("Disabled on", self.sw_disabled_on.clone(), label_disabled),
                    switch_row(
                        "Disabled icon off",
                        self.sw_disabled_icon_off.clone(),
                        label_disabled,
                    ),
                    switch_row(
                        "Disabled icon on",
                        self.sw_disabled_icon_on.clone(),
                        label_disabled,
                    ),
                ],
            ),
            showcase_group(
                cx,
                "Checkbox & Radio",
                [
                    self.cb_a.clone().into_any_element(),
                    self.cb_b.clone().into_any_element(),
                    self.cb_disabled.clone().into_any_element(),
                    div().w(px(8.)).into_any_element(),
                    div()
                        .flex()
                        .items_center()
                        .gap(px(16.))
                        .children(self.radios.clone())
                        .into_any_element(),
                ],
            ),
        ])
    }
}
