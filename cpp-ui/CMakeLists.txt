cmake_minimum_required(VERSION 3.16)
project(CyberMD VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

# Try to find Qt6, fall back to Qt5 if not found
find_package(Qt6 COMPONENTS Core Gui Widgets QUIET)
if (NOT Qt6_FOUND)
    message(STATUS "Qt6 not found, trying Qt5...")
    find_package(Qt5 COMPONENTS Core Gui Widgets REQUIRED)
    set(QT_VERSION_MAJOR 5)
else()
    set(QT_VERSION_MAJOR 6)
endif()

message(STATUS "Using Qt${QT_VERSION_MAJOR}")

# Include directories
include_directories(
    ${CMAKE_SOURCE_DIR}/include
    ${CMAKE_SOURCE_DIR}/../rust-core/cybermd-ffi
)

# Link to Rust FFI library
link_directories(
    ${CMAKE_SOURCE_DIR}/../rust-core/target/release
)

# Source files
set(SOURCES
    src/main.cpp
    src/mainwindow.cpp
    src/codeeditor.cpp
)

# Header files
set(HEADERS
    include/mainwindow.h
    include/codeeditor.h
    include/rustbridge.h
)

# Create executable
add_executable(cybermd
    ${SOURCES}
    ${HEADERS}
)

# Link libraries
target_link_libraries(cybermd
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
    Qt${QT_VERSION_MAJOR}::Widgets
    cybermd_ffi  # Rust FFI library
)

# Set output directory
set_target_properties(cybermd PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)

# Install targets
install(TARGETS cybermd
    RUNTIME DESTINATION bin
)
