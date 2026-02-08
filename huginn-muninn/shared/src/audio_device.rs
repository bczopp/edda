//! Audio Device Management

use crate::error::{AudioError, Result};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioDeviceType {
    Input,
    Output,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub device_type: AudioDeviceType,
    pub sample_rate: u32,
    pub channels: u16,
}

pub struct AudioDeviceManager {
    devices: Vec<AudioDevice>,
    selected_input: Option<String>,
    selected_output: Option<String>,
}

impl AudioDeviceManager {
    pub fn new() -> Self {
        info!("Creating AudioDeviceManager");
        Self {
            devices: Vec::new(),
            selected_input: None,
            selected_output: None,
        }
    }
    
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
    
    /// List all available audio devices
    pub async fn list_devices(&mut self) -> Result<Vec<AudioDevice>> {
        info!("Listing audio devices");
        
        // TODO: Use cpal to enumerate actual devices
        // For now, return mock devices or empty list
        self.devices.clear();
        
        // Try to add default devices (mock for now)
        #[cfg(not(test))]
        {
            // In real implementation, use cpal::default_host().devices()
            warn!("Audio device enumeration not yet implemented - returning empty list");
        }
        
        Ok(self.devices.clone())
    }
    
    /// Get default device by type
    pub async fn get_default_device(&mut self, device_type: AudioDeviceType) -> Result<Option<AudioDevice>> {
        info!("Getting default {:?} device", device_type);
        
        // TODO: Use cpal to get default device
        // For now, return None or mock device
        Ok(None)
    }
    
    /// Select device by ID
    pub async fn select_device(&mut self, device_id: &str) -> Result<()> {
        info!("Selecting device: {}", device_id);
        
        // Find device
        let device = self.devices.iter()
            .find(|d| d.id == device_id)
            .ok_or_else(|| AudioError::DeviceError(format!("Device not found: {}", device_id)))?;
        
        // Store selection
        match device.device_type {
            AudioDeviceType::Input => {
                self.selected_input = Some(device_id.to_string());
            }
            AudioDeviceType::Output => {
                self.selected_output = Some(device_id.to_string());
            }
        }
        
        info!("Device selected: {} ({})", device.name, device_id);
        Ok(())
    }
    
    /// Get currently selected input device
    pub fn get_selected_input(&self) -> Option<&str> {
        self.selected_input.as_deref()
    }
    
    /// Get currently selected output device
    pub fn get_selected_output(&self) -> Option<&str> {
        self.selected_output.as_deref()
    }
}

impl Default for AudioDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_device_manager_new() {
        let manager = AudioDeviceManager::new();
        assert_eq!(manager.device_count(), 0);
        assert!(manager.get_selected_input().is_none());
        assert!(manager.get_selected_output().is_none());
    }
    
    #[tokio::test]
    async fn test_list_devices_empty() {
        let mut manager = AudioDeviceManager::new();
        let devices = manager.list_devices().await.unwrap();
        assert_eq!(devices.len(), 0);
    }
}
