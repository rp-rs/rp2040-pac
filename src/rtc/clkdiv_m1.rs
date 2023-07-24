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
pub type CLKDIV_M1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLKDIV_M1` writer - "]
pub type CLKDIV_M1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKDIV_M1_SPEC, u16, u16, 16, O>;
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
    #[must_use]
    pub fn clkdiv_m1(&mut self) -> CLKDIV_M1_W<0> {
        CLKDIV_M1_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKDIV_M1 to value 0"]
impl crate::Resettable for CLKDIV_M1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
