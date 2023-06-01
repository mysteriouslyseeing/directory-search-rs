# ds-rs

## Usage

To run ds-rs, use the command `ds`:

```console
> ds --help
A command to search and display directory structures.

Usage: ds.exe [OPTIONS]

Options:
  -p, --path <PATH>    The path at which to start. If unspecified, the current directory is used.
  -r, --recursive      Enables recursion
  -d, --depth <DEPTH>  The depth to which to recurse. If unspecified, the depth is unlimited.
  -f, --show-files     Displays files along with directories
  -s, --show-symlinks  Displays symlinks along with directories
  -h, --help           Print help
  -V, --version        Print version
```

## Examples

### Display the directories inside a directory

```console
> ds --path "C:/Users/%USERNAME%/Documents"
Displaying contents of "C:/Users/%USERNAME%/Documents"
="Github"
="Projects"
```

### Display the entire contents of a directory

```console
> ds --path "C:/Users/%USERNAME%/Documents" -f -s
Displaying contents of "C:/Users/%USERNAME%/Documents"
="GitHub"
="Projects"
-"Resume.docx"
-"Resume.pdf"
```

### Display the entire contents of a directory recursively up to a depth of 3:

```console
> ds --path "C:/Users/%USERNAME%/Documents" -r --depth 3 -f -s
Displaying contents of "C:/Users/%USERNAME%/Documents"
="GitHub"
="Projects"
 ="Python"
  ="python_project_1"
  ="python_project_2"
  -"python_script.py"
 ="Rust"
  ="directory-search-rs"
  -"new_bin_and_lib.bat"
-"Resume.docx"
-"Resume.pdf"
```

## Installation

### cargo install

The easiest way to install this binary is through `cargo install`:

```console
cargo install ds-rs
```

### Build from source

To build from source, clone the git repository, build with cargo, and move the binary to a stable folder.
```console
> git clone https://github.com/mysteriouslyseeing/directory-search-rs
> cd directory-search-rs
> cargo build --release
> cp ./target/release/ds /path/to/destination/
```