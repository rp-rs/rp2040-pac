#[repr(C)]
#[doc = "Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
pub struct GPIO_QSPI {
    gpio_status: GPIO_STATUS,
    gpio_ctrl: GPIO_CTRL,
}
impl GPIO_QSPI {
    #[doc = "0x00 - GPIO status"]
    #[inline(always)]
    pub const fn gpio_status(&self) -> &GPIO_STATUS {
        &self.gpio_status
    }
    #[doc = "0x04 - GPIO control including function select and overrides."]
    #[inline(always)]
    pub const fn gpio_ctrl(&self) -> &GPIO_CTRL {
        &self.gpio_ctrl
    }
}
#[doc = "GPIO_STATUS (rw) register accessor: GPIO status  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_status`]
module"]
pub type GPIO_STATUS = crate::Reg<gpio_status::GPIO_STATUS_SPEC>;
#[doc = "GPIO status"]
pub mod gpio_status;
#[doc = "GPIO_CTRL (rw) register accessor: GPIO control including function select and overrides.  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_ctrl`]
module"]
pub type GPIO_CTRL = crate::Reg<gpio_ctrl::GPIO_CTRL_SPEC>;
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_ctrl;
