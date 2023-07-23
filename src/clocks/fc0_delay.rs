#[doc = "Register `FC0_DELAY` reader"]
pub struct R(crate::R<FC0_DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FC0_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FC0_DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FC0_DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FC0_DELAY` writer"]
pub struct W(crate::W<FC0_DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FC0_DELAY_SPEC>;
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
impl From<crate::W<FC0_DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FC0_DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FC0_DELAY` reader - "]
pub type FC0_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FC0_DELAY` writer - "]
pub type FC0_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FC0_DELAY_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn fc0_delay(&self) -> FC0_DELAY_R {
        FC0_DELAY_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_delay(&mut self) -> FC0_DELAY_W<0> {
        FC0_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delays the start of frequency counting to allow the mux to settle  
 Delay is measured in multiples of the reference clock period  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fc0_delay](index.html) module"]
pub struct FC0_DELAY_SPEC;
impl crate::RegisterSpec for FC0_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fc0_delay::R](R) reader structure"]
impl crate::Readable for FC0_DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fc0_delay::W](W) writer structure"]
impl crate::Writable for FC0_DELAY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FC0_DELAY to value 0x01"]
impl crate::Resettable for FC0_DELAY_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
