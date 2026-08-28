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
    Audio = 6,
    Display = 7,
    Dll = 8,
    Wine = 9,
    VirtualRegistry = 10,
    Mutex = 11,
    VpcInvalid = 12,
    VmwareStr = 13,
    Gamarue = 14,
    Cuckoo = 15,
    Trap = 16,
    Ud = 17,
    InterruptShadow = 18,
    Dbvm = 19,
    KernelObjects = 20,
    Nvram = 21,
    CpuHeuristic = 22,
    Clock = 23,
    Msr = 24,
    KvmInterception = 25,
    HypervisorHook = 26,
    SingleStep = 27,
    EipOverflow = 28,
    SvmExceptions = 29,
    MeasuredBoot = 30,
    Tpm = 31,
    SystemRegisters = 32,
    Firmware = 33,
    Devices = 34,
    Azure = 35,
    BootLogo = 36,
    Disk = 37,
    // Linux
    SmbiosVmBit = 38,
    Kmsg = 39,
    Cvendor = 40,
    QemuFwCfg = 41,
    Systemd = 42,
    Ctype = 43,
    Dockerenv = 44,
    Dmidecode = 45,
    Dmesg = 46,
    Hwmon = 47,
    LinuxUserHost = 48,
    QemuVirtualDmi = 49,
    QemuUsb = 50,
    HypervisorDir = 51,
    UmlCpu = 52,
    VboxModule = 53,
    SysinfoProc = 54,
    DmiScan = 55,
    PodmanFile = 56,
    WslProc = 57,
    FileAccessHistory = 58,
    Mac = 59,
    ContainerPid = 60,
    BluestacksFolders = 61,
    AmdSevMsr = 62,
    Temperature = 63,
    Cgroup = 64,
    Processes = 65,
    // Linux + macOS
    ThreadCount = 66,
    // macOS
    MacMemsize = 67,
    MacIokit = 68,
    MacSip = 69,
    IoregGrep = 70,
    Hwmodel = 71,
    MacSys = 72,
    // Cross-platform
    HypervisorBit = 73,
    Vmid = 74,
    ThreadMismatch = 75,
    Timer = 76,
    CpuBrand = 77,
    HypervisorStr = 78,
    CpuidSignature = 79,
    BochsCpu = 80,
    KgtSignature = 81,
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
        Self::Audio,
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
