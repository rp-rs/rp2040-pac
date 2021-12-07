#[doc = "Register `SSPPCELLID0` reader"]
pub struct R(crate::R<SSPPCELLID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPPCELLID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPPCELLID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPPCELLID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SSPPCELLID0` reader - These bits read back as 0x0D"]
pub struct SSPPCELLID0_R(crate::FieldReader<u8, u8>);
impl SSPPCELLID0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSPPCELLID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSPPCELLID0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x0D"]
    #[inline(always)]
    pub fn ssppcellid0(&self) -> SSPPCELLID0_R {
        SSPPCELLID0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ssppcellid0](index.html) module"]
pub struct SSPPCELLID0_SPEC;
impl crate::RegisterSpec for SSPPCELLID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssppcellid0::R](R) reader structure"]
impl crate::Readable for SSPPCELLID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPPCELLID0 to value 0x0d"]
impl crate::Resettable for SSPPCELLID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d
    }
}
