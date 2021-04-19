# ffms2-sys
Automatically generated bindings for FFMS2.

## Building
This crate requires an installed version of FFMS2 on the build system. If possible, it will use *pkg-config* to locate and link the libraries. Otherwise, it could utilize the environment variables FFMS_INCLUDE_DIR and FFMS_LIB_DIR to locate the required files specified by the user. 

### Building on Windows
The strategy with the environment variables is especially helpful on Microsoft Windows. In this case, one does not even have to compile the library at all. Instead, developers could refer to the [official releases](https://github.com/FFMS/ffms2/releases). FFMS_INCLUDE_DIR corresponds to the "include" directory while FFMS_LIB_DIR must point to "x64" or "x32", respectively. For the distribution of the final artifact, the corresponding *.dll must be shipped with the executable.