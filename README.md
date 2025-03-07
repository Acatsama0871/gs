# gs

A CLI tool to interact with your Google Scholar page in Rust.

## Installation

```bash
cargo install --git https://github.com/Acatsama0871/gs.git
```

## Environment Variables

Two environment variables are needed:

- `GOOGLE_SCHOLAR_ID`: This can be got in the profile page: `https://scholar.google.com/citations?user=<This is the GOOGLE_SCHOLAR_ID>=en&oi=ao`

- `SERP_API_KEY`: Under the hood, the program use [Serp API's Google Scholar endpoint](https://serpapi.com/google-scholar-api) to get the data. You can register a free account and get the API key at [here](https://serpapi.com/).

Make sure those environment variables are visible by setting them temporarily or in your shell's `rc` file

```bash
export GOOGLE_SCHOLAR_ID=xxxxx
export SERP_API_KEY=xxxxx
```

## Usage

### All Subcommands

```bash
A simple CLI tool for interacting with Google Scholar

Usage: gs [COMMAND]

Commands:
  show  Show the citation info.
  log   Log the current info as a checkpoint
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `show`

```bash
Show the citation info.

Usage: gs show

Options:
  -h, --help  Print help
```
