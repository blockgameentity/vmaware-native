mod ffi;
mod technique;

pub use technique::Technique;

/// Aggregated result from a full vmaware scan.
#[derive(Debug, Clone)]
pub struct VmInfo {
    pub brand: Option<String>,
    pub vm_type: Option<String>,
    pub conclusion: String,
    pub is_vm: bool,
    pub is_hardened: bool,
    pub percentage: u8,
    pub detected_count: u8,
    /// VMAware's total, including reserved enum slots and registered custom techniques.
    pub technique_count: u16,
    pub detected_techniques: Vec<Technique>,
}

/// Returns `true` if a virtual machine is detected using all default techniques.
pub fn detect() -> bool {
    ffi::vm_detect()
}

fn none_if_unknown(value: String) -> Option<String> {
    let value = value.trim().to_owned();

    if value.is_empty() || value.eq_ignore_ascii_case("unknown") {
        None
    } else {
        Some(value)
    }
}

/// Returns the most likely VM brand string (e.g. `"VirtualBox"`, `"VMware"`).
pub fn brand() -> Option<String> {
    none_if_unknown(ffi::vm_brand())
}

/// Returns a human-readable VM type string (e.g. `"Hypervisor (type 2)"`, `"Container"`).
pub fn vm_type() -> Option<String> {
    none_if_unknown(ffi::vm_type_str())
}

/// Returns a 0–100 certainty percentage for VM detection.
pub fn percentage() -> u8 {
    ffi::vm_percentage()
}

/// Returns a plain-English conclusion string (e.g. `"Running inside a VirtualBox"`).
pub fn conclusion() -> String {
    ffi::vm_conclusion()
}

/// Returns `true` when anti-VM hardening artefacts are detected.
pub fn is_hardened() -> bool {
    ffi::vm_is_hardened()
}

/// Returns how many individual techniques fired a positive result.
pub fn detected_count() -> u8 {
    ffi::vm_detected_count()
}

/// Runs a single technique and returns its result.
///
/// vmaware caches technique results internally, so calling this after [`query`] or
/// [`detect`] is free.
pub fn check(technique: Technique) -> bool {
    ffi::vm_check(technique as u8)
}

/// Runs all default techniques and returns the full aggregated result.
///
/// vmaware caches every technique result, so subsequent calls to the individual
/// free functions are inexpensive.
///
/// # Example
/// ```
/// let info = vmaware_native::query();
/// println!("VM: {} ({}%)", info.is_vm, info.percentage);
/// println!("Brand: {}", info.brand.as_deref().unwrap_or("Unknown"));
/// println!("Type:  {}", info.vm_type.as_deref().unwrap_or("Unknown"));
/// println!("Conclusion: {}", info.conclusion);
/// ```
pub fn query() -> VmInfo {
    let detected_techniques = ffi::vm_detected_techniques()
        .iter()
        .copied()
        .map(|f| {
            Technique::from_u8(f)
                .unwrap_or_else(|| panic!("vmaware returned unknown technique flag {f}"))
        })
        .collect();

    VmInfo {
        brand: brand(),
        vm_type: vm_type(),
        conclusion: conclusion(),
        is_vm: detect(),
        is_hardened: is_hardened(),
        percentage: percentage(),
        detected_count: detected_count(),
        technique_count: ffi::vm_technique_count(),
        detected_techniques,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_query() {
        let info = query();
        assert!(info.percentage <= 100);
        assert!(info.technique_count > 0);
        // VMAware's total includes reserved enum slots and registered custom techniques.
        assert!(Technique::ALL.len() <= info.technique_count as usize);
        assert!(info.detected_count <= info.technique_count as u8);
        assert_eq!(info.detected_techniques.len(), info.detected_count as usize);
    }

    #[test]
    fn smoke_individual_fns() {
        let pct = percentage();
        assert!(pct <= 100);

        if let Some(brand) = brand() {
            assert!(!brand.is_empty());
            assert_ne!(brand, "Unknown");
        }

        if let Some(vm_type) = vm_type() {
            assert!(!vm_type.is_empty());
            assert_ne!(vm_type, "Unknown");
        }

        assert!(!conclusion().is_empty());
    }

    #[test]
    fn check_cross_platform_techniques() {
        let _ = check(Technique::HypervisorBit);
        let _ = check(Technique::Vmid);
        let _ = check(Technique::CpuBrand);
        let _ = check(Technique::Timer);
    }

    #[test]
    fn technique_name_roundtrip() {
        assert_eq!(Technique::HypervisorBit.name(), "HYPERVISOR_BIT");
        assert_eq!(Technique::Vmid.name(), "VMID");
        assert_eq!(Technique::CpuBrand.name(), "CPU_BRAND");
    }
}
