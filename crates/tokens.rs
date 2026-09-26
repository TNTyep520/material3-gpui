#[path = "tokens/Generated.rs"]
mod generated;

pub use generated::*;

use gpui::{Corners, FontWeight, Hsla, Pixels, SharedString, Size, Styled, px, rgb};

use crate::motion::Easing;
use crate::theme::{DEFAULT_FONT_FAMILY, TypeStyle};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dp(pub f32);

impl Dp {
    pub const fn pixels(self) -> Pixels {
        px(self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sp(pub f32);

impl Sp {
    pub fn pixels(self, font_scale: f32) -> Pixels {
        px(self.0 * font_scale)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorValue(pub u32);

impl ColorValue {
    pub fn resolve(self) -> Hsla {
        rgb(self.0).into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontFamilyToken {
    SansSerif,
}

impl FontFamilyToken {
    pub fn resolve(self) -> SharedString {
        DEFAULT_FONT_FAMILY.into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShapeValue {
    Rounded {
        top_start: Dp,
        top_end: Dp,
        bottom_end: Dp,
        bottom_start: Dp,
    },
    Full,
}

impl ShapeValue {
    pub const fn rounded(radius: f32) -> Self {
        Self::Rounded {
            top_start: Dp(radius),
            top_end: Dp(radius),
            bottom_end: Dp(radius),
            bottom_start: Dp(radius),
        }
    }

    pub fn corners(self, size: Size<Pixels>, right_to_left: bool) -> Corners<Pixels> {
        let half_short_side = size.width.min(size.height).max(px(0.)) / 2.;
        match self {
            Self::Full => Corners::all(half_short_side),
            Self::Rounded {
                top_start,
                top_end,
                bottom_end,
                bottom_start,
            } => {
                let corners = Corners {
                    top_left: if right_to_left { top_end } else { top_start }.pixels(),
                    top_right: if right_to_left { top_start } else { top_end }.pixels(),
                    bottom_left: if right_to_left {
                        bottom_end
                    } else {
                        bottom_start
                    }
                    .pixels(),
                    bottom_right: if right_to_left {
                        bottom_start
                    } else {
                        bottom_end
                    }
                    .pixels(),
                };
                let corners = corners.map(|radius| (*radius).max(px(0.)));
                let mut scale = 1.0_f32;
                for (sum, limit) in [
                    (corners.top_left + corners.top_right, size.width),
                    (corners.bottom_left + corners.bottom_right, size.width),
                    (corners.top_left + corners.bottom_left, size.height),
                    (corners.top_right + corners.bottom_right, size.height),
                ] {
                    if sum > px(0.) {
                        scale = scale.min(f32::from(limit.max(px(0.))) / f32::from(sum));
                    }
                }
                corners.map(|radius| *radius * scale)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineHeightAlignment {
    Center,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineHeightTrim {
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineHeightStyle {
    pub alignment: LineHeightAlignment,
    pub trim: LineHeightTrim,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextStyleDefaults {
    pub line_height_style: LineHeightStyle,
    pub platform_default: bool,
}

pub const DEFAULT_LINE_HEIGHT_STYLE: LineHeightStyle = LineHeightStyle {
    alignment: LineHeightAlignment::Center,
    trim: LineHeightTrim::None,
};

pub const DEFAULT_TEXT_STYLE: TextStyleDefaults = TextStyleDefaults {
    line_height_style: DEFAULT_LINE_HEIGHT_STYLE,
    platform_default: true,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextStyleToken {
    pub font_family: FontFamilyToken,
    pub weight: FontWeight,
    pub size: Sp,
    pub line_height: Sp,
    pub tracking: Sp,
}

impl TextStyleToken {
    pub fn type_style(self, font_scale: f32) -> TypeStyle {
        TypeStyle {
            size: self.size.pixels(font_scale),
            line_height: self.line_height.pixels(font_scale),
            weight: self.weight,
            tracking: self.tracking.0 * font_scale,
        }
    }

    pub fn resolve(self, font_family: Option<SharedString>, font_scale: f32) -> ResolvedTextStyle {
        ResolvedTextStyle {
            font_family: font_family.unwrap_or_else(|| self.font_family.resolve()),
            style: self.type_style(font_scale),
            defaults: DEFAULT_TEXT_STYLE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResolvedTextStyle {
    pub font_family: SharedString,
    pub style: TypeStyle,
    pub defaults: TextStyleDefaults,
}

impl ResolvedTextStyle {
    pub fn apply<E: Styled>(&self, element: E) -> E {
        self.style
            .apply(element.font_family(self.font_family.clone()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenValue {
    Dp(Dp),
    Sp(Sp),
    Float(f32),
    Double(f64),
    Color(ColorValue),
    ColorRole(ColorToken),
    Shape(ShapeValue),
    ShapeRole(ShapeToken),
    TypographyRole(TypographyToken),
    MotionRole(MotionSchemeToken),
    Easing(Easing),
    FontFamily(FontFamilyToken),
    FontWeight(FontWeight),
    TextStyle(TextStyleToken),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TokenEntry {
    pub group: &'static str,
    pub name: &'static str,
    pub value: TokenValue,
}

pub fn find(group: &str, name: &str) -> Option<&'static TokenEntry> {
    ALL_TOKENS
        .iter()
        .find(|entry| entry.group == group && entry.name == name)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use gpui::{FontWeight, px, size};

    use crate::motion::{MotionRole, MotionScheme};
    use crate::styles::ButtonStyle;
    use crate::styles::button::ButtonVariant;
    use crate::theme::{Theme, ThemeMode, TokenSet};

    use super::*;

    #[test]
    fn registry_covers_every_source_group_without_duplicate_names() {
        assert_eq!(SOURCE_FILE_COUNT, 120);
        assert_eq!(ALL_TOKENS.len(), 2671);
        let names: HashSet<_> = ALL_TOKENS
            .iter()
            .map(|entry| (entry.group, entry.name))
            .collect();
        let groups: HashSet<_> = ALL_TOKENS.iter().map(|entry| entry.group).collect();
        assert_eq!(names.len(), ALL_TOKENS.len());
        assert_eq!(groups.len(), SOURCE_FILE_COUNT);
        assert_eq!(
            find("SliderTokens", "HandleWidth").map(|entry| entry.value),
            Some(TokenValue::Dp(Dp(4.)))
        );
        assert!(find("SliderTokens", "Missing").is_none());
        assert_eq!(ColorToken::ALL.len(), 48);
        assert_eq!(TypographyToken::ALL.len(), 30);
        assert_eq!(ShapeToken::ALL.len(), 15);
        assert_eq!(MotionSchemeToken::ALL.len(), 6);
        assert_eq!(ColorToken::Primary.id(), 25);
        assert_eq!(TypographyToken::TitleSmallEmphasized.id(), 29);
    }

    #[test]
    fn every_semantic_reference_resolves_in_both_themes() {
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            let tokens = TokenSet::androidx(mode);
            for entry in ALL_TOKENS {
                match entry.value {
                    TokenValue::ColorRole(role) => assert!(role.resolve(&tokens).a.is_finite()),
                    TokenValue::ShapeRole(role) => {
                        let corners = role
                            .resolve(&tokens)
                            .corners(size(px(100.), px(40.)), false);
                        assert!(corners.top_left >= px(0.));
                        assert!(corners.top_left + corners.bottom_left <= px(40.));
                    }
                    TokenValue::TypographyRole(role) => {
                        assert!(role.resolve(&tokens).size > px(0.))
                    }
                    TokenValue::MotionRole(role) => {
                        assert!(role.resolve(&tokens).spring.stiffness > 0.)
                    }
                    _ => {}
                }
            }
            for role in MotionSchemeToken::ALL {
                assert!(role.resolve(&tokens).spring.stiffness > 0.);
            }
        }
    }

    #[test]
    fn androidx_palette_and_fixed_roles_match_upstream() {
        let light = TokenSet::androidx(ThemeMode::Light);
        let dark = TokenSet::androidx(ThemeMode::Dark);
        assert_eq!(light.colors.primary, ColorValue(0x6750a4).resolve());
        assert_eq!(
            light.colors.on_primary_container,
            ColorValue(0x21005d).resolve()
        );
        assert_eq!(dark.colors.surface, ColorValue(0x141218).resolve());
        assert_eq!(light.colors.primary_fixed, dark.colors.primary_fixed);
        assert_eq!(light.colors.primary_fixed, ColorValue(0xeaddff).resolve());
        assert_eq!(
            light.colors.on_tertiary_fixed,
            ColorValue(0x31111d).resolve()
        );
        assert_eq!(light.colors.shadow, ColorValue(0).resolve());
    }

    #[test]
    fn semantic_tokens_use_theme_overrides() {
        let mut tokens = TokenSet::androidx(ThemeMode::Light);
        tokens.colors.primary = ColorValue(0xabcdef).resolve();
        tokens.typography.label_large.size = px(23.);
        tokens.shapes.large = px(19.);
        assert_eq!(
            FilledButtonTokens::CONTAINER_COLOR.resolve(&tokens),
            tokens.colors.primary
        );
        assert_eq!(TypographyToken::LabelLarge.resolve(&tokens).size, px(23.));
        assert_eq!(
            ShapeToken::CornerLarge.resolve(&tokens),
            ShapeValue::rounded(19.)
        );
        let style = ButtonStyle::resolve(&tokens, ButtonVariant::Filled, false, false);
        assert_eq!(style.container_color, Some(tokens.colors.primary));
        assert_eq!(style.label.size, px(23.));
    }

    #[test]
    fn logical_corners_mirror_and_full_shapes_scale_to_bounds() {
        let bounds = size(px(100.), px(40.));
        let left = ShapeTokens::CORNER_LARGE_START.corners(bounds, false);
        let right = ShapeTokens::CORNER_LARGE_START.corners(bounds, true);
        assert_eq!(left.top_left, px(16.));
        assert_eq!(left.top_right, px(0.));
        assert_eq!(left.top_left, right.top_right);
        assert_eq!(left.top_right, right.top_left);
        let full = ShapeTokens::CORNER_FULL.corners(size(px(5000.), px(3000.)), false);
        assert_eq!(full.top_left, px(1500.));
        let small = ShapeTokens::CORNER_EXTRA_LARGE_TOP.corners(size(px(20.), px(10.)), false);
        assert_eq!(small.top_left, px(10.));
        assert_eq!(small.bottom_left, px(0.));
        let empty = ShapeTokens::CORNER_FULL.corners(size(px(0.), px(0.)), false);
        assert_eq!(empty.top_left, px(0.));
    }

    #[test]
    fn typography_retains_emphasis_tracking_and_font_override() {
        let normal = TypographyTokens::BODY_MEDIUM.type_style(1.);
        let emphasized = TypographyTokens::LABEL_LARGE_EMPHASIZED.type_style(1.);
        assert_eq!(normal.tracking, 0.2);
        assert_eq!(emphasized.weight, FontWeight::BOLD);
        assert_eq!(TypographyTokens::DISPLAY_LARGE.tracking, Sp(-0.2));
        let scaled = TypographyTokens::BODY_LARGE.resolve(Some("Example".into()), 1.5);
        assert_eq!(scaled.font_family.as_ref(), "Example");
        assert_eq!(scaled.style.size, px(24.));
        assert_eq!(scaled.style.line_height, px(36.));
        assert_eq!(scaled.defaults.line_height_style, DEFAULT_LINE_HEIGHT_STYLE);
        let theme = Theme::androidx(ThemeMode::Dark);
        assert_eq!(
            theme.typography().label_large_emphasized.weight,
            FontWeight::BOLD
        );
    }

    #[test]
    fn expressive_motion_changes_spatial_but_retains_effects() {
        let standard = MotionScheme::standard();
        let expressive = MotionScheme::expressive();
        for role in [
            MotionRole::FastEffects,
            MotionRole::DefaultEffects,
            MotionRole::SlowEffects,
        ] {
            assert_eq!(standard.spec(role), expressive.spec(role));
        }
        for (role, stiffness, damping) in [
            (MotionRole::FastSpatial, 800., 0.6),
            (MotionRole::DefaultSpatial, 380., 0.8),
            (MotionRole::SlowSpatial, 200., 0.8),
        ] {
            let spring = expressive.spec(role).spring;
            assert_eq!(spring.stiffness, stiffness);
            assert!((spring.damping_ratio - damping).abs() < 0.000001);
        }
    }

    #[cfg(feature = "dynamic-color")]
    #[test]
    fn fixed_roles_are_seed_dependent_but_mode_independent() {
        use crate::theme::{Profile, color_scheme_from_seed};
        let light = color_scheme_from_seed(0x006a6a, false, Profile::Baseline2021);
        let dark = color_scheme_from_seed(0x006a6a, true, Profile::Baseline2021);
        assert_eq!(light.primary_fixed, dark.primary_fixed);
        assert_eq!(light.on_primary_fixed, dark.on_primary_fixed);
        assert_ne!(
            light.primary_fixed,
            ColorLightTokens::PRIMARY_FIXED.resolve()
        );
    }
}
