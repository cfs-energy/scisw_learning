# Python for Engineering

This class focuses on (1) exposure to concepts and (2) providing templates for success.

The underlying goal is to achieve ***good taste in software***, 
which can be reached much earlier than complete understanding.

The repetition required to fully learn the material is expected to
happen under self-study or on-the-job outside class time.

## Syllabus

  This class was originally taught in one hour per week segments over the course of 10 weeks
  without a rigidly fixed schedule.

  Multiple groups were run simultaneously, and while schedule was largely driven by discussion,
  the variability in class time between the groups was only about 10%.

  * Setup & git version control primer
  * Language fundamentals - imports, syntax, types, collections, patterns
  * Base libraries
  * Classes - definitions, inheritance, methods, properties, caching
  * Packaging - project structure, build systems, versioning, executable entrypoints, dependency management, github repo configuration, documentation, command line interfaces, logging
  * Quality control - testing (pytest, doctesting, example testing, coverage), linting & formatting, type hinting & checking, CI w/ github actions, publishing

## Installation & Setup

### Install essentials
* If you're on Windows, install WSL2
  * Anaconda-managed python environments are not recommended for this project
* Python (3.11 or newer)
  * Preferably via `uv`, which allows managing multiple pythons
* Pip, uv, git, vscode

### Set up a virtual environment
* This allows deconflicting dependencies between projects and (usually) prevents you from accidentally bricking your system env
* **Do not work in your global environment! It will rapidly become misconfigured**

Using `<stuff>` to denote examples and placeholders.

```bash
cd ~/
mkdir envs
cd envs
uv venv <short_env_name_without_spaces_like_"310">
ls # should show a folder called `310` or whatever you named it
```

Then, to add a convenience function to activate that virtual env:

```bash
nano ~/.bashrc # linux
OR
nano ~/.profile # mac
```

At the bottom of the file, add this function:

```bash
activate () {
  source ~/envs/$1/bin/activate ;
 }
```

This function will be available next time you open a new terminal, or after running `source ~/.bashrc` (linux) or `source ~/.profile` (mac).

Activate the virtual env using the new function like `activate 310`.
Now, any python packages you install will be placed in that virtual env instead of overwriting your system env.

### Install python deps in virtual environment

Install the class's python library deps:

```bash
uv pip install -r requirements.txt
```

## Git Reference

Using `<stuff>` to denote examples and placeholders.

```bash
# Copy a repo to a local folder
git clone <repo clone link>

# Check what has been modified or staged
git status

# Update local version from server
git pull

# Make a branch
git checkout -b <branch name>

# Switch to a branch that already exists
git checkout <branch name>

# Send changes to server
#   `add` "stages" changes locally; Marks things to commit
#   `commit` marks a group of changes; still locally
#   `push` sends changes to the server
git add .  
git commit -m "<some message>"
git push

# Obliterate local changes (only for files already controlled by the repo)
git reset --hard

# Delete local files and folders that aren't version-controlled
git clean -fd
```

Git has a lot more features not discussed here that are best found when needed.
