#pragma once
#include <string>
#include <unordered_map>
#include <vector>

enum class SequenceStyle {
    SequentialHjkl,
    QuadrantAsdf
};

class KeyMapper {
public:
    KeyMapper(SequenceStyle style);
    std::unordered_map<std::string, size_t> generate_mapping(size_t element_count);

private:
    SequenceStyle style;
    std::string index_to_sequence(size_t index, const std::vector<char>& chars);
};
