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
pub struct SETPEND_R(crate::FieldReader<u32, u32>);
impl SETPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SETPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETPEND_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
pub struct SETPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
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
    pub fn setpend(&mut self) -> SETPEND_W {
        SETPEND_W { w: self }
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
}
#[doc = "`reset()` method sets NVIC_ISPR to value 0"]
impl crate::Resettable for NVIC_ISPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
