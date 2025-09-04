"""Run each notebook in target folder"""

import sys
from pathlib import Path
import papermill as pm


if __name__ == "__main__":
    # Test every notebook file in this folder
    notebooks_dir = Path(sys.argv[1])
    notebooks = [
        notebook_path
        for notebook_path in notebooks_dir.iterdir()
        if notebook_path.is_file() and notebook_path.suffix == ".ipynb"
    ]

    for notebook in notebooks:
        pm.execute_notebook(notebook, "/dev/null")
