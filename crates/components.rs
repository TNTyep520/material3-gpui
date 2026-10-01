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

#[path = "components/Badge.rs"]
mod badge;
#[path = "components/BottomSheet.rs"]
mod bottom_sheet;
#[path = "components/Button.rs"]
mod button;
#[path = "components/ButtonGroup.rs"]
mod button_group;
#[path = "components/Card.rs"]
mod card;
#[path = "components/Carousel.rs"]
mod carousel;
#[path = "components/Checkbox.rs"]
mod checkbox;
#[path = "components/Chip.rs"]
mod chip;
#[path = "components/DatePicker.rs"]
mod date_picker;
#[path = "components/Dialog.rs"]
mod dialog;
#[path = "components/Divider.rs"]
mod divider;
#[path = "components/ExposedDropdownMenu.rs"]
mod exposed_dropdown_menu;
#[path = "components/Fab.rs"]
mod fab;
#[path = "components/FabMenu.rs"]
mod fab_menu;
#[path = "components/FloatingToolbar.rs"]
mod floating_toolbar;
#[path = "components/IconButton.rs"]
mod icon_button;
#[path = "components/List.rs"]
mod list;
#[path = "components/LoadingIndicator.rs"]
mod loading_indicator;
#[path = "components/Navigation.rs"]
mod navigation;
#[path = "components/Progress.rs"]
mod progress;
#[path = "components/Radio.rs"]
mod radio;
#[path = "components/Scaffold.rs"]
mod scaffold;
#[path = "components/Scrollbar.rs"]
mod scrollbar;
#[path = "components/SearchBar.rs"]
mod search_bar;
#[path = "components/SegmentedButton.rs"]
mod segmented_button;
#[path = "components/Slider.rs"]
mod slider;
#[path = "components/SplitButton.rs"]
mod split_button;
#[path = "components/SwipeToDismissBox.rs"]
mod swipe_to_dismiss_box;
#[path = "components/Switch.rs"]
mod switch;
#[path = "components/Tabs.rs"]
mod tabs;
#[path = "components/TextField.rs"]
mod text_field;
#[path = "components/TimePicker.rs"]
mod time_picker;
#[path = "components/ToggleButton.rs"]
mod toggle_button;
#[path = "components/TopAppBar.rs"]
mod top_app_bar;

pub use badge::{Badge, BadgeStyle, BadgedBox, badged};
pub use bottom_sheet::{BottomSheetScaffold, BottomSheetStyle, ModalBottomSheet};
pub use button::{
    Button, ButtonState, ButtonVariant, ElevatedButton, FilledTonalButton, OutlinedButton,
    TextButton,
};
pub use button_group::{ButtonGroup, ButtonGroupMenuState, ButtonGroupStyle};
pub use card::{Card, CardVariant, ElevatedCard, OutlinedCard};
pub use carousel::Carousel;
pub use checkbox::{Checkbox, CheckboxState, ToggleableState, TriStateCheckbox};
pub use chip::{
    AssistChip, Chip, ChipState, ChipVariant, ElevatedAssistChip, ElevatedFilterChip,
    ElevatedSuggestionChip, FilterChip, InputChip, SuggestionChip,
};
pub use date_picker::{DatePicker, DatePickerState, ExposedDatePicker};
pub use dialog::{AlertDialog, BasicAlertDialog, DatePickerDialog, Dialog, TimePickerDialog};
pub use divider::{Divider, HorizontalDivider, VerticalDivider};
pub use exposed_dropdown_menu::{
    ExposedDropdownMenu, ExposedDropdownMenuBox, ExposedDropdownMenuStyle,
};
pub use fab::{
    ExtendedFloatingActionButton, Fab, FabColor, FabSize, FabState, FloatingActionButton,
    LargeExtendedFloatingActionButton, LargeFloatingActionButton,
    MediumExtendedFloatingActionButton, MediumFloatingActionButton,
    SmallExtendedFloatingActionButton, SmallFloatingActionButton,
};
pub use fab_menu::{
    FabMenu, FabMenuState, FloatingActionButtonMenu, FloatingActionButtonMenuItem,
    ToggleFloatingActionButton,
};
pub use floating_toolbar::{
    FloatingToolbar, FloatingToolbarState, HorizontalFloatingToolbar, VerticalFloatingToolbar,
};
pub use icon_button::{
    FilledIconButton, FilledIconToggleButton, FilledTonalIconButton, FilledTonalIconToggleButton,
    IconButton, IconButtonColors, IconButtonDefaults, IconButtonShape, IconButtonSize,
    IconButtonState, IconButtonVariant, IconToggleButton, IconToggleButtonColors,
    OutlinedIconButton, OutlinedIconToggleButton,
};
pub use list::{List, ListItem, SegmentedListItem};
pub use loading_indicator::{ContainedLoadingIndicator, LoadingIndicator, LoadingIndicatorVariant};
pub use navigation::{
    DismissibleDrawerSheet, DismissibleNavigationDrawer, DrawerEntry, ModalDrawerSheet,
    ModalNavigationDrawer, NavigationBar, NavigationBarItem, NavigationBarState, NavigationDrawer,
    NavigationDrawerItem, NavigationDrawerState, NavigationItemSpec, NavigationRail,
    NavigationRailItem, NavigationRailState, PermanentDrawerSheet, PermanentNavigationDrawer,
    WideNavigationRail,
};
pub use progress::{
    CircularProgress, CircularProgressIndicator, CircularWavyProgressIndicator, LinearProgress,
    LinearProgressIndicator, LinearWavyProgressIndicator, WavyCircularProgressIndicator,
    WavyProgressIndicator,
};
pub use radio::{RadioButton, RadioState};
pub use scaffold::Scaffold;
pub use scrollbar::Scrollbar;
pub use search_bar::SearchBar;
pub use segmented_button::{
    MultiChoiceSegmentedButtonRow, SegmentedButton, SegmentedButtonRow, SegmentedButtonRowState,
    SegmentedButtonSelectionMode, SingleChoiceSegmentedButtonRow,
};
pub use slider::{RangeSlider, Slider, SliderSize, SliderState, VerticalSlider};
pub use split_button::{SplitButton, SplitButtonLayout, SplitButtonStyle};
pub use swipe_to_dismiss_box::{SwipeToDismissBox, SwipeToDismissBoxState, SwipeToDismissBoxValue};
pub use switch::{Switch, SwitchState};
pub use tabs::{
    LeadingIconTab, PrimaryScrollableTabRow, PrimaryTabRow, SecondaryScrollableTabRow,
    SecondaryTabRow, Tab, TabBar, TabBarState, TabRow, TabRowVariant,
};
pub use text_field::{OutlinedTextField, SecureTextField, TextField, TextFieldState};
pub use time_picker::{ExposedTimePicker, TimeInput, TimePicker, TimePickerState};
pub use toggle_button::{
    ElevatedToggleButton, FilledTonalToggleButton, OutlinedToggleButton, ToggleButton,
    ToggleButtonState, ToggleButtonStyle, ToggleButtonVariant,
};
pub use top_app_bar::{
    BottomAppBar, BottomAppBarState, CenterAlignedTopAppBar, FlexibleBottomAppBar,
    LargeFlexibleTopAppBar, LargeTopAppBar, MediumFlexibleTopAppBar, MediumTopAppBar, TopAppBar,
    TopAppBarVariant, TwoRowsTopAppBar,
};

#[path = "components/Icon.rs"]
pub mod icon;
#[path = "components/Overlay.rs"]
pub mod overlay;
pub use button::ButtonStyle;
pub use card::CardStyle;
pub use checkbox::CheckboxStyle;
pub use chip::ChipStyle;
pub use dialog::DialogStyle;
pub use divider::DividerStyle;
pub use fab::FabStyle;
pub use icon::{Icon, IconName};
pub use icon_button::IconButtonStyle;
pub use list::ListItemStyle;
pub use navigation::{
    NavigationBarStyle, NavigationDrawerStyle, NavigationItemStyle, NavigationRailStyle,
};
pub use overlay::{
    MenuItem, MenuState, PlainTooltip, RichTooltip, Snackbar, SnackbarHost, TooltipBox,
};
pub use overlay::{MenuStyle, SnackbarStyle, TooltipStyle};
pub use progress::{CircularProgressStyle, LinearProgressStyle};
pub use radio::RadioStyle;
pub use segmented_button::SegmentedButtonStyle;
pub use slider::SliderStyle;
pub use switch::SwitchStyle;
pub use tabs::TabBarStyle;
pub use text_field::TextFieldStyle;
pub use top_app_bar::TopAppBarStyle;
