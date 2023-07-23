#[doc = "Register `FBDIV_INT` reader"]
pub struct R(crate::R<FBDIV_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBDIV_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBDIV_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBDIV_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBDIV_INT` writer"]
pub struct W(crate::W<FBDIV_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBDIV_INT_SPEC>;
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
impl From<crate::W<FBDIV_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBDIV_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBDIV_INT` reader - see ctrl reg description for constraints"]
pub type FBDIV_INT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FBDIV_INT` writer - see ctrl reg description for constraints"]
pub type FBDIV_INT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FBDIV_INT_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - see ctrl reg description for constraints"]
    #[inline(always)]
    pub fn fbdiv_int(&self) -> FBDIV_INT_R {
        FBDIV_INT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - see ctrl reg description for constraints"]
    #[inline(always)]
    #[must_use]
    pub fn fbdiv_int(&mut self) -> FBDIV_INT_W<0> {
        FBDIV_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Feedback divisor  
 (note: this PLL does not support fractional division)  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fbdiv_int](index.html) module"]
pub struct FBDIV_INT_SPEC;
impl crate::RegisterSpec for FBDIV_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbdiv_int::R](R) reader structure"]
impl crate::Readable for FBDIV_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbdiv_int::W](W) writer structure"]
impl crate::Writable for FBDIV_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBDIV_INT to value 0"]
impl crate::Resettable for FBDIV_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
