#include "mapper.h"
#include <algorithm>

KeyMapper::KeyMapper(SequenceStyle style) : style(style) {}

std::unordered_map<std::string, size_t> KeyMapper::generate_mapping(size_t element_count) {
    std::unordered_map<std::string, size_t> map;
    std::vector<char> chars;

    if (style == SequenceStyle::SequentialHjkl) {
        chars = {'h', 'j', 'k', 'l'};
    } else {
        chars = {'a', 's', 'd', 'f'};
    }

    for (size_t i = 0; i < element_count; i++) {
        std::string seq = index_to_sequence(i, chars);
        map[seq] = i;
    }

    return map;
}

std::string KeyMapper::index_to_sequence(size_t index, const std::vector<char>& chars) {
    size_t base = chars.size();
    if (index < base) {
        return std::string(1, chars[index]);
    }

    std::string seq = "";
    while (true) {
        size_t rem = index % base;
        seq = chars[rem] + seq;
        index /= base;
        if (index == 0) break;
        index -= 1;
    }
    return seq;
}
