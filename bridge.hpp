#pragma once
#include "rust/cxx.h"
#include <cstdint>

bool vm_detect();
rust::String vm_brand();
rust::String vm_type_str();
uint8_t vm_percentage();
rust::String vm_conclusion();
bool vm_is_hardened();
uint8_t vm_detected_count();
uint16_t vm_technique_count();
bool vm_check(uint8_t flag);
rust::String vm_flag_to_string(uint8_t flag);
rust::Vec<uint8_t> vm_detected_techniques();
