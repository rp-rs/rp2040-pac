#[doc = "Register `FC0_RESULT` reader"]
pub type R = crate::R<FC0_RESULT_SPEC>;
#[doc = "Register `FC0_RESULT` writer"]
pub type W = crate::W<FC0_RESULT_SPEC>;
#[doc = "Field `FRAC` reader - "]
pub type FRAC_R = crate::FieldReader;
#[doc = "Field `KHZ` reader - "]
pub type KHZ_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:29"]
    #[inline(always)]
    pub fn khz(&self) -> KHZ_R {
        KHZ_R::new((self.bits >> 5) & 0x01ff_ffff)
    }
}
impl W {}
#[doc = "Result of frequency measurement, only valid when status_done=1  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_result::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_result::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC0_RESULT_SPEC;
impl crate::RegisterSpec for FC0_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc0_result::R`](R) reader structure"]
impl crate::Readable for FC0_RESULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc0_result::W`](W) writer structure"]
impl crate::Writable for FC0_RESULT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FC0_RESULT to value 0"]
impl crate::Resettable for FC0_RESULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
