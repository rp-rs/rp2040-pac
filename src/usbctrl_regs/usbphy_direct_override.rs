#[doc = "Register `USBPHY_DIRECT_OVERRIDE` reader"]
pub struct R(crate::R<USBPHY_DIRECT_OVERRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHY_DIRECT_OVERRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHY_DIRECT_OVERRIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHY_DIRECT_OVERRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHY_DIRECT_OVERRIDE` writer"]
pub struct W(crate::W<USBPHY_DIRECT_OVERRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHY_DIRECT_OVERRIDE_SPEC>;
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
impl From<crate::W<USBPHY_DIRECT_OVERRIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHY_DIRECT_OVERRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_DIFFMODE_OVERRIDE_EN` reader - "]
pub struct TX_DIFFMODE_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl TX_DIFFMODE_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DIFFMODE_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DIFFMODE_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DIFFMODE_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `DM_PULLUP_OVERRIDE_EN` reader - "]
pub struct DM_PULLUP_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl DM_PULLUP_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM_PULLUP_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM_PULLUP_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM_PULLUP_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TX_FSSLEW_OVERRIDE_EN` reader - "]
pub struct TX_FSSLEW_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl TX_FSSLEW_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FSSLEW_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FSSLEW_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FSSLEW_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TX_PD_OVERRIDE_EN` reader - "]
pub struct TX_PD_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl TX_PD_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_PD_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PD_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PD_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `RX_PD_OVERRIDE_EN` reader - "]
pub struct RX_PD_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl RX_PD_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_PD_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PD_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PD_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TX_DM_OVERRIDE_EN` reader - "]
pub struct TX_DM_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl TX_DM_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DM_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DM_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DM_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TX_DP_OVERRIDE_EN` reader - "]
pub struct TX_DP_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl TX_DP_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DP_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DP_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DP_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `TX_DM_OE_OVERRIDE_EN` reader - "]
pub struct TX_DM_OE_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl TX_DM_OE_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DM_OE_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DM_OE_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DM_OE_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TX_DP_OE_OVERRIDE_EN` reader - "]
pub struct TX_DP_OE_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl TX_DP_OE_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DP_OE_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DP_OE_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DP_OE_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DM_PULLDN_EN_OVERRIDE_EN` reader - "]
pub struct DM_PULLDN_EN_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl DM_PULLDN_EN_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM_PULLDN_EN_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM_PULLDN_EN_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM_PULLDN_EN_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DP_PULLDN_EN_OVERRIDE_EN` reader - "]
pub struct DP_PULLDN_EN_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl DP_PULLDN_EN_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DP_PULLDN_EN_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DP_PULLDN_EN_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DP_PULLDN_EN_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DP_PULLUP_EN_OVERRIDE_EN` reader - "]
pub struct DP_PULLUP_EN_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl DP_PULLUP_EN_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DP_PULLUP_EN_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DP_PULLUP_EN_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DP_PULLUP_EN_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DM_PULLUP_HISEL_OVERRIDE_EN` reader - "]
pub struct DM_PULLUP_HISEL_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl DM_PULLUP_HISEL_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM_PULLUP_HISEL_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM_PULLUP_HISEL_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM_PULLUP_HISEL_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DP_PULLUP_HISEL_OVERRIDE_EN` reader - "]
pub struct DP_PULLUP_HISEL_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl DP_PULLUP_HISEL_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DP_PULLUP_HISEL_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DP_PULLUP_HISEL_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DP_PULLUP_HISEL_OVERRIDE_EN` writer - "]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Override enable for each control in usbphy_direct  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [usbphy_direct_override](index.html) module"]
pub struct USBPHY_DIRECT_OVERRIDE_SPEC;
impl crate::RegisterSpec for USBPHY_DIRECT_OVERRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbphy_direct_override::R](R) reader structure"]
impl crate::Readable for USBPHY_DIRECT_OVERRIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphy_direct_override::W](W) writer structure"]
impl crate::Writable for USBPHY_DIRECT_OVERRIDE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPHY_DIRECT_OVERRIDE to value 0"]
impl crate::Resettable for USBPHY_DIRECT_OVERRIDE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
