# Learning Module Importer

Tool which takes learning module XML files exported from Stellent and produces a series of Markdown files based on it.

# Getting Started

Requires [pipenv](https://pipenv.readthedocs.io/), which can be installed via `brew`.

```
pipenv install
pipenv run python learning_importer.py ./in_data/benzodiazepines.xml ../web/content/modules/benzodiazepines CON234573 /benzodiazepines/ ../../assets/ ../../../src/components/Expander
pipenv run python learning_importer.py ./in_data/antipsychotics.xml ../web/content/modules/antipsychotics CON155606 /antipsychotics/ ../../assets/ ../../../src/components/Expander
pipenv run python learning_importer.py ./in_data/opioids.xml ../web/content/modules/opioids CON143740 /opioids/ ../../assets/ ../../../src/components/Expander
pipenv run python learning_importer.py ./in_data/ssri.xml ../web/content/modules/ssri CON146583 /ssri/ ../../assets/ ../../../src/components/Expander
```

The CON number is an ID which identifies this learning module in Stellent.

# Example Output

Get some help:

```
$ pipenv run python learning_importer.py --help
Usage: learning_importer.py [OPTIONS] XML_FILE OUT_DIR CON_CODE
                            CONTENT_URL_PREFIX ASSET_URL_PREFIX
                            [PATH_TO_EXPANDER_COMPONENT]

  Convert XML_FILE to a series of MDX files in OUT_DIR.

  Files will be named CON_CODE_1, CON_CODE_2, etc.

  Links to content and assets will be prefixed with CONTENT_URL_PREFIX and
  ASSET_URL_PREFIX respectively.

  Expander component will be loaded from PATH_TO_EXPANDER_COMPONENT.

Options:
  --help  Show this message and exit.
```

Run an import:

```
$ pipenv run python learning_importer.py in_data/ssri.xml ../web/content/modules/ssri CON146583 /ssri/ ../../assets/ ../../../src/components/Expander
Extracting pages from XML  [####################################]  100%
Done!
18 assets to manually download from Stellent to ../../assets/.
 * con134939
 * con134799
 * con134800
 * con131992
 * con132004
 * con126283
 * con134942
 * con131999
 * con131996
 * con134806
 * con134797
 * con134798
 * con134940
 * con132003
 * con132000
 * con131886
 * con131833
 * con132009
```

Here, 18 assets needs to be manually downloaded from Stellent.

# Linting, Testing and Whatnot

```
pipenv install --dev
pipenv run isort --check-only
pipenv run black --check --diff .
find . -iname "*.py" | xargs pipenv run pylint
pipenv run pytest
```
