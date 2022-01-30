#[doc = "Register `NVIC_ISER` reader"]
pub struct R(crate::R<NVIC_ISER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ISER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ISER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ISER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ISER` writer"]
pub struct W(crate::W<NVIC_ISER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ISER_SPEC>;
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
impl From<crate::W<NVIC_ISER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ISER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETENA` reader - Interrupt set-enable bits.  
 Write:  
 0 = No effect.  
 1 = Enable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
pub struct SETENA_R(crate::FieldReader<u32, u32>);
impl SETENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SETENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETENA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETENA` writer - Interrupt set-enable bits.  
 Write:  
 0 = No effect.  
 1 = Enable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
pub struct SETENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-enable bits.  
 Write:  
 0 = No effect.  
 1 = Enable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
    #[inline(always)]
    pub fn setena(&self) -> SETENA_R {
        SETENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-enable bits.  
 Write:  
 0 = No effect.  
 1 = Enable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
    #[inline(always)]
    pub fn setena(&mut self) -> SETENA_W {
        SETENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled.  
 If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [nvic_iser](index.html) module"]
pub struct NVIC_ISER_SPEC;
impl crate::RegisterSpec for NVIC_ISER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_iser::R](R) reader structure"]
impl crate::Readable for NVIC_ISER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_iser::W](W) writer structure"]
impl crate::Writable for NVIC_ISER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_ISER to value 0"]
impl crate::Resettable for NVIC_ISER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
