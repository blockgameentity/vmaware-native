#include <cstdint>
#include <iostream>

#include "vmaware.hpp"

int main() {
    for (int value = 0; value < static_cast<int>(VM::DEFAULT); ++value) {
        const auto flag = static_cast<VM::enum_flags>(value);
        std::cout
            << value
            << '\t'
            << VM::flag_to_string(flag)
            << '\n';
    }

    return 0;
}
