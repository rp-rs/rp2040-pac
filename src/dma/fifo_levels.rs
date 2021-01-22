#[doc = "Reader of register FIFO_LEVELS"]
pub type R = crate::R<u32, super::FIFO_LEVELS>;
#[doc = "Reader of field `RAF_LVL`"]
pub type RAF_LVL_R = crate::R<u8, u8>;
#[doc = "Reader of field `WAF_LVL`"]
pub type WAF_LVL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TDF_LVL`"]
pub type TDF_LVL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:23 - Current Read-Address-FIFO fill level"]
    #[inline(always)]
    pub fn raf_lvl(&self) -> RAF_LVL_R {
        RAF_LVL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Current Write-Address-FIFO fill level"]
    #[inline(always)]
    pub fn waf_lvl(&self) -> WAF_LVL_R {
        WAF_LVL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Current Transfer-Data-FIFO fill level"]
    #[inline(always)]
    pub fn tdf_lvl(&self) -> TDF_LVL_R {
        TDF_LVL_R::new((self.bits & 0xff) as u8)
    }
}
