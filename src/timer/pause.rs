#[doc = "Register `PAUSE` reader"]
pub struct R(crate::R<PAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAUSE` writer"]
pub struct W(crate::W<PAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAUSE_SPEC>;
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
impl From<crate::W<PAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAUSE` reader - "]
pub type PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE` writer - "]
pub type PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAUSE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<0> {
        PAUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set high to pause the timer  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [pause](index.html) module"]
pub struct PAUSE_SPEC;
impl crate::RegisterSpec for PAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pause::R](R) reader structure"]
impl crate::Readable for PAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pause::W](W) writer structure"]
impl crate::Writable for PAUSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAUSE to value 0"]
impl crate::Resettable for PAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
