#[doc = "Register `USBPHY_DIRECT_OVERRIDE` reader"]
pub type R = crate::R<USBPHY_DIRECT_OVERRIDE_SPEC>;
#[doc = "Register `USBPHY_DIRECT_OVERRIDE` writer"]
pub type W = crate::W<USBPHY_DIRECT_OVERRIDE_SPEC>;
#[doc = "Field `DP_PULLUP_HISEL_OVERRIDE_EN` reader - "]
pub type DP_PULLUP_HISEL_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `DP_PULLUP_HISEL_OVERRIDE_EN` writer - "]
pub type DP_PULLUP_HISEL_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLUP_HISEL_OVERRIDE_EN` reader - "]
pub type DM_PULLUP_HISEL_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `DM_PULLUP_HISEL_OVERRIDE_EN` writer - "]
pub type DM_PULLUP_HISEL_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_PULLUP_EN_OVERRIDE_EN` reader - Override default value or value driven from USB Controller to PHY"]
pub type DP_PULLUP_EN_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `DP_PULLUP_EN_OVERRIDE_EN` writer - Override default value or value driven from USB Controller to PHY"]
pub type DP_PULLUP_EN_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_PULLDN_EN_OVERRIDE_EN` reader - Override default value or value driven from USB Controller to PHY"]
pub type DP_PULLDN_EN_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `DP_PULLDN_EN_OVERRIDE_EN` writer - Override default value or value driven from USB Controller to PHY"]
pub type DP_PULLDN_EN_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLDN_EN_OVERRIDE_EN` reader - Override default value or value driven from USB Controller to PHY"]
pub type DM_PULLDN_EN_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `DM_PULLDN_EN_OVERRIDE_EN` writer - Override default value or value driven from USB Controller to PHY"]
pub type DM_PULLDN_EN_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DP_OE_OVERRIDE_EN` reader - Override default value or value driven from USB Controller to PHY"]
pub type TX_DP_OE_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `TX_DP_OE_OVERRIDE_EN` writer - Override default value or value driven from USB Controller to PHY"]
pub type TX_DP_OE_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DM_OE_OVERRIDE_EN` reader - Override default value or value driven from USB Controller to PHY"]
pub type TX_DM_OE_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `TX_DM_OE_OVERRIDE_EN` writer - Override default value or value driven from USB Controller to PHY"]
pub type TX_DM_OE_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DP_OVERRIDE_EN` reader - Override default value or value driven from USB Controller to PHY"]
pub type TX_DP_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `TX_DP_OVERRIDE_EN` writer - Override default value or value driven from USB Controller to PHY"]
pub type TX_DP_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DM_OVERRIDE_EN` reader - Override default value or value driven from USB Controller to PHY"]
pub type TX_DM_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `TX_DM_OVERRIDE_EN` writer - Override default value or value driven from USB Controller to PHY"]
pub type TX_DM_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PD_OVERRIDE_EN` reader - "]
pub type RX_PD_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `RX_PD_OVERRIDE_EN` writer - "]
pub type RX_PD_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PD_OVERRIDE_EN` reader - "]
pub type TX_PD_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `TX_PD_OVERRIDE_EN` writer - "]
pub type TX_PD_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FSSLEW_OVERRIDE_EN` reader - "]
pub type TX_FSSLEW_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `TX_FSSLEW_OVERRIDE_EN` writer - "]
pub type TX_FSSLEW_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLUP_OVERRIDE_EN` reader - "]
pub type DM_PULLUP_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `DM_PULLUP_OVERRIDE_EN` writer - "]
pub type DM_PULLUP_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DIFFMODE_OVERRIDE_EN` reader - "]
pub type TX_DIFFMODE_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `TX_DIFFMODE_OVERRIDE_EN` writer - "]
pub type TX_DIFFMODE_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dp_pullup_hisel_override_en(&self) -> DP_PULLUP_HISEL_OVERRIDE_EN_R {
        DP_PULLUP_HISEL_OVERRIDE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dm_pullup_hisel_override_en(&self) -> DM_PULLUP_HISEL_OVERRIDE_EN_R {
        DM_PULLUP_HISEL_OVERRIDE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    pub fn dp_pullup_en_override_en(&self) -> DP_PULLUP_EN_OVERRIDE_EN_R {
        DP_PULLUP_EN_OVERRIDE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    pub fn dp_pulldn_en_override_en(&self) -> DP_PULLDN_EN_OVERRIDE_EN_R {
        DP_PULLDN_EN_OVERRIDE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    pub fn dm_pulldn_en_override_en(&self) -> DM_PULLDN_EN_OVERRIDE_EN_R {
        DM_PULLDN_EN_OVERRIDE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    pub fn tx_dp_oe_override_en(&self) -> TX_DP_OE_OVERRIDE_EN_R {
        TX_DP_OE_OVERRIDE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    pub fn tx_dm_oe_override_en(&self) -> TX_DM_OE_OVERRIDE_EN_R {
        TX_DM_OE_OVERRIDE_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    pub fn tx_dp_override_en(&self) -> TX_DP_OVERRIDE_EN_R {
        TX_DP_OVERRIDE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    pub fn tx_dm_override_en(&self) -> TX_DM_OVERRIDE_EN_R {
        TX_DM_OVERRIDE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_pd_override_en(&self) -> RX_PD_OVERRIDE_EN_R {
        RX_PD_OVERRIDE_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_pd_override_en(&self) -> TX_PD_OVERRIDE_EN_R {
        TX_PD_OVERRIDE_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_fsslew_override_en(&self) -> TX_FSSLEW_OVERRIDE_EN_R {
        TX_FSSLEW_OVERRIDE_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dm_pullup_override_en(&self) -> DM_PULLUP_OVERRIDE_EN_R {
        DM_PULLUP_OVERRIDE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_diffmode_override_en(&self) -> TX_DIFFMODE_OVERRIDE_EN_R {
        TX_DIFFMODE_OVERRIDE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pullup_hisel_override_en(
        &mut self,
    ) -> DP_PULLUP_HISEL_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        DP_PULLUP_HISEL_OVERRIDE_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pullup_hisel_override_en(
        &mut self,
    ) -> DM_PULLUP_HISEL_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        DM_PULLUP_HISEL_OVERRIDE_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pullup_en_override_en(
        &mut self,
    ) -> DP_PULLUP_EN_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        DP_PULLUP_EN_OVERRIDE_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pulldn_en_override_en(
        &mut self,
    ) -> DP_PULLDN_EN_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        DP_PULLDN_EN_OVERRIDE_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pulldn_en_override_en(
        &mut self,
    ) -> DM_PULLDN_EN_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        DM_PULLDN_EN_OVERRIDE_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dp_oe_override_en(&mut self) -> TX_DP_OE_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        TX_DP_OE_OVERRIDE_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dm_oe_override_en(&mut self) -> TX_DM_OE_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        TX_DM_OE_OVERRIDE_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dp_override_en(&mut self) -> TX_DP_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        TX_DP_OVERRIDE_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Override default value or value driven from USB Controller to PHY"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dm_override_en(&mut self) -> TX_DM_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        TX_DM_OVERRIDE_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pd_override_en(&mut self) -> RX_PD_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        RX_PD_OVERRIDE_EN_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pd_override_en(&mut self) -> TX_PD_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        TX_PD_OVERRIDE_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fsslew_override_en(
        &mut self,
    ) -> TX_FSSLEW_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        TX_FSSLEW_OVERRIDE_EN_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pullup_override_en(
        &mut self,
    ) -> DM_PULLUP_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        DM_PULLUP_OVERRIDE_EN_W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tx_diffmode_override_en(
        &mut self,
    ) -> TX_DIFFMODE_OVERRIDE_EN_W<USBPHY_DIRECT_OVERRIDE_SPEC> {
        TX_DIFFMODE_OVERRIDE_EN_W::new(self, 15)
    }
}
#[doc = "  

You can [`read`](crate::generic::Reg::read) this register and get [`usbphy_direct_override::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy_direct_override::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPHY_DIRECT_OVERRIDE_SPEC;
impl crate::RegisterSpec for USBPHY_DIRECT_OVERRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphy_direct_override::R`](R) reader structure"]
impl crate::Readable for USBPHY_DIRECT_OVERRIDE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbphy_direct_override::W`](W) writer structure"]
impl crate::Writable for USBPHY_DIRECT_OVERRIDE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBPHY_DIRECT_OVERRIDE to value 0"]
impl crate::Resettable for USBPHY_DIRECT_OVERRIDE_SPEC {
    const RESET_VALUE: u32 = 0;
}
