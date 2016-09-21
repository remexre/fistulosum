use ocl::*;

pub fn list_devices() -> ! {
    use std::process::exit;

    Platform::list().iter()
        .flat_map(Device::list_all)
        .enumerate()
        .map(|(i, d)| println!("{}: {}", i, d.name()))
        .last()
        .expect("No devices found");
    exit(0);
}
