# Learning Module Importer

Tool which takes learning module XML files exported from Stellent and produces a series of Markdown files based on it.

# Getting Started

Requires [pipenv](https://pipenv.readthedocs.io/), which can be installed via `brew`.

```
pipenv install
pipenv run python learning_importer.py /path/to/export.xml CON123
```

The CON number is an ID which identifies this learning module in Stellent.

# Example Output

Get some help:

```
$ pipenv run python learning_importer.py --help
Usage: learning_importer.py [OPTIONS] XML_FILE CON_CODE [OUT_DIR]

  Convert XML_FILE containing CON_CODE to a series of Markdown files in
  OUT_DIR.

Options:
  --help  Show this message and exit.
```

Run an import:

```
$ pipenv run python learning_importer.py /path/to/export.xml CON234573
Extracting pages from XML  [####################################]  100%          
Done!
17 assets to manually download from Stellent to CON234573/stellent.
 * con247106
 * con247109
 * con146653
 * con247110
 * con247105
 * con247107
 * con247112
 * con134939
 * con2024428
 * con247111
 * con236837
 * con131833
 * con247104
 * con236838
 * CON123123
 * con247108
 * con236839
1 assets with unknown types.
Extensions for these assets have been set to `.unknown`.
 * CON123123
```

Here, 17 assets needs to be manually downloaded from Stellent.

Also, the asset `CON123123` is of an unknown format and needs to be investigated, then links to the asset need to be manually fixed.

# Linting, Testing and Whatnot

```
pipenv install --dev
pipenv run isort --check-only
pipenv run black --check --diff .
find . -iname "*.py" | xargs pipenv run pylint
pipenv run pytest
```
