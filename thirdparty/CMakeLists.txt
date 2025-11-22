# thirdparty/CMakeLists.txt
cmake_minimum_required(VERSION 3.18)

# Compute include/lib dirs once
set(MSPACK_PREFIX "${CMAKE_BINARY_DIR}/mspack_prefix")
set(MSPACK_INCDIR "${MSPACK_PREFIX}/include")
set(MSPACK_LIBDIR "${MSPACK_PREFIX}/lib")

# Also export to parent so your top-level CMake (and build.rs) can read them
set(MSPACK_PREFIX "${MSPACK_PREFIX}" PARENT_SCOPE)
set(MSPACK_INCDIR "${MSPACK_INCDIR}" PARENT_SCOPE)
set(MSPACK_LIBDIR "${MSPACK_LIBDIR}" PARENT_SCOPE)

file(MAKE_DIRECTORY "${MSPACK_INCDIR}")
file(MAKE_DIRECTORY "${MSPACK_LIBDIR}")

# Locate libmspack sources (handle multiple common layouts)
set(_LM_CANDIDATES
  "${CMAKE_CURRENT_SOURCE_DIR}/libmspack/mspack"
  "${CMAKE_CURRENT_SOURCE_DIR}/libmspack/libmspack/mspack"
  "${CMAKE_CURRENT_SOURCE_DIR}/mspack"
  "${CMAKE_CURRENT_SOURCE_DIR}"
)

set(_LM_SRC_DIR "")
foreach(_c IN LISTS _LM_CANDIDATES)
  if(EXISTS "${_c}/lzxd.c" AND EXISTS "${_c}/lzx.h")
    set(_LM_SRC_DIR "${_c}")
    break()
  endif()
endforeach()

if(_LM_SRC_DIR STREQUAL "")
  message(FATAL_ERROR
    "Could not find libmspack sources (need lzxd.c and lzx.h). Tried:\n  ${_LM_CANDIDATES}")
endif()

message(STATUS "Using libmspack source dir: ${_LM_SRC_DIR}")

# Build minimal static lib with the decompressor
add_library(libmspack STATIC
  "${_LM_SRC_DIR}/lzxd.c"
)

target_include_directories(libmspack PUBLIC "${_LM_SRC_DIR}")

set_target_properties(libmspack PROPERTIES
  OUTPUT_NAME mspack
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"   # -> thirdparty/libmspack.a
  POSITION_INDEPENDENT_CODE ON
)

# Copy headers to a predictable include prefix for Rust consumers
add_custom_command(TARGET libmspack POST_BUILD
  COMMAND ${CMAKE_COMMAND} -E make_directory "${MSPACK_INCDIR}/mspack"
  COMMAND ${CMAKE_COMMAND} -E copy_if_different
          "${_LM_SRC_DIR}/lzx.h"
          "${MSPACK_INCDIR}/mspack/lzx.h"
  COMMAND ${CMAKE_COMMAND} -E copy_if_different
          "${_LM_SRC_DIR}/mspack.h"
          "${MSPACK_INCDIR}/mspack/mspack.h"
  COMMAND ${CMAKE_COMMAND} -E copy_if_different
          "${_LM_SRC_DIR}/system.h"
          "${MSPACK_INCDIR}/mspack/system.h"
  # NEW: ensure the archive is where Cargo (build.rs) looks
  COMMAND ${CMAKE_COMMAND} -E make_directory "${MSPACK_LIBDIR}"
  COMMAND ${CMAKE_COMMAND} -E copy_if_different
          "$<TARGET_FILE:libmspack>"
          "${MSPACK_LIBDIR}/libmspack.a"
  VERBATIM
)
