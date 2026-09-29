set(_TablerSlint_LIBRARY_PATH "tabler=${CMAKE_CURRENT_LIST_DIR}/tabler.slint")

if(CMAKE_CURRENT_SOURCE_DIR STREQUAL CMAKE_SOURCE_DIR)
    set(TablerSlint_LIBRARY_PATH "${_TablerSlint_LIBRARY_PATH}")
    set(TablerSlint_LIBRARY_PATHS "${_TablerSlint_LIBRARY_PATH}")
else()
    set(TablerSlint_LIBRARY_PATH "${_TablerSlint_LIBRARY_PATH}" PARENT_SCOPE)
    set(TablerSlint_LIBRARY_PATHS "${_TablerSlint_LIBRARY_PATH}" PARENT_SCOPE)
endif()
