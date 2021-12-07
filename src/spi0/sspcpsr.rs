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
pub struct CPSDVSR_R(crate::FieldReader<u8, u8>);
impl CPSDVSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPSDVSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPSDVSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPSDVSR` writer - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
pub struct CPSDVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSDVSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
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
    pub fn cpsdvsr(&mut self) -> CPSDVSR_W {
        CPSDVSR_W { w: self }
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
}
#[doc = "`reset()` method sets SSPCPSR to value 0"]
impl crate::Resettable for SSPCPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
