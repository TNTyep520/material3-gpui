use gpui::{Hsla, Pixels, px};

use crate::theme::{TokenSet, TypeStyle};
use crate::tokens::OutlinedSegmentedButtonTokens;

#[derive(Clone, Copy, Debug)]
pub struct SegmentedButtonStyle {
    pub container_color: Option<Hsla>,
    pub content_color: Hsla,
    pub outline_color: Hsla,
    pub outline_width: Pixels,
    pub height: Pixels,
    pub min_width: Pixels,
    pub corner_radius: Pixels,
    pub horizontal_padding: Pixels,
    pub icon_size: Pixels,
    pub gap: Pixels,
    pub label: TypeStyle,
}

impl SegmentedButtonStyle {
    pub fn resolve(tokens: &TokenSet, selected: bool, disabled: bool) -> Self {
        let colors = &tokens.colors;
        Self {
            container_color: selected.then_some(colors.secondary_container),
            content_color: if disabled {
                colors.disabled_content(&tokens.state_layer)
            } else if selected {
                colors.on_secondary_container
            } else {
                colors.on_surface
            },
            outline_color: if disabled {
                colors
                    .outline
                    .opacity(tokens.state_layer.disabled_container)
            } else {
                colors.outline
            },
            outline_width: OutlinedSegmentedButtonTokens::OUTLINE_WIDTH.pixels(),
            height: OutlinedSegmentedButtonTokens::CONTAINER_HEIGHT.pixels(),
            min_width: px(64.),
            corner_radius: tokens.shapes.full,
            horizontal_padding: px(12.),
            icon_size: OutlinedSegmentedButtonTokens::ICON_SIZE.pixels(),
            gap: px(8.),
            label: OutlinedSegmentedButtonTokens::LABEL_TEXT_FONT.resolve(tokens),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::theme::Theme;

    use super::SegmentedButtonStyle;

    #[test]
    fn disabled_selection_retains_container_in_both_themes() {
        for theme in [Theme::light(), Theme::dark()] {
            let tokens = theme.token_set();
            let selected = SegmentedButtonStyle::resolve(tokens, true, true);
            let unselected = SegmentedButtonStyle::resolve(tokens, false, true);
            assert_eq!(
                selected.container_color,
                Some(tokens.colors.secondary_container)
            );
            assert_eq!(unselected.container_color, None);
            assert_eq!(
                selected.content_color,
                tokens.colors.disabled_content(&tokens.state_layer)
            );
            assert_eq!(selected.content_color, unselected.content_color);
            assert_eq!(
                selected.outline_color,
                tokens
                    .colors
                    .outline
                    .opacity(tokens.state_layer.disabled_container)
            );
            let enabled = SegmentedButtonStyle::resolve(tokens, true, false);
            assert_eq!(enabled.content_color, tokens.colors.on_secondary_container);
            assert_eq!(enabled.outline_color, tokens.colors.outline);
        }
    }
}
