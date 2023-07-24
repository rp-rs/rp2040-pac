#[doc = "Register `UARTPERIPHID3` reader"]
pub struct R(crate::R<UARTPERIPHID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTPERIPHID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTPERIPHID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTPERIPHID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONFIGURATION` reader - These bits read back as 0x00"]
pub type CONFIGURATION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x00"]
    #[inline(always)]
    pub fn configuration(&self) -> CONFIGURATION_R {
        CONFIGURATION_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPeriphID3 Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartperiphid3](index.html) module"]
pub struct UARTPERIPHID3_SPEC;
impl crate::RegisterSpec for UARTPERIPHID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartperiphid3::R](R) reader structure"]
impl crate::Readable for UARTPERIPHID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UARTPERIPHID3 to value 0"]
impl crate::Resettable for UARTPERIPHID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
