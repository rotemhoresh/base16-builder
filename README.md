# base16-builder

CLI utility for building themes from base16 color schemes and templates.

## Installation

Clone this repository:

```bash
git clone git@github.com:rotemhoresh/base16-builder.git
```

Build with cargo:

```bash
cd base16-builder
cargo build --release

# optional, move the binary somewhere
mv target/release/base16-builder <path>

# optional, add it's path to the PATH
export PATH="$PATH:<path>"
```

## Usage

```bash
base16-builder --help
```

```
    A utility for building themes from base16 schemes
   
    You need to have a templates directory, with a config.toml file, of the
    following format:
   
    <TEMPLATE_NAME>:
        extension: <FILE_EXTENSION>
        output: <OUTPUT_DIR>
   
    Example:
   
    default:
        extension: toml
        output: themes
   
    For each template you specify in your config.toml, a `<TEMPLATE_NAME>.mustache`
    must exist in the templates directory.
   
    Alongside the templates directory, a schemes directory is required, which
    will contain on your schemes in the base16 format as `.yaml` files:
   
    scheme: <SCHEME_NAME>
    author: <AUTHOR>
    base00: <HEX_WITHOUT_HASH>  e.g., `aabbcc`
    ...
    base0F: <HEX_WITHOUT_HASH>
   
    For each template, a theme will be created for each scheme. Example tree
    state after running the builder using the examples from above:
   
    templates
    ├── config.yaml
    └── default.mustache
    schemes
    └── name.yaml
    themes
    └── base16-default-name.toml
```

## Status

It's a quick and dirty implementation right now.

### Contributions

All, in any form, are welcome and appreciated :)

## License

This project is licenced under [MIT](https://choosealicense.com/licenses/mit/).
