use vmaware_native::{check, query, Technique};

fn main() {
    // Run the full scan once so vmaware caches all results internally.
    let info = query();

    println!("=== vmaware scan results ===");
    println!("  VM detected : {}", info.is_vm);
    println!("  Brand       : {}", info.brand.as_deref().unwrap_or("Unknown"));
    println!("  Type        : {}", info.vm_type.as_deref().unwrap_or("Unknown"));
    println!("  Certainty   : {}%", info.percentage);
    println!("  Hardened    : {}", info.is_hardened);
    println!("  Conclusion  : {}", info.conclusion);
    println!(
        "  Detections  : {}/{}",
        info.detected_count, info.technique_count
    );
    println!();

    println!("=== individual technique results ===");
    let all = Technique::ALL;
    let name_width = all
        .iter()
        .map(|t| t.name().len())
        .max()
        .unwrap_or(0);

    let mut detected = Vec::new();
    let mut not_detected = Vec::new();

    for technique in all {
        let result = check(technique.clone());
        if result {
            detected.push(technique.name());
        } else {
            not_detected.push(technique.name());
        }
        let marker = if result { "[+]" } else { "[ ]" };
        println!("  {} {:<width$}  {}", marker, technique.name(), result, width = name_width);
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
