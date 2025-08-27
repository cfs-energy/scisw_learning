"""Run each notebook in target folder"""

import os
import sys
from pathlib import Path
import papermill as pm


if __name__ == "__main__":
    # Test every notebook file in this folder
    notebooks_dir = Path(sys.argv[1])
    notebooks = [
        notebooks_dir / x
        for x in os.listdir(notebooks_dir)
        if (os.path.isfile(notebooks_dir / x) and x[-3:] == ".ipynb")
    ]

    for notebook in notebooks:
        pm.execute_notebook(notebook, "/dev/null")
