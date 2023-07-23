#[doc = "Register `IDR` reader"]
pub struct R(crate::R<IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDCODE` reader - Peripheral dentification code"]
pub type IDCODE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral dentification code"]
    #[inline(always)]
    pub fn idcode(&self) -> IDCODE_R {
        IDCODE_R::new(self.bits)
    }
}
#[doc = "Identification register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idr::R](R) reader structure"]
impl crate::Readable for IDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDR to value 0x5153_5049"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x5153_5049;
}
