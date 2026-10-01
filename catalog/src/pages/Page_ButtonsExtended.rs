//! Toggle & split 页：开关状态、按钮组方向和菜单按钮对比。

use gpui::{App, Entity, IntoElement, Render, Window, prelude::*};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group, specimen};

/// ButtonsExtended 页视图。
pub struct ButtonsExtendedPage {
    toggle_a: Entity<ToggleButtonState>,
    toggle_b: Entity<ToggleButtonState>,
    menu: Entity<MenuState>,
}

impl ButtonsExtendedPage {
    /// 创建页面及其初始组件状态。
    pub fn new(cx: &mut App) -> Entity<Self> {
        let menu = MenuState::new()
            .item(
                MenuItem::new("Save as copy")
                    .icon(IconName::Edit)
                    .on_click(|window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Copied to drafts"), None);
                    }),
            )
            .item(MenuItem::new("Duplicate row").icon(IconName::Add))
            .item(MenuItem::new("Discard").icon(IconName::Delete))
            .build(cx);
        let toggle_a = ToggleButton::new("toggle-bold", "Emphasis")
            .icon(IconName::Star)
            .build(cx);
        let toggle_b = ToggleButton::new("toggle-favorite", "Starred")
            .icon(IconName::Favorite)
            .checked(true)
            .build(cx);
        cx.new(|_| Self {
            toggle_a,
            toggle_b,
            menu,
        })
    }
}

impl Render for ButtonsExtendedPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let disabled = ToggleButton::new("toggle-disabled", "Locked")
            .disabled(true)
            .build(cx);
        let group = ButtonGroup::new("actions").children([
            Button::new("group-1", "Day").tonal().build(cx),
            Button::new("group-2", "Week").outlined().build(cx),
            Button::new("group-3", "Month").outlined().build(cx),
        ]);
        let vertical = ButtonGroup::new("vertical-actions").vertical().children([
            Button::new("v-1", "Add").filled().build(cx),
            Button::new("v-2", "Remove").outlined().build(cx),
        ]);
        let segmented = SegmentedButtonRow::new("extended-segmented")
            .buttons([
                SegmentedButton::new("Day").selected(true),
                SegmentedButton::new("Week"),
                SegmentedButton::new("Month"),
            ])
            .build(cx);
        let dropdown = ExposedDropdownMenu::new(
            "exposed-menu",
            Button::new("exposed-field", "Save to…")
                .outlined()
                .build(cx),
        )
        .menu(self.menu.clone())
        .expanded(true);

        gallery([
            showcase_group(
                cx,
                "Toggle states",
                [
                    specimen(cx, "Unselected", self.toggle_a.clone()),
                    specimen(cx, "Selected", self.toggle_b.clone()),
                    specimen(cx, "Disabled", disabled),
                ],
            ),
            showcase_group(
                cx,
                "Button groups",
                [group.into_any_element(), vertical.into_any_element()],
            ),
            showcase_group(
                cx,
                "Menu buttons",
                [
                    SplitButton::new("split", "Save")
                        .filled()
                        .menu(self.menu.clone())
                        .into_any_element(),
                    dropdown.into_any_element(),
                ],
            ),
            showcase_group(cx, "Segmented", [segmented.into_any_element()]),
        ])
    }
}
