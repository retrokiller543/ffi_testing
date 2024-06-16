# FFI Testing library

This is a library for testing how I can build robust libraries in rust and use it in diffrent languages with minimal effort

## How to build

To build we need to first run 

```shell
just generate-headers
```

This will generate the headers and other files that are needed to make interopt possible

After we have built the interopt files we can use them to make a program. Remember to link the built .so, .dylib or .dll file depending on your platform!

## Installing the library (not recommended)

Its not recommended to install this library since this is only for testing and not at all for production but if you want to then you can run the following commands:

### C & C++

```shell
just install-c
```

### C#

I have not looked at this yet but could possible do it in the future

### Python

For python simply move the `ffi_lib.py` file to the directory where you want to use it.

Before running any methods in the file you need to init the library by running:

```python
from ffi_lib import *

# Path to the native library
library_path = "../../target/release/libffi_lib.dylib"  # Update this with your library path

# Initialize the library
init_lib(library_path)
```
