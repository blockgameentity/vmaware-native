// This file is generated from vendor/vmaware.hpp.
// Do not edit manually. Run ci/update-vmaware.sh.

use crate::ffi;

/// Every detection technique vmaware exposes, in the same order as the C++ `enum_flags`.
///
/// The discriminant value matches the C++ enum value and is passed directly across the FFI.
#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Technique {
    // Windows
    GpuCapabilities = 0,
    AcpiSignature = 1,
    PowerCapabilities = 2,
    Drivers = 3,
    Handles = 4,
    VirtualProcessors = 5,
    Display = 6,
    Dll = 7,
    Wine = 8,
    VirtualRegistry = 9,
    Mutex = 10,
    VpcInvalid = 11,
    VmwareStr = 12,
    Gamarue = 13,
    Cuckoo = 14,
    Trap = 15,
    Ud = 16,
    InterruptShadow = 17,
    Dbvm = 18,
    KernelObjects = 19,
    Nvram = 20,
    CpuHeuristic = 21,
    Clock = 22,
    Msr = 23,
    KvmInterception = 24,
    HypervisorHook = 25,
    SingleStep = 26,
    EipOverflow = 27,
    SvmExceptions = 28,
    MeasuredBoot = 29,
    Tpm = 30,
    SystemRegisters = 31,
    Firmware = 32,
    Devices = 33,
    Azure = 34,
    BootLogo = 35,
    Disk = 36,
    // Linux
    SmbiosVmBit = 37,
    Kmsg = 38,
    Cvendor = 39,
    QemuFwCfg = 40,
    Systemd = 41,
    Ctype = 42,
    Dockerenv = 43,
    Dmidecode = 44,
    Dmesg = 45,
    Hwmon = 46,
    LinuxUserHost = 47,
    QemuVirtualDmi = 48,
    QemuUsb = 49,
    HypervisorDir = 50,
    UmlCpu = 51,
    VboxModule = 52,
    SysinfoProc = 53,
    DmiScan = 54,
    PodmanFile = 55,
    WslProc = 56,
    FileAccessHistory = 57,
    Mac = 58,
    ContainerPid = 59,
    BluestacksFolders = 60,
    AmdSevMsr = 61,
    Temperature = 62,
    Cgroup = 63,
    Processes = 64,
    // Linux + macOS
    ThreadCount = 65,
    // macOS
    MacMemsize = 66,
    MacIokit = 67,
    MacSip = 68,
    IoregGrep = 69,
    Hwmodel = 70,
    MacSys = 71,
    // Cross-platform
    HypervisorBit = 72,
    Vmid = 73,
    ThreadMismatch = 74,
    Timer = 75,
    CpuBrand = 76,
    HypervisorStr = 77,
    CpuidSignature = 78,
    BochsCpu = 79,
    KgtSignature = 80,
}

impl Technique {
    pub const ALL: &'static [Self] = &[
        // Windows
        Self::GpuCapabilities,
        Self::AcpiSignature,
        Self::PowerCapabilities,
        Self::Drivers,
        Self::Handles,
        Self::VirtualProcessors,
        Self::Display,
        Self::Dll,
        Self::Wine,
        Self::VirtualRegistry,
        Self::Mutex,
        Self::VpcInvalid,
        Self::VmwareStr,
        Self::Gamarue,
        Self::Cuckoo,
        Self::Trap,
        Self::Ud,
        Self::InterruptShadow,
        Self::Dbvm,
        Self::KernelObjects,
        Self::Nvram,
        Self::CpuHeuristic,
        Self::Clock,
        Self::Msr,
        Self::KvmInterception,
        Self::HypervisorHook,
        Self::SingleStep,
        Self::EipOverflow,
        Self::SvmExceptions,
        Self::MeasuredBoot,
        Self::Tpm,
        Self::SystemRegisters,
        Self::Firmware,
        Self::Devices,
        Self::Azure,
        Self::BootLogo,
        Self::Disk,
        // Linux
        Self::SmbiosVmBit,
        Self::Kmsg,
        Self::Cvendor,
        Self::QemuFwCfg,
        Self::Systemd,
        Self::Ctype,
        Self::Dockerenv,
        Self::Dmidecode,
        Self::Dmesg,
        Self::Hwmon,
        Self::LinuxUserHost,
        Self::QemuVirtualDmi,
        Self::QemuUsb,
        Self::HypervisorDir,
        Self::UmlCpu,
        Self::VboxModule,
        Self::SysinfoProc,
        Self::DmiScan,
        Self::PodmanFile,
        Self::WslProc,
        Self::FileAccessHistory,
        Self::Mac,
        Self::ContainerPid,
        Self::BluestacksFolders,
        Self::AmdSevMsr,
        Self::Temperature,
        Self::Cgroup,
        Self::Processes,
        // Linux + macOS
        Self::ThreadCount,
        // macOS
        Self::MacMemsize,
        Self::MacIokit,
        Self::MacSip,
        Self::IoregGrep,
        Self::Hwmodel,
        Self::MacSys,
        // Cross-platform
        Self::HypervisorBit,
        Self::Vmid,
        Self::ThreadMismatch,
        Self::Timer,
        Self::CpuBrand,
        Self::HypervisorStr,
        Self::CpuidSignature,
        Self::BochsCpu,
        Self::KgtSignature,
    ];

    pub fn from_u8(value: u8) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|technique| *technique as u8 == value)
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn name(self) -> String {
        ffi::vm_flag_to_string(self.as_u8())
    }
}

#[test]
fn technique_all_is_valid() {
    use std::collections::HashSet;

    let mut seen = HashSet::new();

    for technique in Technique::ALL.iter().copied() {
        assert!(
            seen.insert(technique.as_u8()),
            "duplicate technique discriminant: {}",
            technique.as_u8()
        );

        assert_eq!(Technique::from_u8(technique.as_u8()), Some(technique));

        assert_ne!(technique.name(), "Unknown flag");
    }
}
