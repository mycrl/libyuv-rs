# libyuv-rs

Raw FFI bindings to libyuv libraries，This is an unsafe package.


### Quick start

Add the following to your Cargo.toml:

```toml
libyuv = "0.1"
```

Convert ARGB to NV12:

```rs
let argb = vec![0u8; 1280 * 720 * 4];
let mut nv12 = vec![0u8; 1280 * 720 * 1.5];

let ret = libyuv::argb_to_nv12(
    argb.as_ptr(),
    1280 * 4,
    nv12.as_mut_ptr(),
    1280,
    nv12.as_mut_ptr().add(1280 * 720),
    1280,
    1280,
    720,
);

assert_eq!(ret, 0);
```


### License
[MIT](./LICENSE) Copyright (c) 2022 Mr.Panda.
