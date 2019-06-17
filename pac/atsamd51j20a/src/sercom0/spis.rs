#[doc = "SPIS Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIS Control A"]
pub mod ctrla;
#[doc = "SPIS Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIS Control B"]
pub mod ctrlb;
#[doc = "SPIS Control C"]
pub struct CTRLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIS Control C"]
pub mod ctrlc;
#[doc = "SPIS Baud Rate"]
pub struct BAUD {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPIS Baud Rate"]
pub mod baud;
#[doc = "SPIS Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPIS Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "SPIS Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPIS Interrupt Enable Set"]
pub mod intenset;
#[doc = "SPIS Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPIS Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "SPIS Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "SPIS Status"]
pub mod status;
#[doc = "SPIS Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIS Synchronization Busy"]
pub mod syncbusy;
#[doc = "SPIS Length"]
pub struct LENGTH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "SPIS Length"]
pub mod length;
#[doc = "SPIS Address"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIS Address"]
pub mod addr;
#[doc = "SPIS Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIS Data"]
pub mod data;
#[doc = "SPIS Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPIS Debug Control"]
pub mod dbgctrl;
