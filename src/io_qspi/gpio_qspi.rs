#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi__status](gpio_qspi__status) module"]
pub type GPIO_QSPI__STATUS = crate::Reg<u32, _GPIO_QSPI__STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI__STATUS;
#[doc = "`read()` method returns [gpio_qspi__status::R](gpio_qspi__status::R) reader structure"]
impl crate::Readable for GPIO_QSPI__STATUS {}
#[doc = "GPIO status"]
pub mod gpio_qspi__status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi__ctrl](gpio_qspi__ctrl) module"]
pub type GPIO_QSPI__CTRL = crate::Reg<u32, _GPIO_QSPI__CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI__CTRL;
#[doc = "`read()` method returns [gpio_qspi__ctrl::R](gpio_qspi__ctrl::R) reader structure"]
impl crate::Readable for GPIO_QSPI__CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi__ctrl::W](gpio_qspi__ctrl::W) writer structure"]
impl crate::Writable for GPIO_QSPI__CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi__ctrl;
