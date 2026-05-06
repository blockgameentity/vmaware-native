#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("bridge.hpp");

        fn vm_detect() -> bool;
        fn vm_brand() -> String;
        fn vm_type_str() -> String;
        fn vm_percentage() -> u8;
        fn vm_conclusion() -> String;
        fn vm_is_hardened() -> bool;
        fn vm_detected_count() -> u8;
        fn vm_technique_count() -> u16;
        fn vm_check(flag: u8) -> bool;
        fn vm_flag_to_string(flag: u8) -> String;
        fn vm_detected_techniques() -> Vec<u8>;
    }
}

pub(crate) use ffi::*;
