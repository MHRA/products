# Configuration

Configuration of applications should be through environment variables (as is [required of 12-factor apps](https://12factor.net/config)).

## Setting up Environment Variables for Development

The projects in this monorepo will require some configuration via environment variables.

The names of these environment variables will be listed in a file named `.env.example`. You should make a copy of this file called `.env` before filling in values.

### Getting Development Environment Variables from MHRA's Azure Key Vault

If you have access to MHRA's Azure Key Vault, you'll be able to get the files with all required details filled in by running a `make` command (normally `make get-env`). Run `make help` to check.

Note that these assume:

* you've got the `az` command line tool installed;
* you're authenticated with your @mhra.gov.uk account; and
* the access policies have been set up to give you access to the `mhra-dev` vault.
