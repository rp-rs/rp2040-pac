#[doc = "Reader of register PROC_CONFIG"]
pub type R = crate::R<u32, super::PROC_CONFIG>;
#[doc = "Writer for register PROC_CONFIG"]
pub type W = crate::W<u32, super::PROC_CONFIG>;
#[doc = "Register PROC_CONFIG `reset()`'s with value 0x1000_0000"]
impl crate::ResetValue for super::PROC_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_0000
    }
}
#[doc = "Reader of field `PROC1_DAP_INSTID`"]
pub type PROC1_DAP_INSTID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROC1_DAP_INSTID`"]
pub struct PROC1_DAP_INSTID_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_DAP_INSTID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `PROC0_DAP_INSTID`"]
pub type PROC0_DAP_INSTID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROC0_DAP_INSTID`"]
pub struct PROC0_DAP_INSTID_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_DAP_INSTID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PROC1_HALTED`"]
pub type PROC1_HALTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROC0_HALTED`"]
pub type PROC0_HALTED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 28:31 - Configure proc1 DAP instance ID.\\n Recommend that this is NOT changed until you require debug access in multi-chip environment\\n WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn proc1_dap_instid(&self) -> PROC1_DAP_INSTID_R {
        PROC1_DAP_INSTID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Configure proc0 DAP instance ID.\\n Recommend that this is NOT changed until you require debug access in multi-chip environment\\n WARNING: do not set to 15 as this is reserved for RescueDP"]
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
    #[doc = "Bits 28:31 - Configure proc1 DAP instance ID.\\n Recommend that this is NOT changed until you require debug access in multi-chip environment\\n WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn proc1_dap_instid(&mut self) -> PROC1_DAP_INSTID_W {
        PROC1_DAP_INSTID_W { w: self }
    }
    #[doc = "Bits 24:27 - Configure proc0 DAP instance ID.\\n Recommend that this is NOT changed until you require debug access in multi-chip environment\\n WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn proc0_dap_instid(&mut self) -> PROC0_DAP_INSTID_W {
        PROC0_DAP_INSTID_W { w: self }
    }
}
