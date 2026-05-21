use vmaware_native::{check, query, Technique};

const ALL_TECHNIQUES: &[(Technique, &str)] = &[
    // Windows
    (Technique::GpuCapabilities,    "GPU_CAPABILITIES"),
    (Technique::AcpiSignature,      "ACPI_SIGNATURE"),
    (Technique::PowerCapabilities,  "POWER_CAPABILITIES"),
    (Technique::Ivshmem,            "IVSHMEM"),
    (Technique::Drivers,            "DRIVERS"),
    (Technique::Handles,            "HANDLES"),
    (Technique::VirtualProcessors,  "VIRTUAL_PROCESSORS"),
    (Technique::HypervisorQuery,    "HYPERVISOR_QUERY"),
    (Technique::Audio,              "AUDIO"),
    (Technique::Display,            "DISPLAY"),
    (Technique::Dll,                "DLL"),
    (Technique::VmwareBackdoor,     "VMWARE_BACKDOOR"),
    (Technique::Wine,               "WINE"),
    (Technique::VirtualRegistry,    "VIRTUAL_REGISTRY"),
    (Technique::Mutex,              "MUTEX"),
    (Technique::DeviceString,       "DEVICE_STRING"),
    (Technique::VpcInvalid,         "VPC_INVALID"),
    (Technique::VmwareStr,          "VMWARE_STR"),
    (Technique::Gamarue,            "GAMARUE"),
    (Technique::CuckooDir,          "CUCKOO_DIR"),
    (Technique::CuckooPipe,         "CUCKOO_PIPE"),
    (Technique::Trap,               "TRAP"),
    (Technique::Ud,                 "UD"),
    (Technique::InterruptShadow,    "INTERRUPT_SHADOW"),
    (Technique::Dbvm,               "DBVM"),
    (Technique::KernelObjects,      "KERNEL_OBJECTS"),
    (Technique::Nvram,              "NVRAM"),
    (Technique::Edid,               "EDID"),
    (Technique::CpuHeuristic,       "CPU_HEURISTIC"),
    (Technique::Clock,              "CLOCK"),
    (Technique::Msr,                "MSR"),
    (Technique::KvmInterception,    "KVM_INTERCEPTION"),
    (Technique::HypervisorHook,     "HYPERVISOR_HOOK"),
    (Technique::Popf,               "POPF"),
    (Technique::EipOverflow,        "EIP_OVERFLOW"),
    // Linux + Windows
    (Technique::SystemRegisters,    "SYSTEM_REGISTERS"),
    (Technique::Firmware,           "FIRMWARE"),
    (Technique::Devices,            "DEVICES"),
    (Technique::Azure,              "AZURE"),
    (Technique::BootLogo,           "BOOT_LOGO"),
    (Technique::DiskSerial,         "DISK_SERIAL"),
    // Linux
    (Technique::SmbiosVmBit,        "SMBIOS_VM_BIT"),
    (Technique::Kmsg,               "KMSG"),
    (Technique::Cvendor,            "CVENDOR"),
    (Technique::QemuFwCfg,          "QEMU_FW_CFG"),
    (Technique::Systemd,            "SYSTEMD"),
    (Technique::Ctype,              "CTYPE"),
    (Technique::Dockerenv,          "DOCKERENV"),
    (Technique::Dmidecode,          "DMIDECODE"),
    (Technique::Dmesg,              "DMESG"),
    (Technique::Hwmon,              "HWMON"),
    (Technique::LinuxUserHost,      "LINUX_USER_HOST"),
    (Technique::VmwareIomem,        "VMWARE_IOMEM"),
    (Technique::VmwareIoports,      "VMWARE_IOPORTS"),
    (Technique::VmwareScsi,         "VMWARE_SCSI"),
    (Technique::VmwareDmesg,        "VMWARE_DMESG"),
    (Technique::QemuVirtualDmi,     "QEMU_VIRTUAL_DMI"),
    (Technique::QemuUsb,            "QEMU_USB"),
    (Technique::HypervisorDir,      "HYPERVISOR_DIR"),
    (Technique::UmlCpu,             "UML_CPU"),
    (Technique::VboxModule,         "VBOX_MODULE"),
    (Technique::SysinfoProc,        "SYSINFO_PROC"),
    (Technique::DmiScan,            "DMI_SCAN"),
    (Technique::PodmanFile,         "PODMAN_FILE"),
    (Technique::WslProc,            "WSL_PROC"),
    (Technique::FileAccessHistory,  "FILE_ACCESS_HISTORY"),
    (Technique::Mac,                "MAC"),
    (Technique::ContainerPid,       "CONTAINER_PID"),
    (Technique::BluestacksFolders,  "BLUESTACKS_FOLDERS"),
    (Technique::AmdSevMsr,          "AMD_SEV_MSR"),
    (Technique::Temperature,        "TEMPERATURE"),
    (Technique::Cgroup,             "CGROUP"),
    (Technique::Processes,          "PROCESSES"),
    // Linux + macOS
    (Technique::ThreadCount,        "THREAD_COUNT"),
    // macOS
    (Technique::MacMemsize,         "MAC_MEMSIZE"),
    (Technique::MacIokit,           "MAC_IOKIT"),
    (Technique::MacSip,             "MAC_SIP"),
    (Technique::IoregGrep,          "IOREG_GREP"),
    (Technique::Hwmodel,            "HWMODEL"),
    (Technique::MacSys,             "MAC_SYS"),
    // Cross-platform
    (Technique::HypervisorBit,      "HYPERVISOR_BIT"),
    (Technique::Vmid,               "VMID"),
    (Technique::ThreadMismatch,     "THREAD_MISMATCH"),
    (Technique::Timer,              "TIMER"),
    (Technique::CpuBrand,           "CPU_BRAND"),
    (Technique::HypervisorStr,      "HYPERVISOR_STR"),
    (Technique::CpuidSignature,     "CPUID_SIGNATURE"),
    (Technique::BochsCpu,           "BOCHS_CPU"),
    (Technique::KgtSignature,       "KGT_SIGNATURE"),
];

fn main() {
    // Run the full scan once so vmaware caches all results internally.
    let info = query();

    println!("=== vmaware scan results ===");
    println!("  VM detected : {}", info.is_vm);
    println!("  Brand       : {}", info.brand);
    println!("  Type        : {}", info.vm_type);
    println!("  Certainty   : {}%", info.percentage);
    println!("  Hardened    : {}", info.is_hardened);
    println!("  Conclusion  : {}", info.conclusion);
    println!(
        "  Detections  : {}/{}",
        info.detected_count, info.technique_count
    );
    println!();

    println!("=== individual technique results ===");
    let name_width = ALL_TECHNIQUES
        .iter()
        .map(|(_, n)| n.len())
        .max()
        .unwrap_or(0);

    let mut detected = Vec::new();
    let mut not_detected = Vec::new();

    for &(technique, label) in ALL_TECHNIQUES {
        let result = check(technique);
        if result {
            detected.push(label);
        } else {
            not_detected.push(label);
        }
        let marker = if result { "[+]" } else { "[ ]" };
        println!("  {} {:<width$}  {}", marker, label, result, width = name_width);
    }

    println!();
    println!("=== summary ===");
    println!("  DETECTED ({}):", detected.len());
    for name in &detected {
        println!("    {}", name);
    }
    println!("  NOT DETECTED ({}):", not_detected.len());
    for name in &not_detected {
        println!("    {}", name);
    }
}
