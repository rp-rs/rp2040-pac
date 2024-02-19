#[doc = "Register `REASON` reader"]
pub type R = crate::R<REASON_SPEC>;
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
#[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset.  

You can [`read`](crate::generic::Reg::read) this register and get [`reason::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REASON_SPEC;
impl crate::RegisterSpec for REASON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reason::R`](R) reader structure"]
impl crate::Readable for REASON_SPEC {}
#[doc = "`reset()` method sets REASON to value 0"]
impl crate::Resettable for REASON_SPEC {
    const RESET_VALUE: u32 = 0;
}
