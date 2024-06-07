cmake_minimum_required(VERSION 3.13)
project(Wrapper)
find_package(Threads)

# Define the include directories.
include_directories(${PERFETTO_DIR}/sdk)

# Add the C++ source file for the wrapper.
add_library(wrapper STATIC wrapper.cpp ${PERFETTO_DIR}/sdk/perfetto.cc)
target_link_libraries(wrapper perfetto ${CMAKE_THREAD_LIBS_INIT})
