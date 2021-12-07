#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_NOTLEAPYEAR` reader - If set, leapyear is forced off.  
 Useful for years divisible by 100 but not by 400"]
pub struct FORCE_NOTLEAPYEAR_R(crate::FieldReader<bool, bool>);
impl FORCE_NOTLEAPYEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_NOTLEAPYEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_NOTLEAPYEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_NOTLEAPYEAR` writer - If set, leapyear is forced off.  
 Useful for years divisible by 100 but not by 400"]
pub struct FORCE_NOTLEAPYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_NOTLEAPYEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `LOAD` reader - Load RTC"]
pub struct LOAD_R(crate::FieldReader<bool, bool>);
impl LOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOAD` writer - Load RTC"]
pub struct LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RTC_ACTIVE` reader - RTC enabled (running)"]
pub struct RTC_ACTIVE_R(crate::FieldReader<bool, bool>);
impl RTC_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ENABLE` reader - Enable RTC"]
pub struct RTC_ENABLE_R(crate::FieldReader<bool, bool>);
impl RTC_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ENABLE` writer - Enable RTC"]
pub struct RTC_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ENABLE_W<'a> {
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
    #[doc = "Bit 8 - If set, leapyear is forced off.  
 Useful for years divisible by 100 but not by 400"]
    #[inline(always)]
    pub fn force_notleapyear(&self) -> FORCE_NOTLEAPYEAR_R {
        FORCE_NOTLEAPYEAR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Load RTC"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC enabled (running)"]
    #[inline(always)]
    pub fn rtc_active(&self) -> RTC_ACTIVE_R {
        RTC_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable RTC"]
    #[inline(always)]
    pub fn rtc_enable(&self) -> RTC_ENABLE_R {
        RTC_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - If set, leapyear is forced off.  
 Useful for years divisible by 100 but not by 400"]
    #[inline(always)]
    pub fn force_notleapyear(&mut self) -> FORCE_NOTLEAPYEAR_W {
        FORCE_NOTLEAPYEAR_W { w: self }
    }
    #[doc = "Bit 4 - Load RTC"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W { w: self }
    }
    #[doc = "Bit 0 - Enable RTC"]
    #[inline(always)]
    pub fn rtc_enable(&mut self) -> RTC_ENABLE_W {
        RTC_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control and status  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
