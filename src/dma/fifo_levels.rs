#[doc = "Register `FIFO_LEVELS` reader"]
pub type R = crate::R<FIFO_LEVELS_SPEC>;
#[doc = "Field `TDF_LVL` reader - Current Transfer-Data-FIFO fill level"]
pub type TDF_LVL_R = crate::FieldReader;
#[doc = "Field `WAF_LVL` reader - Current Write-Address-FIFO fill level"]
pub type WAF_LVL_R = crate::FieldReader;
#[doc = "Field `RAF_LVL` reader - Current Read-Address-FIFO fill level"]
pub type RAF_LVL_R = crate::FieldReader;
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

You can [`read`](crate::Reg::read) this register and get [`fifo_levels::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_LEVELS_SPEC;
impl crate::RegisterSpec for FIFO_LEVELS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_levels::R`](R) reader structure"]
impl crate::Readable for FIFO_LEVELS_SPEC {}
#[doc = "`reset()` method sets FIFO_LEVELS to value 0"]
impl crate::Resettable for FIFO_LEVELS_SPEC {
    const RESET_VALUE: u32 = 0;
}
