use giallo::HighlightedText;

/// Coalesce raw tokens before Giallo escapes and renders them. Never cross line
/// boundaries or merge styles that differ in either theme. Structural wrappers,
/// hidden/highlighted lines, numbering and metadata remain owned by Giallo.
pub(crate) fn merge_highlight_tokens(lines: &mut [Vec<HighlightedText>]) {
    for line in lines {
        line.dedup_by(|token, previous| {
            if token.style == previous.style {
                previous.text.push_str(&token.text);
                true
            } else {
                false
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use giallo::{FontStyle, Style, ThemeVariant};

    fn token(text: &str, style: ThemeVariant<Style>) -> HighlightedText {
        HighlightedText { text: text.into(), style }
    }

    #[test]
    fn compact_preserves_raw_entities_whitespace_and_lines() {
        let style = ThemeVariant::Single(Style::default());
        let mut lines = vec![
            vec![token("&", style), token("apos", style), token(";\t<中文>", style)],
            vec![],
            vec![token("  ", style), token("&#x", style), token("3c;", style)],
        ];
        merge_highlight_tokens(&mut lines);
        assert_eq!(
            lines,
            vec![vec![token("&apos;\t<中文>", style)], vec![], vec![token("  &#x3c;", style)],]
        );
    }

    #[test]
    fn compact_compares_both_themes_and_all_style_attributes() {
        let normal = Style::default();
        let decorated = Style { font_style: FontStyle::UNDERLINE, ..normal };
        let base = ThemeVariant::Dual { light: normal, dark: normal };
        let dark_only = ThemeVariant::Dual { light: normal, dark: decorated };
        let light_only = ThemeVariant::Dual { light: decorated, dark: normal };
        let mut lines = vec![vec![
            token("a", base),
            token("b", base),
            token("c", dark_only),
            token("d", light_only),
            token("e", base),
        ]];
        merge_highlight_tokens(&mut lines);
        assert_eq!(
            lines[0],
            vec![
                token("ab", base),
                token("c", dark_only),
                token("d", light_only),
                token("e", base),
            ]
        );
    }

    #[test]
    fn compact_plain_spans_escape_text_and_preserve_dark_only_styles() {
        use giallo::{HighlightOptions, HtmlRenderer, Registry, RenderOptions};
        let registry = Registry::builtin().unwrap();
        let options = HighlightOptions::new(
            "plain",
            ThemeVariant::Dual { light: "github-light", dark: "github-dark" },
        );
        let mut highlighted =
            registry.highlight("&apos; <script>\t中文\n\nlast", &options).unwrap();
        let mut renderer =
            HtmlRenderer { css_class_prefix: Some("z-".into()), ..Default::default() };
        let before = renderer.render(&highlighted, &RenderOptions::default());
        assert!(before.contains("<span>"));
        renderer.omit_plain_spans = true;
        let after = renderer.render(&highlighted, &RenderOptions::default());
        assert!(!after.contains("<span>"));
        assert!(after.contains("&amp;apos; &lt;script&gt;\t中文"));
        assert_eq!(before.matches("giallo-l").count(), after.matches("giallo-l").count());
        if let ThemeVariant::Dual { dark, .. } = &mut highlighted.tokens[0][0].style {
            dark.font_style = FontStyle::BOLD;
        }
        let styled = renderer.render(&highlighted, &RenderOptions::default());
        assert!(styled.contains("z-d-b"));
    }
}
