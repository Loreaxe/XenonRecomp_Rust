cmake_minimum_required(VERSION 3.18)
project(XenonRecomp_Rust LANGUAGES C)

# If you also want to compile any C/C++ later, add languages here.

# Build the vendored libmspack as a static library (no Autotools)
add_subdirectory(thirdparty)

# Where our mspack artifacts will live
# (thirdparty/CMakeLists.txt sets these vars for us)
message(STATUS "libmspack include dir: ${MSPACK_INCDIR}")
message(STATUS "libmspack library dir: ${MSPACK_LIBDIR}")

# Decide which Cargo profile to use based on CMake build type
# ----------------------------------------------------------
# Debug        -> cargo build
# Release      -> cargo build --release
# RelWithDebInfo -> cargo build --release (our release has debug info)
# MinSizeRel   -> cargo build --release
if(NOT CMAKE_BUILD_TYPE)
  # Default if user didn't specify e.g. -DCMAKE_BUILD_TYPE=RelWithDebInfo
  set(CMAKE_BUILD_TYPE "RelWithDebInfo" CACHE STRING "" FORCE)
endif()

string(TOUPPER "${CMAKE_BUILD_TYPE}" _BUILD_TYPE_UPPER)
message(STATUS "CMake build type: ${CMAKE_BUILD_TYPE}")

set(CARGO_PROFILE_ARGS "")
if(_BUILD_TYPE_UPPER STREQUAL "DEBUG")
  set(CARGO_PROFILE_ARGS "")
elseif(_BUILD_TYPE_UPPER STREQUAL "RELEASE")
  set(CARGO_PROFILE_ARGS "--release")
elseif(_BUILD_TYPE_UPPER STREQUAL "RELWITHDEBINFO")
  set(CARGO_PROFILE_ARGS "--release")
elseif(_BUILD_TYPE_UPPER STREQUAL "MINSIZEREL")
  set(CARGO_PROFILE_ARGS "--release")
else()
  # Fallback: treat unknown types like Debug
  message(WARNING "Unknown CMAKE_BUILD_TYPE='${CMAKE_BUILD_TYPE}', using Cargo dev profile")
  set(CARGO_PROFILE_ARGS "")
endif()

# Cargo build: pass paths so build.rs can find & link libmspack
add_custom_target(cargo_build ALL
  COMMAND ${CMAKE_COMMAND} -E env
          MSPACK_INCDIR=${MSPACK_INCDIR}
          MSPACK_LIBDIR=${MSPACK_LIBDIR}
          CARGO_TARGET_DIR=${CMAKE_BINARY_DIR}/cargo
          cargo build ${CARGO_PROFILE_ARGS}
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
  USES_TERMINAL
  VERBATIM
)

# Ensure libmspack is built before cargo tries to link it
add_dependencies(cargo_build libmspack)
