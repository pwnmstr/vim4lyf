cmake_minimum_required(VERSION 3.16)
project(kbd_daemon CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find dependencies
find_package(PkgConfig REQUIRED)
pkg_check_modules(GTKMM REQUIRED gtkmm-4.0)
pkg_check_modules(LAYER_SHELL REQUIRED gtk4-layer-shell-0)
pkg_check_modules(ATSPI REQUIRED atspi-2)

add_subdirectory(daemon)
add_subdirectory(cli)
