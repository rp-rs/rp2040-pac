#[doc = "Register `DIV` reader"]
pub struct R(crate::R<DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV` writer"]
pub struct W(crate::W<DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_SPEC>;
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
impl From<crate::W<DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAC` reader - Fractional part of clock divisor. First-order delta-sigma."]
pub type FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAC` writer - Fractional part of clock divisor. First-order delta-sigma."]
pub type FRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIV_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT` reader - Integer part of clock divisor."]
pub type INT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT` writer - Integer part of clock divisor."]
pub type INT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - Fractional part of clock divisor. First-order delta-sigma."]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Integer part of clock divisor."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fractional part of clock divisor. First-order delta-sigma."]
    #[inline(always)]
    #[must_use]
    pub fn frac(&mut self) -> FRAC_W<0> {
        FRAC_W::new(self)
    }
    #[doc = "Bits 8:23 - Integer part of clock divisor."]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<8> {
        INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions  
 at regular intervals rather than back-to-back.  
 The divider is reset when either of these fields are written.  
 Total period is 1 + INT + FRAC / 256  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [div](index.html) module"]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div::R](R) reader structure"]
impl crate::Readable for DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div::W](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
