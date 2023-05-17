# libyuv-rs

Raw FFI bindings to libyuv libraries.


### Quick start

Add the following to your Cargo.toml:

```toml
libyuv = "0.1"
```

### Building

The libyuv crate will automatically find the precompiled static library files in the git libyuv repo release.

* `YUV_LIBRARY_PATH` - libyuv static library path, this will skip downloading and use your static library.

#### Pre-requisites

You'll need to have depot tools installed: https://www.chromium.org/developers/how-tos/install-depot-tools Refer to chromium instructions for each platform for other prerequisites.

#### Getting the Code

Create a working directory, enter it, and run:

```bash
gclient config --name src https://chromium.googlesource.com/libyuv/libyuv
gclient sync
```

Then you'll get a .gclient file like:

```js
solutions = [
  { "name"        : "src",
    "url"         : "https://chromium.googlesource.com/libyuv/libyuv",
    "deps_file"   : "DEPS",
    "managed"     : True,
    "custom_deps" : {
    },
    "safesync_url": "",
  },
];
```

For iOS add ;target_os=['ios']; to your OSX .gclient and run gclient sync.

#### Android

For Android add ;target_os=['android']; to your Linux .gclient

```js
solutions = [
  { "name"        : "src",
    "url"         : "https://chromium.googlesource.com/libyuv/libyuv",
    "deps_file"   : "DEPS",
    "managed"     : True,
    "custom_deps" : {
    },
    "safesync_url": "",
  },
];
target_os = ["android", "linux"];
```

Then run:

```bash
gclient sync
```

To get just the source (not buildable):

```bash
git clone https://chromium.googlesource.com/libyuv/libyuv
```

#### Building the Library

##### Windows

```bash
call gn gen out\Release "--args=is_debug=false target_cpu=\"x64\""
call gn gen out\Debug "--args=is_debug=true target_cpu=\"x64\""
ninja -v -C out\Release
ninja -v -C out\Debug

call gn gen out\Release "--args=is_debug=false target_cpu=\"x86\""
call gn gen out\Debug "--args=is_debug=true target_cpu=\"x86\""
ninja -v -C out\Release
ninja -v -C out\Debug
```

##### macOS and Linux

```bash
gn gen out/Release "--args=is_debug=false"
gn gen out/Debug "--args=is_debug=true"
ninja -v -C out/Release
ninja -v -C out/Debug
```

##### iOS

[http://www.chromium.org/developers/how-tos/build-instructions-ios](http://www.chromium.org/developers/how-tos/build-instructions-ios)

Add to .gclient last line: target_os=['ios'];

* arm64

```bash
gn gen out/Release "--args=is_debug=false target_os=\"ios\" ios_enable_code_signing=false target_cpu=\"arm64\""
gn gen out/Debug "--args=is_debug=true target_os=\"ios\" ios_enable_code_signing=false target_cpu=\"arm64\""
ninja -v -C out/Debug libyuv_unittest
ninja -v -C out/Release libyuv_unittest
```

* ios simulator

```bash
gn gen out/Release "--args=is_debug=false target_os=\"ios\" ios_enable_code_signing=false use_xcode_clang=true target_cpu=\"x86\""
gn gen out/Debug "--args=is_debug=true target_os=\"ios\" ios_enable_code_signing=false use_xcode_clang=true target_cpu=\"x86\""
ninja -v -C out/Debug libyuv_unittest
ninja -v -C out/Release libyuv_unittest
```

##### Android

[https://code.google.com/p/chromium/wiki/AndroidBuildInstructions](https://code.google.com/p/chromium/wiki/AndroidBuildInstructions)

Add to .gclient last line: target_os=['android'];

* arm64

```bash
gn gen out/Release "--args=is_debug=false target_os=\"android\" target_cpu=\"arm64\""
gn gen out/Debug "--args=is_debug=true target_os=\"android\" target_cpu=\"arm64\""
ninja -v -C out/Debug libyuv_unittest
ninja -v -C out/Release libyuv_unittest
```

* armv7

```bash
gn gen out/Release "--args=is_debug=false target_os=\"android\" target_cpu=\"arm\""
gn gen out/Debug "--args=is_debug=true target_os=\"android\" target_cpu=\"arm\""
ninja -v -C out/Debug libyuv_unittest
ninja -v -C out/Release libyuv_unittest
```

* ia32

```bash
gn gen out/Release "--args=is_debug=false target_os=\"android\" target_cpu=\"x86\""
gn gen out/Debug "--args=is_debug=true target_os=\"android\" target_cpu=\"x86\""
ninja -v -C out/Debug libyuv_unittest
ninja -v -C out/Release libyuv_unittest
```

* mips

```bash
gn gen out/Release "--args=is_debug=false target_os=\"android\" target_cpu=\"mips64el\" mips_arch_variant=\"r6\" mips_use_msa=true is_component_build=true"
gn gen out/Debug "--args=is_debug=true target_os=\"android\" target_cpu=\"mips64el\" mips_arch_variant=\"r6\" mips_use_msa=true is_component_build=true"
ninja -v -C out/Debug libyuv_unittest
ninja -v -C out/Release libyuv_unittest
```

##### Build targets

```bash
ninja -C out/Debug libyuv
ninja -C out/Debug libyuv_unittest
ninja -C out/Debug compare
ninja -C out/Debug yuvconvert
ninja -C out/Debug yuvconstants
ninja -C out/Debug psnr
ninja -C out/Debug cpuid
```

##### ARM Linux

```bash
gn gen out/Release "--args=is_debug=false target_cpu=\"arm64\""
gn gen out/Debug "--args=is_debug=true target_cpu=\"arm64\""
ninja -v -C out/Debug libyuv_unittest
ninja -v -C out/Release libyuv_unittest
```

##### MIPS Linux

```bash
gn gen out/Release "--args=is_debug=false target_os="linux" target_cpu="mips64el" mips_arch_variant="loongson3" is_component_build=false use_sysroot=false use_gold=false” 
gn gen out/Debug "--args=is_debug=true target_os="linux" target_cpu="mips64el" mips_arch_variant="loongson3" is_component_build=false use_sysroot=false use_gold=false” 
ninja -v -C out/Debug libyuv_unittest 
ninja -v -C out/Release libyuv_unittest
```

#### Building the Library with make

##### Linux

```bash
make V=1 -f linux.mk
make V=1 -f linux.mk clean
make V=1 -f linux.mk CXX=clang++ CC=clang
```

#### Building the library with cmake

Install cmake: [http://www.cmake.org/](http://www.cmake.org/)

##### Default debug build:

```bash
mkdir out
cd out
cmake ..
cmake --build .
```

##### Release build

```bash
mkdir out
cd out
cmake -DCMAKE_BUILD_TYPE="Release" ..
cmake --build . --config Release
```

#### Sanitizers

Sanitizers available: asan, msan, tsan, ubsan, lsan, ubsan_vptr

```bash
gn gen out/Release "--args=is_debug=false is_msan=true"
ninja -v -C out/Release
```


### License
[MIT](./LICENSE) Copyright (c) 2022 Mr.Panda.
