# libyuv-rs

Raw FFI bindings to libyuv libraries.


### Quick start

Add the following to your Cargo.toml:

```toml
libyuv = "0.1.0"
```

### Building

#### Automatic

The libyuv crate will automatically find the precompiled static library files in the git libyuv repo release.

#### Manual

A set of environment variables can be used to point libyuv towards. They will override the automatic detection logic.
* `YUV_LIBRARY_PATH` - libyuv static library path, this will skip downloading and use your static library.


### License
[GPL](./LICENSE) Copyright (c) 2022 Mr.Panda.