#include "bridge.hpp"
#include "vmaware.hpp"

bool vm_detect() {
    return VM::detect();
}

rust::String vm_brand() {
    return rust::String(VM::brand());
}

rust::String vm_type_str() {
    return rust::String(VM::type());
}

uint8_t vm_percentage() {
    return VM::percentage();
}

rust::String vm_conclusion() {
    return rust::String(VM::conclusion());
}

bool vm_is_hardened() {
    return VM::is_hardened();
}

uint8_t vm_detected_count() {
    return static_cast<uint8_t>(VM::detected_count());
}

uint16_t vm_technique_count() {
    return static_cast<uint16_t>(VM::technique_count.load());
}

bool vm_check(uint8_t flag) {
    return VM::check(static_cast<VM::enum_flags>(flag));
}

rust::String vm_flag_to_string(uint8_t flag) {
    return rust::String(VM::flag_to_string(static_cast<VM::enum_flags>(flag)));
}

rust::Vec<uint8_t> vm_detected_techniques() {
    auto enums = VM::detected_enums();
    rust::Vec<uint8_t> result;
    for (auto e : enums) {
        result.push_back(static_cast<uint8_t>(e));
    }
    return result;
}
