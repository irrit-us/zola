# Site compiler fork

This fork starts at upstream Zola v0.23.4, commit
`28daab8d47cacb1e6c863b97739148f424433f5b`. It retains the site's native HTML
optimizations without requiring downstream patch application.

`markdown.highlighting.merge_highlight_tokens` merges adjacent raw tokens only
when their complete single/dual theme styles match. Merging precedes HTML escaping
and never crosses code lines. `markdown.highlighting.omit_plain_spans` omits spans
only for tokens matching the complete default style of both themes. Both options
default to false; source text, line numbers, hidden/highlighted lines and metadata
remain handled by Giallo.

A standalone top-level `[TOC]` paragraph (case-insensitive) renders a static,
nested table of contents using the same headings and anchors as `page.toc`.
Escaped markers, code, raw HTML, nested blocks, and defined reference links remain
literal. Source offsets distinguish escaped text from the actual marker. No
headings means no empty navigation. Override `templates/toc.html` to customize
the built-in markup; it receives `toc` and `lang`. Heading titles are escaped.

`markdown.render_math` (default false) enables pulldown-cmark's dollar-delimited
math parser, protecting TeX backslashes, underscores and asterisks from Markdown
interpretation. Math events are escaped and wrapped with MathJax delimiters;
code and escaped dollar signs stay literal. MathJax remains a browser renderer.

`vendor/giallo` contains the published Giallo 0.5.2 crate plus the plain-span
renderer change in `src/renderers/html.rs`. The original crate SHA-256 is
`019550a7656d0e9fd71fe163b491ae053ee252d8761f0c55787bdfb393aca7c9`.
Its license and bundled grammar data are retained. The workspace uses this local
dependency, with all other dependency versions preserved in Cargo.lock.

Build and check with Rust 1.95.0:

```sh
cargo +1.95.0 build --release --locked
cargo +1.95.0 test --release --locked -p markdown --lib compact::tests
cargo +1.95.0 test --release --locked -p markdown --lib toc_marker
cargo +1.95.0 test --release --locked -p markdown --lib math_markup
```

The website repository owns CI builds, regression tests and binary caching.
Upstream test, documentation deployment and release-publication workflows are
removed from this source fork; use the commands above for standalone validation.
