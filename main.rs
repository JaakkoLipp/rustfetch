use sysinfo::{System};

fn main(){
    let mut sys = System::new_all();
    sys.refresh_all();
    // System and os identification
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // Processor and GPU details


    // RAM and swap information
    println!("total memory: {} MB", sys.total_memory() / 1024);
    println!("used memory : {} MB", sys.used_memory() / 1024);
    println!("total swap  : {} MB", sys.total_swap() / 1024);
    println!("used swap   : {} MB", sys.used_swap() / 1024);
    

}