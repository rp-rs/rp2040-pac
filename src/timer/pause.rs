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
pub struct PAUSE_R(crate::FieldReader<bool, bool>);
impl PAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAUSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAUSE` writer - "]
pub struct PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pause(&mut self) -> PAUSE_W {
        PAUSE_W { w: self }
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
}
#[doc = "`reset()` method sets PAUSE to value 0"]
impl crate::Resettable for PAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
