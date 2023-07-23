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
#[doc = "Field `RTC_ENABLE` reader - Enable RTC"]
pub type RTC_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_ENABLE` writer - Enable RTC"]
pub type RTC_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RTC_ACTIVE` reader - RTC enabled (running)"]
pub type RTC_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `LOAD` reader - Load RTC"]
pub type LOAD_R = crate::BitReader<bool>;
#[doc = "Field `LOAD` writer - Load RTC"]
pub type LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FORCE_NOTLEAPYEAR` reader - If set, leapyear is forced off.  
 Useful for years divisible by 100 but not by 400"]
pub type FORCE_NOTLEAPYEAR_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_NOTLEAPYEAR` writer - If set, leapyear is forced off.  
 Useful for years divisible by 100 but not by 400"]
pub type FORCE_NOTLEAPYEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable RTC"]
    #[inline(always)]
    pub fn rtc_enable(&self) -> RTC_ENABLE_R {
        RTC_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC enabled (running)"]
    #[inline(always)]
    pub fn rtc_active(&self) -> RTC_ACTIVE_R {
        RTC_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Load RTC"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - If set, leapyear is forced off.  
 Useful for years divisible by 100 but not by 400"]
    #[inline(always)]
    pub fn force_notleapyear(&self) -> FORCE_NOTLEAPYEAR_R {
        FORCE_NOTLEAPYEAR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_enable(&mut self) -> RTC_ENABLE_W<0> {
        RTC_ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Load RTC"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<4> {
        LOAD_W::new(self)
    }
    #[doc = "Bit 8 - If set, leapyear is forced off.  
 Useful for years divisible by 100 but not by 400"]
    #[inline(always)]
    #[must_use]
    pub fn force_notleapyear(&mut self) -> FORCE_NOTLEAPYEAR_W<8> {
        FORCE_NOTLEAPYEAR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
