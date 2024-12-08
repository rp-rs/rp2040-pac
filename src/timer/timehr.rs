#[doc = "Register `TIMEHR` reader"]
pub type R = crate::R<TIMEHR_SPEC>;
#[doc = "Register `TIMEHR` writer"]
pub type W = crate::W<TIMEHR_SPEC>;
#[doc = "Field `TIMEHR` reader - "]
pub type TIMEHR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timehr(&self) -> TIMEHR_R {
        TIMEHR_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read from bits 63:32 of time always read timelr before timehr  

You can [`read`](crate::generic::Reg::read) this register and get [`timehr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timehr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEHR_SPEC;
impl crate::RegisterSpec for TIMEHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timehr::R`](R) reader structure"]
impl crate::Readable for TIMEHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timehr::W`](W) writer structure"]
impl crate::Writable for TIMEHR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEHR to value 0"]
impl crate::Resettable for TIMEHR_SPEC {
    const RESET_VALUE: u32 = 0;
}
