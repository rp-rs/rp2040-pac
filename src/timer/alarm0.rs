#[doc = "Register `ALARM0` reader"]
pub type R = crate::R<ALARM0_SPEC>;
#[doc = "Register `ALARM0` writer"]
pub type W = crate::W<ALARM0_SPEC>;
#[doc = "Field `ALARM0` reader - "]
pub type ALARM0_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM0` writer - "]
pub type ALARM0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn alarm0(&mut self) -> ALARM0_W<ALARM0_SPEC> {
        ALARM0_W::new(self, 0)
    }
}
#[doc = "Arm alarm 0, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM0 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.  

You can [`read`](crate::generic::Reg::read) this register and get [`alarm0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM0_SPEC;
impl crate::RegisterSpec for ALARM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0::R`](R) reader structure"]
impl crate::Readable for ALARM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm0::W`](W) writer structure"]
impl crate::Writable for ALARM0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARM0 to value 0"]
impl crate::Resettable for ALARM0_SPEC {
    const RESET_VALUE: u32 = 0;
}
