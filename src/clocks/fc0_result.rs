#[doc = "Register `FC0_RESULT` reader"]
pub type R = crate::R<FC0_RESULT_SPEC>;
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
#[doc = "Result of frequency measurement, only valid when status_done=1  

You can [`read`](crate::Reg::read) this register and get [`fc0_result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC0_RESULT_SPEC;
impl crate::RegisterSpec for FC0_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc0_result::R`](R) reader structure"]
impl crate::Readable for FC0_RESULT_SPEC {}
#[doc = "`reset()` method sets FC0_RESULT to value 0"]
impl crate::Resettable for FC0_RESULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
