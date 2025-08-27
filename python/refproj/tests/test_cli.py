from click.testing import CliRunner

from refproj.cli import cli


def test_cli():
    runner = CliRunner()
    result = runner.invoke(cli, ["echo", "Peter", "--suffix", "XVI"])
    assert result.exit_code == 0
    assert result.output == "Hello, Peter XVI!\n"

    # Note we don't test the GUI here because it does not return
