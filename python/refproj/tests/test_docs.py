from pathlib import Path

from mktestdocs import check_md_file


def test_md():
    """Test python snippets in markdown files"""
    for item in Path(__file__).parent.parent.glob("docs/**/*.md"):
        print(f"doctesting {item}")
        if item.is_file():
            check_md_file(item)
    check_md_file(Path(__file__).parent / "../README.md")
