#[doc = "Register `CHIP_RESET` reader"]
pub struct R(crate::R<CHIP_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIP_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIP_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIP_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHIP_RESET` writer"]
pub struct W(crate::W<CHIP_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIP_RESET_SPEC>;
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
impl From<crate::W<CHIP_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIP_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HAD_POR` reader - Last reset was from the power-on reset or brown-out detection blocks"]
pub type HAD_POR_R = crate::BitReader<bool>;
#[doc = "Field `HAD_RUN` reader - Last reset was from the RUN pin"]
pub type HAD_RUN_R = crate::BitReader<bool>;
#[doc = "Field `HAD_PSM_RESTART` reader - Last reset was from the debug port"]
pub type HAD_PSM_RESTART_R = crate::BitReader<bool>;
#[doc = "Field `PSM_RESTART_FLAG` reader - This is set by psm_restart from the debugger.  
 Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up.  
 In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
pub type PSM_RESTART_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `PSM_RESTART_FLAG` writer - This is set by psm_restart from the debugger.  
 Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up.  
 In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
pub type PSM_RESTART_FLAG_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, CHIP_RESET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - Last reset was from the power-on reset or brown-out detection blocks"]
    #[inline(always)]
    pub fn had_por(&self) -> HAD_POR_R {
        HAD_POR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Last reset was from the RUN pin"]
    #[inline(always)]
    pub fn had_run(&self) -> HAD_RUN_R {
        HAD_RUN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Last reset was from the debug port"]
    #[inline(always)]
    pub fn had_psm_restart(&self) -> HAD_PSM_RESTART_R {
        HAD_PSM_RESTART_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - This is set by psm_restart from the debugger.  
 Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up.  
 In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
    #[inline(always)]
    pub fn psm_restart_flag(&self) -> PSM_RESTART_FLAG_R {
        PSM_RESTART_FLAG_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - This is set by psm_restart from the debugger.  
 Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up.  
 In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
    #[inline(always)]
    #[must_use]
    pub fn psm_restart_flag(&mut self) -> PSM_RESTART_FLAG_W<24> {
        PSM_RESTART_FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip reset control and status  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [chip_reset](index.html) module"]
pub struct CHIP_RESET_SPEC;
impl crate::RegisterSpec for CHIP_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chip_reset::R](R) reader structure"]
impl crate::Readable for CHIP_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chip_reset::W](W) writer structure"]
impl crate::Writable for CHIP_RESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0100_0000;
}
#[doc = "`reset()` method sets CHIP_RESET to value 0"]
impl crate::Resettable for CHIP_RESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
