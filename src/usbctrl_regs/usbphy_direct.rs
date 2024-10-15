#[doc = "Register `USBPHY_DIRECT` reader"]
pub type R = crate::R<USBPHY_DIRECT_SPEC>;
#[doc = "Register `USBPHY_DIRECT` writer"]
pub type W = crate::W<USBPHY_DIRECT_SPEC>;
#[doc = "Field `DP_PULLUP_HISEL` reader - Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
pub type DP_PULLUP_HISEL_R = crate::BitReader;
#[doc = "Field `DP_PULLUP_HISEL` writer - Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
pub type DP_PULLUP_HISEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_PULLUP_EN` reader - DP pull up enable"]
pub type DP_PULLUP_EN_R = crate::BitReader;
#[doc = "Field `DP_PULLUP_EN` writer - DP pull up enable"]
pub type DP_PULLUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_PULLDN_EN` reader - DP pull down enable"]
pub type DP_PULLDN_EN_R = crate::BitReader;
#[doc = "Field `DP_PULLDN_EN` writer - DP pull down enable"]
pub type DP_PULLDN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLUP_HISEL` reader - Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
pub type DM_PULLUP_HISEL_R = crate::BitReader;
#[doc = "Field `DM_PULLUP_HISEL` writer - Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
pub type DM_PULLUP_HISEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLUP_EN` reader - DM pull up enable"]
pub type DM_PULLUP_EN_R = crate::BitReader;
#[doc = "Field `DM_PULLUP_EN` writer - DM pull up enable"]
pub type DM_PULLUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLDN_EN` reader - DM pull down enable"]
pub type DM_PULLDN_EN_R = crate::BitReader;
#[doc = "Field `DM_PULLDN_EN` writer - DM pull down enable"]
pub type DM_PULLDN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DP_OE` reader - Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving  
 If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
pub type TX_DP_OE_R = crate::BitReader;
#[doc = "Field `TX_DP_OE` writer - Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving  
 If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
pub type TX_DP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DM_OE` reader - Output enable. If TX_DIFFMODE=1, Ignored.  
 If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
pub type TX_DM_OE_R = crate::BitReader;
#[doc = "Field `TX_DM_OE` writer - Output enable. If TX_DIFFMODE=1, Ignored.  
 If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
pub type TX_DM_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DP` reader - Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP  
 If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
pub type TX_DP_R = crate::BitReader;
#[doc = "Field `TX_DP` writer - Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP  
 If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
pub type TX_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DM` reader - Output data. TX_DIFFMODE=1, Ignored  
 TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
pub type TX_DM_R = crate::BitReader;
#[doc = "Field `TX_DM` writer - Output data. TX_DIFFMODE=1, Ignored  
 TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
pub type TX_DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PD` reader - RX power down override (if override enable is set). 1 = powered down."]
pub type RX_PD_R = crate::BitReader;
#[doc = "Field `RX_PD` writer - RX power down override (if override enable is set). 1 = powered down."]
pub type RX_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PD` reader - TX power down override (if override enable is set). 1 = powered down."]
pub type TX_PD_R = crate::BitReader;
#[doc = "Field `TX_PD` writer - TX power down override (if override enable is set). 1 = powered down."]
pub type TX_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FSSLEW` reader - TX_FSSLEW=0: Low speed slew rate  
 TX_FSSLEW=1: Full speed slew rate"]
pub type TX_FSSLEW_R = crate::BitReader;
#[doc = "Field `TX_FSSLEW` writer - TX_FSSLEW=0: Low speed slew rate  
 TX_FSSLEW=1: Full speed slew rate"]
pub type TX_FSSLEW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DIFFMODE` reader - TX_DIFFMODE=0: Single ended mode  
 TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)"]
pub type TX_DIFFMODE_R = crate::BitReader;
#[doc = "Field `TX_DIFFMODE` writer - TX_DIFFMODE=0: Single ended mode  
 TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)"]
pub type TX_DIFFMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DD` reader - Differential RX"]
pub type RX_DD_R = crate::BitReader;
#[doc = "Field `RX_DP` reader - DPP pin state"]
pub type RX_DP_R = crate::BitReader;
#[doc = "Field `RX_DM` reader - DPM pin state"]
pub type RX_DM_R = crate::BitReader;
#[doc = "Field `DP_OVCN` reader - DP overcurrent"]
pub type DP_OVCN_R = crate::BitReader;
#[doc = "Field `DM_OVCN` reader - DM overcurrent"]
pub type DM_OVCN_R = crate::BitReader;
#[doc = "Field `DP_OVV` reader - DP over voltage"]
pub type DP_OVV_R = crate::BitReader;
#[doc = "Field `DM_OVV` reader - DM over voltage"]
pub type DM_OVV_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub fn dp_pullup_hisel(&self) -> DP_PULLUP_HISEL_R {
        DP_PULLUP_HISEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DP pull up enable"]
    #[inline(always)]
    pub fn dp_pullup_en(&self) -> DP_PULLUP_EN_R {
        DP_PULLUP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DP pull down enable"]
    #[inline(always)]
    pub fn dp_pulldn_en(&self) -> DP_PULLDN_EN_R {
        DP_PULLDN_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub fn dm_pullup_hisel(&self) -> DM_PULLUP_HISEL_R {
        DM_PULLUP_HISEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DM pull up enable"]
    #[inline(always)]
    pub fn dm_pullup_en(&self) -> DM_PULLUP_EN_R {
        DM_PULLUP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DM pull down enable"]
    #[inline(always)]
    pub fn dm_pulldn_en(&self) -> DM_PULLDN_EN_R {
        DM_PULLDN_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving  
 If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
    #[inline(always)]
    pub fn tx_dp_oe(&self) -> TX_DP_OE_R {
        TX_DP_OE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output enable. If TX_DIFFMODE=1, Ignored.  
 If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
    #[inline(always)]
    pub fn tx_dm_oe(&self) -> TX_DM_OE_R {
        TX_DM_OE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP  
 If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
    #[inline(always)]
    pub fn tx_dp(&self) -> TX_DP_R {
        TX_DP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output data. TX_DIFFMODE=1, Ignored  
 TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
    #[inline(always)]
    pub fn tx_dm(&self) -> TX_DM_R {
        TX_DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    pub fn rx_pd(&self) -> RX_PD_R {
        RX_PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    pub fn tx_pd(&self) -> TX_PD_R {
        TX_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TX_FSSLEW=0: Low speed slew rate  
 TX_FSSLEW=1: Full speed slew rate"]
    #[inline(always)]
    pub fn tx_fsslew(&self) -> TX_FSSLEW_R {
        TX_FSSLEW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TX_DIFFMODE=0: Single ended mode  
 TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)"]
    #[inline(always)]
    pub fn tx_diffmode(&self) -> TX_DIFFMODE_R {
        TX_DIFFMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Differential RX"]
    #[inline(always)]
    pub fn rx_dd(&self) -> RX_DD_R {
        RX_DD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPP pin state"]
    #[inline(always)]
    pub fn rx_dp(&self) -> RX_DP_R {
        RX_DP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DPM pin state"]
    #[inline(always)]
    pub fn rx_dm(&self) -> RX_DM_R {
        RX_DM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DP overcurrent"]
    #[inline(always)]
    pub fn dp_ovcn(&self) -> DP_OVCN_R {
        DP_OVCN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DM overcurrent"]
    #[inline(always)]
    pub fn dm_ovcn(&self) -> DM_OVCN_R {
        DM_OVCN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DP over voltage"]
    #[inline(always)]
    pub fn dp_ovv(&self) -> DP_OVV_R {
        DP_OVV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DM over voltage"]
    #[inline(always)]
    pub fn dm_ovv(&self) -> DM_OVV_R {
        DM_OVV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pullup_hisel(&mut self) -> DP_PULLUP_HISEL_W<USBPHY_DIRECT_SPEC> {
        DP_PULLUP_HISEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - DP pull up enable"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pullup_en(&mut self) -> DP_PULLUP_EN_W<USBPHY_DIRECT_SPEC> {
        DP_PULLUP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DP pull down enable"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pulldn_en(&mut self) -> DP_PULLDN_EN_W<USBPHY_DIRECT_SPEC> {
        DP_PULLDN_EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pullup_hisel(&mut self) -> DM_PULLUP_HISEL_W<USBPHY_DIRECT_SPEC> {
        DM_PULLUP_HISEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - DM pull up enable"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pullup_en(&mut self) -> DM_PULLUP_EN_W<USBPHY_DIRECT_SPEC> {
        DM_PULLUP_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - DM pull down enable"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pulldn_en(&mut self) -> DM_PULLDN_EN_W<USBPHY_DIRECT_SPEC> {
        DM_PULLDN_EN_W::new(self, 6)
    }
    #[doc = "Bit 8 - Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving  
 If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dp_oe(&mut self) -> TX_DP_OE_W<USBPHY_DIRECT_SPEC> {
        TX_DP_OE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Output enable. If TX_DIFFMODE=1, Ignored.  
 If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dm_oe(&mut self) -> TX_DM_OE_W<USBPHY_DIRECT_SPEC> {
        TX_DM_OE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP  
 If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dp(&mut self) -> TX_DP_W<USBPHY_DIRECT_SPEC> {
        TX_DP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output data. TX_DIFFMODE=1, Ignored  
 TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dm(&mut self) -> TX_DM_W<USBPHY_DIRECT_SPEC> {
        TX_DM_W::new(self, 11)
    }
    #[doc = "Bit 12 - RX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    #[must_use]
    pub fn rx_pd(&mut self) -> RX_PD_W<USBPHY_DIRECT_SPEC> {
        RX_PD_W::new(self, 12)
    }
    #[doc = "Bit 13 - TX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pd(&mut self) -> TX_PD_W<USBPHY_DIRECT_SPEC> {
        TX_PD_W::new(self, 13)
    }
    #[doc = "Bit 14 - TX_FSSLEW=0: Low speed slew rate  
 TX_FSSLEW=1: Full speed slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fsslew(&mut self) -> TX_FSSLEW_W<USBPHY_DIRECT_SPEC> {
        TX_FSSLEW_W::new(self, 14)
    }
    #[doc = "Bit 15 - TX_DIFFMODE=0: Single ended mode  
 TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn tx_diffmode(&mut self) -> TX_DIFFMODE_W<USBPHY_DIRECT_SPEC> {
        TX_DIFFMODE_W::new(self, 15)
    }
}
#[doc = "This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit.  

You can [`read`](crate::Reg::read) this register and get [`usbphy_direct::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphy_direct::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
