[![Build Status](https://github.com/JKRhb/sdf-wot-converter/actions/workflows/test.yml/badge.svg)](https://github.com/JKRhb/sdf-wot-converter/actions/workflows/test.yml)

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
cargo install --git https://github.com/JKRhb/sdf-wot-converter.git
```

in a terminal of your choice.
The installation takes a while but once its compilation has been finished the converter should be usable right away.
It can be called using `sdf-wot-converter-cli` and one of the subcommands described below.

## Using the Command Line Tool

So far, the converter provides two commands (`convert` and `print`) that can be invoked from the command line.

### `convert`

This command converts an SDF to a WoT Thing Model file (the other direction will be implemented in an upcoming version).
It accepts a file path pointing to an SDF file as its first and the output path as its second argument.
Both SDF protocol bindings and Thing Descriptions are not covered yet but will be added soon.

**Example:**

```
sdf-wot-converter-cli convert examples/sdf/example.sdf.json example.tm.json
```

### `print`

This command serves primarily for debug purposes and will probably be removed in a later version.
It accepts a file path that must point to a WoT TD (whose filename must end with `td.json`), a WoT TM (ending with `tm.json`), or an SDF model (ending with `sdf.json`).
It reads in the model, and performs first a deserialization into Rust data structures followed by a serialization back into JSON.
The result is then printed in the terminal.

**Examples:**

```
sdf-wot-converter-cli print examples/sdf/example.sdf.json
sdf-wot-converter-cli print examples/wot/example.tm.json
sdf-wot-converter-cli print examples/wot/example.td.json
```

## License

This project is licensed under the MIT license.

```
SPDX-License-Identifier: MIT
```
