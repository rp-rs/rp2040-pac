#[doc = "Register `FIFO_LEVELS` reader"]
pub struct R(crate::R<FIFO_LEVELS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_LEVELS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_LEVELS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_LEVELS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TDF_LVL` reader - Current Transfer-Data-FIFO fill level"]
pub type TDF_LVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAF_LVL` reader - Current Write-Address-FIFO fill level"]
pub type WAF_LVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAF_LVL` reader - Current Read-Address-FIFO fill level"]
pub type RAF_LVL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Current Transfer-Data-FIFO fill level"]
    #[inline(always)]
    pub fn tdf_lvl(&self) -> TDF_LVL_R {
        TDF_LVL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Current Write-Address-FIFO fill level"]
    #[inline(always)]
    pub fn waf_lvl(&self) -> WAF_LVL_R {
        WAF_LVL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Current Read-Address-FIFO fill level"]
    #[inline(always)]
    pub fn raf_lvl(&self) -> RAF_LVL_R {
        RAF_LVL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Debug RAF, WAF, TDF levels  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fifo_levels](index.html) module"]
pub struct FIFO_LEVELS_SPEC;
impl crate::RegisterSpec for FIFO_LEVELS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_levels::R](R) reader structure"]
impl crate::Readable for FIFO_LEVELS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFO_LEVELS to value 0"]
impl crate::Resettable for FIFO_LEVELS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
