#[doc = "Register `NVIC_ICER` reader"]
pub struct R(crate::R<NVIC_ICER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ICER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ICER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ICER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ICER` writer"]
pub struct W(crate::W<NVIC_ICER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ICER_SPEC>;
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
impl From<crate::W<NVIC_ICER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ICER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRENA` reader - Interrupt clear-enable bits.  
 Write:  
 0 = No effect.  
 1 = Disable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
pub struct CLRENA_R(crate::FieldReader<u32, u32>);
impl CLRENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CLRENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA` writer - Interrupt clear-enable bits.  
 Write:  
 0 = No effect.  
 1 = Disable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
pub struct CLRENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits.  
 Write:  
 0 = No effect.  
 1 = Disable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
    #[inline(always)]
    pub fn clrena(&self) -> CLRENA_R {
        CLRENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits.  
 Write:  
 0 = No effect.  
 1 = Disable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
    #[inline(always)]
    pub fn clrena(&mut self) -> CLRENA_W {
        CLRENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [nvic_icer](index.html) module"]
pub struct NVIC_ICER_SPEC;
impl crate::RegisterSpec for NVIC_ICER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_icer::R](R) reader structure"]
impl crate::Readable for NVIC_ICER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_icer::W](W) writer structure"]
impl crate::Writable for NVIC_ICER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_ICER to value 0"]
impl crate::Resettable for NVIC_ICER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
