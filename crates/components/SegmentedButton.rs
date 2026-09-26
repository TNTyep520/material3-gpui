use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, Corners, Div, ElementId, Entity, FocusHandle,
    InteractiveElement as _, IntoElement, ParentElement as _, Render, SharedString, Stateful,
    StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _, px,
};

use crate::icon::{Icon, IconName};
use crate::interaction::{InteractiveSurface, wire_events};
use crate::motion::{AnimatedComponent, AnimationDriver};
use crate::styles::SegmentedButtonStyle;
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(&[usize], &mut Window, &mut App)>;
type StyleOverride = Rc<dyn Fn(&mut SegmentedButtonStyle)>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SegmentedButtonSelectionMode {
    #[default]
    Single,
    Multiple,
}

#[derive(Clone)]
pub struct SegmentedButton {
    label: SharedString,
    icon: Option<IconName>,
    selected: bool,
    disabled: bool,
}

impl SegmentedButton {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            selected: false,
            disabled: false,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub struct SegmentedButtonRow {
    id: ElementId,
    buttons: Vec<SegmentedButton>,
    selection_mode: SegmentedButtonSelectionMode,
    disabled: bool,
    on_change: Option<ChangeHandler>,
    style_override: Option<StyleOverride>,
}

pub struct SegmentedButtonRowState {
    row: SegmentedButtonRow,
    surfaces: Vec<InteractiveSurface>,
    focus_handles: Vec<FocusHandle>,
    driver: AnimationDriver,
}

impl SegmentedButtonRow {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            buttons: Vec::new(),
            selection_mode: SegmentedButtonSelectionMode::Single,
            disabled: false,
            on_change: None,
            style_override: None,
        }
    }

    pub fn buttons(mut self, buttons: impl IntoIterator<Item = SegmentedButton>) -> Self {
        self.buttons.extend(buttons);
        self
    }

    pub fn selection_mode(mut self, mode: SegmentedButtonSelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&[usize], &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn style(mut self, style: impl Fn(&mut SegmentedButtonStyle) + 'static) -> Self {
        self.style_override = Some(Rc::new(style));
        self
    }

    pub fn build(mut self, cx: &mut App) -> Entity<SegmentedButtonRowState> {
        self.normalize_selection();
        cx.new(|cx| SegmentedButtonRowState {
            surfaces: self
                .buttons
                .iter()
                .map(|_| InteractiveSurface::new())
                .collect(),
            focus_handles: self.buttons.iter().map(|_| cx.focus_handle()).collect(),
            row: self,
            driver: AnimationDriver::default(),
        })
    }

    fn normalize_selection(&mut self) {
        if self.selection_mode == SegmentedButtonSelectionMode::Single {
            let mut found = false;
            for button in &mut self.buttons {
                let selected = button.selected;
                button.selected &= !found;
                found |= selected;
            }
        }
    }

    fn selected_indices(&self) -> Vec<usize> {
        self.buttons
            .iter()
            .enumerate()
            .filter_map(|(index, button)| button.selected.then_some(index))
            .collect()
    }

    fn set_selected(&mut self, index: usize, selected: bool) -> bool {
        let Some(button) = self.buttons.get(index) else {
            return false;
        };
        if button.selected == selected {
            return false;
        }
        if selected && self.selection_mode == SegmentedButtonSelectionMode::Single {
            for (button_index, button) in self.buttons.iter_mut().enumerate() {
                button.selected = button_index == index;
            }
        } else if let Some(button) = self.buttons.get_mut(index) {
            button.selected = selected;
        }
        true
    }

    fn activate(&mut self, index: usize) -> bool {
        let Some(button) = self.buttons.get(index) else {
            return false;
        };
        if self.disabled || button.disabled {
            return false;
        }
        let selected =
            self.selection_mode == SegmentedButtonSelectionMode::Single || !button.selected;
        self.set_selected(index, selected)
    }
}

impl SegmentedButtonRowState {
    pub fn selected(&self) -> Vec<usize> {
        self.row.selected_indices()
    }

    pub fn set_selected(&mut self, index: usize, selected: bool, cx: &mut Context<Self>) {
        if self.row.set_selected(index, selected) {
            cx.notify();
        }
    }

    fn activate(entity: &Entity<Self>, index: usize, window: &mut Window, cx: &mut App) {
        let change = entity.update(cx, |state, cx| {
            if !state.row.activate(index) {
                return None;
            }
            cx.notify();
            state
                .row
                .on_change
                .clone()
                .map(|handler| (handler, state.selected()))
        });
        if let Some((handler, selected)) = change {
            handler(&selected, window, cx);
        }
    }
}

impl AnimatedComponent for SegmentedButtonRowState {
    fn step(&mut self, now: Instant) -> bool {
        let mut running = false;
        for surface in &mut self.surfaces {
            running |= surface.step(now);
        }
        running
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for SegmentedButtonRowState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.surfaces.iter().any(InteractiveSurface::is_animating) {
            self.schedule_next(window, cx);
        }
        let theme = cx.theme();
        let entity = cx.entity();
        let count = self.row.buttons.len();
        let mut row = div().id(self.row.id.clone()).flex().w_full();
        for (index, ((button, surface), focus)) in self
            .row
            .buttons
            .iter()
            .zip(&self.surfaces)
            .zip(&self.focus_handles)
            .enumerate()
        {
            let disabled = self.row.disabled || button.disabled;
            let mut style =
                SegmentedButtonStyle::resolve(theme.token_set(), button.selected, disabled);
            if let Some(customize) = &self.row.style_override {
                customize(&mut style);
            }
            let first = index == 0;
            let last = index + 1 == count;
            let shape = |element: Stateful<Div>| {
                element
                    .when(first, |element| {
                        element
                            .rounded_tl(style.corner_radius)
                            .rounded_bl(style.corner_radius)
                    })
                    .when(last, |element| {
                        element
                            .rounded_tr(style.corner_radius)
                            .rounded_br(style.corner_radius)
                    })
            };
            let base = div()
                .id(index)
                .relative()
                .flex_1()
                .min_w(style.min_width)
                .h(style.height)
                .border(style.outline_width)
                .border_color(style.outline_color)
                .when(!first, |element| element.ml(-style.outline_width))
                .when_some(style.container_color, |element, color| element.bg(color));
            let base = shape(base);
            let content = div()
                .id(("content", index))
                .relative()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .px(style.horizontal_padding)
                .gap(style.gap)
                .text_color(style.content_color)
                .overflow_hidden();
            let mut content = shape(content);
            if !disabled {
                content = wire_events(content, &entity, theme.motion(), move |state| {
                    &mut state.surfaces[index]
                });
                let corners = Corners {
                    top_left: if first { style.corner_radius } else { px(0.) },
                    bottom_left: if first { style.corner_radius } else { px(0.) },
                    top_right: if last { style.corner_radius } else { px(0.) },
                    bottom_right: if last { style.corner_radius } else { px(0.) },
                };
                content = surface
                    .overlay_with_corners(style.content_color, theme.state_layer().pressed, corners)
                    .apply(content)
                    .child(surface.bounds.capture_element());
                let click_entity = entity.clone();
                let key_entity = entity.clone();
                let click_focus = focus.clone();
                let focus_color = style.content_color.opacity(theme.state_layer().focus);
                content = content
                    .cursor_pointer()
                    .track_focus(focus)
                    .tab_stop(true)
                    .focus(move |style| style.bg(focus_color))
                    .on_click(move |_, window, cx| {
                        window.focus(&click_focus);
                        Self::activate(&click_entity, index, window, cx);
                    })
                    .on_key_down(move |event, window, cx| {
                        if !event.is_held
                            && matches!(event.keystroke.key.as_str(), "space" | "enter")
                        {
                            cx.stop_propagation();
                            Self::activate(&key_entity, index, window, cx);
                        }
                    });
            }
            let icon = if button.selected {
                Some(IconName::Check)
            } else {
                button.icon
            };
            content = style
                .label
                .apply(content)
                .when_some(icon, |element, icon| {
                    element.child(
                        Icon::new(icon)
                            .size(style.icon_size)
                            .color(style.content_color),
                    )
                })
                .child(button.label.clone());
            row = row.child(base.child(content));
        }
        row
    }
}

#[cfg(test)]
mod tests {
    use super::{SegmentedButton, SegmentedButtonRow, SegmentedButtonSelectionMode};

    #[test]
    fn single_selection_is_exclusive_and_cannot_be_toggled_off() {
        let mut row = SegmentedButtonRow::new("single").buttons([
            SegmentedButton::new("A").selected(true),
            SegmentedButton::new("B").selected(true),
            SegmentedButton::new("C"),
        ]);
        row.normalize_selection();
        assert_eq!(row.selected_indices(), vec![0]);
        assert!(!row.activate(0));
        assert!(row.activate(2));
        assert_eq!(row.selected_indices(), vec![2]);
        assert!(!row.activate(2));
    }

    #[test]
    fn multiple_selection_can_be_empty() {
        let mut row = SegmentedButtonRow::new("multi")
            .selection_mode(SegmentedButtonSelectionMode::Multiple)
            .buttons([SegmentedButton::new("A"), SegmentedButton::new("B")]);
        assert!(row.activate(0));
        assert!(row.activate(1));
        assert_eq!(row.selected_indices(), vec![0, 1]);
        assert!(row.activate(0));
        assert!(row.activate(1));
        assert!(row.selected_indices().is_empty());
    }

    #[test]
    fn disabled_and_invalid_buttons_do_not_change_selection() {
        let mut row = SegmentedButtonRow::new("disabled").buttons([
            SegmentedButton::new("A").selected(true),
            SegmentedButton::new("B").disabled(true),
        ]);
        assert!(!row.activate(1));
        assert!(!row.activate(usize::MAX));
        assert_eq!(row.selected_indices(), vec![0]);
        row.disabled = true;
        assert!(!row.activate(0));
        assert!(row.set_selected(1, true));
        assert_eq!(row.selected_indices(), vec![1]);
        assert!(!row.set_selected(2, true));
        assert!(row.set_selected(1, false));
        assert!(row.selected_indices().is_empty());
        assert!(!SegmentedButtonRow::new("empty").activate(0));
    }
}
