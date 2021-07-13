[![Build Status](https://github.com/JKRhb/sdf-wot-converter/actions/workflows/test.yml/badge.svg)](https://github.com/JKRhb/sdf-wot-converter/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/JKRhb/sdf-wot-converter/branch/main/graph/badge.svg?token=X7EEI07LXD)](https://codecov.io/gh/JKRhb/sdf-wot-converter)

# SDF-WoT-Converter

This repository provides a converter from [SDF](https://datatracker.ietf.org/doc/html/draft-ietf-asdf-sdf-05) (with protocol bindings) to [WoT TD](https://www.w3.org/TR/wot-thing-description/) including Thing Models.
The converter is still WIP but can already be used for experimenting with it.
It will be continously updated over the upcoming weeks.

In the end, the converter is supposed to be usable both as a tool for the command line as well as a library that can be built upon in other WoT and SDF related projects.

## Installing the Command Line Tool

For now, the converter has not been published to crates.io yet.
You can, however, install it from the Git repository using Rust's package manager `cargo`.

If you have [Rust installed](https://www.rust-lang.org/tools/install) run

```
cargo install --git https://github.com/JKRhb/sdf-wot-converter.git --branch main
```

in a terminal of your choice.
The installation takes a while but once its compilation has been finished the converter should be usable right away.
It can be called using `sdf-wot-converter` and one of the subcommands described below.
More detailed information can be obtained using `sdf-wot-converter --help`.

## Using the Command Line Tool

So far, the converter provides two commands (`convert` and `print`) that can be invoked from the command line.
The input path can either be an absolute or relative file path, or a URL with a `http` or `https` schema.

### `convert`

So far, this command can convert between an SDF model and a WoT Thing Model.
The conversion from WoT TM to SDF, however, is not really implemented at the moment.

The command accepts the arguments `--from-sdf` or `--from-tm` to specificy input files and `--to-tm` or `--to-sdf` to
specify output files. 
For the input arguments the file endings have to `sdf.json` or `tm.json`, respectively.

Both SDF protocol bindings and Thing Descriptions are not covered yet but will be added soon.

**Example:**

```
sdf-wot-converter convert --from-sdf examples/sdf/example.sdf.json --to-tm result.tm.json
sdf-wot-converter convert --from-tm examples/wot/example.tm.json --to-sdf result.sdf.json
```

Using the same file format as input and output will simply copy the content of the input file.

### `print`

This command serves primarily for debug purposes and will probably be removed in a later version.
It accepts a file path that must point to a WoT TD (whose filename must end with `td.json`), a WoT TM (ending with `tm.json`), or an SDF model (ending with `sdf.json`).
It reads in the model, and performs first a deserialization into Rust data structures followed by a serialization back into JSON.
The result is then printed in the terminal.

**Examples:**

```
sdf-wot-converter print examples/sdf/example.sdf.json
sdf-wot-converter print examples/wot/example.tm.json
sdf-wot-converter print examples/wot/example.td.json
```

## License

This project is licensed under the MIT license.

```
SPDX-License-Identifier: MIT
```
