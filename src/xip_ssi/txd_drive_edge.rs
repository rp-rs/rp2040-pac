#[doc = "Register `TXD_DRIVE_EDGE` reader"]
pub struct R(crate::R<TXD_DRIVE_EDGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXD_DRIVE_EDGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXD_DRIVE_EDGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXD_DRIVE_EDGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXD_DRIVE_EDGE` writer"]
pub struct W(crate::W<TXD_DRIVE_EDGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXD_DRIVE_EDGE_SPEC>;
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
impl From<crate::W<TXD_DRIVE_EDGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXD_DRIVE_EDGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDE` reader - TXD drive edge"]
pub struct TDE_R(crate::FieldReader<u8, u8>);
impl TDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDE` writer - TXD drive edge"]
pub struct TDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TXD drive edge"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TXD drive edge"]
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W {
        TDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX drive edge  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [txd_drive_edge](index.html) module"]
pub struct TXD_DRIVE_EDGE_SPEC;
impl crate::RegisterSpec for TXD_DRIVE_EDGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txd_drive_edge::R](R) reader structure"]
impl crate::Readable for TXD_DRIVE_EDGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txd_drive_edge::W](W) writer structure"]
impl crate::Writable for TXD_DRIVE_EDGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXD_DRIVE_EDGE to value 0"]
impl crate::Resettable for TXD_DRIVE_EDGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
