#[doc = "Register `SETUP_1` reader"]
pub type R = crate::R<SETUP_1_SPEC>;
#[doc = "Register `SETUP_1` writer"]
pub type W = crate::W<SETUP_1_SPEC>;
#[doc = "Field `SEC` reader - Seconds"]
pub type SEC_R = crate::FieldReader;
#[doc = "Field `SEC` writer - Seconds"]
pub type SEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `MIN` reader - Minutes"]
pub type MIN_R = crate::FieldReader;
#[doc = "Field `MIN` writer - Minutes"]
pub type MIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `HOUR` reader - Hours"]
pub type HOUR_R = crate::FieldReader;
#[doc = "Field `HOUR` writer - Hours"]
pub type HOUR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DOTW` reader - Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
pub type DOTW_R = crate::FieldReader;
#[doc = "Field `DOTW` writer - Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
pub type DOTW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
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
    pub fn sec(&mut self) -> SEC_W<SETUP_1_SPEC, 0> {
        SEC_W::new(self)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<SETUP_1_SPEC, 8> {
        MIN_W::new(self)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<SETUP_1_SPEC, 16> {
        HOUR_W::new(self)
    }
    #[doc = "Bits 24:26 - Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
    #[inline(always)]
    #[must_use]
    pub fn dotw(&mut self) -> DOTW_W<SETUP_1_SPEC, 24> {
        DOTW_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC setup register 1  

You can [`read`](crate::generic::Reg::read) this register and get [`setup_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SETUP_1_SPEC;
impl crate::RegisterSpec for SETUP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setup_1::R`](R) reader structure"]
impl crate::Readable for SETUP_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`setup_1::W`](W) writer structure"]
impl crate::Writable for SETUP_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETUP_1 to value 0"]
impl crate::Resettable for SETUP_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
