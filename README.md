# Vitrum

Vitrum is a pure rust implementation of a 3D renderer. It is not aiming to be efficient or ultra modern.
It is primarily for me to play with implementing ideas from papers and learning more about the field.

## Running with Cargo

To see the usage instructions run (arguments & output should not be considered stable):

```
cargo run --release -- -h
```

## Features

Implemented:

 - Render output to PNG
 - Run in a window with arrow key movement
 - Support simple Lambert shading
 - Supports STL (binary & ascii) files
