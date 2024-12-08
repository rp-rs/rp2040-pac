#[doc = "Register `TIMELR` reader"]
pub type R = crate::R<TIMELR_SPEC>;
#[doc = "Register `TIMELR` writer"]
pub type W = crate::W<TIMELR_SPEC>;
#[doc = "Field `TIMELR` reader -   

The field is **modified** in some way after a read operation."]
pub type TIMELR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timelr(&self) -> TIMELR_R {
        TIMELR_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read from bits 31:0 of time  

You can [`read`](crate::generic::Reg::read) this register and get [`timelr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timelr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMELR_SPEC;
impl crate::RegisterSpec for TIMELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timelr::R`](R) reader structure"]
impl crate::Readable for TIMELR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timelr::W`](W) writer structure"]
impl crate::Writable for TIMELR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMELR to value 0"]
impl crate::Resettable for TIMELR_SPEC {
    const RESET_VALUE: u32 = 0;
}
