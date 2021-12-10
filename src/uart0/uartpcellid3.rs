#[doc = "Register `UARTPCELLID3` reader"]
pub struct R(crate::R<UARTPCELLID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTPCELLID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTPCELLID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTPCELLID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UARTPCELLID3` reader - These bits read back as 0xB1"]
pub struct UARTPCELLID3_R(crate::FieldReader<u8, u8>);
impl UARTPCELLID3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UARTPCELLID3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UARTPCELLID3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xB1"]
    #[inline(always)]
    pub fn uartpcellid3(&self) -> UARTPCELLID3_R {
        UARTPCELLID3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPCellID3 Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartpcellid3](index.html) module"]
pub struct UARTPCELLID3_SPEC;
impl crate::RegisterSpec for UARTPCELLID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartpcellid3::R](R) reader structure"]
impl crate::Readable for UARTPCELLID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UARTPCELLID3 to value 0xb1"]
impl crate::Resettable for UARTPCELLID3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb1
    }
}
