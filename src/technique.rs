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
    Ivshmem = 3,
    Drivers = 4,
    Handles = 5,
    VirtualProcessors = 6,
    HypervisorQuery = 7,
    Audio = 8,
    Display = 9,
    Dll = 10,
    VmwareBackdoor = 11,
    WineFunc = 12,
    VirtualRegistry = 13,
    Mutex = 14,
    DeviceString = 15,
    VpcInvalid = 16,
    VmwareStr = 17,
    Gamarue = 18,
    CuckooDir = 19,
    CuckooPipe = 20,
    Trap = 21,
    UndefinedInstruction = 22,
    InterruptShadow = 23,
    DbvmHypercall = 24,
    KernelObjects = 25,
    Nvram = 26,
    Edid = 27,
    CpuHeuristic = 28,
    Clock = 29,
    Msr = 30,
    KvmInterception = 31,
    Breakpoint = 32,
    Popf = 33,
    EipOverflow = 34,
    // Linux + Windows
    TaskSegment = 35,
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
    pub const ALL: &'static [Self] = &[
        // Windows
        Self::GpuCapabilities,
        Self::AcpiSignature,
        Self::PowerCapabilities,
        Self::Ivshmem,
        Self::Drivers,
        Self::Handles,
        Self::VirtualProcessors,
        Self::HypervisorQuery,
        Self::Audio,
        Self::Display,
        Self::Dll,
        Self::VmwareBackdoor,
        Self::WineFunc,
        Self::VirtualRegistry,
        Self::Mutex,
        Self::DeviceString,
        Self::VpcInvalid,
        Self::VmwareStr,
        Self::Gamarue,
        Self::CuckooDir,
        Self::CuckooPipe,
        Self::Trap,
        Self::UndefinedInstruction,
        Self::InterruptShadow,
        Self::DbvmHypercall,
        Self::KernelObjects,
        Self::Nvram,
        Self::Edid,
        Self::CpuHeuristic,
        Self::Clock,
        Self::Msr,
        Self::KvmInterception,
        Self::Breakpoint,
        Self::Popf,
        Self::EipOverflow,
        // Linux + Windows
        Self::TaskSegment,
        Self::Firmware,
        Self::Devices,
        Self::Azure,
        Self::BootLogo,
        Self::DiskSerial,
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
        Self::VmwareIomem,
        Self::VmwareIoports,
        Self::VmwareScsi,
        Self::VmwareDmesg,
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
