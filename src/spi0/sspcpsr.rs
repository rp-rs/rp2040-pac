#[doc = "Register `SSPCPSR` reader"]
pub struct R(crate::R<SSPCPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPCPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPCPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPCPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPCPSR` writer"]
pub struct W(crate::W<SSPCPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPCPSR_SPEC>;
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
impl From<crate::W<SSPCPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPCPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPSDVSR` reader - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
pub type CPSDVSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPSDVSR` writer - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
pub type CPSDVSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSPCPSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CPSDVSR_R {
        CPSDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    #[inline(always)]
    #[must_use]
    pub fn cpsdvsr(&mut self) -> CPSDVSR_W<0> {
        CPSDVSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock prescale register, SSPCPSR on page 3-8  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspcpsr](index.html) module"]
pub struct SSPCPSR_SPEC;
impl crate::RegisterSpec for SSPCPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspcpsr::R](R) reader structure"]
impl crate::Readable for SSPCPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspcpsr::W](W) writer structure"]
impl crate::Writable for SSPCPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSPCPSR to value 0"]
impl crate::Resettable for SSPCPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
