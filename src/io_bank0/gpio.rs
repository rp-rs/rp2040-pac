#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_status](gpio_status) module"]
pub type GPIO_STATUS = crate::Reg<u32, _GPIO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STATUS;
#[doc = "`read()` method returns [gpio_status::R](gpio_status::R) reader structure"]
impl crate::Readable for GPIO_STATUS {}
#[doc = "GPIO status"]
pub mod gpio_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_ctrl](gpio_ctrl) module"]
pub type GPIO_CTRL = crate::Reg<u32, _GPIO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CTRL;
#[doc = "`read()` method returns [gpio_ctrl::R](gpio_ctrl::R) reader structure"]
impl crate::Readable for GPIO_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio_ctrl::W](gpio_ctrl::W) writer structure"]
impl crate::Writable for GPIO_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_ctrl;
