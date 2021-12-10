#[doc = "Register `CLKDIV_M1` reader"]
pub struct R(crate::R<CLKDIV_M1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV_M1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV_M1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV_M1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV_M1` writer"]
pub struct W(crate::W<CLKDIV_M1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV_M1_SPEC>;
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
impl From<crate::W<CLKDIV_M1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV_M1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV_M1` reader - "]
pub struct CLKDIV_M1_R(crate::FieldReader<u16, u16>);
impl CLKDIV_M1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CLKDIV_M1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_M1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV_M1` writer - "]
pub struct CLKDIV_M1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_M1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn clkdiv_m1(&self) -> CLKDIV_M1_R {
        CLKDIV_M1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn clkdiv_m1(&mut self) -> CLKDIV_M1_W {
        CLKDIV_M1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [clkdiv_m1](index.html) module"]
pub struct CLKDIV_M1_SPEC;
impl crate::RegisterSpec for CLKDIV_M1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv_m1::R](R) reader structure"]
impl crate::Readable for CLKDIV_M1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv_m1::W](W) writer structure"]
impl crate::Writable for CLKDIV_M1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKDIV_M1 to value 0"]
impl crate::Resettable for CLKDIV_M1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
