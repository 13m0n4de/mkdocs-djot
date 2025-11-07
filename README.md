# mkdocs-djot: Use Djot in MkDocs

[English](README.md) | [中文](README.zh.md)

A [MkDocs](https://github.com/mkdocs/mkdocs) plugin that lets you write docs in [Djot](https://djot.net/) markup.

Since there's no Python Djot parser available, this plugin uses a mixed Rust/Python project, calling the [jotdown](https://github.com/hellux/jotdown) parser through [PyO3](https://github.com/pyo3/pyo3) bindings.

## Features

- Render Djot documents
- Custom file extensions
- Auto-generated TOC and anchors

## Installation

Prerequisites: Rust toolchain installed.

```bash
uv add git+https://github.com/13m0n4de/mkdocs-djot
```

## Usage

Enable the plugin in your `mkdocs.yml`:

```yaml
plugins:
  - djot
```

Then create `.dj` or `.djot` files for your documentation pages.

### Configuration

```yaml
plugins:
  - djot:
      extensions: [.dj, .djot] # default
```

## Using with Other Plugins

Many MkDocs plugins depend on the Markdown parser and token info, so compatibility isn't guaranteed.

But in some cases, you can get the same effect by constructing the same HTML structure in Djot. For example, [Admonitions](https://squidfunk.github.io/mkdocs-material/reference/admonitions/):

Original Markdown:

```markdown
!!! note "Phasellus posuere in sem ut cursus"

    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla et euismod
    nulla. Curabitur feugiat, tortor non consequat finibus, justo purus auctor
    massa, nec semper lorem quam in massa.
```

Djot equivalent:

```djot
{.admonition .note}
:::
{.admonition-title}
Phasellus posuere in sem ut cursus

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla et euismod
nulla. Curabitur feugiat, tortor non consequat finibus, justo purus auctor
massa, nec semper lorem quam in massa.
:::
```

They generate the same HTML:

```html
<div class="admonition note">
 <p class="admonition-title">
  Phasellus posuere in sem ut cursus
 </p>
 <p>
  Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla et euismod
nulla. Curabitur feugiat, tortor non consequat finibus, justo purus auctor
massa, nec semper lorem quam in massa.
 </p>
</div>
```

## Known Issues

### Material for MkDocs Blog Plugin

When using the [Material for MkDocs blog plugin](https://squidfunk.github.io/mkdocs-material/plugins/blog/), index pages will still process documents as Markdown.

Workarounds:

1. Use Markdown-compatible Djot syntax in excerpt sections
1. Change the blog plugin's excerpt separator to use Djot comment syntax:

```yaml
plugins:
  - blog:
      post_excerpt_separator: {% more %}
```

Note: The blog plugin doesn't support multiple separators. If you have both Markdown and Djot docs, you'll have to accept writing "comment markers" from one format in the other.

### Code Highlighting

Code blocks won't be highlighted by [Pygments](https://pygments.org/). Use [highlight.js](https://highlightjs.org/) instead:

```yaml
extra_css:
  - https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/styles/default.min.css

extra_javascript:
  - https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/highlight.min.js
  - javascripts/init.js  # Contains: hljs.initHighlightingOnLoad();

markdown_extensions:
  - pymdownx.highlight:
      use_pygments: false
```
