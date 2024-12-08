#[doc = "Register `USBPHY_DIRECT` reader"]
pub type R = crate::R<USBPHY_DIRECT_SPEC>;
#[doc = "Register `USBPHY_DIRECT` writer"]
pub type W = crate::W<USBPHY_DIRECT_SPEC>;
#[doc = "Field `DP_PULLUP_HISEL` reader - when dp_pullup_en is set high, this enables second resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
pub type DP_PULLUP_HISEL_R = crate::BitReader;
#[doc = "Field `DP_PULLUP_HISEL` writer - when dp_pullup_en is set high, this enables second resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
pub type DP_PULLUP_HISEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_PULLUP_EN` reader - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller"]
pub type DP_PULLUP_EN_R = crate::BitReader;
#[doc = "Field `DP_PULLUP_EN` writer - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller"]
pub type DP_PULLUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_PULLDN_EN` reader - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpd on DPP"]
pub type DP_PULLDN_EN_R = crate::BitReader;
#[doc = "Field `DP_PULLDN_EN` writer - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpd on DPP"]
pub type DP_PULLDN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLUP_HISEL` reader - when dm_pullup_en is set high, this enables second resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
pub type DM_PULLUP_HISEL_R = crate::BitReader;
#[doc = "Field `DM_PULLUP_HISEL` writer - when dm_pullup_en is set high, this enables second resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
pub type DM_PULLUP_HISEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLUP_EN` reader - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpu on DPM"]
pub type DM_PULLUP_EN_R = crate::BitReader;
#[doc = "Field `DM_PULLUP_EN` writer - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpu on DPM"]
pub type DM_PULLUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLDN_EN` reader - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpd on DPM"]
pub type DM_PULLDN_EN_R = crate::BitReader;
#[doc = "Field `DM_PULLDN_EN` writer - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpd on DPM"]
pub type DM_PULLDN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DP_OE` reader - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving TX_SEMODE=1, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
pub type TX_DP_OE_R = crate::BitReader;
#[doc = "Field `TX_DP_OE` writer - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving TX_SEMODE=1, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
pub type TX_DP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DM_OE` reader - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Ignored. TX_SEMODE=1, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
pub type TX_DM_OE_R = crate::BitReader;
#[doc = "Field `TX_DM_OE` writer - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Ignored. TX_SEMODE=1, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
pub type TX_DM_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DP` reader - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP TX_SEMODE=1, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
pub type TX_DP_R = crate::BitReader;
#[doc = "Field `TX_DP` writer - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP TX_SEMODE=1, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
pub type TX_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DM` reader - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Ignored TX_SEMODE=1, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
pub type TX_DM_R = crate::BitReader;
#[doc = "Field `TX_DM` writer - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Ignored TX_SEMODE=1, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
pub type TX_DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PD` reader - "]
pub type RX_PD_R = crate::BitReader;
#[doc = "Field `RX_PD` writer - "]
pub type RX_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PD` reader - "]
pub type TX_PD_R = crate::BitReader;
#[doc = "Field `TX_PD` writer - "]
pub type TX_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FSSLEW` reader - "]
pub type TX_FSSLEW_R = crate::BitReader;
#[doc = "Field `TX_FSSLEW` writer - "]
pub type TX_FSSLEW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DIFFMODE` reader - "]
pub type TX_DIFFMODE_R = crate::BitReader;
#[doc = "Field `TX_DIFFMODE` writer - "]
pub type TX_DIFFMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DD` reader - Status bit from USB PHY RX Diff data"]
pub type RX_DD_R = crate::BitReader;
#[doc = "Field `RX_DP` reader - Status bit from USB PHY DPP pin state"]
pub type RX_DP_R = crate::BitReader;
#[doc = "Field `RX_DM` reader - Status bit from USB PHY DPM pin state"]
pub type RX_DM_R = crate::BitReader;
#[doc = "Field `DP_OVCN` reader - Status bit from USB PHY"]
pub type DP_OVCN_R = crate::BitReader;
#[doc = "Field `DM_OVCN` reader - Status bit from USB PHY"]
pub type DM_OVCN_R = crate::BitReader;
#[doc = "Field `DP_OVV` reader - Status bit from USB PHY"]
pub type DP_OVV_R = crate::BitReader;
#[doc = "Field `DM_OVV` reader - Status bit from USB PHY"]
pub type DM_OVV_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - when dp_pullup_en is set high, this enables second resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub fn dp_pullup_hisel(&self) -> DP_PULLUP_HISEL_R {
        DP_PULLUP_HISEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller"]
    #[inline(always)]
    pub fn dp_pullup_en(&self) -> DP_PULLUP_EN_R {
        DP_PULLUP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpd on DPP"]
    #[inline(always)]
    pub fn dp_pulldn_en(&self) -> DP_PULLDN_EN_R {
        DP_PULLDN_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - when dm_pullup_en is set high, this enables second resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub fn dm_pullup_hisel(&self) -> DM_PULLUP_HISEL_R {
        DM_PULLUP_HISEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpu on DPM"]
    #[inline(always)]
    pub fn dm_pullup_en(&self) -> DM_PULLUP_EN_R {
        DM_PULLUP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpd on DPM"]
    #[inline(always)]
    pub fn dm_pulldn_en(&self) -> DM_PULLDN_EN_R {
        DM_PULLDN_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving TX_SEMODE=1, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
    #[inline(always)]
    pub fn tx_dp_oe(&self) -> TX_DP_OE_R {
        TX_DP_OE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Ignored. TX_SEMODE=1, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
    #[inline(always)]
    pub fn tx_dm_oe(&self) -> TX_DM_OE_R {
        TX_DM_OE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP TX_SEMODE=1, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
    #[inline(always)]
    pub fn tx_dp(&self) -> TX_DP_R {
        TX_DP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Ignored TX_SEMODE=1, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
    #[inline(always)]
    pub fn tx_dm(&self) -> TX_DM_R {
        TX_DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rx_pd(&self) -> RX_PD_R {
        RX_PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tx_pd(&self) -> TX_PD_R {
        TX_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tx_fsslew(&self) -> TX_FSSLEW_R {
        TX_FSSLEW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_diffmode(&self) -> TX_DIFFMODE_R {
        TX_DIFFMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Status bit from USB PHY RX Diff data"]
    #[inline(always)]
    pub fn rx_dd(&self) -> RX_DD_R {
        RX_DD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Status bit from USB PHY DPP pin state"]
    #[inline(always)]
    pub fn rx_dp(&self) -> RX_DP_R {
        RX_DP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Status bit from USB PHY DPM pin state"]
    #[inline(always)]
    pub fn rx_dm(&self) -> RX_DM_R {
        RX_DM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Status bit from USB PHY"]
    #[inline(always)]
    pub fn dp_ovcn(&self) -> DP_OVCN_R {
        DP_OVCN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Status bit from USB PHY"]
    #[inline(always)]
    pub fn dm_ovcn(&self) -> DM_OVCN_R {
        DM_OVCN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Status bit from USB PHY"]
    #[inline(always)]
    pub fn dp_ovv(&self) -> DP_OVV_R {
        DP_OVV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Status bit from USB PHY"]
    #[inline(always)]
    pub fn dm_ovv(&self) -> DM_OVV_R {
        DM_OVV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - when dp_pullup_en is set high, this enables second resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pullup_hisel(&mut self) -> DP_PULLUP_HISEL_W<USBPHY_DIRECT_SPEC> {
        DP_PULLUP_HISEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pullup_en(&mut self) -> DP_PULLUP_EN_W<USBPHY_DIRECT_SPEC> {
        DP_PULLUP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpd on DPP"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pulldn_en(&mut self) -> DP_PULLDN_EN_W<USBPHY_DIRECT_SPEC> {
        DP_PULLDN_EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - when dm_pullup_en is set high, this enables second resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pullup_hisel(&mut self) -> DM_PULLUP_HISEL_W<USBPHY_DIRECT_SPEC> {
        DM_PULLUP_HISEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpu on DPM"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pullup_en(&mut self) -> DM_PULLUP_EN_W<USBPHY_DIRECT_SPEC> {
        DM_PULLUP_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller 1 - Enable Rpd on DPM"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pulldn_en(&mut self) -> DM_PULLDN_EN_W<USBPHY_DIRECT_SPEC> {
        DM_PULLDN_EN_W::new(self, 6)
    }
    #[doc = "Bit 8 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving TX_SEMODE=1, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dp_oe(&mut self) -> TX_DP_OE_W<USBPHY_DIRECT_SPEC> {
        TX_DP_OE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Ignored. TX_SEMODE=1, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dm_oe(&mut self) -> TX_DM_OE_W<USBPHY_DIRECT_SPEC> {
        TX_DM_OE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP TX_SEMODE=1, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dp(&mut self) -> TX_DP_W<USBPHY_DIRECT_SPEC> {
        TX_DP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Value to drive to USB PHY when override enable is set (which will override the default value or value driven from USB controller TX_SEMODE=0, Ignored TX_SEMODE=1, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dm(&mut self) -> TX_DM_W<USBPHY_DIRECT_SPEC> {
        TX_DM_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pd(&mut self) -> RX_PD_W<USBPHY_DIRECT_SPEC> {
        RX_PD_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pd(&mut self) -> TX_PD_W<USBPHY_DIRECT_SPEC> {
        TX_PD_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fsslew(&mut self) -> TX_FSSLEW_W<USBPHY_DIRECT_SPEC> {
        TX_FSSLEW_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tx_diffmode(&mut self) -> TX_DIFFMODE_W<USBPHY_DIRECT_SPEC> {
        TX_DIFFMODE_W::new(self, 15)
    }
}
#[doc = "Note that most functions are driven directly from usb_fsls controller. This register allows more detailed control/status from the USB PHY. Useful for debug but not expected to be used in normal operation Use in conjunction with usbphy_direct_override register  

You can [`read`](crate::generic::Reg::read) this register and get [`usbphy_direct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy_direct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPHY_DIRECT_SPEC;
impl crate::RegisterSpec for USBPHY_DIRECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphy_direct::R`](R) reader structure"]
impl crate::Readable for USBPHY_DIRECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbphy_direct::W`](W) writer structure"]
impl crate::Writable for USBPHY_DIRECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBPHY_DIRECT to value 0"]
impl crate::Resettable for USBPHY_DIRECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
