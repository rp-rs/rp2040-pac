#[doc = "Register `PROC_IN_SYNC_BYPASS_HI` reader"]
pub struct R(crate::R<PROC_IN_SYNC_BYPASS_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC_IN_SYNC_BYPASS_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC_IN_SYNC_BYPASS_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC_IN_SYNC_BYPASS_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROC_IN_SYNC_BYPASS_HI` writer"]
pub struct W(crate::W<PROC_IN_SYNC_BYPASS_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROC_IN_SYNC_BYPASS_HI_SPEC>;
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
impl From<crate::W<PROC_IN_SYNC_BYPASS_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROC_IN_SYNC_BYPASS_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROC_IN_SYNC_BYPASS_HI` reader - "]
pub struct PROC_IN_SYNC_BYPASS_HI_R(crate::FieldReader<u8, u8>);
impl PROC_IN_SYNC_BYPASS_HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROC_IN_SYNC_BYPASS_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC_IN_SYNC_BYPASS_HI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC_IN_SYNC_BYPASS_HI` writer - "]
pub struct PROC_IN_SYNC_BYPASS_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC_IN_SYNC_BYPASS_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn proc_in_sync_bypass_hi(&self) -> PROC_IN_SYNC_BYPASS_HI_R {
        PROC_IN_SYNC_BYPASS_HI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn proc_in_sync_bypass_hi(&mut self) -> PROC_IN_SYNC_BYPASS_HI_W {
        PROC_IN_SYNC_BYPASS_HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO  
 and the GPIO input register in the SIO. The input synchronizers should  
 generally be unbypassed, to avoid injecting metastabilities into processors.  
 If you're feeling brave, you can bypass to save two cycles of input  
 latency. This register applies to GPIO 30...35 (the QSPI IOs).  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [proc_in_sync_bypass_hi](index.html) module"]
pub struct PROC_IN_SYNC_BYPASS_HI_SPEC;
impl crate::RegisterSpec for PROC_IN_SYNC_BYPASS_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc_in_sync_bypass_hi::R](R) reader structure"]
impl crate::Readable for PROC_IN_SYNC_BYPASS_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [proc_in_sync_bypass_hi::W](W) writer structure"]
impl crate::Writable for PROC_IN_SYNC_BYPASS_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROC_IN_SYNC_BYPASS_HI to value 0"]
impl crate::Resettable for PROC_IN_SYNC_BYPASS_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
