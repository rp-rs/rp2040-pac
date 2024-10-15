#[doc = "Register `PAUSE` reader"]
pub type R = crate::R<PAUSE_SPEC>;
#[doc = "Register `PAUSE` writer"]
pub type W = crate::W<PAUSE_SPEC>;
#[doc = "Field `PAUSE` reader - "]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `PAUSE` writer - "]
pub type PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<PAUSE_SPEC> {
        PAUSE_W::new(self, 0)
    }
}
#[doc = "Set high to pause the timer  

You can [`read`](crate::Reg::read) this register and get [`pause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAUSE_SPEC;
impl crate::RegisterSpec for PAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pause::R`](R) reader structure"]
impl crate::Readable for PAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pause::W`](W) writer structure"]
impl crate::Writable for PAUSE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAUSE to value 0"]
impl crate::Resettable for PAUSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
