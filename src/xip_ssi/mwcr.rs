#[doc = "Register `MWCR` reader"]
pub struct R(crate::R<MWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MWCR` writer"]
pub struct W(crate::W<MWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MWCR_SPEC>;
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
impl From<crate::W<MWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MWMOD` reader - Microwire transfer mode"]
pub type MWMOD_R = crate::BitReader<bool>;
#[doc = "Field `MWMOD` writer - Microwire transfer mode"]
pub type MWMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MWCR_SPEC, bool, O>;
#[doc = "Field `MDD` reader - Microwire control"]
pub type MDD_R = crate::BitReader<bool>;
#[doc = "Field `MDD` writer - Microwire control"]
pub type MDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MWCR_SPEC, bool, O>;
#[doc = "Field `MHS` reader - Microwire handshaking"]
pub type MHS_R = crate::BitReader<bool>;
#[doc = "Field `MHS` writer - Microwire handshaking"]
pub type MHS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MWCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Microwire transfer mode"]
    #[inline(always)]
    pub fn mwmod(&self) -> MWMOD_R {
        MWMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Microwire control"]
    #[inline(always)]
    pub fn mdd(&self) -> MDD_R {
        MDD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Microwire handshaking"]
    #[inline(always)]
    pub fn mhs(&self) -> MHS_R {
        MHS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Microwire transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn mwmod(&mut self) -> MWMOD_W<0> {
        MWMOD_W::new(self)
    }
    #[doc = "Bit 1 - Microwire control"]
    #[inline(always)]
    #[must_use]
    pub fn mdd(&mut self) -> MDD_W<1> {
        MDD_W::new(self)
    }
    #[doc = "Bit 2 - Microwire handshaking"]
    #[inline(always)]
    #[must_use]
    pub fn mhs(&mut self) -> MHS_W<2> {
        MHS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Microwire Control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [mwcr](index.html) module"]
pub struct MWCR_SPEC;
impl crate::RegisterSpec for MWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mwcr::R](R) reader structure"]
impl crate::Readable for MWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mwcr::W](W) writer structure"]
impl crate::Writable for MWCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MWCR to value 0"]
impl crate::Resettable for MWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
