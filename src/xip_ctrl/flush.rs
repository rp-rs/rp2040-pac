#[doc = "Register `FLUSH` reader"]
pub type R = crate::R<FLUSH_SPEC>;
#[doc = "Register `FLUSH` writer"]
pub type W = crate::W<FLUSH_SPEC>;
#[doc = "Field `FLUSH` writer - Write 1 to flush the cache. This clears the tag memory, but the data memory retains its contents. (This means cache-as-SRAM contents is not affected by flush or reset.) Reading will hold the bus (stall the processor) until the flush completes. Alternatively STAT can be polled until completion."]
pub type FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to flush the cache. This clears the tag memory, but the data memory retains its contents. (This means cache-as-SRAM contents is not affected by flush or reset.) Reading will hold the bus (stall the processor) until the flush completes. Alternatively STAT can be polled until completion."]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<FLUSH_SPEC> {
        FLUSH_W::new(self, 0)
    }
}
#[doc = "Cache Flush control  

You can [`read`](crate::generic::Reg::read) this register and get [`flush::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flush::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLUSH_SPEC;
impl crate::RegisterSpec for FLUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flush::R`](R) reader structure"]
impl crate::Readable for FLUSH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flush::W`](W) writer structure"]
impl crate::Writable for FLUSH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLUSH to value 0"]
impl crate::Resettable for FLUSH_SPEC {
    const RESET_VALUE: u32 = 0;
}
