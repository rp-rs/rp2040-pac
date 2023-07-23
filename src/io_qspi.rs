#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x30 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    pub gpio_qspi: [GPIO_QSPI; 6],
    #[doc = "0x30 - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0x34 - Interrupt Enable for proc0"]
    pub proc0_inte: PROC0_INTE,
    #[doc = "0x38 - Interrupt Force for proc0"]
    pub proc0_intf: PROC0_INTF,
    #[doc = "0x3c - Interrupt status after masking & forcing for proc0"]
    pub proc0_ints: PROC0_INTS,
    #[doc = "0x40 - Interrupt Enable for proc1"]
    pub proc1_inte: PROC1_INTE,
    #[doc = "0x44 - Interrupt Force for proc1"]
    pub proc1_intf: PROC1_INTF,
    #[doc = "0x48 - Interrupt status after masking & forcing for proc1"]
    pub proc1_ints: PROC1_INTS,
    #[doc = "0x4c - Interrupt Enable for dormant_wake"]
    pub dormant_wake_inte: DORMANT_WAKE_INTE,
    #[doc = "0x50 - Interrupt Force for dormant_wake"]
    pub dormant_wake_intf: DORMANT_WAKE_INTF,
    #[doc = "0x54 - Interrupt status after masking & forcing for dormant_wake"]
    pub dormant_wake_ints: DORMANT_WAKE_INTS,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub fn gpio_qspisclk(&self) -> &GPIO_QSPI {
        &self.gpio_qspi[0]
    }
    #[doc = "0x08..0x10 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub fn gpio_qspiss(&self) -> &GPIO_QSPI {
        &self.gpio_qspi[1]
    }
    #[doc = "0x10..0x18 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub fn gpio_qspisd0(&self) -> &GPIO_QSPI {
        &self.gpio_qspi[2]
    }
    #[doc = "0x18..0x20 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub fn gpio_qspisd1(&self) -> &GPIO_QSPI {
        &self.gpio_qspi[3]
    }
    #[doc = "0x20..0x28 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub fn gpio_qspisd2(&self) -> &GPIO_QSPI {
        &self.gpio_qspi[4]
    }
    #[doc = "0x28..0x30 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub fn gpio_qspisd3(&self) -> &GPIO_QSPI {
        &self.gpio_qspi[5]
    }
}
#[doc = "Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
pub use self::gpio_qspi::GPIO_QSPI;
#[doc = r"Cluster"]
#[doc = "Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
pub mod gpio_qspi;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "PROC0_INTE (rw) register accessor: an alias for `Reg<PROC0_INTE_SPEC>`"]
pub type PROC0_INTE = crate::Reg<proc0_inte::PROC0_INTE_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte;
#[doc = "PROC0_INTF (rw) register accessor: an alias for `Reg<PROC0_INTF_SPEC>`"]
pub type PROC0_INTF = crate::Reg<proc0_intf::PROC0_INTF_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf;
#[doc = "PROC0_INTS (r) register accessor: an alias for `Reg<PROC0_INTS_SPEC>`"]
pub type PROC0_INTS = crate::Reg<proc0_ints::PROC0_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc0"]
pub mod proc0_ints;
#[doc = "PROC1_INTE (rw) register accessor: an alias for `Reg<PROC1_INTE_SPEC>`"]
pub type PROC1_INTE = crate::Reg<proc1_inte::PROC1_INTE_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte;
#[doc = "PROC1_INTF (rw) register accessor: an alias for `Reg<PROC1_INTF_SPEC>`"]
pub type PROC1_INTF = crate::Reg<proc1_intf::PROC1_INTF_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf;
#[doc = "PROC1_INTS (r) register accessor: an alias for `Reg<PROC1_INTS_SPEC>`"]
pub type PROC1_INTS = crate::Reg<proc1_ints::PROC1_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for proc1"]
pub mod proc1_ints;
#[doc = "DORMANT_WAKE_INTE (rw) register accessor: an alias for `Reg<DORMANT_WAKE_INTE_SPEC>`"]
pub type DORMANT_WAKE_INTE = crate::Reg<dormant_wake_inte::DORMANT_WAKE_INTE_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte;
#[doc = "DORMANT_WAKE_INTF (rw) register accessor: an alias for `Reg<DORMANT_WAKE_INTF_SPEC>`"]
pub type DORMANT_WAKE_INTF = crate::Reg<dormant_wake_intf::DORMANT_WAKE_INTF_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf;
#[doc = "DORMANT_WAKE_INTS (r) register accessor: an alias for `Reg<DORMANT_WAKE_INTS_SPEC>`"]
pub type DORMANT_WAKE_INTS = crate::Reg<dormant_wake_ints::DORMANT_WAKE_INTS_SPEC>;
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
pub mod dormant_wake_ints;
