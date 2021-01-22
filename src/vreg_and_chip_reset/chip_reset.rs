#[doc = "Reader of register CHIP_RESET"]
pub type R = crate::R<u32, super::CHIP_RESET>;
#[doc = "Writer for register CHIP_RESET"]
pub type W = crate::W<u32, super::CHIP_RESET>;
#[doc = "Register CHIP_RESET `reset()`'s with value 0"]
impl crate::ResetValue for super::CHIP_RESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSM_RESTART_FLAG`"]
pub type PSM_RESTART_FLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PSM_RESTART_FLAG`"]
pub struct PSM_RESTART_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> PSM_RESTART_FLAG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `HAD_PSM_RESTART`"]
pub type HAD_PSM_RESTART_R = crate::R<bool, bool>;
#[doc = "Reader of field `HAD_RUN`"]
pub type HAD_RUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `HAD_POR`"]
pub type HAD_POR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 24 - This is set by psm_restart from the debugger.\\n Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up.\\n In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
    #[inline(always)]
    pub fn psm_restart_flag(&self) -> PSM_RESTART_FLAG_R {
        PSM_RESTART_FLAG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Last reset was from the debug port"]
    #[inline(always)]
    pub fn had_psm_restart(&self) -> HAD_PSM_RESTART_R {
        HAD_PSM_RESTART_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Last reset was from the RUN pin"]
    #[inline(always)]
    pub fn had_run(&self) -> HAD_RUN_R {
        HAD_RUN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Last reset was from the power-on reset or brown-out detection blocks"]
    #[inline(always)]
    pub fn had_por(&self) -> HAD_POR_R {
        HAD_POR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - This is set by psm_restart from the debugger.\\n Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up.\\n In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
    #[inline(always)]
    pub fn psm_restart_flag(&mut self) -> PSM_RESTART_FLAG_W {
        PSM_RESTART_FLAG_W { w: self }
    }
}
