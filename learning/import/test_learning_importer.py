"""Learning module importer tests."""

import click
import pytest

import learning_importer


def html_to_markdown(html):
    """Convert HTML to Markdown."""
    md_converter = learning_importer.MHRAMarkdownConverter()
    return md_converter.convert(html)


def test_glossary_link():
    """Test glossary links."""
    result = html_to_markdown(
        '<a class="glossary tooltip" title="Standard placeholder text">'
        "Lorem ipsum foobar</a>"
    )
    assert "Lorem ipsum foobar[^1]" in result
    assert "[^1]: Standard placeholder text" in result


def test_slash_learning_module_href():
    """Test links to Stellent controlled pages."""
    result = html_to_markdown(
        '<a href="/some-learning-module/ABC123?useSecondary=&showpage=456">'
        "Page 456 of ABC 123</a>"
    )
    assert "[Page 456 of ABC 123](ABC123_456.html)" in result

    # Also test with weird encoded ampersand thing.
    result = html_to_markdown(
        '<a href="/some-learning-module/ABC123?useSecondary=&#38;showpage=456">'
        "Page 456 of ABC 123</a>"
    )
    assert "[Page 456 of ABC 123](ABC123_456.html)" in result


def test_site_root_directive_href():
    """Test links with $ssServerRelativeSiteRoot directives."""
    result = html_to_markdown(
        '<a href="[!--$ssServerRelativeSiteRoot--]A/Bunch/Of/Things/'
        'ABC123?useSecondary=&#38;showpage=456">'
        "Page 456 of ABC 123</a>"
    )
    assert "[Page 456 of ABC 123](ABC123_456.html)" in result

    # Test with PDF content.
    md_converter = learning_importer.MHRAMarkdownConverter()
    result = md_converter.convert(
        '<a href="[!--$ssServerRelativeSiteRoot--]Opendocuments/OpenPDFdocuments/'
        'ABC123">ABC 123 document</a>'
    )
    assert "[ABC 123 document](stellent/ABC123.pdf)" in result
    assert md_converter.stellent_assets_to_download == set(["ABC123"])
    assert md_converter.assets_with_unknown_type == set()

    # Test with unknown content.
    md_converter = learning_importer.MHRAMarkdownConverter()
    result = md_converter.convert(
        '<a href="[!--$ssServerRelativeSiteRoot--]A/Bunch/Of/Things/'
        'ABC123">ABC 123 thing</a>'
    )
    assert "[ABC 123 thing](stellent/ABC123.unknown)" in result
    assert md_converter.stellent_assets_to_download == set(["ABC123"])
    assert md_converter.assets_with_unknown_type == set(["ABC123"])


def test_sslink_directive_href():
    """Test links with $ssLink directives."""
    result = html_to_markdown(
        "<a href='[!--$ssLink(\"ABC123?useSecondary=&#38;showpage=456\")--]'>"
        "Page 456 of ABC 123</a>"
    )
    assert "[Page 456 of ABC 123](ABC123_456.html)" in result

    # Test with fragment.
    result = html_to_markdown(
        "<a href='[!--$ssLink(\"ABC123?useSecondary=&#38;showpage=456#fragment\")--]'>"
        "Page 456 of ABC 123</a>"
    )
    assert "[Page 456 of ABC 123](ABC123_456.html#fragment)" in result


def test_http_relative_web_root_directive_href():
    """Test links with $HttpRelativeWebRoot directives."""
    md_converter = learning_importer.MHRAMarkdownConverter()
    result = md_converter.convert(
        "<a href='[!--$HttpRelativeWebRoot--]/something/abc123.pdf'>"
        "ABC 123 document</a>"
    )
    assert "[ABC 123 document](stellent/abc123.pdf)" in result
    assert md_converter.stellent_assets_to_download == set(["abc123"])


def test_web_layout_url_src():
    """Test image URLs with $ssWeblayoutUrl directives."""
    md_converter = learning_importer.MHRAMarkdownConverter()
    result = md_converter.convert(
        "<img src=\"[!--$ssWeblayoutUrl('ab/cd/abc123.jpg')--]\" "
        "alt='ABC 123' title='Image for ABC 123' />"
        "<img src=\"[!--$ssWeblayoutUrl('ab/cd/abc123.jpg')--]\" />"
        "<img src=\"[!--$ssWeblayoutUrl('ab/cd/xyz789.jpg')--]\" />"
    )
    assert '![ABC 123](stellent/abc123.jpg "Image for ABC 123")' in result
    assert md_converter.stellent_assets_to_download == set(["abc123", "xyz789"])


@pytest.mark.parametrize("con_code", ["CON0", "CON123", "CON9999999999999"])
def test_valid_con_code(con_code):
    """Test valid CON codes."""
    assert con_code == learning_importer.validate_con_code(None, None, con_code)


@pytest.mark.parametrize("con_code", ["NOTCON123", "CON", "CON-1", "con123"])
def test_invalid_con_code(con_code):
    """Test invalid CON codes."""
    with pytest.raises(click.BadParameter):
        learning_importer.validate_con_code(None, None, con_code)
