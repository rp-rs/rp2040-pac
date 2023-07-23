#[doc = "Register `SETUP_0` reader"]
pub struct R(crate::R<SETUP_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP_0` writer"]
pub struct W(crate::W<SETUP_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP_0_SPEC>;
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
impl From<crate::W<SETUP_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAY` reader - Day of the month (1..31)"]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAY` writer - Day of the month (1..31)"]
pub type DAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `MONTH` reader - Month (1..12)"]
pub type MONTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MONTH` writer - Month (1..12)"]
pub type MONTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `YEAR` reader - Year"]
pub type YEAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEAR` writer - Year"]
pub type YEAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_0_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<0> {
        DAY_W::new(self)
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MONTH_W<8> {
        MONTH_W::new(self)
    }
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<12> {
        YEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC setup register 0  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [setup_0](index.html) module"]
pub struct SETUP_0_SPEC;
impl crate::RegisterSpec for SETUP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup_0::R](R) reader structure"]
impl crate::Readable for SETUP_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup_0::W](W) writer structure"]
impl crate::Writable for SETUP_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETUP_0 to value 0"]
impl crate::Resettable for SETUP_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
