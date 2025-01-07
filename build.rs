use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    env::set_current_dir("6502_65C02_functional_tests").expect("Failed to change directory");

    let build_test_binary = |path: &Path| {
        Command::new("./as65.exe")
            .arg("-l")
            .arg("-m")
            .arg("-w")
            .arg("-h0")
            .arg(path)
            .output()
            .expect("Failed to assemble test suite");
    };
    let run_sed = |path: &Path, sedstring: &str| {
        Command::new("sed")
            .arg("--in-place")
            .arg(sedstring)
            .arg(path)
            .output()
            .expect("Failed to run sed");
    };
    let get_sha1 = |path: &Path| -> String {
        let checksum = Command::new("sha1sum")
            .arg(path)
            .output()
            .expect("Failed to run sha1sum");
        String::from_utf8_lossy(&checksum.stdout)
            .split(" ")
            .next()
            .expect("Couldn't get sha1sum string")
            .to_string()
    };

    // Extract as65_142.zip
    let zip_path = Path::new("as65_142.zip");
    Command::new("unzip")
        .arg(zip_path)
        .output()
        .expect("Failed to unzip as65_142.zip");

    // Assemble functional test
    let functional_test_source = Path::new("6502_functional_test.a65");
    if get_sha1(functional_test_source) != "7b1181d8b7da846aedcbf31da7b6d2a38b9c9f2e" {
        run_sed(
            functional_test_source,
            r"s/zero_page = $a/zero_page = $0/;s/disable_decimal = 0/disable_decimal = 1/",
        );
    }
    build_test_binary(functional_test_source);

    // Assemble interrupt test
    let interrupt_test_source = Path::new("6502_interrupt_test.a65");
    if get_sha1(interrupt_test_source) != "ac34dba4de1849d9ba6ad610c570831f5ae87d43" {
        run_sed(
            interrupt_test_source,
            r"s/zero_page = $a/zero_page = $0/;s/I_drive     = 1/I_drive     = 0/",
        );
    }
    build_test_binary(interrupt_test_source);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=6502_65C02_functional_tests/6502_functional_test.a65");
    println!("cargo:rerun-if-changed=6502_65C02_functional_tests/6502_interrupt_test.a65");
}
