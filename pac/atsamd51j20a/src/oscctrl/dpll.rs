#[doc = "DPLL Control A"]
pub struct DPLLCTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLL Ratio Control"]
pub struct DPLLRATIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLL Control B"]
pub struct DPLLCTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLL Synchronization Busy"]
pub struct DPLLSYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLL Status"]
pub struct DPLLSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Status"]
pub mod dpllstatus;
