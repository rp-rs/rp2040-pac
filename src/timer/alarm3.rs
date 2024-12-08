#[doc = "Register `ALARM3` reader"]
pub type R = crate::R<ALARM3_SPEC>;
#[doc = "Register `ALARM3` writer"]
pub type W = crate::W<ALARM3_SPEC>;
#[doc = "Field `ALARM3` reader - "]
pub type ALARM3_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM3` writer - "]
pub type ALARM3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn alarm3(&self) -> ALARM3_R {
        ALARM3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn alarm3(&mut self) -> ALARM3_W<ALARM3_SPEC> {
        ALARM3_W::new(self, 0)
    }
}
#[doc = "Arm alarm 3, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM3 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.  

You can [`read`](crate::generic::Reg::read) this register and get [`alarm3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM3_SPEC;
impl crate::RegisterSpec for ALARM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm3::R`](R) reader structure"]
impl crate::Readable for ALARM3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm3::W`](W) writer structure"]
impl crate::Writable for ALARM3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARM3 to value 0"]
impl crate::Resettable for ALARM3_SPEC {
    const RESET_VALUE: u32 = 0;
}
