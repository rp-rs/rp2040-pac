#[doc = "Register `MSTICR` reader"]
pub type R = crate::R<MSTICR_SPEC>;
#[doc = "Field `MSTICR` reader - Clear-on-read multi-master contention interrupt"]
pub type MSTICR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear-on-read multi-master contention interrupt"]
    #[inline(always)]
    pub fn msticr(&self) -> MSTICR_R {
        MSTICR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Multi-master interrupt clear  

You can [`read`](crate::generic::Reg::read) this register and get [`msticr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSTICR_SPEC;
impl crate::RegisterSpec for MSTICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msticr::R`](R) reader structure"]
impl crate::Readable for MSTICR_SPEC {}
#[doc = "`reset()` method sets MSTICR to value 0"]
impl crate::Resettable for MSTICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
