# Petri

Petri is minimal language to model, simulate and visualize P/T nets.

## Installation

> [!Note]
> For the time being the Rust toolchain is required. Prebuild releases will be added at a future point in time.

```
git clone https://github.com/Palladium02/petri.git
cd petri
cargo build --release
```

## Usage

### Exporting a Petri net definition to an image format.

> [!Note]
> The export subcommand requires graphviz to be installed.

```shell
petri export -i path/to/file.ptr -o path/to/output.png
```

### Simulating a Petri net

The interactive subcommand loads the Petri net definition into a REPL.

```shell
petri interactive -i path/to/file.ptr
```

#### REPL Commands
- `.quit`: used to exit the REPL
- `.help`: used to print all available commands
- `fire <T>`: used to fire transition
- `show`: used to show the state of the net
- `trace`: used to print a trace of all fired transitions in their respective order
