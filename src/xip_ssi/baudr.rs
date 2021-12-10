#[doc = "Register `BAUDR` reader"]
pub struct R(crate::R<BAUDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUDR` writer"]
pub struct W(crate::W<BAUDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUDR_SPEC>;
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
impl From<crate::W<BAUDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCKDV` reader - SSI clock divider"]
pub struct SCKDV_R(crate::FieldReader<u16, u16>);
impl SCKDV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SCKDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCKDV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCKDV` writer - SSI clock divider"]
pub struct SCKDV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKDV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - SSI clock divider"]
    #[inline(always)]
    pub fn sckdv(&self) -> SCKDV_R {
        SCKDV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SSI clock divider"]
    #[inline(always)]
    pub fn sckdv(&mut self) -> SCKDV_W {
        SCKDV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud rate  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [baudr](index.html) module"]
pub struct BAUDR_SPEC;
impl crate::RegisterSpec for BAUDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baudr::R](R) reader structure"]
impl crate::Readable for BAUDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baudr::W](W) writer structure"]
impl crate::Writable for BAUDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUDR to value 0"]
impl crate::Resettable for BAUDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
