#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `FLUSH_READY` reader - Reads as 0 while a cache flush is in progress, and 1 otherwise. The cache is flushed whenever the XIP block is reset, and also when requested via the FLUSH register."]
pub type FLUSH_READY_R = crate::BitReader;
#[doc = "Field `FIFO_EMPTY` reader - When 1, indicates the XIP streaming FIFO is completely empty."]
pub type FIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `FIFO_FULL` reader - When 1, indicates the XIP streaming FIFO is completely full. The streaming FIFO is 2 entries deep, so the full and empty flag allow its level to be ascertained."]
pub type FIFO_FULL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reads as 0 while a cache flush is in progress, and 1 otherwise. The cache is flushed whenever the XIP block is reset, and also when requested via the FLUSH register."]
    #[inline(always)]
    pub fn flush_ready(&self) -> FLUSH_READY_R {
        FLUSH_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, indicates the XIP streaming FIFO is completely empty."]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, indicates the XIP streaming FIFO is completely full. The streaming FIFO is 2 entries deep, so the full and empty flag allow its level to be ascertained."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {}
#[doc = "Cache Status  

You can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0x02"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
