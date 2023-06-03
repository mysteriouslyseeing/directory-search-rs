# ds-rs

## Usage

To run ds-rs, use the command `ds`:

```console
> ds --help
A command to search and display directory structures.

Usage: ds.exe [OPTIONS]

Options:
  -p, --path <PATH>      The path at which to start. If unspecified, the current directory is used.
  -r, --recursive        Enables recursion
  -d, --depth <DEPTH>    The depth to which to recurse. If unspecified, the depth is unlimited.
  -f, --show-files       Displays files along with directories
  -s, --show-symlinks    Displays symlinks along with directories
  -m, --matches <REGEX>  Only display directories, files and symlinks which match the given regex
  -h, --help             Print help
  -V, --version          Print version
```

## Regex

The regex parser is the one used by the Rust [regex](https://docs.rs/regex/1.8.3/regex/) crate. This means that the regex can match anywhere in a string. Therefore, the regex "a" matches any file name with an "a" in it. The start of a string can be matched with ^, and the end of a string can be matched with $. Most special characters, such as ".", must be escaped with a backslash ("\\")


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

### Display the entire contents of a directory recursively up to a depth of 4, only showing the files that end in .py:

```console
> ds --path "C:/Users/%USERNAME%/Documents" -r --depth 4 -f -s --matches \.py$
Displaying contents of "C:/Users/%USERNAME%/Documents"
="Projects"
 ="Python"
  ="python_project_1"
   -"main.py"
   -"utils.py"
  ="python_project_2"
   -"main.py"
  -"python_script.py"
```
Note that while even though none of the folders end in .py, they are still displayed in order to show directory structure.


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