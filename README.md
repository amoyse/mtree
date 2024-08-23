# mtree

This is a copy of the "tree" utility - I just wanted to try and build it in Rust.

## Installation

### Clone this repository
```
git clone https://github.com/amoyse/mtree.git
```

### Build as release
```
cargo build --release
```

### Install the binary locally
```
cargo install --path .
```

### Ensure binary directory in PATH
Add the below line to your .zshrc file.
```
export PATH="${HOME}/.cargo/bin:${PATH}"
```

Then reload shell config
```
source ~/.zhsrc
```

### Run it!
```
mtree --help
```

