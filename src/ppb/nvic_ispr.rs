#[doc = "Register `NVIC_ISPR` reader"]
pub struct R(crate::R<NVIC_ISPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ISPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ISPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ISPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ISPR` writer"]
pub struct W(crate::W<NVIC_ISPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ISPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NVIC_ISPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ISPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETPEND` reader - Interrupt set-pending bits.  
 Write:  
 0 = No effect.  
 1 = Changes interrupt state to pending.  
 Read:  
 0 = Interrupt is not pending.  
 1 = Interrupt is pending.  
 Note: Writing 1 to the NVIC_ISPR bit corresponding to:  
 An interrupt that is pending has no effect.  
 A disabled interrupt sets the state of that interrupt to pending."]
pub type SETPEND_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SETPEND` writer - Interrupt set-pending bits.  
 Write:  
 0 = No effect.  
 1 = Changes interrupt state to pending.  
 Read:  
 0 = Interrupt is not pending.  
 1 = Interrupt is pending.  
 Note: Writing 1 to the NVIC_ISPR bit corresponding to:  
 An interrupt that is pending has no effect.  
 A disabled interrupt sets the state of that interrupt to pending."]
pub type SETPEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_ISPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits.  
 Write:  
 0 = No effect.  
 1 = Changes interrupt state to pending.  
 Read:  
 0 = Interrupt is not pending.  
 1 = Interrupt is pending.  
 Note: Writing 1 to the NVIC_ISPR bit corresponding to:  
 An interrupt that is pending has no effect.  
 A disabled interrupt sets the state of that interrupt to pending."]
    #[inline(always)]
    pub fn setpend(&self) -> SETPEND_R {
        SETPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits.  
 Write:  
 0 = No effect.  
 1 = Changes interrupt state to pending.  
 Read:  
 0 = Interrupt is not pending.  
 1 = Interrupt is pending.  
 Note: Writing 1 to the NVIC_ISPR bit corresponding to:  
 An interrupt that is pending has no effect.  
 A disabled interrupt sets the state of that interrupt to pending."]
    #[inline(always)]
    #[must_use]
    pub fn setpend(&mut self) -> SETPEND_W<0> {
        SETPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [nvic_ispr](index.html) module"]
pub struct NVIC_ISPR_SPEC;
impl crate::RegisterSpec for NVIC_ISPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ispr::R](R) reader structure"]
impl crate::Readable for NVIC_ISPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ispr::W](W) writer structure"]
impl crate::Writable for NVIC_ISPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ISPR to value 0"]
impl crate::Resettable for NVIC_ISPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
