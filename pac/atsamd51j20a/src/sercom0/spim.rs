#[doc = "SPIM Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIM Control A"]
pub mod ctrla;
#[doc = "SPIM Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIM Control B"]
pub mod ctrlb;
#[doc = "SPIM Control C"]
pub struct CTRLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIM Control C"]
pub mod ctrlc;
#[doc = "SPIM Baud Rate"]
pub struct BAUD {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPIM Baud Rate"]
pub mod baud;
#[doc = "SPIM Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPIM Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "SPIM Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPIM Interrupt Enable Set"]
pub mod intenset;
#[doc = "SPIM Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPIM Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "SPIM Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "SPIM Status"]
pub mod status;
#[doc = "SPIM Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIM Synchronization Busy"]
pub mod syncbusy;
#[doc = "SPIM Length"]
pub struct LENGTH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "SPIM Length"]
pub mod length;
#[doc = "SPIM Address"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIM Address"]
pub mod addr;
#[doc = "SPIM Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIM Data"]
pub mod data;
#[doc = "SPIM Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPIM Debug Control"]
pub mod dbgctrl;
