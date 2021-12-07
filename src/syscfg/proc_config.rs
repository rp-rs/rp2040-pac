#[doc = "Register `PROC_CONFIG` reader"]
pub struct R(crate::R<PROC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROC_CONFIG` writer"]
pub struct W(crate::W<PROC_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROC_CONFIG_SPEC>;
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
impl From<crate::W<PROC_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROC_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROC1_DAP_INSTID` reader - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub struct PROC1_DAP_INSTID_R(crate::FieldReader<u8, u8>);
impl PROC1_DAP_INSTID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROC1_DAP_INSTID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC1_DAP_INSTID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC1_DAP_INSTID` writer - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub struct PROC1_DAP_INSTID_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_DAP_INSTID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `PROC0_DAP_INSTID` reader - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub struct PROC0_DAP_INSTID_R(crate::FieldReader<u8, u8>);
impl PROC0_DAP_INSTID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROC0_DAP_INSTID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC0_DAP_INSTID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC0_DAP_INSTID` writer - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub struct PROC0_DAP_INSTID_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_DAP_INSTID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PROC1_HALTED` reader - Indication that proc1 has halted"]
pub struct PROC1_HALTED_R(crate::FieldReader<bool, bool>);
impl PROC1_HALTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC1_HALTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC1_HALTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC0_HALTED` reader - Indication that proc0 has halted"]
pub struct PROC0_HALTED_R(crate::FieldReader<bool, bool>);
impl PROC0_HALTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC0_HALTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC0_HALTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 28:31 - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn proc1_dap_instid(&self) -> PROC1_DAP_INSTID_R {
        PROC1_DAP_INSTID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn proc0_dap_instid(&self) -> PROC0_DAP_INSTID_R {
        PROC0_DAP_INSTID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - Indication that proc1 has halted"]
    #[inline(always)]
    pub fn proc1_halted(&self) -> PROC1_HALTED_R {
        PROC1_HALTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indication that proc0 has halted"]
    #[inline(always)]
    pub fn proc0_halted(&self) -> PROC0_HALTED_R {
        PROC0_HALTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31 - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn proc1_dap_instid(&mut self) -> PROC1_DAP_INSTID_W {
        PROC1_DAP_INSTID_W { w: self }
    }
    #[doc = "Bits 24:27 - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn proc0_dap_instid(&mut self) -> PROC0_DAP_INSTID_W {
        PROC0_DAP_INSTID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for processors  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [proc_config](index.html) module"]
pub struct PROC_CONFIG_SPEC;
impl crate::RegisterSpec for PROC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc_config::R](R) reader structure"]
impl crate::Readable for PROC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [proc_config::W](W) writer structure"]
impl crate::Writable for PROC_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROC_CONFIG to value 0x1000_0000"]
impl crate::Resettable for PROC_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0000
    }
}
