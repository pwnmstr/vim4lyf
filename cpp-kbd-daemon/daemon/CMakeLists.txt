add_executable(kbd-daemon
    main.cpp
    scanner.cpp
    mapper.cpp
    overlay.cpp
    ipc.cpp
)

target_include_directories(kbd-daemon PRIVATE ${GTKMM_INCLUDE_DIRS} ${LAYER_SHELL_INCLUDE_DIRS} ${ATSPI_INCLUDE_DIRS})
target_link_directories(kbd-daemon PRIVATE ${GTKMM_LIBRARY_DIRS} ${LAYER_SHELL_LIBRARY_DIRS} ${ATSPI_LIBRARY_DIRS})
target_link_libraries(kbd-daemon PRIVATE ${GTKMM_LIBRARIES} ${LAYER_SHELL_LIBRARIES} ${ATSPI_LIBRARIES} pthread)
