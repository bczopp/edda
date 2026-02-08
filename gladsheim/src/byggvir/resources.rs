//! Byggvir - Resource Manager

use crate::utils::Result;
use sysinfo::{System, Pid};
use tracing::info;

pub struct Byggvir {
    system: System,
}

impl Byggvir {
    pub fn new() -> Result<Self> {
        info!("Initializing Byggvir (Resource Manager)");
        
        let mut system = System::new_all();
        system.refresh_all();
        
        Ok(Self { system })
    }
    
    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }
    
    pub fn total_memory(&self) -> u64 {
        self.system.total_memory()
    }
    
    pub fn used_memory(&self) -> u64 {
        self.system.used_memory()
    }
    
    pub fn total_cpu(&self) -> f32 {
        self.system.global_cpu_info().cpu_usage()
    }
    
    pub fn process_memory(&self, pid: Pid) -> Option<u64> {
        self.system.process(pid).map(|p| p.memory())
    }
    
    pub fn process_cpu(&self, pid: Pid) -> Option<f32> {
        self.system.process(pid).map(|p| p.cpu_usage())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_byggvir_creation() {
        let byggvir = Byggvir::new();
        assert!(byggvir.is_ok());
    }
    
    #[test]
    fn test_memory_info() {
        let byggvir = Byggvir::new().unwrap();
        let total = byggvir.total_memory();
        let used = byggvir.used_memory();
        
        assert!(total > 0);
        assert!(used <= total);
    }
}
