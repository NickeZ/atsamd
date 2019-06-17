#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug Halting Control and Status Register"]
    pub dhcsr: DHCSR,
    #[doc = "0x04 - Debug Core Register Selector Register"]
    pub dcrsr: DCRSR,
    #[doc = "0x08 - Debug Core Register Data Register"]
    pub dcrdr: DCRDR,
    #[doc = "0x0c - Debug Exception and Monitor Control Register"]
    pub demcr: DEMCR,
}
#[doc = "Debug Halting Control and Status Register"]
pub struct DHCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Halting Control and Status Register"]
pub mod dhcsr;
#[doc = "Debug Core Register Selector Register"]
pub struct DCRSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Core Register Selector Register"]
pub mod dcrsr;
#[doc = "Debug Core Register Data Register"]
pub struct DCRDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Core Register Data Register"]
pub mod dcrdr;
#[doc = "Debug Exception and Monitor Control Register"]
pub struct DEMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Exception and Monitor Control Register"]
pub mod demcr;
