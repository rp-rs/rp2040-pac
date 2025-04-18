#[doc = "Register `MSTICR` reader"]
pub type R = crate::R<MSTICR_SPEC>;
#[doc = "Register `MSTICR` writer"]
pub type W = crate::W<MSTICR_SPEC>;
#[doc = "Field `MSTICR` reader - Clear-on-read multi-master contention interrupt"]
pub type MSTICR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear-on-read multi-master contention interrupt"]
    #[inline(always)]
    pub fn msticr(&self) -> MSTICR_R {
        MSTICR_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Multi-master interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`msticr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msticr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSTICR_SPEC;
impl crate::RegisterSpec for MSTICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msticr::R`](R) reader structure"]
impl crate::Readable for MSTICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msticr::W`](W) writer structure"]
impl crate::Writable for MSTICR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTICR to value 0"]
impl crate::Resettable for MSTICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
