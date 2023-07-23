#[doc = "Register `SSPICR` reader"]
pub struct R(crate::R<SSPICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPICR` writer"]
pub struct W(crate::W<SSPICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPICR_SPEC>;
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
impl From<crate::W<SSPICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RORIC` reader - Clears the SSPRORINTR interrupt"]
pub type RORIC_R = crate::BitReader<bool>;
#[doc = "Field `RORIC` writer - Clears the SSPRORINTR interrupt"]
pub type RORIC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SSPICR_SPEC, bool, O>;
#[doc = "Field `RTIC` reader - Clears the SSPRTINTR interrupt"]
pub type RTIC_R = crate::BitReader<bool>;
#[doc = "Field `RTIC` writer - Clears the SSPRTINTR interrupt"]
pub type RTIC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SSPICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clears the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn roric(&self) -> RORIC_R {
        RORIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clears the SSPRORINTR interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn roric(&mut self) -> RORIC_W<0> {
        RORIC_W::new(self)
    }
    #[doc = "Bit 1 - Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RTIC_W<1> {
        RTIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear register, SSPICR on page 3-11  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspicr](index.html) module"]
pub struct SSPICR_SPEC;
impl crate::RegisterSpec for SSPICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspicr::R](R) reader structure"]
impl crate::Readable for SSPICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspicr::W](W) writer structure"]
impl crate::Writable for SSPICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
}
#[doc = "`reset()` method sets SSPICR to value 0"]
impl crate::Resettable for SSPICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
