#[doc = "Register `TIMERAWH` reader"]
pub type R = crate::R<TIMERAWH_SPEC>;
#[doc = "Register `TIMERAWH` writer"]
pub type W = crate::W<TIMERAWH_SPEC>;
#[doc = "Field `TIMERAWH` reader - "]
pub type TIMERAWH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timerawh(&self) -> TIMERAWH_R {
        TIMERAWH_R::new(self.bits)
    }
}
impl W {}
#[doc = "Raw read from bits 63:32 of time (no side effects)  

You can [`read`](crate::generic::Reg::read) this register and get [`timerawh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timerawh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERAWH_SPEC;
impl crate::RegisterSpec for TIMERAWH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timerawh::R`](R) reader structure"]
impl crate::Readable for TIMERAWH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timerawh::W`](W) writer structure"]
impl crate::Writable for TIMERAWH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMERAWH to value 0"]
impl crate::Resettable for TIMERAWH_SPEC {
    const RESET_VALUE: u32 = 0;
}
