#[doc = "Register `UARTPERIPHID0` reader"]
pub struct R(crate::R<UARTPERIPHID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTPERIPHID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTPERIPHID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTPERIPHID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTNUMBER0` reader - These bits read back as 0x11"]
pub struct PARTNUMBER0_R(crate::FieldReader<u8, u8>);
impl PARTNUMBER0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PARTNUMBER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTNUMBER0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x11"]
    #[inline(always)]
    pub fn partnumber0(&self) -> PARTNUMBER0_R {
        PARTNUMBER0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPeriphID0 Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartperiphid0](index.html) module"]
pub struct UARTPERIPHID0_SPEC;
impl crate::RegisterSpec for UARTPERIPHID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartperiphid0::R](R) reader structure"]
impl crate::Readable for UARTPERIPHID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UARTPERIPHID0 to value 0x11"]
impl crate::Resettable for UARTPERIPHID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x11
    }
}
