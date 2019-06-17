#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Interrupt Controller Type Register"]
    pub ictr: ICTR,
    #[doc = "0x08 - Auxiliary Control Register"]
    pub actlr: ACTLR,
    _reserved1: [u8; 3316usize],
    #[doc = "0xd00 - CPUID Base Register"]
    pub cpuid: CPUID,
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: ICSR,
    #[doc = "0xd08 - Vector Table Offset Register"]
    pub vtor: VTOR,
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: AIRCR,
    #[doc = "0xd10 - System Control Register"]
    pub scr: SCR,
    #[doc = "0xd14 - Configuration and Control Register"]
    pub ccr: CCR,
    #[doc = "0xd18 - System Handler Priority Register 1"]
    pub shpr1: SHPR1,
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: SHPR2,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: SHPR3,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: SHCSR,
    #[doc = "0xd28 - Configurable Fault Status Register"]
    pub cfsr: CFSR,
    #[doc = "0xd2c - HardFault Status Register"]
    pub hfsr: HFSR,
    #[doc = "0xd30 - Debug Fault Status Register"]
    pub dfsr: DFSR,
    #[doc = "0xd34 - MemManage Fault Address Register"]
    pub mmfar: MMFAR,
    #[doc = "0xd38 - BusFault Address Register"]
    pub bfar: BFAR,
    #[doc = "0xd3c - Auxiliary Fault Status Register"]
    pub afsr: AFSR,
    #[doc = "0xd40 - Processor Feature Register"]
    pub pfr: [PFR; 2],
    #[doc = "0xd48 - Debug Feature Register"]
    pub dfr: DFR,
    #[doc = "0xd4c - Auxiliary Feature Register"]
    pub adr: ADR,
    #[doc = "0xd50 - Memory Model Feature Register"]
    pub mmfr: [MMFR; 4],
    #[doc = "0xd60 - Instruction Set Attributes Register"]
    pub isar: [ISAR; 5],
    _reserved2: [u8; 20usize],
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    pub cpacr: CPACR,
}
#[doc = "Interrupt Controller Type Register"]
pub struct ICTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Controller Type Register"]
pub mod ictr;
#[doc = "Auxiliary Control Register"]
pub struct ACTLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Control Register"]
pub mod actlr;
#[doc = "CPUID Base Register"]
pub struct CPUID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "Interrupt Control and State Register"]
pub struct ICSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "Vector Table Offset Register"]
pub struct VTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "Application Interrupt and Reset Control Register"]
pub struct AIRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "System Control Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Control Register"]
pub mod scr;
#[doc = "Configuration and Control Register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "System Handler Priority Register 1"]
pub struct SHPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "System Handler Priority Register 2"]
pub struct SHPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "System Handler Priority Register 3"]
pub struct SHPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "System Handler Control and State Register"]
pub struct SHCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "Configurable Fault Status Register"]
pub struct CFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configurable Fault Status Register"]
pub mod cfsr;
#[doc = "HardFault Status Register"]
pub struct HFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HardFault Status Register"]
pub mod hfsr;
#[doc = "Debug Fault Status Register"]
pub struct DFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Fault Status Register"]
pub mod dfsr;
#[doc = "MemManage Fault Address Register"]
pub struct MMFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MemManage Fault Address Register"]
pub mod mmfar;
#[doc = "BusFault Address Register"]
pub struct BFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "Auxiliary Fault Status Register"]
pub struct AFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Fault Status Register"]
pub mod afsr;
#[doc = "Processor Feature Register"]
pub struct PFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor Feature Register"]
pub mod pfr;
#[doc = "Debug Feature Register"]
pub struct DFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Feature Register"]
pub mod dfr;
#[doc = "Auxiliary Feature Register"]
pub struct ADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Feature Register"]
pub mod adr;
#[doc = "Memory Model Feature Register"]
pub struct MMFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Model Feature Register"]
pub mod mmfr;
#[doc = "Instruction Set Attributes Register"]
pub struct ISAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Set Attributes Register"]
pub mod isar;
#[doc = "Coprocessor Access Control Register"]
pub struct CPACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
