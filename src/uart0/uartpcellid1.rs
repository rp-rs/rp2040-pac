#[doc = "Register `UARTPCELLID1` reader"]
pub struct R(crate::R<UARTPCELLID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTPCELLID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTPCELLID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTPCELLID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UARTPCELLID1` reader - These bits read back as 0xF0"]
pub struct UARTPCELLID1_R(crate::FieldReader<u8, u8>);
impl UARTPCELLID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UARTPCELLID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UARTPCELLID1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xF0"]
    #[inline(always)]
    pub fn uartpcellid1(&self) -> UARTPCELLID1_R {
        UARTPCELLID1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPCellID1 Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartpcellid1](index.html) module"]
pub struct UARTPCELLID1_SPEC;
impl crate::RegisterSpec for UARTPCELLID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartpcellid1::R](R) reader structure"]
impl crate::Readable for UARTPCELLID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UARTPCELLID1 to value 0xf0"]
impl crate::Resettable for UARTPCELLID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
