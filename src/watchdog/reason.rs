#[doc = "Register `REASON` reader"]
pub type R = crate::R<REASON_SPEC>;
#[doc = "Register `REASON` writer"]
pub type W = crate::W<REASON_SPEC>;
#[doc = "Field `TIMER` reader - "]
pub type TIMER_R = crate::BitReader;
#[doc = "Field `FORCE` reader - "]
pub type FORCE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn force(&self) -> FORCE_R {
        FORCE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset.  

You can [`read`](crate::generic::Reg::read) this register and get [`reason::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reason::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REASON_SPEC;
impl crate::RegisterSpec for REASON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reason::R`](R) reader structure"]
impl crate::Readable for REASON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reason::W`](W) writer structure"]
impl crate::Writable for REASON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REASON to value 0"]
impl crate::Resettable for REASON_SPEC {
    const RESET_VALUE: u32 = 0;
}
