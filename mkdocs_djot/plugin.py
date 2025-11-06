from mkdocs.config import config_options
from mkdocs.config.defaults import MkDocsConfig
from mkdocs.plugins import BasePlugin
from mkdocs.structure.files import File, Files
from mkdocs.structure.pages import Page
from mkdocs.structure.toc import get_toc

from mkdocs_djot.jotdown_py import extract_metadata, render_to_html


class DjotPlugin(BasePlugin):
    config_scheme = (
        (
            "extensions",
            config_options.ListOfItems(
                config_options.Type(str), default=[".dj", ".djot"]
            ),
        ),
    )

    def should_include(self, file: File) -> bool:
        return file.src_path.endswith(tuple(self.config["extensions"]))

    def on_files(self, files: Files, /, *, config: MkDocsConfig) -> Files | None:
        for file in files:
            if self.should_include(file):
                file.is_documentation_page = lambda: True

                # Clear cached properties so they get recalculated
                for attr in ("dest_uri", "url", "abs_dest_path", "name"):
                    file.__dict__.pop(attr, None)

        return files

    def on_pre_page(
        self, page: Page, /, *, config: MkDocsConfig, files: Files
    ) -> Page | None:
        if not self.should_include(page.file):
            return page

        def djot_render(config: MkDocsConfig, files: Files) -> None:
            if page.markdown is None:
                raise RuntimeError("`markdown` field hasn't been set")

            metadata = extract_metadata(page.markdown)

            page._title_from_render = metadata["title"]
            page.toc = get_toc(metadata["toc_tokens"])
            page.present_anchor_ids = metadata["anchors"]

            page.content = render_to_html(page.markdown)

            page.links_to_anchors = {}

        page.render = djot_render

        return page
