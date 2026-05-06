mod ffi;

/// Every detection technique vmaware exposes, in the same order as the C++ `enum_flags`.
///
/// The discriminant value matches the C++ enum value and is passed directly across the FFI.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Technique {
    // Windows
    GpuCapabilities = 0,
    AcpiSignature = 1,
    PowerCapabilities = 2,
    Ivshmem = 3,
    Drivers = 4,
    Handles = 5,
    VirtualProcessors = 6,
    HypervisorQuery = 7,
    Audio = 8,
    Display = 9,
    Dll = 10,
    VmwareBackdoor = 11,
    Wine = 12,
    VirtualRegistry = 13,
    Mutex = 14,
    DeviceString = 15,
    VpcInvalid = 16,
    VmwareStr = 17,
    Gamarue = 18,
    CuckooDir = 19,
    CuckooPipe = 20,
    Trap = 21,
    Ud = 22,
    InterruptShadow = 23,
    Dbvm = 24,
    KernelObjects = 25,
    Nvram = 26,
    Edid = 27,
    CpuHeuristic = 28,
    Clock = 29,
    Msr = 30,
    KvmInterception = 31,
    HypervisorHook = 32,
    Popf = 33,
    EipOverflow = 34,
    // Linux + Windows
    SystemRegisters = 35,
    Firmware = 36,
    Devices = 37,
    Azure = 38,
    BootLogo = 39,
    DiskSerial = 40,
    // Linux
    SmbiosVmBit = 41,
    Kmsg = 42,
    Cvendor = 43,
    QemuFwCfg = 44,
    Systemd = 45,
    Ctype = 46,
    Dockerenv = 47,
    Dmidecode = 48,
    Dmesg = 49,
    Hwmon = 50,
    LinuxUserHost = 51,
    VmwareIomem = 52,
    VmwareIoports = 53,
    VmwareScsi = 54,
    VmwareDmesg = 55,
    QemuVirtualDmi = 56,
    QemuUsb = 57,
    HypervisorDir = 58,
    UmlCpu = 59,
    VboxModule = 60,
    SysinfoProc = 61,
    DmiScan = 62,
    PodmanFile = 63,
    WslProc = 64,
    FileAccessHistory = 65,
    Mac = 66,
    ContainerPid = 67,
    BluestacksFolders = 68,
    AmdSevMsr = 69,
    Temperature = 70,
    Cgroup = 71,
    Processes = 72,
    // Linux + macOS
    ThreadCount = 73,
    // macOS
    MacMemsize = 74,
    MacIokit = 75,
    MacSip = 76,
    IoregGrep = 77,
    Hwmodel = 78,
    MacSys = 79,
    // Cross-platform
    HypervisorBit = 80,
    Vmid = 81,
    ThreadMismatch = 82,
    Timer = 83,
    CpuBrand = 84,
    HypervisorStr = 85,
    CpuidSignature = 86,
    BochsCpu = 87,
    KgtSignature = 88,
}

impl Technique {
    const MAX_VALUE: u8 = 88; // KgtSignature

    fn from_u8(v: u8) -> Option<Self> {
        if v <= Self::MAX_VALUE {
            // SAFETY: repr(u8), all values 0..=88 have a named discriminant
            Some(unsafe { core::mem::transmute(v) })
        } else {
            None
        }
    }

    /// Returns the vmaware canonical name for this technique (e.g. `"VMID"`, `"CPU_BRAND"`).
    pub fn name(self) -> String {
        ffi::vm_flag_to_string(self as u8)
    }
}

/// Aggregated result from a full vmaware scan.
#[derive(Debug, Clone)]
pub struct VmInfo {
    pub brand: String,
    pub vm_type: String,
    pub conclusion: String,
    pub is_vm: bool,
    pub is_hardened: bool,
    pub percentage: u8,
    pub detected_count: u8,
    pub technique_count: u16,
    pub detected_techniques: Vec<Technique>,
}

/// Returns `true` if a virtual machine is detected using all default techniques.
pub fn detect() -> bool {
    ffi::vm_detect()
}

/// Returns the most likely VM brand string (e.g. `"VirtualBox"`, `"VMware"`).
pub fn brand() -> String {
    ffi::vm_brand()
}

/// Returns a human-readable VM type string (e.g. `"Hypervisor (type 2)"`, `"Container"`).
pub fn vm_type() -> String {
    ffi::vm_type_str()
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
/// println!("Brand: {}", info.brand);
/// println!("Type:  {}", info.vm_type);
/// println!("Conclusion: {}", info.conclusion);
/// ```
pub fn query() -> VmInfo {
    let detected_techniques = ffi::vm_detected_techniques()
        .iter()
        .filter_map(|&f| Technique::from_u8(f))
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
        assert!(info.detected_count <= info.technique_count as u8);
    }

    #[test]
    fn smoke_individual_fns() {
        let pct = percentage();
        assert!(pct <= 100);
        assert!(!brand().is_empty());
        assert!(!vm_type().is_empty());
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
