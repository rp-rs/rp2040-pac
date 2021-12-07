#[doc = "Register `PROC_IN_SYNC_BYPASS` reader"]
pub struct R(crate::R<PROC_IN_SYNC_BYPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC_IN_SYNC_BYPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC_IN_SYNC_BYPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC_IN_SYNC_BYPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROC_IN_SYNC_BYPASS` writer"]
pub struct W(crate::W<PROC_IN_SYNC_BYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROC_IN_SYNC_BYPASS_SPEC>;
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
impl From<crate::W<PROC_IN_SYNC_BYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROC_IN_SYNC_BYPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROC_IN_SYNC_BYPASS` reader - "]
pub struct PROC_IN_SYNC_BYPASS_R(crate::FieldReader<u32, u32>);
impl PROC_IN_SYNC_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PROC_IN_SYNC_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC_IN_SYNC_BYPASS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC_IN_SYNC_BYPASS` writer - "]
pub struct PROC_IN_SYNC_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC_IN_SYNC_BYPASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn proc_in_sync_bypass(&self) -> PROC_IN_SYNC_BYPASS_R {
        PROC_IN_SYNC_BYPASS_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn proc_in_sync_bypass(&mut self) -> PROC_IN_SYNC_BYPASS_W {
        PROC_IN_SYNC_BYPASS_W { w: self }
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
 latency. This register applies to GPIO 0...29.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [proc_in_sync_bypass](index.html) module"]
pub struct PROC_IN_SYNC_BYPASS_SPEC;
impl crate::RegisterSpec for PROC_IN_SYNC_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc_in_sync_bypass::R](R) reader structure"]
impl crate::Readable for PROC_IN_SYNC_BYPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [proc_in_sync_bypass::W](W) writer structure"]
impl crate::Writable for PROC_IN_SYNC_BYPASS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROC_IN_SYNC_BYPASS to value 0"]
impl crate::Resettable for PROC_IN_SYNC_BYPASS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
