"""Learning module importer."""

import json
import re
from pathlib import Path
from urllib.parse import parse_qs, urlparse

import click
import markdownify
import yaml
from bs4 import BeautifulSoup, NavigableString
from lxml import etree
from markdown import markdown

NAMESPACES = {"wcm": "http://www.stellent.com/wcm-data/ns/8.0.0"}
SITE_ROOT_DIRECTIVE = "[!--$ssServerRelativeSiteRoot--]"
HTTP_ROOT_DIRECTIVE = "[!--$HttpRelativeWebRoot--]"

# Map of CON codes which serve as redirects to known URLs.
CON_CODE_URL_MAP = {
    "CON123123": (
        "https://www.gov.uk/drug-safety-update/"
        "addiction-to-benzodiazepines-and-codeine"
    )
}


class MHRAMarkdownConverter(markdownify.MarkdownConverter):
    """MHRA learning module HTML to Markdown converter."""

    def __init__(self, content_prefix, asset_prefix, **kwargs):
        """Initialise converter."""
        super().__init__(**kwargs)
        self.content_prefix = content_prefix
        self.asset_prefix = asset_prefix
        self.stellent_assets_to_download = set()
        self.assets_with_unknown_type = set()
        self.index = dict()

    def convert(self, html):
        """Add footnotes to the end of converted document."""
        self.footnotes = []  # pylint: disable=attribute-defined-outside-init
        content = super().convert(html)
        for index, footnote in enumerate(self.footnotes):
            footnote_index = index + 1  # footnotes use 1-based index
            content += f"\n\n[^{footnote_index}]: {footnote}\n"

        return content

    def convert_a(self, el, text):
        """
        Convert an HTML anchor element to Markdown.

        If the anchor has the class glossary, convert it into a footnote.

        Handle Stellent's directives in URLs.
        """
        # pylint: disable=too-complex

        # Add an empty href if required.
        if el.get("href") is None:
            el["href"] = ""

        # If the anchor has the class glossary, convert it into a footnote.
        try:
            if "glossary" in el["class"]:
                self.footnotes.append(el["title"])
                footnote_index = len(self.footnotes)
                return f"{text}[^{footnote_index}]"
        except KeyError:
            # Element doesn't have a class attribute.
            pass

        # Strip out [!--$ssLink("…")--] directives.
        if el["href"].startswith("[!--$ssLink"):
            el["href"] = el["href"].replace(
                '[!--$ssLink("', "").replace('")--]', "")

        # Handle links to pages like
        # [!--$ssServerRelativeSiteRoot--]Opendocuments/OpenPDFdocuments/CON123
        if el["href"].startswith(
            SITE_ROOT_DIRECTIVE + "Opendocuments/OpenPDFdocuments"
        ):
            path = Path(el["href"])
            el["href"] = self.asset_prefix + path.stem.lower() + ".pdf"
            self.stellent_assets_to_download.add(path.stem)

        # Handle links to pages like /something/CON123?useSecondary=&showpage=456 or
        # [!--$ssServerRelativeSiteRoot--]something/CON123?useSecondary=&showpage=456
        url = urlparse(el["href"])
        query = parse_qs(url.query)
        if "showpage" in query:
            path = Path(url.path)
            el["href"] = self.content_prefix + \
                path.stem + "_" + query["showpage"][0]
            if url.fragment:
                el["href"] += "#" + url.fragment

        # Handle links to pages like [!--$ssServerRelativeSiteRoot--]Something/CON123
        if el["href"].startswith(SITE_ROOT_DIRECTIVE):
            path = Path(el["href"])

            if path.stem in CON_CODE_URL_MAP:
                el["href"] = CON_CODE_URL_MAP[path.stem]

            else:
                el["href"] = self.asset_prefix + path.stem.lower() + ".unknown"
                self.stellent_assets_to_download.add(path.stem)
                self.assets_with_unknown_type.add(path.stem)

        # Handle links to pages like [!--$HttpRelativeWebRoot--]/something/abc123.pdf
        if el["href"].startswith(HTTP_ROOT_DIRECTIVE):
            path = Path(el["href"])
            el["href"] = self.asset_prefix + path.name.lower()
            self.stellent_assets_to_download.add(path.stem)

        return super().convert_a(el, text)

    def convert_img(self, el, text):
        """Handle Stellent image URLs."""
        # Handle [!--$ssWeblayoutUrl()--] directives.
        if el["src"].startswith("[!--$ssWeblayoutUrl("):
            img_src = Path(
                el["src"].replace(
                    "[!--$ssWeblayoutUrl('", "").replace("')--]", "")
            )
            el["src"] = self.asset_prefix + img_src.name.lower()
            self.stellent_assets_to_download.add(img_src.stem)

        return super().convert_img(el, text)

    def process_text(self, text):
        """Ignore whitespace in document tree."""
        # Addressing https://github.com/matthewwithanm/python-markdownify/issues/17
        if text.isspace():
            text = ""

        return super().process_text(text)

    def markdown_to_html(self, tag, text):  # pylint: disable=no-self-use
        """Convert the processed content back into HTML."""
        tag.clear()
        if text:
            soup = BeautifulSoup(markdown(text), "lxml")
            for child_tag in soup.body.contents:
                tag.append(child_tag)

    def convert_td(self, cell_tag, text):
        """Convert a table cell."""
        for attr_to_delete in ["width", "valign", "style"]:
            if attr_to_delete in cell_tag.attrs:
                del cell_tag.attrs[attr_to_delete]

        self.markdown_to_html(cell_tag, text)

        return cell_tag.prettify()

    def convert_th(self, header_tag, text):
        """Convert a table heading cell."""
        return self.convert_td(header_tag, text)

    def convert_tr(self, row_tag, text):
        """Convert a table row."""
        row_tag.attrs = {}

        for el in row_tag.children:
            if isinstance(el, NavigableString):
                text += self.process_text(str(el))
            else:
                text += self.process_tag(el)

        return row_tag.prettify()

    def convert_table(self, table_tag, text):
        """Convert a table."""
        table_tag.attrs = {}

        # Collect footnotes from text.
        footnotes = ["<div class='collected-footnotes'>"]
        for match in re.finditer(r"\[\^(\d+)\]", text):
            footnotes.append(
                self.substitute_markdown_footnotes_with_html(match[0]))
        footnotes.append("</div>")

        # Replace footnotes with links.
        text = self.substitute_markdown_footnotes_with_html(
            text) + "".join(footnotes)

        self.markdown_to_html(table_tag, text)

        return table_tag.prettify()

    def convert_sub(
        self, sub_tag, text
    ):  # pylint: disable=unused-argument, no-self-use
        """Convert subscript text."""
        return str(sub_tag)

    def convert_expander(self, el, text):
        """Return expander HTML."""
        el.name = "Expander"

        # Collect footnotes from text.
        footnotes = ["<div class='collected-footnotes'>"]
        for match in re.finditer(r"\[\^(\d+)\]", text):
            footnotes.append(
                self.substitute_markdown_footnotes_with_html(match[0]))
        footnotes.append("</div>")

        # Replace footnotes with links.
        text = self.substitute_markdown_footnotes_with_html(
            text) + "".join(footnotes)

        self.markdown_to_html(el, text)

        return el.prettify()

    def substitute_markdown_footnotes_with_html(self, markdown):
        return re.sub(
            r"\[\^(\d+)\]",
            r"<sup><a href='#fn-\1' class='footnote-ref'>\1</a></sup>",
            markdown,
        )


def inject_expanders(html):
    """Inject Expander components into HTML."""
    soup = BeautifulSoup(html, "lxml")
    showhide_re = re.compile(r"^showhide\(['\"]([^'\"]*)['\"]\)")

    # Remove all close links.
    for close_tag in soup.find_all("a", onclick=showhide_re, title="Close"):
        assert len(close_tag.parent.contents) == 1
        close_tag.parent.extract()

    # Process all remaining links which match showhide_re.
    for expander_tag in soup.find_all("a", onclick=showhide_re):
        # Set up Expander component.
        expander_tag.name = "Expander"
        body_id = showhide_re.search(expander_tag["onclick"])[1]
        expander_tag.attrs = {}

        # Add title to Expander component.
        expander_tag.attrs["title"] = expander_tag.text
        expander_tag.clear()

        # Find body and move it into the Expander component.
        body_tag = soup.find(id=body_id)
        expander_tag.append(body_tag)

    return str(soup)


# pylint: disable=too-many-arguments
def import_row(row, index, md_converter, out_dir, con_code, path_to_expander_component):
    """Handle import of a row element."""
    # Generate base filename which is used to set up links between pages.
    stem = con_code + "_" + str(index + 1)

    # Extract data from XML.
    title = row.find("wcm:element[@name='Head']", namespaces=NAMESPACES).text
    html = row.find("wcm:element[@name='Body']", namespaces=NAMESPACES).text

    # Inject expanders into HTML
    html = inject_expanders(html)

    # Generate MDX
    front_matter = {"title": title}
    front_matter = yaml.dump(front_matter)
    mdx = (
        f"---\n{front_matter}---\n\n"
        + f"import Expander from '{path_to_expander_component}'\n\n"
        + md_converter.convert(html)
    )

    # Write MDX
    outfile = Path(out_dir) / Path(f"{stem}.mdx")
    outfile.write_text(mdx)

    # Return title and filename.
    return title, stem


def validate_con_code(context, param, value):  # pylint: disable=unused-argument
    """Validate CON code."""
    if not re.search(r"^CON\d+$", value):
        raise click.BadParameter("CON_CODE must be in the format CON123.")

    return value


@click.command()
@click.argument("xml_file", type=click.File("rb"))
@click.argument(
    "out_dir", type=click.Path(file_okay=False, dir_okay=True, writable=True)
)
@click.argument("con_code", type=click.STRING, callback=validate_con_code)
@click.argument("content_url_prefix", type=click.STRING)
@click.argument("asset_url_prefix", type=click.STRING)
@click.argument(
    "path_to_expander_component", type=click.STRING, default="../../components/expander"
)
def learning_importer(
    xml_file,
    out_dir,
    con_code,
    content_url_prefix,
    asset_url_prefix,
    path_to_expander_component,
):  # pylint: disable=too-many-arguments, too-many-locals
    """
    Convert XML_FILE to a series of MDX files in OUT_DIR.

    Files will be named CON_CODE_1, CON_CODE_2, etc.

    Links to content and assets will be prefixed with CONTENT_URL_PREFIX and
    ASSET_URL_PREFIX respectively.

    Expander component will be loaded from PATH_TO_EXPANDER_COMPONENT.
    """
    out_dir = Path(out_dir)
    if not out_dir.exists():
        click.echo(f"Creating output directory {out_dir}.")
        out_dir.mkdir()

    md_converter = MHRAMarkdownConverter(
        content_prefix=content_url_prefix, asset_prefix=asset_url_prefix
    )

    xml = etree.parse(xml_file)
    modules = []
    with click.progressbar(
        xml.findall("//wcm:row", namespaces=NAMESPACES),
        label="Extracting pages from XML",
    ) as rows:
        for index, row in enumerate(rows):
            name, content_stem = import_row(
                row, index, md_converter, out_dir, con_code, path_to_expander_component
            )

            modules.append(
                {"name": name, "link": content_url_prefix + content_stem})

    click.echo("Done!")

    outfile = Path(out_dir) / Path("modules.json")
    outfile.write_text(json.dumps(modules, indent=2))

    num_assets = len(md_converter.stellent_assets_to_download)
    click.echo(
        f"{num_assets} assets to manually download from Stellent to {asset_url_prefix}."
    )
    for asset in md_converter.stellent_assets_to_download:
        click.echo(f" * {asset}")

    if md_converter.assets_with_unknown_type:
        num_assets = len(md_converter.assets_with_unknown_type)
        click.echo(f"{num_assets} assets with unknown types.")
        click.echo("Extensions for these assets have been set to `.unknown`.")
        for asset in md_converter.assets_with_unknown_type:
            click.echo(f" * {asset}")


if __name__ == "__main__":
    learning_importer()  # pylint: disable=no-value-for-parameter
