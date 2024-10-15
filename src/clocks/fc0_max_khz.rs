#[doc = "Register `FC0_MAX_KHZ` reader"]
pub type R = crate::R<FC0_MAX_KHZ_SPEC>;
#[doc = "Register `FC0_MAX_KHZ` writer"]
pub type W = crate::W<FC0_MAX_KHZ_SPEC>;
#[doc = "Field `FC0_MAX_KHZ` reader - "]
pub type FC0_MAX_KHZ_R = crate::FieldReader<u32>;
#[doc = "Field `FC0_MAX_KHZ` writer - "]
pub type FC0_MAX_KHZ_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn fc0_max_khz(&self) -> FC0_MAX_KHZ_R {
        FC0_MAX_KHZ_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_max_khz(&mut self) -> FC0_MAX_KHZ_W<FC0_MAX_KHZ_SPEC> {
        FC0_MAX_KHZ_W::new(self, 0)
    }
}
#[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags  

You can [`read`](crate::Reg::read) this register and get [`fc0_max_khz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc0_max_khz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC0_MAX_KHZ_SPEC;
impl crate::RegisterSpec for FC0_MAX_KHZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc0_max_khz::R`](R) reader structure"]
impl crate::Readable for FC0_MAX_KHZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc0_max_khz::W`](W) writer structure"]
impl crate::Writable for FC0_MAX_KHZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FC0_MAX_KHZ to value 0x01ff_ffff"]
impl crate::Resettable for FC0_MAX_KHZ_SPEC {
    const RESET_VALUE: u32 = 0x01ff_ffff;
}
