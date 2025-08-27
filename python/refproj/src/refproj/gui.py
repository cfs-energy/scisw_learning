import numpy as np
import plotly.graph_objects as go
from dash import Dash, Input, Output, callback, dcc, html

from .logging import log, logger_setup_default


def gui():
    """Run a simple example GUI"""
    logger_setup_default()
    log().info("Initializing GUI")

    # Set up the GUI page layout
    app = Dash()
    app.layout = [
        html.H1(children="Wiggly Lines", style={"textAlign": "center"}),
        dcc.Slider(0.0, 20.0, 0.1, marks=None, value=2.0, id="slider"),
        dcc.Graph(id="graph-content"),
    ]

    # Set up GUI callback function tree
    @callback(Output("graph-content", "figure"), Input("slider", "value"))
    def update_graph(value):
        x = np.linspace(0.0, 2.0 * np.pi, 1000)
        y = np.sin(value * x) + np.sin(np.cosh(value / 20.0 * x))
        fig = go.Figure(data=go.Scatter(x=x, y=y, mode="lines", line={"color": "black"}))
        return fig

    # NOTE: disabling threading is often necessary for GUI frontends for
    # simulations that have some state that is hidden from the GUI, which
    # otherwise causes race conditions because the GUI is not aware of a
    # need to control the ordering of updates.
    app.run(debug=True, threaded=True)
