use crate::components::{Fab, FilledIconToggleButton, TextField, TextFieldState};
use crate::theme::ActiveTheme;
use gpui::{
    Animation, AnimationExt, AnyElement, App, ElementId, Entity, IntoElement, MouseButton,
    ParentElement, RenderOnce, SharedString, Styled, Window, div, prelude::*, px, relative,
};
use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type RangeChangeHandler = Rc<dyn Fn((f32, f32), &mut Window, &mut App)>;
type ChangeHandler = Rc<dyn Fn(&str, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct LoadingIndicator {
    id: ElementId,
    size: gpui::Pixels,
}
impl LoadingIndicator {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: px(48.),
        }
    }
    pub fn size(mut self, size: gpui::Pixels) -> Self {
        self.size = size;
        self
    }
}
impl RenderOnce for LoadingIndicator {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        div()
            .id(self.id)
            .size(self.size)
            .rounded_full()
            .border_3()
            .border_color(c.primary)
            .text_color(c.primary)
            .with_animation(
                "md3-loading",
                Animation::new(Duration::from_millis(1200)).repeat(),
                |el, _| el,
            )
    }
}

#[derive(IntoElement)]
pub struct WavyProgressIndicator {
    id: ElementId,
    value: Option<f32>,
}
impl WavyProgressIndicator {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
        }
    }
    pub fn value(mut self, value: f32) -> Self {
        self.value = Some(value.clamp(0., 1.));
        self
    }
}
impl RenderOnce for WavyProgressIndicator {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        let v = self.value.unwrap_or(0.35);
        div()
            .id(self.id)
            .h(px(6.))
            .w_full()
            .rounded_full()
            .bg(c.secondary_container)
            .child(div().h_full().w(relative(v)).rounded_full().bg(c.primary))
    }
}

#[derive(IntoElement)]
pub struct RangeSlider {
    id: ElementId,
    start: f32,
    end: f32,
    enabled: bool,
    on_value_change: Option<RangeChangeHandler>,
}
impl RangeSlider {
    pub fn new(id: impl Into<ElementId>, start: f32, end: f32) -> Self {
        Self {
            id: id.into(),
            start: start.clamp(0., 1.).min(end.clamp(0., 1.)),
            end: end.clamp(0., 1.).max(start.clamp(0., 1.)),
            enabled: true,
            on_value_change: None,
        }
    }
    pub fn range(mut self, start: f32, end: f32) -> Self {
        self.start = start.clamp(0., 1.).min(end.clamp(0., 1.));
        self.end = end.clamp(0., 1.).max(self.start);
        self
    }

    /// 设置是否接受指针操作。
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// 设置区间变化回调；调用者应保存新值并重新渲染。
    pub fn on_value_change(
        mut self,
        handler: impl Fn((f32, f32), &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for RangeSlider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        let bounds = crate::interaction::BoundsHandle::new();
        let values = Rc::new(Cell::new((self.start, self.end)));
        let active_start = Rc::new(Cell::new(None::<bool>));
        let on_mouse_down = {
            let bounds = bounds.clone();
            let values = values.clone();
            let active_start = active_start.clone();
            let callback = self.on_value_change.clone();
            move |event: &gpui::MouseDownEvent, window: &mut Window, cx: &mut App| {
                let rect = bounds.get();
                let width = f32::from(rect.size.width);
                if width <= 0. {
                    return;
                }
                let fraction = (f32::from(event.position.x - rect.origin.x) / width).clamp(0., 1.);
                let (start, end) = values.get();
                let is_start = (fraction - start).abs() <= (fraction - end).abs();
                active_start.set(Some(is_start));
                let next = if is_start {
                    (fraction.min(end), end)
                } else {
                    (start, fraction.max(start))
                };
                values.set(next);
                if let Some(handler) = &callback {
                    handler(next, window, cx);
                }
            }
        };
        let on_mouse_move = {
            let bounds = bounds.clone();
            let active_start = active_start.clone();
            let callback = self.on_value_change;
            move |event: &gpui::MouseMoveEvent, window: &mut Window, cx: &mut App| {
                let Some(is_start) = active_start.get() else {
                    return;
                };
                if event.pressed_button != Some(MouseButton::Left) {
                    return;
                }
                let rect = bounds.get();
                let width = f32::from(rect.size.width);
                if width <= 0. {
                    return;
                }
                let fraction = (f32::from(event.position.x - rect.origin.x) / width).clamp(0., 1.);
                let (start, end) = values.get();
                let next = if is_start {
                    (fraction.min(end), end)
                } else {
                    (start, fraction.max(start))
                };
                if next != (start, end) {
                    values.set(next);
                    if let Some(handler) = &callback {
                        handler(next, window, cx);
                    }
                }
            }
        };
        div()
            .id(self.id)
            .relative()
            .h(px(44.))
            .w_full()
            .when(self.enabled, |el| {
                el.cursor_pointer()
                    .on_mouse_down(MouseButton::Left, on_mouse_down)
                    .on_mouse_move(on_mouse_move)
                    .on_mouse_up(MouseButton::Left, move |_, _, _| active_start.set(None))
            })
            .child(
                div()
                    .absolute()
                    .top(px(20.))
                    .h(px(4.))
                    .w_full()
                    .rounded_full()
                    .bg(c.secondary_container),
            )
            .child(
                div()
                    .absolute()
                    .top(px(20.))
                    .left(relative(self.start))
                    .right(relative(1. - self.end))
                    .h(px(4.))
                    .bg(c.primary),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left(relative(self.start))
                    .ml(px(-2.))
                    .w(px(4.))
                    .h(px(44.))
                    .rounded_full()
                    .bg(c.primary),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left(relative(self.end))
                    .ml(px(-2.))
                    .w(px(4.))
                    .h(px(44.))
                    .rounded_full()
                    .bg(c.primary),
            )
            .child(bounds.capture_element())
    }
}

#[derive(IntoElement)]
pub struct Scrollbar {
    id: ElementId,
    position: f32,
    thickness: gpui::Pixels,
}
impl Scrollbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            position: 0.,
            thickness: px(4.),
        }
    }
    pub fn position(mut self, p: f32) -> Self {
        self.position = p.clamp(0., 1.);
        self
    }
    pub fn thickness(mut self, t: gpui::Pixels) -> Self {
        self.thickness = t;
        self
    }
}
impl RenderOnce for Scrollbar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .absolute()
            .right_0()
            .top(relative(self.position))
            .w(self.thickness)
            .h(px(48.))
            .rounded_full()
            .bg(cx.theme().colors().on_surface_variant.opacity(0.5))
    }
}

pub struct SecureTextField {
    id: ElementId,
    label: SharedString,
    value: SharedString,
    enabled: bool,
    on_value_change: Option<ChangeHandler>,
}
impl SecureTextField {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: SharedString::default(),
            enabled: true,
            on_value_change: None,
        }
    }
    pub fn value(mut self, v: impl Into<SharedString>) -> Self {
        self.value = v.into();
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn on_value_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}
impl SecureTextField {
    pub fn build(self, cx: &mut App) -> Entity<TextFieldState> {
        let mut field = TextField::new(self.id, self.label)
            .password(true)
            .value(self.value)
            .enabled(self.enabled);
        if let Some(handler) = self.on_value_change {
            field = field.on_value_change(move |value, window, cx| handler(value, window, cx));
        }
        field.build(cx)
    }
}

#[derive(IntoElement)]
pub struct SearchBar {
    id: ElementId,
    field: Entity<TextFieldState>,
    expanded: bool,
    suggestions: Vec<gpui::AnyElement>,
}
impl SearchBar {
    pub fn new(id: impl Into<ElementId>, field: Entity<TextFieldState>) -> Self {
        Self {
            id: id.into(),
            field,
            expanded: false,
            suggestions: Vec::new(),
        }
    }
    pub fn expanded(mut self, e: bool) -> Self {
        self.expanded = e;
        self
    }
}

impl ParentElement for SearchBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.suggestions.extend(elements);
    }
}
impl RenderOnce for SearchBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        div()
            .id(self.id)
            .w_full()
            .min_h(px(56.))
            .when(!self.expanded, |el| el.rounded_full())
            .when(self.expanded, |el| el.rounded(px(28.)))
            .bg(c.surface_container_high)
            .overflow_hidden()
            .flex()
            .flex_col()
            .text_color(c.on_surface)
            .child(self.field)
            .when(self.expanded, |el| el.children(self.suggestions))
    }
}

#[derive(IntoElement)]
pub struct SwipeToDismissBox {
    id: ElementId,
    content: gpui::AnyElement,
    background: Option<gpui::AnyElement>,
    enabled: bool,
    state: SwipeToDismissBoxState,
    on_value_change: Option<SwipeChangeHandler>,
}

/// AndroidX SwipeToDismissBoxValue 对应的滑动结果。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwipeToDismissBoxValue {
    /// 内容停留在原位。
    #[default]
    Settled,
    /// 从左向右滑动完成。
    StartToEnd,
    /// 从右向左滑动完成。
    EndToStart,
}

/// AndroidX SwipeToDismissBoxState 对应的当前滑动结果。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SwipeToDismissBoxState {
    /// 当前显示位置。
    pub current_value: SwipeToDismissBoxValue,
}

type SwipeChangeHandler = Rc<dyn Fn(SwipeToDismissBoxValue, &mut Window, &mut App)>;

impl SwipeToDismissBox {
    /// 创建未滑动的内容容器。
    pub fn new(id: impl Into<ElementId>, content: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            content: content.into_any_element(),
            background: None,
            enabled: true,
            state: SwipeToDismissBoxState::default(),
            on_value_change: None,
        }
    }
    /// 设置滑动时露出的背景。
    pub fn background(mut self, b: impl IntoElement) -> Self {
        self.background = Some(b.into_any_element());
        self
    }
    /// 设置当前滑动结果。
    pub fn state(mut self, state: SwipeToDismissBoxState) -> Self {
        self.state = state;
        self
    }
    /// 设置是否允许滑动手势。
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    /// 手势横向移动超过 80dp 时报告滑动方向。
    pub fn on_value_change(
        mut self,
        handler: impl Fn(SwipeToDismissBoxValue, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for SwipeToDismissBox {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let start = Rc::new(Cell::new(None));
        let pointer_down = start.clone();
        let callback = self.on_value_change;
        div()
            .id(self.id)
            .relative()
            .when(self.enabled, |el| {
                el.on_mouse_down(MouseButton::Left, move |event, _, _| {
                    pointer_down.set(Some(event.position.x));
                })
                .on_mouse_up(MouseButton::Left, move |event, window, cx| {
                    let Some(start_x) = start.replace(None) else {
                        return;
                    };
                    let distance = f32::from(event.position.x - start_x);
                    let result = if distance > 80. {
                        SwipeToDismissBoxValue::StartToEnd
                    } else if distance < -80. {
                        SwipeToDismissBoxValue::EndToStart
                    } else {
                        SwipeToDismissBoxValue::Settled
                    };
                    if result != SwipeToDismissBoxValue::Settled
                        && let Some(handler) = &callback
                    {
                        handler(result, window, cx);
                    }
                })
            })
            .when_some(self.background, |el, b| el.child(b))
            .when(
                self.state.current_value == SwipeToDismissBoxValue::Settled,
                |el| el.child(self.content),
            )
    }
}

#[derive(IntoElement)]
pub struct FloatingToolbar {
    id: ElementId,
    children: Vec<gpui::AnyElement>,
    vertical: bool,
    state: FloatingToolbarState,
}

/// AndroidX FloatingToolbarState 对应的展开状态。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FloatingToolbarState {
    /// 工具栏内容是否展开显示。
    pub expanded: bool,
}

impl FloatingToolbarState {
    /// 创建给定初始状态的工具栏状态。
    pub fn new(expanded: bool) -> Self {
        Self { expanded }
    }

    /// 展开工具栏内容。
    pub fn expand(&mut self) {
        self.expanded = true;
    }

    /// 收起工具栏内容。
    pub fn collapse(&mut self) {
        self.expanded = false;
    }
}

impl Default for FloatingToolbarState {
    fn default() -> Self {
        Self::new(true)
    }
}

/// AndroidX HorizontalFloatingToolbar 对应的横向浮动工具栏。
pub type HorizontalFloatingToolbar = FloatingToolbar;

/// AndroidX VerticalFloatingToolbar 对应的纵向浮动工具栏。
#[derive(IntoElement)]
pub struct VerticalFloatingToolbar(FloatingToolbar);

impl FloatingToolbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            vertical: false,
            state: FloatingToolbarState::default(),
        }
    }

    /// 将工具栏内容按纵向排列。
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// 设置工具栏展开状态。
    pub fn state(mut self, state: FloatingToolbarState) -> Self {
        self.state = state;
        self
    }
}

impl VerticalFloatingToolbar {
    /// 创建纵向浮动工具栏。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self(FloatingToolbar::new(id).vertical())
    }

    /// 设置工具栏展开状态。
    pub fn state(mut self, state: FloatingToolbarState) -> Self {
        self.0 = self.0.state(state);
        self
    }
}

impl ParentElement for VerticalFloatingToolbar {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.0.extend(elements);
    }
}

impl RenderOnce for VerticalFloatingToolbar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.0.render(window, cx)
    }
}
impl ParentElement for FloatingToolbar {
    fn extend(&mut self, e: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(e)
    }
}
impl RenderOnce for FloatingToolbar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .when(self.vertical, |el| el.flex_col())
            .gap(px(8.))
            .p(px(8.))
            .rounded_full()
            .bg(cx.theme().colors().surface_container_high)
            .when(self.state.expanded, |el| el.children(self.children))
    }
}

#[derive(IntoElement)]
pub struct FabMenu {
    id: ElementId,
    expanded: bool,
    actions: Vec<gpui::AnyElement>,
}

/// AndroidX FloatingActionButtonMenu 对应的展开式 FAB 菜单。
pub type FloatingActionButtonMenu = FabMenu;
/// AndroidX FloatingActionButtonMenuItem 对应的菜单操作项。
pub type FloatingActionButtonMenuItem = Fab;
/// AndroidX ToggleFloatingActionButton 对应的可切换 FAB。
pub type ToggleFloatingActionButton = FilledIconToggleButton;
impl FabMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            expanded: false,
            actions: Vec::new(),
        }
    }
    pub fn expanded(mut self, e: bool) -> Self {
        self.expanded = e;
        self
    }
    pub fn action(mut self, a: impl IntoElement) -> Self {
        self.actions.push(a.into_any_element());
        self
    }
}
impl RenderOnce for FabMenu {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .items_end()
            .gap(px(8.))
            .when(self.expanded, |el| el.children(self.actions))
    }
}

/// 日期选择器当前显示月份和选中日期。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DatePickerState {
    /// 当前显示的年份。
    pub year: i32,
    /// 当前显示的月份，范围为 1..=12。
    pub month: u32,
    /// 当前月份中选中的日期。
    pub selected_day: Option<u32>,
}

impl DatePickerState {
    /// 创建并限制月份及日期到合法范围。
    pub fn new(year: i32, month: u32, selected_day: Option<u32>) -> Self {
        let month = month.clamp(1, 12);
        Self {
            year,
            month,
            selected_day: selected_day
                .filter(|day| *day >= 1 && *day <= days_in_month(year, month)),
        }
    }

    /// 使用当前 UTC 月份创建默认状态。
    pub fn today() -> Self {
        let days = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            / 86_400;
        let (year, month) = year_month_from_days(days as i64);
        Self::new(year, month, None)
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 31,
    }
}

fn weekday_of_first(year: i32, month: u32) -> u32 {
    let offsets = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = year - i32::from(month < 3);
    (year + year / 4 - year / 100 + year / 400 + offsets[(month - 1) as usize]).rem_euclid(7) as u32
}

fn year_month_from_days(days: i64) -> (i32, u32) {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let year = year + i64::from(month <= 2);
    (year as i32, month as u32)
}

type DateChangeHandler = Rc<dyn Fn(i32, u32, u32, &mut Window, &mut App)>;
type MonthChangeHandler = Rc<dyn Fn(i32, u32, &mut Window, &mut App)>;

/// 带月份导航和日期选择的 AndroidX DatePicker 对应控件。
#[derive(IntoElement)]
pub struct DatePicker {
    id: ElementId,
    value: SharedString,
    state: DatePickerState,
    on_date_change: Option<DateChangeHandler>,
    on_month_change: Option<MonthChangeHandler>,
}
impl DatePicker {
    /// 创建显示当前 UTC 月份的日期选择器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: "Select date".into(),
            state: DatePickerState::today(),
            on_date_change: None,
            on_month_change: None,
        }
    }
    /// 设置标题文字。
    pub fn value(mut self, v: impl Into<SharedString>) -> Self {
        self.value = v.into();
        self
    }
    /// 设置显示月份与选中日期。
    pub fn state(mut self, state: DatePickerState) -> Self {
        self.state = state;
        self
    }
    /// 设置日期选择回调，依次传递年、月、日。
    pub fn on_date_change(
        mut self,
        handler: impl Fn(i32, u32, u32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_date_change = Some(Rc::new(handler));
        self
    }
    /// 设置月份导航回调，依次传递新年、新月。
    pub fn on_month_change(
        mut self,
        handler: impl Fn(i32, u32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_month_change = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for DatePicker {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let year = self.state.year;
        let month = self.state.month;
        let first = weekday_of_first(year, month);
        let days = days_in_month(year, month);
        let calendar_id = self.id.clone();
        let previous = if month == 1 {
            (year - 1, 12)
        } else {
            (year, month - 1)
        };
        let next = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
        let month_change = self.on_month_change.clone();
        let previous_button = div()
            .id((self.id.clone(), "previous"))
            .cursor_pointer()
            .child("‹")
            .when_some(month_change, |el, handler| {
                el.on_click(move |_, window, cx| handler(previous.0, previous.1, window, cx))
            });
        let next_button = div()
            .id((self.id.clone(), "next"))
            .cursor_pointer()
            .child("›")
            .when_some(self.on_month_change, |el, handler| {
                el.on_click(move |_, window, cx| handler(next.0, next.1, window, cx))
            });
        div()
            .id(self.id)
            .p(px(16.))
            .rounded(px(12.))
            .bg(colors.surface_container_high)
            .text_color(colors.on_surface)
            .child(self.value)
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(previous_button)
                    .child(format!("{year:04}-{month:02}"))
                    .child(next_button),
            )
            .children((0_u32..6).map(|week| {
                div().flex().children((0_u32..7).map(|weekday| {
                    let day: u32 = week * 7 + weekday + 1;
                    let day = day
                        .checked_sub(first)
                        .filter(|day| *day >= 1 && *day <= days);
                    let selected = day == self.state.selected_day && day.is_some();
                    let handler = self.on_date_change.clone();
                    div()
                        .id((calendar_id.clone(), format!("day-{}", week * 7 + weekday)))
                        .size(px(36.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .when(selected, |el| {
                            el.bg(colors.primary).text_color(colors.on_primary)
                        })
                        .when_some(day, |el, day| {
                            el.child(day.to_string()).when_some(handler, |el, handler| {
                                el.cursor_pointer().on_click(move |_, window, cx| {
                                    handler(year, month, day, window, cx)
                                })
                            })
                        })
                }))
            }))
    }
}
pub type ExposedDatePicker = DatePicker;

/// AndroidX TimePickerState 对应的 24 小时时间值。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimePickerState {
    /// 小时，范围为 0..=23。
    pub hour: u8,
    /// 分钟，范围为 0..=59。
    pub minute: u8,
}

impl TimePickerState {
    /// 创建并限制小时与分钟到合法范围。
    pub fn new(hour: u8, minute: u8) -> Self {
        Self {
            hour: hour.min(23),
            minute: minute.min(59),
        }
    }
}

type TimeChangeHandler = Rc<dyn Fn(TimePickerState, &mut Window, &mut App)>;

fn time_adjustment_button(
    id: ElementId,
    label: &'static str,
    next: TimePickerState,
    handler: Option<TimeChangeHandler>,
) -> AnyElement {
    div()
        .id(id)
        .size(px(36.))
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .when_some(handler, |el, handler| {
            el.cursor_pointer()
                .on_click(move |_, window, cx| handler(next, window, cx))
        })
        .child(label)
        .into_any_element()
}

#[derive(IntoElement)]
pub struct TimePicker {
    id: ElementId,
    value: SharedString,
    state: TimePickerState,
    on_time_change: Option<TimeChangeHandler>,
}
impl TimePicker {
    /// 创建初始为零点的时间选择器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: "Select time".into(),
            state: TimePickerState::new(0, 0),
            on_time_change: None,
        }
    }
    /// 设置标题文字。
    pub fn value(mut self, v: impl Into<SharedString>) -> Self {
        self.value = v.into();
        self
    }
    /// 设置当前小时和分钟。
    pub fn state(mut self, state: TimePickerState) -> Self {
        self.state = state;
        self
    }
    /// 设置时间变化回调；调用者应保存新状态并重新渲染。
    pub fn on_time_change(
        mut self,
        handler: impl Fn(TimePickerState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_time_change = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for TimePicker {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let hour = self.state.hour;
        let minute = self.state.minute;
        let adjust = self.on_time_change.clone();
        let hour_controls = div()
            .flex()
            .flex_col()
            .items_center()
            .child(time_adjustment_button(
                (self.id.clone(), "hour-plus").into(),
                "+",
                TimePickerState::new((hour + 1) % 24, minute),
                adjust.clone(),
            ))
            .child(format!("{hour:02}"))
            .child(time_adjustment_button(
                (self.id.clone(), "hour-minus").into(),
                "−",
                TimePickerState::new((hour + 23) % 24, minute),
                adjust.clone(),
            ));
        let minute_controls = div()
            .flex()
            .flex_col()
            .items_center()
            .child(time_adjustment_button(
                (self.id.clone(), "minute-plus").into(),
                "+",
                TimePickerState::new(hour, (minute + 1) % 60),
                adjust.clone(),
            ))
            .child(format!("{minute:02}"))
            .child(time_adjustment_button(
                (self.id.clone(), "minute-minus").into(),
                "−",
                TimePickerState::new(hour, (minute + 59) % 60),
                adjust,
            ));
        div()
            .id(self.id)
            .p(px(16.))
            .rounded(px(12.))
            .bg(cx.theme().colors().surface_container_high)
            .text_color(cx.theme().colors().on_surface)
            .child(self.value)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(hour_controls)
                    .child(":")
                    .child(minute_controls),
            )
    }
}
pub type ExposedTimePicker = TimePicker;

/// AndroidX TimeInput 对应的 HH:MM 文本输入控件。
pub struct TimeInput {
    id: ElementId,
    state: TimePickerState,
    enabled: bool,
    on_time_change: Option<TimeChangeHandler>,
}

impl TimeInput {
    /// 创建 24 小时制时间输入框。
    pub fn new(id: impl Into<ElementId>, state: TimePickerState) -> Self {
        Self {
            id: id.into(),
            state,
            enabled: true,
            on_time_change: None,
        }
    }

    /// 设置输入框是否可编辑。
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// 设置合法时间变化回调；未完成或越界的中间输入不会提交。
    pub fn on_time_change(
        mut self,
        handler: impl Fn(TimePickerState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_time_change = Some(Rc::new(handler));
        self
    }

    /// 构建可编辑文本框实体。
    pub fn build(self, cx: &mut App) -> Entity<TextFieldState> {
        let mut input = TextField::new(self.id, "Time")
            .value(format!("{:02}:{:02}", self.state.hour, self.state.minute))
            .enabled(self.enabled);
        if let Some(handler) = self.on_time_change {
            input = input.on_value_change(move |text, window, cx| {
                if let Some((hour, minute)) = text.split_once(':')
                    && let (Ok(hour), Ok(minute)) = (hour.parse::<u8>(), minute.parse::<u8>())
                    && hour < 24
                    && minute < 60
                {
                    handler(TimePickerState::new(hour, minute), window, cx);
                }
            });
        }
        input.build(cx)
    }
}

#[derive(IntoElement)]
pub struct WideNavigationRail {
    id: ElementId,
    children: Vec<gpui::AnyElement>,
}
impl WideNavigationRail {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
        }
    }
}
impl ParentElement for WideNavigationRail {
    fn extend(&mut self, e: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(e)
    }
}
impl RenderOnce for WideNavigationRail {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .w(px(256.))
            .h_full()
            .flex()
            .flex_col()
            .gap(px(8.))
            .p(px(12.))
            .bg(cx.theme().colors().surface_container)
            .children(self.children)
    }
}

#[cfg(test)]
mod date_time_tests {
    use super::{
        DatePickerState, TimePickerState, days_in_month, weekday_of_first, year_month_from_days,
    };

    #[test]
    fn gregorian_month_lengths_and_weekdays_are_valid() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2100, 2), 28);
        assert_eq!(days_in_month(2000, 2), 29);
        assert_eq!(weekday_of_first(2026, 10), 3);
        assert_eq!(year_month_from_days(0), (1970, 1));
        assert_eq!(DatePickerState::new(2025, 2, Some(29)).selected_day, None);
    }

    #[test]
    fn time_state_limits_invalid_values() {
        assert_eq!(TimePickerState::new(24, 60), TimePickerState::new(23, 59));
    }
}
