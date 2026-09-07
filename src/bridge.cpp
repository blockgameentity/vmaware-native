#include "bridge.hpp"

// vendor/vmaware.hpp is third-party upstream code validated by checksum, so
// silence its internal warnings here instead of editing the vendored file.
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wtype-limits"
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#endif
#if defined(_MSC_VER)
#pragma warning(push, 0)
#endif
#include "vmaware.hpp"
#if defined(_MSC_VER)
#pragma warning(pop)
#endif
#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic pop
#endif

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
    // Upstream VM::is_hardened() is deprecated (scheduled for removal
    // post-2.8.1, "use detect() instead") and is currently stubbed to always
    // return false (see vendor/vmaware.hpp and VM::vmaware::initialise()).
    // Don't call it so we stay warning-free and survive its future removal.
    return false;
}

uint8_t vm_detected_count() {
    return static_cast<uint8_t>(VM::detected_count());
}

uint16_t vm_technique_count() {
    return static_cast<uint16_t>(VM::technique_count);
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
