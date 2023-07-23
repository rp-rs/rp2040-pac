#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLUSH_READY` reader - Reads as 0 while a cache flush is in progress, and 1 otherwise.  
 The cache is flushed whenever the XIP block is reset, and also  
 when requested via the FLUSH register."]
pub type FLUSH_READY_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_EMPTY` reader - When 1, indicates the XIP streaming FIFO is completely empty."]
pub type FIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_FULL` reader - When 1, indicates the XIP streaming FIFO is completely full.  
 The streaming FIFO is 2 entries deep, so the full and empty  
 flag allow its level to be ascertained."]
pub type FIFO_FULL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Reads as 0 while a cache flush is in progress, and 1 otherwise.  
 The cache is flushed whenever the XIP block is reset, and also  
 when requested via the FLUSH register."]
    #[inline(always)]
    pub fn flush_ready(&self) -> FLUSH_READY_R {
        FLUSH_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, indicates the XIP streaming FIFO is completely empty."]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, indicates the XIP streaming FIFO is completely full.  
 The streaming FIFO is 2 entries deep, so the full and empty  
 flag allow its level to be ascertained."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Cache Status  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0x02"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
