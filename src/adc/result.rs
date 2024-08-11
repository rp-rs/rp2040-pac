#[doc = "Register `RESULT` reader"]
pub type R = crate::R<RESULT_SPEC>;
#[doc = "Register `RESULT` writer"]
pub type W = crate::W<RESULT_SPEC>;
#[doc = "Field `RESULT` reader - "]
pub type RESULT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {}
#[doc = "Result of most recent ADC conversion  

You can [`read`](crate::generic::Reg::read) this register and get [`result::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`result::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for RESULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`result::W`](W) writer structure"]
impl crate::Writable for RESULT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
