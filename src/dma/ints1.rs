#[doc = "Register `INTS1` reader"]
pub struct R(crate::R<INTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTS1` writer"]
pub struct W(crate::W<INTS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTS1_SPEC>;
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
impl From<crate::W<INTS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTS1` reader - Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted.  
 Channel interrupts can be cleared by writing a bit mask here."]
pub type INTS1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INTS1` writer - Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted.  
 Channel interrupts can be cleared by writing a bit mask here."]
pub type INTS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTS1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted.  
 Channel interrupts can be cleared by writing a bit mask here."]
    #[inline(always)]
    pub fn ints1(&self) -> INTS1_R {
        INTS1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted.  
 Channel interrupts can be cleared by writing a bit mask here."]
    #[inline(always)]
    #[must_use]
    pub fn ints1(&mut self) -> INTS1_W<0> {
        INTS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status (masked) for IRQ 1  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ints1](index.html) module"]
pub struct INTS1_SPEC;
impl crate::RegisterSpec for INTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ints1::R](R) reader structure"]
impl crate::Readable for INTS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ints1::W](W) writer structure"]
impl crate::Writable for INTS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff;
}
#[doc = "`reset()` method sets INTS1 to value 0"]
impl crate::Resettable for INTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
