use gtk::{prelude::*, Box, Label, ProgressBar};
use sysinfo::{System, Disks, Networks, Components};

pub fn memory_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    format!(
        "Used mem {1}/{0}",
        sys.total_memory() / (1024 * 1024),
        sys.used_memory() / (1024 * 1024)
    )
}

pub fn swap_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    format!(
        "Used swap {1}/{0}",
        sys.total_swap() / (1024 * 1024),
        sys.used_swap() / (1024 * 1024)
    )
}

pub fn os_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    format!(
        "System name {}\nKernel ver {}\nOs ver {}\nHostname {}",
        System::name().unwrap(),
        System::kernel_version().unwrap(),
        System::os_version().unwrap(),
        System::host_name().unwrap()
    )
}

pub fn disk_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut string_info = String::new();

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        string_info.push_str(&format!(
            "Disk: {}\n\tSpace used {} / {}\n\tMount point {}\n",
            disk.name().to_str().unwrap(),
            disk.available_space() / (1024 * 1024),
            disk.total_space() / (1024 * 1024),
            disk.mount_point().to_str().unwrap()
        ));
    }

    string_info
}

pub fn memory_page(mem_bar: &ProgressBar, swap_bar: &ProgressBar) -> Box {
    let mbox = Box::new(gtk::Orientation::Vertical, 0);
    let mem_label = Label::new(Some(&memory_info()));
    mem_label.add_css_class("text");

    let swap_label = Label::new(Some(&swap_info()));
    swap_label.add_css_class("text");

    mbox.append(mem_bar);
    mbox.append(&mem_label);
    mbox.append(swap_bar);
    mbox.append(&swap_label);
    mbox
}

pub fn networks() -> String {
    let mut networks = Networks::new_with_refreshed_list();
    let mut networks_string = String::new();
    for (interface_name, data) in &networks {
        networks_string.push_str(&format!(
            "{interface_name}: {} B (down) / {} B (up)\n",
            data.total_received() / (1024 * 1024),
            data.total_transmitted() / (1024 * 1024),
        ));
    }
    networks_string
}

pub fn components_info() -> String {
    let components = Components::new_with_refreshed_list();
    let mut components_string = String::new();
    println!("=> components:");
    for component in &components {
        components_string.push_str(
            &format!("{component:?}")
        );
        components_string.push('\n');
    }
    components_string
}