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
#[doc = "Field `PROC0_HALTED` reader - Indication that proc0 has halted"]
pub type PROC0_HALTED_R = crate::BitReader<bool>;
#[doc = "Field `PROC1_HALTED` reader - Indication that proc1 has halted"]
pub type PROC1_HALTED_R = crate::BitReader<bool>;
#[doc = "Field `PROC0_DAP_INSTID` reader - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub type PROC0_DAP_INSTID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROC0_DAP_INSTID` writer - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub type PROC0_DAP_INSTID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PROC_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `PROC1_DAP_INSTID` reader - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub type PROC1_DAP_INSTID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROC1_DAP_INSTID` writer - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub type PROC1_DAP_INSTID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PROC_CONFIG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Indication that proc0 has halted"]
    #[inline(always)]
    pub fn proc0_halted(&self) -> PROC0_HALTED_R {
        PROC0_HALTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indication that proc1 has halted"]
    #[inline(always)]
    pub fn proc1_halted(&self) -> PROC1_HALTED_R {
        PROC1_HALTED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn proc0_dap_instid(&self) -> PROC0_DAP_INSTID_R {
        PROC0_DAP_INSTID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn proc1_dap_instid(&self) -> PROC1_DAP_INSTID_R {
        PROC1_DAP_INSTID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    #[must_use]
    pub fn proc0_dap_instid(&mut self) -> PROC0_DAP_INSTID_W<24> {
        PROC0_DAP_INSTID_W::new(self)
    }
    #[doc = "Bits 28:31 - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    #[must_use]
    pub fn proc1_dap_instid(&mut self) -> PROC1_DAP_INSTID_W<28> {
        PROC1_DAP_INSTID_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROC_CONFIG to value 0x1000_0000"]
impl crate::Resettable for PROC_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
