//! actionos-host — run simulated boot and print phase.

use actionos_core::BootPhase;
use actionos_host::HostPlatform;
use actionos_platform::Clock;

fn main() {
    let mut platform = HostPlatform::new();
    match platform.boot_to_running() {
        Ok(()) => {
            println!("phase={}", platform.boot.phase());
            let caps = platform.boot.capabilities();
            println!("host_simulated={}", caps.host_simulated);
            println!("has_clock={}", caps.has_clock);
            println!("has_console={}", caps.has_console);
            println!("now_ms={}", platform.clock.now_ms());
            let console_len = platform.console.buffer.len();
            println!("console_bytes={}", console_len);
            assert_eq!(platform.boot.phase(), BootPhase::Running);
        }
        Err(e) => {
            eprintln!("boot failed: {:?}", e);
            eprintln!("phase={}", platform.boot.phase());
            std::process::exit(1);
        }
    }
}
