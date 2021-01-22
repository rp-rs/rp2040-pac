#[doc = "Reader of register USBPHY_DIRECT_OVERRIDE"]
pub type R = crate::R<u32, super::USBPHY_DIRECT_OVERRIDE>;
#[doc = "Writer for register USBPHY_DIRECT_OVERRIDE"]
pub type W = crate::W<u32, super::USBPHY_DIRECT_OVERRIDE>;
#[doc = "Register USBPHY_DIRECT_OVERRIDE `reset()`'s with value 0"]
impl crate::ResetValue for super::USBPHY_DIRECT_OVERRIDE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_DIFFMODE_OVERRIDE_EN`"]
pub type TX_DIFFMODE_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DIFFMODE_OVERRIDE_EN`"]
pub struct TX_DIFFMODE_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DIFFMODE_OVERRIDE_EN_W<'a> {
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
#[doc = "Reader of field `DM_PULLUP_OVERRIDE_EN`"]
pub type DM_PULLUP_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM_PULLUP_OVERRIDE_EN`"]
pub struct DM_PULLUP_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLUP_OVERRIDE_EN_W<'a> {
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
#[doc = "Reader of field `TX_FSSLEW_OVERRIDE_EN`"]
pub type TX_FSSLEW_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_FSSLEW_OVERRIDE_EN`"]
pub struct TX_FSSLEW_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FSSLEW_OVERRIDE_EN_W<'a> {
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
#[doc = "Reader of field `TX_PD_OVERRIDE_EN`"]
pub type TX_PD_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PD_OVERRIDE_EN`"]
pub struct TX_PD_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PD_OVERRIDE_EN_W<'a> {
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
#[doc = "Reader of field `RX_PD_OVERRIDE_EN`"]
pub type RX_PD_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_PD_OVERRIDE_EN`"]
pub struct RX_PD_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PD_OVERRIDE_EN_W<'a> {
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
#[doc = "Reader of field `TX_DM_OVERRIDE_EN`"]
pub type TX_DM_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DM_OVERRIDE_EN`"]
pub struct TX_DM_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DM_OVERRIDE_EN_W<'a> {
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
#[doc = "Reader of field `TX_DP_OVERRIDE_EN`"]
pub type TX_DP_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DP_OVERRIDE_EN`"]
pub struct TX_DP_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DP_OVERRIDE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TX_DM_OE_OVERRIDE_EN`"]
pub type TX_DM_OE_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DM_OE_OVERRIDE_EN`"]
pub struct TX_DM_OE_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DM_OE_OVERRIDE_EN_W<'a> {
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
#[doc = "Reader of field `TX_DP_OE_OVERRIDE_EN`"]
pub type TX_DP_OE_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DP_OE_OVERRIDE_EN`"]
pub struct TX_DP_OE_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DP_OE_OVERRIDE_EN_W<'a> {
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
#[doc = "Reader of field `DM_PULLDN_EN_OVERRIDE_EN`"]
pub type DM_PULLDN_EN_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM_PULLDN_EN_OVERRIDE_EN`"]
pub struct DM_PULLDN_EN_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLDN_EN_OVERRIDE_EN_W<'a> {
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
#[doc = "Reader of field `DP_PULLDN_EN_OVERRIDE_EN`"]
pub type DP_PULLDN_EN_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DP_PULLDN_EN_OVERRIDE_EN`"]
pub struct DP_PULLDN_EN_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLDN_EN_OVERRIDE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DP_PULLUP_EN_OVERRIDE_EN`"]
pub type DP_PULLUP_EN_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DP_PULLUP_EN_OVERRIDE_EN`"]
pub struct DP_PULLUP_EN_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLUP_EN_OVERRIDE_EN_W<'a> {
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
#[doc = "Reader of field `DM_PULLUP_HISEL_OVERRIDE_EN`"]
pub type DM_PULLUP_HISEL_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM_PULLUP_HISEL_OVERRIDE_EN`"]
pub struct DM_PULLUP_HISEL_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLUP_HISEL_OVERRIDE_EN_W<'a> {
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
#[doc = "Reader of field `DP_PULLUP_HISEL_OVERRIDE_EN`"]
pub type DP_PULLUP_HISEL_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DP_PULLUP_HISEL_OVERRIDE_EN`"]
pub struct DP_PULLUP_HISEL_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLUP_HISEL_OVERRIDE_EN_W<'a> {
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
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_diffmode_override_en(&self) -> TX_DIFFMODE_OVERRIDE_EN_R {
        TX_DIFFMODE_OVERRIDE_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dm_pullup_override_en(&self) -> DM_PULLUP_OVERRIDE_EN_R {
        DM_PULLUP_OVERRIDE_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_fsslew_override_en(&self) -> TX_FSSLEW_OVERRIDE_EN_R {
        TX_FSSLEW_OVERRIDE_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_pd_override_en(&self) -> TX_PD_OVERRIDE_EN_R {
        TX_PD_OVERRIDE_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_pd_override_en(&self) -> RX_PD_OVERRIDE_EN_R {
        RX_PD_OVERRIDE_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_dm_override_en(&self) -> TX_DM_OVERRIDE_EN_R {
        TX_DM_OVERRIDE_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_dp_override_en(&self) -> TX_DP_OVERRIDE_EN_R {
        TX_DP_OVERRIDE_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_dm_oe_override_en(&self) -> TX_DM_OE_OVERRIDE_EN_R {
        TX_DM_OE_OVERRIDE_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_dp_oe_override_en(&self) -> TX_DP_OE_OVERRIDE_EN_R {
        TX_DP_OE_OVERRIDE_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dm_pulldn_en_override_en(&self) -> DM_PULLDN_EN_OVERRIDE_EN_R {
        DM_PULLDN_EN_OVERRIDE_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dp_pulldn_en_override_en(&self) -> DP_PULLDN_EN_OVERRIDE_EN_R {
        DP_PULLDN_EN_OVERRIDE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dp_pullup_en_override_en(&self) -> DP_PULLUP_EN_OVERRIDE_EN_R {
        DP_PULLUP_EN_OVERRIDE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dm_pullup_hisel_override_en(&self) -> DM_PULLUP_HISEL_OVERRIDE_EN_R {
        DM_PULLUP_HISEL_OVERRIDE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dp_pullup_hisel_override_en(&self) -> DP_PULLUP_HISEL_OVERRIDE_EN_R {
        DP_PULLUP_HISEL_OVERRIDE_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_diffmode_override_en(&mut self) -> TX_DIFFMODE_OVERRIDE_EN_W {
        TX_DIFFMODE_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dm_pullup_override_en(&mut self) -> DM_PULLUP_OVERRIDE_EN_W {
        DM_PULLUP_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_fsslew_override_en(&mut self) -> TX_FSSLEW_OVERRIDE_EN_W {
        TX_FSSLEW_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_pd_override_en(&mut self) -> TX_PD_OVERRIDE_EN_W {
        TX_PD_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_pd_override_en(&mut self) -> RX_PD_OVERRIDE_EN_W {
        RX_PD_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_dm_override_en(&mut self) -> TX_DM_OVERRIDE_EN_W {
        TX_DM_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_dp_override_en(&mut self) -> TX_DP_OVERRIDE_EN_W {
        TX_DP_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_dm_oe_override_en(&mut self) -> TX_DM_OE_OVERRIDE_EN_W {
        TX_DM_OE_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_dp_oe_override_en(&mut self) -> TX_DP_OE_OVERRIDE_EN_W {
        TX_DP_OE_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dm_pulldn_en_override_en(&mut self) -> DM_PULLDN_EN_OVERRIDE_EN_W {
        DM_PULLDN_EN_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dp_pulldn_en_override_en(&mut self) -> DP_PULLDN_EN_OVERRIDE_EN_W {
        DP_PULLDN_EN_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dp_pullup_en_override_en(&mut self) -> DP_PULLUP_EN_OVERRIDE_EN_W {
        DP_PULLUP_EN_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dm_pullup_hisel_override_en(&mut self) -> DM_PULLUP_HISEL_OVERRIDE_EN_W {
        DM_PULLUP_HISEL_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dp_pullup_hisel_override_en(&mut self) -> DP_PULLUP_HISEL_OVERRIDE_EN_W {
        DP_PULLUP_HISEL_OVERRIDE_EN_W { w: self }
    }
}
