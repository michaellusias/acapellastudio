// Audio I/O device enumeration prototype — AcapellaStudio Phase 3 Feasibility Study
// Run on Michael's actual reference hardware (AMD Ryzen 7 8840HS, Kubuntu, PipeWire 1.6.2).
//
// Confirms cpal (with pipewire + realtime features) can see the native PipeWire host
// and enumerate real audio devices, before attempting to open a live input stream.

use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    let available_hosts = cpal::available_hosts();
    println!("Available audio hosts: {:?}", available_hosts);

    let host = cpal::default_host();
    println!("Using host: {:?}", host.id());

    match host.default_input_device() {
        Some(device) => {
            println!("\nDefault input device: {:?}", device.description());
            match device.default_input_config() {
                Ok(config) => println!("Default input config: {:?}", config),
                Err(e) => println!("Could not get default input config: {}", e),
            }
        }
        None => println!("\nNo default input device found."),
    }

    println!("\nAll input devices:");
    match host.input_devices() {
        Ok(devices) => {
            for device in devices {
                println!("  - {:?}", device.description());
            }
        }
        Err(e) => println!("  Error enumerating input devices: {}", e),
    }
}
