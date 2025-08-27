import click

from . import gui


@click.group()
def cli():
    pass


@cli.command("echo")
@click.argument("name")
@click.option("--suffix", "suffix")
def echo(name: str, suffix: str | None):
    extra = "" if suffix is None else " " + suffix
    print(f"Hello, {name}{extra}!")


@cli.command("gui")
def gui_cmd():
    gui.gui()
