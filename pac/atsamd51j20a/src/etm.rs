#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ETM Main Control Register"]
    pub cr: CR,
    #[doc = "0x04 - ETM Configuration Code Register"]
    pub ccr: CCR,
    #[doc = "0x08 - ETM Trigger Event Register"]
    pub trigger: TRIGGER,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - ETM Status Register"]
    pub sr: SR,
    #[doc = "0x14 - ETM System Configuration Register"]
    pub scr: SCR,
    _reserved1: [u8; 8usize],
    #[doc = "0x20 - ETM TraceEnable Event Register"]
    pub teevr: TEEVR,
    #[doc = "0x24 - ETM TraceEnable Control 1 Register"]
    pub tecr1: TECR1,
    #[doc = "0x28 - ETM FIFO Full Level Register"]
    pub fflr: FFLR,
    _reserved2: [u8; 276usize],
    #[doc = "0x140 - ETM Free-running Counter Reload Value"]
    pub cntrldvr1: CNTRLDVR1,
    _reserved3: [u8; 156usize],
    #[doc = "0x1e0 - ETM Synchronization Frequency Register"]
    pub syncfr: SYNCFR,
    #[doc = "0x1e4 - ETM ID Register"]
    pub idr: IDR,
    #[doc = "0x1e8 - ETM Configuration Code Extension Register"]
    pub ccer: CCER,
    _reserved4: [u8; 4usize],
    #[doc = "0x1f0 - ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
    pub tesseicr: TESSEICR,
    _reserved5: [u8; 4usize],
    #[doc = "0x1f8 - ETM TimeStamp Event Register"]
    pub tsevt: TSEVT,
    _reserved6: [u8; 4usize],
    #[doc = "0x200 - ETM CoreSight Trace ID Register"]
    pub traceidr: TRACEIDR,
    _reserved7: [u8; 4usize],
    #[doc = "0x208 - ETM ID Register 2"]
    pub idr2: IDR2,
    _reserved8: [u8; 264usize],
    #[doc = "0x314 - ETM Device Power-Down Status Register"]
    pub pdsr: PDSR,
    _reserved9: [u8; 3016usize],
    #[doc = "0xee0 - ETM Integration Test Miscellaneous Inputs"]
    pub itmiscin: ITMISCIN,
    _reserved10: [u8; 4usize],
    #[doc = "0xee8 - ETM Integration Test Trigger Out"]
    pub ittrigout: ITTRIGOUT,
    _reserved11: [u8; 4usize],
    #[doc = "0xef0 - ETM Integration Test ATB Control 2"]
    pub itatbctr2: ITATBCTR2,
    _reserved12: [u8; 4usize],
    #[doc = "0xef8 - ETM Integration Test ATB Control 0"]
    pub itatbctr0: ITATBCTR0,
    _reserved13: [u8; 4usize],
    #[doc = "0xf00 - ETM Integration Mode Control Register"]
    pub itctrl: ITCTRL,
    _reserved14: [u8; 156usize],
    #[doc = "0xfa0 - ETM Claim Tag Set Register"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - ETM Claim Tag Clear Register"]
    pub claimclr: CLAIMCLR,
    _reserved15: [u8; 8usize],
    #[doc = "0xfb0 - ETM Lock Access Register"]
    pub lar: LAR,
    #[doc = "0xfb4 - ETM Lock Status Register"]
    pub lsr: LSR,
    #[doc = "0xfb8 - ETM Authentication Status Register"]
    pub authstatus: AUTHSTATUS,
    _reserved16: [u8; 16usize],
    #[doc = "0xfcc - ETM CoreSight Device Type Register"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - ETM Peripheral Identification Register #4"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - ETM Peripheral Identification Register #5"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - ETM Peripheral Identification Register #6"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - ETM Peripheral Identification Register #7"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - ETM Peripheral Identification Register #0"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - ETM Peripheral Identification Register #1"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - ETM Peripheral Identification Register #2"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - ETM Peripheral Identification Register #3"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - ETM Component Identification Register #0"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - ETM Component Identification Register #1"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - ETM Component Identification Register #2"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - ETM Component Identification Register #3"]
    pub cidr3: CIDR3,
}
#[doc = "ETM Main Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Main Control Register"]
pub mod cr;
#[doc = "ETM Configuration Code Register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Configuration Code Register"]
pub mod ccr;
#[doc = "ETM Trigger Event Register"]
pub struct TRIGGER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Trigger Event Register"]
pub mod trigger;
#[doc = "ETM Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Status Register"]
pub mod sr;
#[doc = "ETM System Configuration Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM System Configuration Register"]
pub mod scr;
#[doc = "ETM TraceEnable Event Register"]
pub struct TEEVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM TraceEnable Event Register"]
pub mod teevr;
#[doc = "ETM TraceEnable Control 1 Register"]
pub struct TECR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM TraceEnable Control 1 Register"]
pub mod tecr1;
#[doc = "ETM FIFO Full Level Register"]
pub struct FFLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM FIFO Full Level Register"]
pub mod fflr;
#[doc = "ETM Free-running Counter Reload Value"]
pub struct CNTRLDVR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Free-running Counter Reload Value"]
pub mod cntrldvr1;
#[doc = "ETM Synchronization Frequency Register"]
pub struct SYNCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Synchronization Frequency Register"]
pub mod syncfr;
#[doc = "ETM ID Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM ID Register"]
pub mod idr;
#[doc = "ETM Configuration Code Extension Register"]
pub struct CCER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Configuration Code Extension Register"]
pub mod ccer;
#[doc = "ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
pub struct TESSEICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod tesseicr;
#[doc = "ETM TimeStamp Event Register"]
pub struct TSEVT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM TimeStamp Event Register"]
pub mod tsevt;
#[doc = "ETM CoreSight Trace ID Register"]
pub struct TRACEIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM CoreSight Trace ID Register"]
pub mod traceidr;
#[doc = "ETM ID Register 2"]
pub struct IDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM ID Register 2"]
pub mod idr2;
#[doc = "ETM Device Power-Down Status Register"]
pub struct PDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Device Power-Down Status Register"]
pub mod pdsr;
#[doc = "ETM Integration Test Miscellaneous Inputs"]
pub struct ITMISCIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Integration Test Miscellaneous Inputs"]
pub mod itmiscin;
#[doc = "ETM Integration Test Trigger Out"]
pub struct ITTRIGOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Integration Test Trigger Out"]
pub mod ittrigout;
#[doc = "ETM Integration Test ATB Control 2"]
pub struct ITATBCTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Integration Test ATB Control 2"]
pub mod itatbctr2;
#[doc = "ETM Integration Test ATB Control 0"]
pub struct ITATBCTR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Integration Test ATB Control 0"]
pub mod itatbctr0;
#[doc = "ETM Integration Mode Control Register"]
pub struct ITCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Integration Mode Control Register"]
pub mod itctrl;
#[doc = "ETM Claim Tag Set Register"]
pub struct CLAIMSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Claim Tag Set Register"]
pub mod claimset;
#[doc = "ETM Claim Tag Clear Register"]
pub struct CLAIMCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Claim Tag Clear Register"]
pub mod claimclr;
#[doc = "ETM Lock Access Register"]
pub struct LAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Lock Access Register"]
pub mod lar;
#[doc = "ETM Lock Status Register"]
pub struct LSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Lock Status Register"]
pub mod lsr;
#[doc = "ETM Authentication Status Register"]
pub struct AUTHSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Authentication Status Register"]
pub mod authstatus;
#[doc = "ETM CoreSight Device Type Register"]
pub struct DEVTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM CoreSight Device Type Register"]
pub mod devtype;
#[doc = "ETM Peripheral Identification Register #4"]
pub struct PIDR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Peripheral Identification Register #4"]
pub mod pidr4;
#[doc = "ETM Peripheral Identification Register #5"]
pub struct PIDR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Peripheral Identification Register #5"]
pub mod pidr5;
#[doc = "ETM Peripheral Identification Register #6"]
pub struct PIDR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Peripheral Identification Register #6"]
pub mod pidr6;
#[doc = "ETM Peripheral Identification Register #7"]
pub struct PIDR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Peripheral Identification Register #7"]
pub mod pidr7;
#[doc = "ETM Peripheral Identification Register #0"]
pub struct PIDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Peripheral Identification Register #0"]
pub mod pidr0;
#[doc = "ETM Peripheral Identification Register #1"]
pub struct PIDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Peripheral Identification Register #1"]
pub mod pidr1;
#[doc = "ETM Peripheral Identification Register #2"]
pub struct PIDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Peripheral Identification Register #2"]
pub mod pidr2;
#[doc = "ETM Peripheral Identification Register #3"]
pub struct PIDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Peripheral Identification Register #3"]
pub mod pidr3;
#[doc = "ETM Component Identification Register #0"]
pub struct CIDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Component Identification Register #0"]
pub mod cidr0;
#[doc = "ETM Component Identification Register #1"]
pub struct CIDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Component Identification Register #1"]
pub mod cidr1;
#[doc = "ETM Component Identification Register #2"]
pub struct CIDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Component Identification Register #2"]
pub mod cidr2;
#[doc = "ETM Component Identification Register #3"]
pub struct CIDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETM Component Identification Register #3"]
pub mod cidr3;
