#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
    pub ch: [CH; 8],
    #[doc = "0xa0 - This register aliases the CSR_EN bits for all channels.  
 Writing to this register allows multiple channels to be enabled  
 or disabled simultaneously, so they can run in perfect sync.  
 For each channel, there is only one physical EN register bit,  
 which can be accessed through here or CHx_CSR."]
    pub en: EN,
    #[doc = "0xa4 - Raw Interrupts"]
    pub intr: INTR,
    #[doc = "0xa8 - Interrupt Enable"]
    pub inte: INTE,
    #[doc = "0xac - Interrupt Force"]
    pub intf: INTF,
    #[doc = "0xb0 - Interrupt status after masking & forcing"]
    pub ints: INTS,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Control and status register"]
    pub csr: self::ch::CSR,
    #[doc = "0x04 - INT and FRAC form a fixed-point fractional number.  
 Counting rate is system clock frequency divided by this number.  
 Fractional division uses simple 1st-order sigma-delta."]
    pub div: self::ch::DIV,
    #[doc = "0x08 - Direct access to the PWM counter"]
    pub ctr: self::ch::CTR,
    #[doc = "0x0c - Counter compare values"]
    pub cc: self::ch::CC,
    #[doc = "0x10 - Counter wrap value"]
    pub top: self::ch::TOP,
}
#[doc = r"Register block"]
#[doc = "Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
pub mod ch;
#[doc = "This register aliases the CSR_EN bits for all channels.  
 Writing to this register allows multiple channels to be enabled  
 or disabled simultaneously, so they can run in perfect sync.  
 For each channel, there is only one physical EN register bit,  
 which can be accessed through here or CHx_CSR.  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [en](en) module"]
pub type EN = crate::Reg<u32, _EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN;
#[doc = "`read()` method returns [en::R](en::R) reader structure"]
impl crate::Readable for EN {}
#[doc = "`write(|w| ..)` method takes [en::W](en::W) writer structure"]
impl crate::Writable for EN {}
#[doc = "This register aliases the CSR_EN bits for all channels.  
 Writing to this register allows multiple channels to be enabled  
 or disabled simultaneously, so they can run in perfect sync.  
 For each channel, there is only one physical EN register bit,  
 which can be accessed through here or CHx_CSR."]
pub mod en;
#[doc = "Raw Interrupts  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "Interrupt Enable  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [inte](inte) module"]
pub type INTE = crate::Reg<u32, _INTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTE;
#[doc = "`read()` method returns [inte::R](inte::R) reader structure"]
impl crate::Readable for INTE {}
#[doc = "`write(|w| ..)` method takes [inte::W](inte::W) writer structure"]
impl crate::Writable for INTE {}
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "Interrupt Force  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [intf](intf) module"]
pub type INTF = crate::Reg<u32, _INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF;
#[doc = "`read()` method returns [intf::R](intf::R) reader structure"]
impl crate::Readable for INTF {}
#[doc = "`write(|w| ..)` method takes [intf::W](intf::W) writer structure"]
impl crate::Writable for INTF {}
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "Interrupt status after masking & forcing  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ints](ints) module"]
pub type INTS = crate::Reg<u32, _INTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTS;
#[doc = "`read()` method returns [ints::R](ints::R) reader structure"]
impl crate::Readable for INTS {}
#[doc = "Interrupt status after masking & forcing"]
pub mod ints;
