
struct System {
    sys: sysinfo::System
}
impl System {
    pub fn new() -> Self {
        return Self {
            sys: sysinfo::System::new_all()
        }
    }

    // The system information and return it as a json object
    pub fn info(&self) -> serde_json::Value {
        let map: serde_json::Value = serde_json::json!({
            "hardware_info": self.hardware_info(),
            "processes": self.processes(),
            "users": self.users(),
            "disks": self.disks(),
            "networks": self.networks(),
            "components": self.components()
        });
    }

    // Get the hardware information
    fn hardware_info(&self) -> Vec<serde_json::Value> {
        return serde_json::json!({
            "total_memory": sys.total_memory(),
            "used_memory": sys.used_memory(),
            "total_swap": sys.total_swap(),
            "used_swap": sys.used_swap(),
            "boot_time": sys.boot_time(),
            "up_time": sys.uptime(),
            "system_name": sys.name(),
            "system_kernel_version": sys.kernel_version(),
            "system_os_version": sys.long_os_version(),
            "system_host_name": sys.host_name(),
            "distribution_id": sys.distribution_id(),
            "global_cpu_info": sys.global_cpu_info(),
            "available_memory": sys.available_memory(),
            "number_of_cpus": sys.physical_core_count()
        });
    }

    // Get the processes
    fn processes(&self) -> Vec<serde_json::Value> {
        return vec![
            for (pid, process) in sys.processes() {
                {
                    "pid": pid,
                    "name": process.name(),
                    "disk_usage": process.disk_usage(),
                    "memory_usage": process.memory(),
                    "virtual_memory": process.virtual_memory(),
                    "cpu_usage": process.cpu_usage(),
                    "status": process.status(),
                    "start_time": process.start_time(),
                    "run_time": process.run_time()
                }
            }
        ];
    }

    // Get the users
    fn users(&self) -> Vec<serde_json::Value> {
        return vec![
            for user in sys.users() {
                {
                    "name": user.name(),
                    "terminal": user.terminal(),
                    "host": user.host(),
                    "started": user.started(),
                    "process": user.process()
                }
            }
        ];
    }

    // Get the disks
    fn disks(&self) -> Vec<serde_json::Value> {
        return self.sys.disks().iter().map(|d| {
            serde_json::json!({
                "name": d.name(),
                "total_space": d.total_space(),
                "available_space": d.available_space(),
                "file_system": d.file_system(),
                "mount_point": d.mount_point()
            });
        }).collect();
    }

    // Get the networks
    fn networks(&self) -> Vec<serde_json::Value> {
        return self.sys.networks().iter().map(|(name, data)| {
            serde_json::json!({
                "name": name,
                "received": data.received(),
                "transmitted": data.transmitted()
            });
        }).collect();
    }

    // Get the components
    fn components(&self) -> Vec<serde_json::Value> {
        return self.sys.components().iter().map(|c| {
            serde_json::json!({
                "label": c.label(),
                "temperature": c.temperature(),
                "max": c.max(),
                "min": c.min(),
                "critical": c.critical(),
                "control": c.control(),
                "control_max": c.control_max(),
                "control_min": c.control_min()
            });
        }).collect();
    }
}