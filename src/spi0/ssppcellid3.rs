#[doc = "Register `SSPPCELLID3` reader"]
pub struct R(crate::R<SSPPCELLID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPPCELLID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPPCELLID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPPCELLID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SSPPCELLID3` reader - These bits read back as 0xB1"]
pub struct SSPPCELLID3_R(crate::FieldReader<u8, u8>);
impl SSPPCELLID3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSPPCELLID3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSPPCELLID3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xB1"]
    #[inline(always)]
    pub fn ssppcellid3(&self) -> SSPPCELLID3_R {
        SSPPCELLID3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ssppcellid3](index.html) module"]
pub struct SSPPCELLID3_SPEC;
impl crate::RegisterSpec for SSPPCELLID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssppcellid3::R](R) reader structure"]
impl crate::Readable for SSPPCELLID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPPCELLID3 to value 0xb1"]
impl crate::Resettable for SSPPCELLID3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb1
    }
}
