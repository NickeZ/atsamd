#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supported Parallel Port Size Register"]
    pub sspsr: SSPSR,
    #[doc = "0x04 - Current Parallel Port Size Register"]
    pub cspsr: CSPSR,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Asynchronous Clock Prescaler Register"]
    pub acpr: ACPR,
    _reserved1: [u8; 220usize],
    #[doc = "0xf0 - Selected Pin Protocol Register"]
    pub sppr: SPPR,
    _reserved2: [u8; 524usize],
    #[doc = "0x300 - Formatter and Flush Status Register"]
    pub ffsr: FFSR,
    #[doc = "0x304 - Formatter and Flush Control Register"]
    pub ffcr: FFCR,
    #[doc = "0x308 - Formatter Synchronization Counter Register"]
    pub fscr: FSCR,
    _reserved3: [u8; 3036usize],
    #[doc = "0xee8 - TRIGGER"]
    pub trigger: TRIGGER,
    #[doc = "0xeec - Integration ETM Data"]
    pub fifo0: FIFO0,
    #[doc = "0xef0 - ITATBCTR2"]
    pub itatbctr2: ITATBCTR2,
    _reserved4: [u8; 4usize],
    #[doc = "0xef8 - ITATBCTR0"]
    pub itatbctr0: ITATBCTR0,
    #[doc = "0xefc - Integration ITM Data"]
    pub fifo1: FIFO1,
    #[doc = "0xf00 - Integration Mode Control"]
    pub itctrl: ITCTRL,
    _reserved5: [u8; 156usize],
    #[doc = "0xfa0 - Claim tag set"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - Claim tag clear"]
    pub claimclr: CLAIMCLR,
    _reserved6: [u8; 32usize],
    #[doc = "0xfc8 - TPIU_DEVID"]
    pub devid: DEVID,
    #[doc = "0xfcc - TPIU_DEVTYPE"]
    pub devtype: DEVTYPE,
}
#[doc = "Supported Parallel Port Size Register"]
pub struct SSPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supported Parallel Port Size Register"]
pub mod sspsr;
#[doc = "Current Parallel Port Size Register"]
pub struct CSPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Parallel Port Size Register"]
pub mod cspsr;
#[doc = "Asynchronous Clock Prescaler Register"]
pub struct ACPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Asynchronous Clock Prescaler Register"]
pub mod acpr;
#[doc = "Selected Pin Protocol Register"]
pub struct SPPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selected Pin Protocol Register"]
pub mod sppr;
#[doc = "Formatter and Flush Status Register"]
pub struct FFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Formatter and Flush Status Register"]
pub mod ffsr;
#[doc = "Formatter and Flush Control Register"]
pub struct FFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Formatter and Flush Control Register"]
pub mod ffcr;
#[doc = "Formatter Synchronization Counter Register"]
pub struct FSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Formatter Synchronization Counter Register"]
pub mod fscr;
#[doc = "TRIGGER"]
pub struct TRIGGER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRIGGER"]
pub mod trigger;
#[doc = "Integration ETM Data"]
pub struct FIFO0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integration ETM Data"]
pub mod fifo0;
#[doc = "ITATBCTR2"]
pub struct ITATBCTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ITATBCTR2"]
pub mod itatbctr2;
#[doc = "ITATBCTR0"]
pub struct ITATBCTR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ITATBCTR0"]
pub mod itatbctr0;
#[doc = "Integration ITM Data"]
pub struct FIFO1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integration ITM Data"]
pub mod fifo1;
#[doc = "Integration Mode Control"]
pub struct ITCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integration Mode Control"]
pub mod itctrl;
#[doc = "Claim tag set"]
pub struct CLAIMSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Claim tag set"]
pub mod claimset;
#[doc = "Claim tag clear"]
pub struct CLAIMCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Claim tag clear"]
pub mod claimclr;
#[doc = "TPIU_DEVID"]
pub struct DEVID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TPIU_DEVID"]
pub mod devid;
#[doc = "TPIU_DEVTYPE"]
pub struct DEVTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TPIU_DEVTYPE"]
pub mod devtype;
