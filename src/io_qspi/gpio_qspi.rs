#[doc = r"Register block"]
#[repr(C)]
pub struct GPIO_QSPI {
    #[doc = "0x00 - GPIO status"]
    pub gpio_status: GPIO_STATUS,
    #[doc = "0x04 - GPIO control including function select and overrides."]
    pub gpio_ctrl: GPIO_CTRL,
}
#[doc = "GPIO_STATUS (r) register accessor: an alias for `Reg<GPIO_STATUS_SPEC>`"]
pub type GPIO_STATUS = crate::Reg<gpio_status::GPIO_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio_status;
#[doc = "GPIO_CTRL (rw) register accessor: an alias for `Reg<GPIO_CTRL_SPEC>`"]
pub type GPIO_CTRL = crate::Reg<gpio_ctrl::GPIO_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_ctrl;
