# CyberMD Build Instructions

## Project Structure

CyberMD is now a hybrid Rust/C++ application:

- **rust-core/** - Core parsing and analysis engine (Rust)
  - `cybermd-ast` - AST node definitions
  - `cybermd-parser` - Markdown parser
  - `cybermd-core` - Document analyzer and core logic
  - `cybermd-highlighter` - Semantic highlighting engine
  - `cybermd-ffi` - C FFI bindings for C++ integration

- **cpp-ui/** - Qt-based user interface (C++)
  - Main editor window
  - UI widgets (folding gutter, minimap, outline, breadcrumb)
  - Theme management

- **python-legacy/** - Original Python implementation (reference only)

## Prerequisites

### Rust
- Rust 1.70 or later
- Cargo (comes with Rust)

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### C++ and Qt
- C++17 compatible compiler (GCC 7+, Clang 5+, MSVC 2017+)
- CMake 3.16 or later
- Qt 6.2 or later

#### Ubuntu/Debian:
```bash
sudo apt-get update
sudo apt-get install build-essential cmake qt6-base-dev
```

#### macOS:
```bash
brew install cmake qt@6
```

#### Windows:
- Install Visual Studio 2019 or later
- Install CMake from https://cmake.org/
- Install Qt 6 from https://www.qt.io/

## Building

### Step 1: Build Rust Core

```bash
cd rust-core
cargo build --release
```

This creates the shared library at:
- Linux: `target/release/libcybermd_ffi.so`
- macOS: `target/release/libcybermd_ffi.dylib`
- Windows: `target/release/cybermd_ffi.dll`

### Step 2: Build C++ UI

```bash
cd cpp-ui
mkdir build
cd build
cmake ..
cmake --build . --config Release
```

The executable will be in `cpp-ui/build/bin/cybermd`

### Step 3: Run

```bash
./cpp-ui/build/bin/cybermd
```

## Development

### Testing Rust Code

```bash
cd rust-core
cargo test
```

### Rust Code Formatting

```bash
cd rust-core
cargo fmt
```

### Rust Linting

```bash
cd rust-core
cargo clippy
```

### C++ Debugging

Build with debug symbols:
```bash
cd cpp-ui/build
cmake -DCMAKE_BUILD_TYPE=Debug ..
cmake --build .
```

## Architecture

### Data Flow

```
C++ UI (Qt)
    ↓
FFI Bridge (C API)
    ↓
Rust Core
    ├── Parser → AST
    ├── Analyzer → Structure
    └── Highlighter → Tokens
```

### FFI Interface

The Rust core exposes a C-compatible API that the C++ UI calls:

```c
// Example FFI functions
Parser* parser_new();
AST* parser_parse(Parser* parser, const char* text);
void parser_free(Parser* parser);
```

### Performance

- Rust handles all parsing, AST manipulation, and analysis
- C++ only handles UI rendering and user input
- Communication via FFI is minimized through batching
