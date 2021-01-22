#[doc = "Reader of register USBPHY_DIRECT"]
pub type R = crate::R<u32, super::USBPHY_DIRECT>;
#[doc = "Writer for register USBPHY_DIRECT"]
pub type W = crate::W<u32, super::USBPHY_DIRECT>;
#[doc = "Register USBPHY_DIRECT `reset()`'s with value 0"]
impl crate::ResetValue for super::USBPHY_DIRECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DM_OVV`"]
pub type DM_OVV_R = crate::R<bool, bool>;
#[doc = "Reader of field `DP_OVV`"]
pub type DP_OVV_R = crate::R<bool, bool>;
#[doc = "Reader of field `DM_OVCN`"]
pub type DM_OVCN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DP_OVCN`"]
pub type DP_OVCN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_DM`"]
pub type RX_DM_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_DP`"]
pub type RX_DP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_DD`"]
pub type RX_DD_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_DIFFMODE`"]
pub type TX_DIFFMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DIFFMODE`"]
pub struct TX_DIFFMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DIFFMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TX_FSSLEW`"]
pub type TX_FSSLEW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_FSSLEW`"]
pub struct TX_FSSLEW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FSSLEW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TX_PD`"]
pub type TX_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PD`"]
pub struct TX_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RX_PD`"]
pub type RX_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_PD`"]
pub struct RX_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TX_DM`"]
pub type TX_DM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DM`"]
pub struct TX_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TX_DP`"]
pub type TX_DP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DP`"]
pub struct TX_DP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TX_DM_OE`"]
pub type TX_DM_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DM_OE`"]
pub struct TX_DM_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DM_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TX_DP_OE`"]
pub type TX_DP_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DP_OE`"]
pub struct TX_DP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DP_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DM_PULLDN_EN`"]
pub type DM_PULLDN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM_PULLDN_EN`"]
pub struct DM_PULLDN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLDN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DM_PULLUP_EN`"]
pub type DM_PULLUP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM_PULLUP_EN`"]
pub struct DM_PULLUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLUP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DM_PULLUP_HISEL`"]
pub type DM_PULLUP_HISEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM_PULLUP_HISEL`"]
pub struct DM_PULLUP_HISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLUP_HISEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DP_PULLDN_EN`"]
pub type DP_PULLDN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DP_PULLDN_EN`"]
pub struct DP_PULLDN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLDN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DP_PULLUP_EN`"]
pub type DP_PULLUP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DP_PULLUP_EN`"]
pub struct DP_PULLUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLUP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DP_PULLUP_HISEL`"]
pub type DP_PULLUP_HISEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DP_PULLUP_HISEL`"]
pub struct DP_PULLUP_HISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLUP_HISEL_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 22 - DM over voltage"]
    #[inline(always)]
    pub fn dm_ovv(&self) -> DM_OVV_R {
        DM_OVV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DP over voltage"]
    #[inline(always)]
    pub fn dp_ovv(&self) -> DP_OVV_R {
        DP_OVV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DM overcurrent"]
    #[inline(always)]
    pub fn dm_ovcn(&self) -> DM_OVCN_R {
        DM_OVCN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DP overcurrent"]
    #[inline(always)]
    pub fn dp_ovcn(&self) -> DP_OVCN_R {
        DP_OVCN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DPM pin state"]
    #[inline(always)]
    pub fn rx_dm(&self) -> RX_DM_R {
        RX_DM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPP pin state"]
    #[inline(always)]
    pub fn rx_dp(&self) -> RX_DP_R {
        RX_DP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Differential RX"]
    #[inline(always)]
    pub fn rx_dd(&self) -> RX_DD_R {
        RX_DD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TX_DIFFMODE=0: Single ended mode\\n TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)"]
    #[inline(always)]
    pub fn tx_diffmode(&self) -> TX_DIFFMODE_R {
        TX_DIFFMODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TX_FSSLEW=0: Low speed slew rate\\n TX_FSSLEW=1: Full speed slew rate"]
    #[inline(always)]
    pub fn tx_fsslew(&self) -> TX_FSSLEW_R {
        TX_FSSLEW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    pub fn tx_pd(&self) -> TX_PD_R {
        TX_PD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    pub fn rx_pd(&self) -> RX_PD_R {
        RX_PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output data. TX_DIFFMODE=1, Ignored\\n TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
    #[inline(always)]
    pub fn tx_dm(&self) -> TX_DM_R {
        TX_DM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP\\n If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
    #[inline(always)]
    pub fn tx_dp(&self) -> TX_DP_R {
        TX_DP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Output enable. If TX_DIFFMODE=1, Ignored.\\n If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
    #[inline(always)]
    pub fn tx_dm_oe(&self) -> TX_DM_OE_R {
        TX_DM_OE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving\\n If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
    #[inline(always)]
    pub fn tx_dp_oe(&self) -> TX_DP_OE_R {
        TX_DP_OE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DM pull down enable"]
    #[inline(always)]
    pub fn dm_pulldn_en(&self) -> DM_PULLDN_EN_R {
        DM_PULLDN_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DM pull up enable"]
    #[inline(always)]
    pub fn dm_pullup_en(&self) -> DM_PULLUP_EN_R {
        DM_PULLUP_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub fn dm_pullup_hisel(&self) -> DM_PULLUP_HISEL_R {
        DM_PULLUP_HISEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DP pull down enable"]
    #[inline(always)]
    pub fn dp_pulldn_en(&self) -> DP_PULLDN_EN_R {
        DP_PULLDN_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DP pull up enable"]
    #[inline(always)]
    pub fn dp_pullup_en(&self) -> DP_PULLUP_EN_R {
        DP_PULLUP_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub fn dp_pullup_hisel(&self) -> DP_PULLUP_HISEL_R {
        DP_PULLUP_HISEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - TX_DIFFMODE=0: Single ended mode\\n TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)"]
    #[inline(always)]
    pub fn tx_diffmode(&mut self) -> TX_DIFFMODE_W {
        TX_DIFFMODE_W { w: self }
    }
    #[doc = "Bit 14 - TX_FSSLEW=0: Low speed slew rate\\n TX_FSSLEW=1: Full speed slew rate"]
    #[inline(always)]
    pub fn tx_fsslew(&mut self) -> TX_FSSLEW_W {
        TX_FSSLEW_W { w: self }
    }
    #[doc = "Bit 13 - TX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    pub fn tx_pd(&mut self) -> TX_PD_W {
        TX_PD_W { w: self }
    }
    #[doc = "Bit 12 - RX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    pub fn rx_pd(&mut self) -> RX_PD_W {
        RX_PD_W { w: self }
    }
    #[doc = "Bit 11 - Output data. TX_DIFFMODE=1, Ignored\\n TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
    #[inline(always)]
    pub fn tx_dm(&mut self) -> TX_DM_W {
        TX_DM_W { w: self }
    }
    #[doc = "Bit 10 - Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP\\n If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
    #[inline(always)]
    pub fn tx_dp(&mut self) -> TX_DP_W {
        TX_DP_W { w: self }
    }
    #[doc = "Bit 9 - Output enable. If TX_DIFFMODE=1, Ignored.\\n If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
    #[inline(always)]
    pub fn tx_dm_oe(&mut self) -> TX_DM_OE_W {
        TX_DM_OE_W { w: self }
    }
    #[doc = "Bit 8 - Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving\\n If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
    #[inline(always)]
    pub fn tx_dp_oe(&mut self) -> TX_DP_OE_W {
        TX_DP_OE_W { w: self }
    }
    #[doc = "Bit 6 - DM pull down enable"]
    #[inline(always)]
    pub fn dm_pulldn_en(&mut self) -> DM_PULLDN_EN_W {
        DM_PULLDN_EN_W { w: self }
    }
    #[doc = "Bit 5 - DM pull up enable"]
    #[inline(always)]
    pub fn dm_pullup_en(&mut self) -> DM_PULLUP_EN_W {
        DM_PULLUP_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub fn dm_pullup_hisel(&mut self) -> DM_PULLUP_HISEL_W {
        DM_PULLUP_HISEL_W { w: self }
    }
    #[doc = "Bit 2 - DP pull down enable"]
    #[inline(always)]
    pub fn dp_pulldn_en(&mut self) -> DP_PULLDN_EN_W {
        DP_PULLDN_EN_W { w: self }
    }
    #[doc = "Bit 1 - DP pull up enable"]
    #[inline(always)]
    pub fn dp_pullup_en(&mut self) -> DP_PULLUP_EN_W {
        DP_PULLUP_EN_W { w: self }
    }
    #[doc = "Bit 0 - Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub fn dp_pullup_hisel(&mut self) -> DP_PULLUP_HISEL_W {
        DP_PULLUP_HISEL_W { w: self }
    }
}
