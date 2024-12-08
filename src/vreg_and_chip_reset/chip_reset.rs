#[doc = "Register `CHIP_RESET` reader"]
pub type R = crate::R<CHIP_RESET_SPEC>;
#[doc = "Register `CHIP_RESET` writer"]
pub type W = crate::W<CHIP_RESET_SPEC>;
#[doc = "Field `HAD_POR` reader - Last reset was from the power-on reset or brown-out detection blocks"]
pub type HAD_POR_R = crate::BitReader;
#[doc = "Field `HAD_RUN` reader - Last reset was from the RUN pin"]
pub type HAD_RUN_R = crate::BitReader;
#[doc = "Field `HAD_PSM_RESTART` reader - Last reset was from the debug port"]
pub type HAD_PSM_RESTART_R = crate::BitReader;
#[doc = "Field `PSM_RESTART_FLAG` reader - This is set by psm_restart from the debugger. Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up. In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
pub type PSM_RESTART_FLAG_R = crate::BitReader;
#[doc = "Field `PSM_RESTART_FLAG` writer - This is set by psm_restart from the debugger. Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up. In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
pub type PSM_RESTART_FLAG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
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
    #[doc = "Bit 24 - This is set by psm_restart from the debugger. Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up. In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
    #[inline(always)]
    pub fn psm_restart_flag(&self) -> PSM_RESTART_FLAG_R {
        PSM_RESTART_FLAG_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - This is set by psm_restart from the debugger. Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up. In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
    #[inline(always)]
    #[must_use]
    pub fn psm_restart_flag(&mut self) -> PSM_RESTART_FLAG_W<CHIP_RESET_SPEC> {
        PSM_RESTART_FLAG_W::new(self, 24)
    }
}
#[doc = "Chip reset control and status  

You can [`read`](crate::generic::Reg::read) this register and get [`chip_reset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chip_reset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIP_RESET_SPEC;
impl crate::RegisterSpec for CHIP_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chip_reset::R`](R) reader structure"]
impl crate::Readable for CHIP_RESET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chip_reset::W`](W) writer structure"]
impl crate::Writable for CHIP_RESET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0100_0000;
}
#[doc = "`reset()` method sets CHIP_RESET to value 0"]
impl crate::Resettable for CHIP_RESET_SPEC {
    const RESET_VALUE: u32 = 0;
}
