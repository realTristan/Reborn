use sysinfo::{ProcessExt, SystemExt};

pub struct System {
    sys: sysinfo::System
}
impl System {
    pub fn new() -> Self {
        return Self {
            sys: sysinfo::System::new_all()
        }
    }

    // The system information and return it as a json object
    pub fn info(&mut self) -> serde_json::Value {
        self.sys.refresh_all();
        return serde_json::json!({
            "hardware_info": self.hardware_info(),
            "processes": self.processes()
        });
    }

    // Get the hardware information
    fn hardware_info(&self) -> serde_json::Value {
        let os_name = match self.sys.long_os_version() {
            Some(name) => name,
            None => String::from("Unknown")
        };
        let host_name = match self.sys.host_name() {
            Some(name) => name,
            None => String::from("Unknown")
        };
        return serde_json::json!({
            "total_memory": self.sys.total_memory().to_string(),
            "used_memory": self.sys.used_memory().to_string(),
            "total_swap": self.sys.total_swap().to_string(),
            "used_swap": self.sys.used_swap().to_string(),
            "boot_time": self.sys.boot_time().to_string(),
            "up_time": self.sys.uptime().to_string(),
            "system_name": self.sys.name().unwrap().to_string(),
            "system_os_version": os_name,
            "system_host_name": host_name,
            "distribution_id": self.sys.distribution_id().to_string(),
            "available_memory": self.sys.available_memory().to_string(),
            "number_of_cpus": self.sys.physical_core_count()
        });
    }

    // Get the processes
    fn processes(&self) -> Vec<serde_json::Value> {
        return self.sys.processes().iter().map(|(pid, process)| {
            serde_json::json!({
                "pid": pid.to_string(),
                "name": process.name().to_string(),
                "memory_usage": process.memory().to_string(),
                "virtual_memory": process.virtual_memory().to_string(),
                "cpu_usage": process.cpu_usage().to_string(),
                "start_time": process.start_time().to_string(),
                "run_time": process.run_time().to_string()
            })
        }).collect();
    }
}