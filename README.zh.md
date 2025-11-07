# mkdocs-djot: Use Djot in MkDocs

[English](README.md) | [中文](README.zh.md)

mkdocs-djot 是一个 [MkDocs](https://github.com/mkdocs/mkdocs) 插件，让你可以使用 [Djot](https://djot.net/) 标记语言编写文档。

由于目前没有 Python 版本的 Djot 解析库，本插件采用 Python/Rust 混合架构，通过 [PyO3](https://github.com/pyo3/pyo3) 绑定调用 Rust 实现的 [jotdown](https://github.com/hellux/jotdown) 解析器。

## 特性

- 渲染 Djot 格式文档
- 自定义文件扩展名
- 生成目录和锚点

## 安装

前置要求：已安装 Rust 工具链。

```
uv add git+https://github.com/13m0n4de/mkdocs-djot
```

## 使用

在 `mkdocs.yml` 中启用插件：

```yaml
plugins:
  - djot
```

然后创建 `.dj` 或 `.djot` 文件作为文档页面。

### 配置选项

```yaml
plugins:
  - djot:
      extensions: [.dj, .djot] # 默认值
```

## 与其他插件一起使用

由于许多 MkDocs 插件依赖 Markdown 解析器和 Markdown Token 信息，无法保证与它们兼容。

不过在某些情况下，可以通过在 Djot 中构造相同的 HTML 结构来达到相同效果。例如 [Admonitions](https://squidfunk.github.io/mkdocs-material/reference/admonitions/)：

原 Markdown 写法：

```markdown
!!! note "Phasellus posuere in sem ut cursus"

    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla et euismod
    nulla. Curabitur feugiat, tortor non consequat finibus, justo purus auctor
    massa, nec semper lorem quam in massa.
```

在 Djot 中可以这么写：

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

它们会生成同样的 HTML，获得同样的页面效果：

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

## 已知问题

### Material for MkDocs blog 插件

与 [Material for MkDocs 的 blog 插件](https://squidfunk.github.io/mkdocs-material/plugins/blog/) 一起使用时，索引页面仍会将文档以 Markdown 格式处理。

解决方法：

1. 摘要部分尽量使用与 Markdown 兼容的 Djot 语法
1. 修改 blog 插件的摘要分隔符配置，使用 Djot 注释语法

```yaml
plugins:
  - blog:
      post_excerpt_separator: {% more %}
```

注意：blog 插件不支持多个分隔符。混用 Markdown 和 Djot 时，只能接受插入不合自身语法的“注释标记”了。

### 代码高亮

代码块无法使用 [Pygments](https://pygments.org/) 进行高亮，可以使用 [highlight.js](https://highlightjs.org/) 替代：

```yaml
extra_css:
  - https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/styles/default.min.css

extra_javascript:
  - https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/highlight.min.js
  - javascripts/init.js # hljs.initHighlightingOnLoad();

markdown_extensions:
  - pymdownx.highlight:
      use_pygments: false
```
