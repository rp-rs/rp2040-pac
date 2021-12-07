#[doc = "Register `INTE` reader"]
pub struct R(crate::R<INTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTE` writer"]
pub struct W(crate::W<INTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTE_SPEC>;
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
impl From<crate::W<INTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC` reader - "]
pub struct RTC_R(crate::FieldReader<bool, bool>);
impl RTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC` writer - "]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
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
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [inte](index.html) module"]
pub struct INTE_SPEC;
impl crate::RegisterSpec for INTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inte::R](R) reader structure"]
impl crate::Readable for INTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inte::W](W) writer structure"]
impl crate::Writable for INTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTE to value 0"]
impl crate::Resettable for INTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
