#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crystal Oscillator Control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Crystal Oscillator Status"]
    pub status: STATUS,
    #[doc = "0x08 - Crystal Oscillator pause control  
 This is used to save power by pausing the XOSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 WARNING: stop the PLLs before selecting dormant mode  
 WARNING: setup the irq before selecting dormant mode"]
    pub dormant: DORMANT,
    #[doc = "0x0c - Controls the startup delay"]
    pub startup: STARTUP,
}
#[doc = "Crystal Oscillator Control  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Crystal Oscillator Control"]
pub mod ctrl;
#[doc = "Crystal Oscillator Status  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Crystal Oscillator Status"]
pub mod status;
#[doc = "Crystal Oscillator pause control  
 This is used to save power by pausing the XOSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 WARNING: stop the PLLs before selecting dormant mode  
 WARNING: setup the irq before selecting dormant mode  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dormant](dormant) module"]
pub type DORMANT = crate::Reg<u32, _DORMANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT;
#[doc = "`read()` method returns [dormant::R](dormant::R) reader structure"]
impl crate::Readable for DORMANT {}
#[doc = "`write(|w| ..)` method takes [dormant::W](dormant::W) writer structure"]
impl crate::Writable for DORMANT {}
#[doc = "Crystal Oscillator pause control  
 This is used to save power by pausing the XOSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 WARNING: stop the PLLs before selecting dormant mode  
 WARNING: setup the irq before selecting dormant mode"]
pub mod dormant;
#[doc = "Controls the startup delay  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [startup](startup) module"]
pub type STARTUP = crate::Reg<u32, _STARTUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTUP;
#[doc = "`read()` method returns [startup::R](startup::R) reader structure"]
impl crate::Readable for STARTUP {}
#[doc = "`write(|w| ..)` method takes [startup::W](startup::W) writer structure"]
impl crate::Writable for STARTUP {}
#[doc = "Controls the startup delay"]
pub mod startup;
