# days

A simple CLI tool for calculating how far away dates are from each other. Inspired by [WolframAlpha](https://www.wolframalpha.com/)'s ability to do this with natural language inputs.

The focus is primarily on days as the unit of time (hence the name of the crate), but support for other units of time — both small and large — is planned.

Features:

- Calculate the date N days from today.
- Calculate how many days are left until a given date.
- Calculate how many days have passed since a given date.

## Usage

```bash
# install it
cargo install days
# use it!
days since 1970-01-01
```

### Supported commands

You can always run `days --help` to see the supported commands.

`since`: Calculate how many days have passed since a given date.

```text
days since 1970-01-01
```

`until`: Calculate how many days are left until a given date.

```text
days until 2038-01-19
```

`count`: Calculate the date N days from today.

```text
days count 100
```

## Features Yet to Come

The intention is to add support for natural language inputs to make it easier to use on the fly while keeping the feel of a robust CLI tool by providing consistent options for input and output formats.

Currently planned:

- Support for automatically detecting certain date formats.
- Support for manually specifying the date format you want to use.
- Options to output date results in a customisable format (or choose from a list of built-in ones).
- Options to output number-of-days results in a different unit of time (seconds, weeks, centuries).
- Support for date-and-time inputs and outputs.
- Support for calculating number of days between two arbitrary dates.
- Support for referring to past and upcoming dates by their name (like "Christmas" or "next Tuesday").

## Limitations

This tool is based on [`chrono`](https://github.com/chronotope/chrono#readme), the excellent Rust date/time library. Note that this has some limitations when working with dates outside of the Gregorian calendar. See their [limitations section](https://github.com/chronotope/chrono#limitations) for full details.
