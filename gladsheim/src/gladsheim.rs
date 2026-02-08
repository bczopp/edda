//! Main Gladsheim struct

use crate::thjalfi::Thjalfi;
use crate::byggvir::Byggvir;
use crate::roskva::Roskva;
use crate::skirnir::Skirnir;
use crate::utils::Result;
use tracing::info;

pub struct Gladsheim {
    thjalfi: Thjalfi,
    byggvir: Byggvir,
    roskva: Roskva,
    skirnir: Skirnir,
}

impl Gladsheim {
    pub fn new() -> Result<Self> {
        info!("Initializing Gladsheim");
        
        Ok(Self {
            thjalfi: Thjalfi::new()?,
            byggvir: Byggvir::new()?,
            roskva: Roskva::new()?,
            skirnir: Skirnir::new()?,
        })
    }
    
    pub fn thjalfi(&self) -> &Thjalfi {
        &self.thjalfi
    }
    
    pub fn byggvir(&self) -> &Byggvir {
        &self.byggvir
    }
    
    pub fn roskva(&self) -> &Roskva {
        &self.roskva
    }
    
    pub fn skirnir(&self) -> &Skirnir {
        &self.skirnir
    }
}

impl Default for Gladsheim {
    fn default() -> Self {
        Self::new().expect("Failed to initialize Gladsheim")
    }
}
