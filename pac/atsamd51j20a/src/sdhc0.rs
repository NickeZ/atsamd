#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    pub ssar: SSAR,
    #[doc = "0x04 - Block Size"]
    pub bsr: BSR,
    #[doc = "0x06 - Block Count"]
    pub bcr: BCR,
    #[doc = "0x08 - Argument 1"]
    pub arg1r: ARG1R,
    #[doc = "0x0c - Transfer Mode"]
    pub tmr: TMR,
    #[doc = "0x0e - Command"]
    pub cr: CR,
    #[doc = "0x10 - Response"]
    pub rr: [RR; 4],
    #[doc = "0x20 - Buffer Data Port"]
    pub bdpr: BDPR,
    #[doc = "0x24 - Present State"]
    pub psr: PSR,
    #[doc = "0x28 - Host Control 1"]
    pub hc1r: HC1R,
    #[doc = "0x29 - Power Control"]
    pub pcr: PCR,
    #[doc = "0x2a - Block Gap Control"]
    pub bgcr: BGCR,
    #[doc = "0x2b - Wakeup Control"]
    pub wcr: WCR,
    #[doc = "0x2c - Clock Control"]
    pub ccr: CCR,
    #[doc = "0x2e - Timeout Control"]
    pub tcr: TCR,
    #[doc = "0x2f - Software Reset"]
    pub srr: SRR,
    #[doc = "0x30 - Normal Interrupt Status"]
    pub nistr: NISTR,
    #[doc = "0x32 - Error Interrupt Status"]
    pub eistr: EISTR,
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    pub nister: NISTER,
    #[doc = "0x36 - Error Interrupt Status Enable"]
    pub eister: EISTER,
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    pub nisier: NISIER,
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    pub eisier: EISIER,
    #[doc = "0x3c - Auto CMD Error Status"]
    pub acesr: ACESR,
    #[doc = "0x3e - Host Control 2"]
    pub hc2r: HC2R,
    #[doc = "0x40 - Capabilities 0"]
    pub ca0r: CA0R,
    #[doc = "0x44 - Capabilities 1"]
    pub ca1r: CA1R,
    #[doc = "0x48 - Maximum Current Capabilities"]
    pub mccar: MCCAR,
    _reserved0: [u8; 4usize],
    #[doc = "0x50 - Force Event for Auto CMD Error Status"]
    pub feraces: FERACES,
    #[doc = "0x52 - Force Event for Error Interrupt Status"]
    pub fereis: FEREIS,
    #[doc = "0x54 - ADMA Error Status"]
    pub aesr: AESR,
    _reserved1: [u8; 3usize],
    #[doc = "0x58 - ADMA System Address n"]
    pub asar: [ASAR; 1],
    _reserved2: [u8; 4usize],
    #[doc = "0x60 - Preset Value n"]
    pub pvr: [PVR; 8],
    _reserved3: [u8; 140usize],
    #[doc = "0xfc - Slot Interrupt Status"]
    pub sisr: SISR,
    #[doc = "0xfe - Host Controller Version"]
    pub hcvr: HCVR,
    _reserved4: [u8; 260usize],
    #[doc = "0x204 - MMC Control 1"]
    pub mc1r: MC1R,
    #[doc = "0x205 - MMC Control 2"]
    pub mc2r: MC2R,
    _reserved5: [u8; 2usize],
    #[doc = "0x208 - AHB Control"]
    pub acr: ACR,
    #[doc = "0x20c - Clock Control 2"]
    pub cc2r: CC2R,
    _reserved6: [u8; 32usize],
    #[doc = "0x230 - Capabilities Control"]
    pub cacr: CACR,
    #[doc = "0x234 - Debug"]
    pub dbgr: DBGR,
}
#[doc = "SDMA System Address / Argument 2"]
pub struct SSAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar;
#[doc = "SDMA System Address / Argument 2"]
pub struct SSAR_CMD23_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar_cmd23_mode;
#[doc = "Block Size"]
pub struct BSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Block Size"]
pub mod bsr;
#[doc = "Block Count"]
pub struct BCR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Block Count"]
pub mod bcr;
#[doc = "Argument 1"]
pub struct ARG1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Argument 1"]
pub mod arg1r;
#[doc = "Transfer Mode"]
pub struct TMR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transfer Mode"]
pub mod tmr;
#[doc = "Command"]
pub struct CR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Command"]
pub mod cr;
#[doc = "Response"]
pub struct RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response"]
pub mod rr;
#[doc = "Buffer Data Port"]
pub struct BDPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer Data Port"]
pub mod bdpr;
#[doc = "Present State"]
pub struct PSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Present State"]
pub mod psr;
#[doc = "Host Control 1"]
pub struct HC1R {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Host Control 1"]
pub mod hc1r;
#[doc = "Host Control 1"]
pub struct HC1R_EMMC_MODE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Host Control 1"]
pub mod hc1r_emmc_mode;
#[doc = "Power Control"]
pub struct PCR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Power Control"]
pub mod pcr;
#[doc = "Block Gap Control"]
pub struct BGCR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Block Gap Control"]
pub mod bgcr;
#[doc = "Block Gap Control"]
pub struct BGCR_EMMC_MODE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Block Gap Control"]
pub mod bgcr_emmc_mode;
#[doc = "Wakeup Control"]
pub struct WCR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Wakeup Control"]
pub mod wcr;
#[doc = "Clock Control"]
pub struct CCR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Clock Control"]
pub mod ccr;
#[doc = "Timeout Control"]
pub struct TCR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timeout Control"]
pub mod tcr;
#[doc = "Software Reset"]
pub struct SRR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Software Reset"]
pub mod srr;
#[doc = "Normal Interrupt Status"]
pub struct NISTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Status"]
pub mod nistr;
#[doc = "Normal Interrupt Status"]
pub struct NISTR_EMMC_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Status"]
pub mod nistr_emmc_mode;
#[doc = "Error Interrupt Status"]
pub struct EISTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Status"]
pub mod eistr;
#[doc = "Error Interrupt Status"]
pub struct EISTR_EMMC_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Status"]
pub mod eistr_emmc_mode;
#[doc = "Normal Interrupt Status Enable"]
pub struct NISTER {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Status Enable"]
pub mod nister;
#[doc = "Normal Interrupt Status Enable"]
pub struct NISTER_EMMC_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Status Enable"]
pub mod nister_emmc_mode;
#[doc = "Error Interrupt Status Enable"]
pub struct EISTER {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Status Enable"]
pub mod eister;
#[doc = "Error Interrupt Status Enable"]
pub struct EISTER_EMMC_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Status Enable"]
pub mod eister_emmc_mode;
#[doc = "Normal Interrupt Signal Enable"]
pub struct NISIER {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier;
#[doc = "Normal Interrupt Signal Enable"]
pub struct NISIER_EMMC_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier_emmc_mode;
#[doc = "Error Interrupt Signal Enable"]
pub struct EISIER {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier;
#[doc = "Error Interrupt Signal Enable"]
pub struct EISIER_EMMC_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier_emmc_mode;
#[doc = "Auto CMD Error Status"]
pub struct ACESR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Auto CMD Error Status"]
pub mod acesr;
#[doc = "Host Control 2"]
pub struct HC2R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Host Control 2"]
pub mod hc2r;
#[doc = "Host Control 2"]
pub struct HC2R_EMMC_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Host Control 2"]
pub mod hc2r_emmc_mode;
#[doc = "Capabilities 0"]
pub struct CA0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capabilities 0"]
pub mod ca0r;
#[doc = "Capabilities 1"]
pub struct CA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capabilities 1"]
pub mod ca1r;
#[doc = "Maximum Current Capabilities"]
pub struct MCCAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum Current Capabilities"]
pub mod mccar;
#[doc = "Force Event for Auto CMD Error Status"]
pub struct FERACES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Force Event for Auto CMD Error Status"]
pub mod feraces;
#[doc = "Force Event for Error Interrupt Status"]
pub struct FEREIS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Force Event for Error Interrupt Status"]
pub mod fereis;
#[doc = "ADMA Error Status"]
pub struct AESR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ADMA Error Status"]
pub mod aesr;
#[doc = "ADMA System Address n"]
pub struct ASAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADMA System Address n"]
pub mod asar;
#[doc = "Preset Value n"]
pub struct PVR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Preset Value n"]
pub mod pvr;
#[doc = "Slot Interrupt Status"]
pub struct SISR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Slot Interrupt Status"]
pub mod sisr;
#[doc = "Host Controller Version"]
pub struct HCVR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Host Controller Version"]
pub mod hcvr;
#[doc = "MMC Control 1"]
pub struct MC1R {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MMC Control 1"]
pub mod mc1r;
#[doc = "MMC Control 2"]
pub struct MC2R {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MMC Control 2"]
pub mod mc2r;
#[doc = "AHB Control"]
pub struct ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Control"]
pub mod acr;
#[doc = "Clock Control 2"]
pub struct CC2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Control 2"]
pub mod cc2r;
#[doc = "Capabilities Control"]
pub struct CACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capabilities Control"]
pub mod cacr;
#[doc = "Debug"]
pub struct DBGR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug"]
pub mod dbgr;
