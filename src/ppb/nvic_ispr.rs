#[doc = "Register `NVIC_ISPR` reader"]
pub type R = crate::R<NVIC_ISPR_SPEC>;
#[doc = "Register `NVIC_ISPR` writer"]
pub type W = crate::W<NVIC_ISPR_SPEC>;
#[doc = "Field `SETPEND` reader - Interrupt set-pending bits. Write: 0 = No effect. 1 = Changes interrupt state to pending. Read: 0 = Interrupt is not pending. 1 = Interrupt is pending. Note: Writing 1 to the NVIC_ISPR bit corresponding to: An interrupt that is pending has no effect. A disabled interrupt sets the state of that interrupt to pending."]
pub type SETPEND_R = crate::FieldReader<u32>;
#[doc = "Field `SETPEND` writer - Interrupt set-pending bits. Write: 0 = No effect. 1 = Changes interrupt state to pending. Read: 0 = Interrupt is not pending. 1 = Interrupt is pending. Note: Writing 1 to the NVIC_ISPR bit corresponding to: An interrupt that is pending has no effect. A disabled interrupt sets the state of that interrupt to pending."]
pub type SETPEND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits. Write: 0 = No effect. 1 = Changes interrupt state to pending. Read: 0 = Interrupt is not pending. 1 = Interrupt is pending. Note: Writing 1 to the NVIC_ISPR bit corresponding to: An interrupt that is pending has no effect. A disabled interrupt sets the state of that interrupt to pending."]
    #[inline(always)]
    pub fn setpend(&self) -> SETPEND_R {
        SETPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits. Write: 0 = No effect. 1 = Changes interrupt state to pending. Read: 0 = Interrupt is not pending. 1 = Interrupt is pending. Note: Writing 1 to the NVIC_ISPR bit corresponding to: An interrupt that is pending has no effect. A disabled interrupt sets the state of that interrupt to pending."]
    #[inline(always)]
    #[must_use]
    pub fn setpend(&mut self) -> SETPEND_W<NVIC_ISPR_SPEC> {
        SETPEND_W::new(self, 0)
    }
}
#[doc = "The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ISPR_SPEC;
impl crate::RegisterSpec for NVIC_ISPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ispr::R`](R) reader structure"]
impl crate::Readable for NVIC_ISPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ispr::W`](W) writer structure"]
impl crate::Writable for NVIC_ISPR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ISPR to value 0"]
impl crate::Resettable for NVIC_ISPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
