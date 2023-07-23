#[doc = "Register `SETUP_1` reader"]
pub struct R(crate::R<SETUP_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP_1` writer"]
pub struct W(crate::W<SETUP_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP_1_SPEC>;
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
impl From<crate::W<SETUP_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - Seconds"]
pub type SEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC` writer - Seconds"]
pub type SEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `MIN` reader - Minutes"]
pub type MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN` writer - Minutes"]
pub type MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `HOUR` reader - Hours"]
pub type HOUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOUR` writer - Hours"]
pub type HOUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DOTW` reader - Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
pub type DOTW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOTW` writer - Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
pub type DOTW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
    #[inline(always)]
    pub fn dotw(&self) -> DOTW_R {
        DOTW_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<0> {
        SEC_W::new(self)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<8> {
        MIN_W::new(self)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<16> {
        HOUR_W::new(self)
    }
    #[doc = "Bits 24:26 - Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
    #[inline(always)]
    #[must_use]
    pub fn dotw(&mut self) -> DOTW_W<24> {
        DOTW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC setup register 1  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [setup_1](index.html) module"]
pub struct SETUP_1_SPEC;
impl crate::RegisterSpec for SETUP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup_1::R](R) reader structure"]
impl crate::Readable for SETUP_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup_1::W](W) writer structure"]
impl crate::Writable for SETUP_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETUP_1 to value 0"]
impl crate::Resettable for SETUP_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
