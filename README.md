# Pocket-resources

## Usage

See the demo crate.

Tweak your Cargo.toml to use a build script:

```toml
[package]
# ...
build = "build.rs"

[build-dependencies]
pocket-resources = "*"
```

Create a `build.rs` file:

```rust
extern crate pocket_resources;

fn main() {
    pocket_resources::package(&[("resources","path/to/image.png")]).unwrap();
}
```

Include the resources where you want:

```rust
include!(concat!(env!("OUT_DIR"), "/pocket-resources.rs"));
```

This creates a public enum named `Resource`. If you want to name it something else, or if you want it private, you should use a module.

You can then load the resource directly from the enum:

```rust
let data: &[u8] = ResourceId::PathToImagepng.load();
```

Or load it at runtime:

```rust
let data: &[u8] = ResourceId::from_name("path/to/image.png").unwrap().load();
```
