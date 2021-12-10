#[doc = "Register `SIE_CTRL` reader"]
pub struct R(crate::R<SIE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIE_CTRL` writer"]
pub struct W(crate::W<SIE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIE_CTRL_SPEC>;
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
impl From<crate::W<SIE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP0_INT_STALL` reader - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
pub struct EP0_INT_STALL_R(crate::FieldReader<bool, bool>);
impl EP0_INT_STALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_INT_STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_INT_STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0_INT_STALL` writer - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
pub struct EP0_INT_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INT_STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `EP0_DOUBLE_BUF` reader - Device: EP0 single buffered = 0, double buffered = 1"]
pub struct EP0_DOUBLE_BUF_R(crate::FieldReader<bool, bool>);
impl EP0_DOUBLE_BUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_DOUBLE_BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_DOUBLE_BUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0_DOUBLE_BUF` writer - Device: EP0 single buffered = 0, double buffered = 1"]
pub struct EP0_DOUBLE_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_DOUBLE_BUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `EP0_INT_1BUF` reader - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
pub struct EP0_INT_1BUF_R(crate::FieldReader<bool, bool>);
impl EP0_INT_1BUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_INT_1BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_INT_1BUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0_INT_1BUF` writer - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
pub struct EP0_INT_1BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INT_1BUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `EP0_INT_2BUF` reader - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
pub struct EP0_INT_2BUF_R(crate::FieldReader<bool, bool>);
impl EP0_INT_2BUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_INT_2BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_INT_2BUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0_INT_2BUF` writer - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
pub struct EP0_INT_2BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INT_2BUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `EP0_INT_NAK` reader - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
pub struct EP0_INT_NAK_R(crate::FieldReader<bool, bool>);
impl EP0_INT_NAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_INT_NAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_INT_NAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0_INT_NAK` writer - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
pub struct EP0_INT_NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INT_NAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `DIRECT_EN` reader - Direct bus drive enable"]
pub struct DIRECT_EN_R(crate::FieldReader<bool, bool>);
impl DIRECT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIRECT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRECT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRECT_EN` writer - Direct bus drive enable"]
pub struct DIRECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `DIRECT_DP` reader - Direct control of DP"]
pub struct DIRECT_DP_R(crate::FieldReader<bool, bool>);
impl DIRECT_DP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIRECT_DP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRECT_DP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRECT_DP` writer - Direct control of DP"]
pub struct DIRECT_DP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECT_DP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `DIRECT_DM` reader - Direct control of DM"]
pub struct DIRECT_DM_R(crate::FieldReader<bool, bool>);
impl DIRECT_DM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIRECT_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRECT_DM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRECT_DM` writer - Direct control of DM"]
pub struct DIRECT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECT_DM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `TRANSCEIVER_PD` reader - Power down bus transceiver"]
pub struct TRANSCEIVER_PD_R(crate::FieldReader<bool, bool>);
impl TRANSCEIVER_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANSCEIVER_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSCEIVER_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANSCEIVER_PD` writer - Power down bus transceiver"]
pub struct TRANSCEIVER_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSCEIVER_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RPU_OPT` reader - Device: Pull-up strength (0=1K2, 1=2k3)"]
pub struct RPU_OPT_R(crate::FieldReader<bool, bool>);
impl RPU_OPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RPU_OPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPU_OPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPU_OPT` writer - Device: Pull-up strength (0=1K2, 1=2k3)"]
pub struct RPU_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> RPU_OPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PULLUP_EN` reader - Device: Enable pull up resistor"]
pub struct PULLUP_EN_R(crate::FieldReader<bool, bool>);
impl PULLUP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULLUP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULLUP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULLUP_EN` writer - Device: Enable pull up resistor"]
pub struct PULLUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLUP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PULLDOWN_EN` reader - Host: Enable pull down resistors"]
pub struct PULLDOWN_EN_R(crate::FieldReader<bool, bool>);
impl PULLDOWN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULLDOWN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULLDOWN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULLDOWN_EN` writer - Host: Enable pull down resistors"]
pub struct PULLDOWN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLDOWN_EN_W<'a> {
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
#[doc = "Field `RESET_BUS` reader - Host: Reset bus"]
pub struct RESET_BUS_R(crate::FieldReader<bool, bool>);
impl RESET_BUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_BUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_BUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_BUS` writer - Host: Reset bus"]
pub struct RESET_BUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_BUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `RESUME` reader - Device: Remote wakeup. Device can initiate its own resume after suspend."]
pub struct RESUME_R(crate::FieldReader<bool, bool>);
impl RESUME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME` writer - Device: Remote wakeup. Device can initiate its own resume after suspend."]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
#[doc = "Field `VBUS_EN` reader - Host: Enable VBUS"]
pub struct VBUS_EN_R(crate::FieldReader<bool, bool>);
impl VBUS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUS_EN` writer - Host: Enable VBUS"]
pub struct VBUS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_EN_W<'a> {
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
#[doc = "Field `KEEP_ALIVE_EN` reader - Host: Enable keep alive packet (for low speed bus)"]
pub struct KEEP_ALIVE_EN_R(crate::FieldReader<bool, bool>);
impl KEEP_ALIVE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEEP_ALIVE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEEP_ALIVE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEEP_ALIVE_EN` writer - Host: Enable keep alive packet (for low speed bus)"]
pub struct KEEP_ALIVE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KEEP_ALIVE_EN_W<'a> {
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
#[doc = "Field `SOF_EN` reader - Host: Enable SOF generation (for full speed bus)"]
pub struct SOF_EN_R(crate::FieldReader<bool, bool>);
impl SOF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF_EN` writer - Host: Enable SOF generation (for full speed bus)"]
pub struct SOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_EN_W<'a> {
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
#[doc = "Field `SOF_SYNC` reader - Host: Delay packet(s) until after SOF"]
pub struct SOF_SYNC_R(crate::FieldReader<bool, bool>);
impl SOF_SYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOF_SYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_SYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF_SYNC` writer - Host: Delay packet(s) until after SOF"]
pub struct SOF_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_SYNC_W<'a> {
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
#[doc = "Field `PREAMBLE_EN` reader - Host: Preable enable for LS device on FS hub"]
pub struct PREAMBLE_EN_R(crate::FieldReader<bool, bool>);
impl PREAMBLE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREAMBLE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREAMBLE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREAMBLE_EN` writer - Host: Preable enable for LS device on FS hub"]
pub struct PREAMBLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREAMBLE_EN_W<'a> {
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
#[doc = "Field `STOP_TRANS` reader - Host: Stop transaction"]
pub struct STOP_TRANS_R(crate::FieldReader<bool, bool>);
impl STOP_TRANS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_TRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_TRANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_TRANS` writer - Host: Stop transaction"]
pub struct STOP_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_TRANS_W<'a> {
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
#[doc = "Field `RECEIVE_DATA` reader - Host: Receive transaction (IN to host)"]
pub struct RECEIVE_DATA_R(crate::FieldReader<bool, bool>);
impl RECEIVE_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECEIVE_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECEIVE_DATA` writer - Host: Receive transaction (IN to host)"]
pub struct RECEIVE_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_DATA_W<'a> {
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
#[doc = "Field `SEND_DATA` reader - Host: Send transaction (OUT from host)"]
pub struct SEND_DATA_R(crate::FieldReader<bool, bool>);
impl SEND_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_DATA` writer - Host: Send transaction (OUT from host)"]
pub struct SEND_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_DATA_W<'a> {
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
#[doc = "Field `SEND_SETUP` reader - Host: Send Setup packet"]
pub struct SEND_SETUP_R(crate::FieldReader<bool, bool>);
impl SEND_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_SETUP` writer - Host: Send Setup packet"]
pub struct SEND_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_SETUP_W<'a> {
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
#[doc = "Field `START_TRANS` reader - Host: Start transaction"]
pub struct START_TRANS_R(crate::FieldReader<bool, bool>);
impl START_TRANS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_TRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_TRANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_TRANS` writer - Host: Start transaction"]
pub struct START_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> START_TRANS_W<'a> {
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
    #[doc = "Bit 31 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
    #[inline(always)]
    pub fn ep0_int_stall(&self) -> EP0_INT_STALL_R {
        EP0_INT_STALL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Device: EP0 single buffered = 0, double buffered = 1"]
    #[inline(always)]
    pub fn ep0_double_buf(&self) -> EP0_DOUBLE_BUF_R {
        EP0_DOUBLE_BUF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
    #[inline(always)]
    pub fn ep0_int_1buf(&self) -> EP0_INT_1BUF_R {
        EP0_INT_1BUF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
    #[inline(always)]
    pub fn ep0_int_2buf(&self) -> EP0_INT_2BUF_R {
        EP0_INT_2BUF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
    #[inline(always)]
    pub fn ep0_int_nak(&self) -> EP0_INT_NAK_R {
        EP0_INT_NAK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Direct bus drive enable"]
    #[inline(always)]
    pub fn direct_en(&self) -> DIRECT_EN_R {
        DIRECT_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Direct control of DP"]
    #[inline(always)]
    pub fn direct_dp(&self) -> DIRECT_DP_R {
        DIRECT_DP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Direct control of DM"]
    #[inline(always)]
    pub fn direct_dm(&self) -> DIRECT_DM_R {
        DIRECT_DM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Power down bus transceiver"]
    #[inline(always)]
    pub fn transceiver_pd(&self) -> TRANSCEIVER_PD_R {
        TRANSCEIVER_PD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Device: Pull-up strength (0=1K2, 1=2k3)"]
    #[inline(always)]
    pub fn rpu_opt(&self) -> RPU_OPT_R {
        RPU_OPT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Device: Enable pull up resistor"]
    #[inline(always)]
    pub fn pullup_en(&self) -> PULLUP_EN_R {
        PULLUP_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Host: Enable pull down resistors"]
    #[inline(always)]
    pub fn pulldown_en(&self) -> PULLDOWN_EN_R {
        PULLDOWN_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Host: Reset bus"]
    #[inline(always)]
    pub fn reset_bus(&self) -> RESET_BUS_R {
        RESET_BUS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Device: Remote wakeup. Device can initiate its own resume after suspend."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Host: Enable VBUS"]
    #[inline(always)]
    pub fn vbus_en(&self) -> VBUS_EN_R {
        VBUS_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Host: Enable keep alive packet (for low speed bus)"]
    #[inline(always)]
    pub fn keep_alive_en(&self) -> KEEP_ALIVE_EN_R {
        KEEP_ALIVE_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Host: Enable SOF generation (for full speed bus)"]
    #[inline(always)]
    pub fn sof_en(&self) -> SOF_EN_R {
        SOF_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Host: Delay packet(s) until after SOF"]
    #[inline(always)]
    pub fn sof_sync(&self) -> SOF_SYNC_R {
        SOF_SYNC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Host: Preable enable for LS device on FS hub"]
    #[inline(always)]
    pub fn preamble_en(&self) -> PREAMBLE_EN_R {
        PREAMBLE_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Host: Stop transaction"]
    #[inline(always)]
    pub fn stop_trans(&self) -> STOP_TRANS_R {
        STOP_TRANS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Host: Receive transaction (IN to host)"]
    #[inline(always)]
    pub fn receive_data(&self) -> RECEIVE_DATA_R {
        RECEIVE_DATA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Host: Send transaction (OUT from host)"]
    #[inline(always)]
    pub fn send_data(&self) -> SEND_DATA_R {
        SEND_DATA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Host: Send Setup packet"]
    #[inline(always)]
    pub fn send_setup(&self) -> SEND_SETUP_R {
        SEND_SETUP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Host: Start transaction"]
    #[inline(always)]
    pub fn start_trans(&self) -> START_TRANS_R {
        START_TRANS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
    #[inline(always)]
    pub fn ep0_int_stall(&mut self) -> EP0_INT_STALL_W {
        EP0_INT_STALL_W { w: self }
    }
    #[doc = "Bit 30 - Device: EP0 single buffered = 0, double buffered = 1"]
    #[inline(always)]
    pub fn ep0_double_buf(&mut self) -> EP0_DOUBLE_BUF_W {
        EP0_DOUBLE_BUF_W { w: self }
    }
    #[doc = "Bit 29 - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
    #[inline(always)]
    pub fn ep0_int_1buf(&mut self) -> EP0_INT_1BUF_W {
        EP0_INT_1BUF_W { w: self }
    }
    #[doc = "Bit 28 - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
    #[inline(always)]
    pub fn ep0_int_2buf(&mut self) -> EP0_INT_2BUF_W {
        EP0_INT_2BUF_W { w: self }
    }
    #[doc = "Bit 27 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
    #[inline(always)]
    pub fn ep0_int_nak(&mut self) -> EP0_INT_NAK_W {
        EP0_INT_NAK_W { w: self }
    }
    #[doc = "Bit 26 - Direct bus drive enable"]
    #[inline(always)]
    pub fn direct_en(&mut self) -> DIRECT_EN_W {
        DIRECT_EN_W { w: self }
    }
    #[doc = "Bit 25 - Direct control of DP"]
    #[inline(always)]
    pub fn direct_dp(&mut self) -> DIRECT_DP_W {
        DIRECT_DP_W { w: self }
    }
    #[doc = "Bit 24 - Direct control of DM"]
    #[inline(always)]
    pub fn direct_dm(&mut self) -> DIRECT_DM_W {
        DIRECT_DM_W { w: self }
    }
    #[doc = "Bit 18 - Power down bus transceiver"]
    #[inline(always)]
    pub fn transceiver_pd(&mut self) -> TRANSCEIVER_PD_W {
        TRANSCEIVER_PD_W { w: self }
    }
    #[doc = "Bit 17 - Device: Pull-up strength (0=1K2, 1=2k3)"]
    #[inline(always)]
    pub fn rpu_opt(&mut self) -> RPU_OPT_W {
        RPU_OPT_W { w: self }
    }
    #[doc = "Bit 16 - Device: Enable pull up resistor"]
    #[inline(always)]
    pub fn pullup_en(&mut self) -> PULLUP_EN_W {
        PULLUP_EN_W { w: self }
    }
    #[doc = "Bit 15 - Host: Enable pull down resistors"]
    #[inline(always)]
    pub fn pulldown_en(&mut self) -> PULLDOWN_EN_W {
        PULLDOWN_EN_W { w: self }
    }
    #[doc = "Bit 13 - Host: Reset bus"]
    #[inline(always)]
    pub fn reset_bus(&mut self) -> RESET_BUS_W {
        RESET_BUS_W { w: self }
    }
    #[doc = "Bit 12 - Device: Remote wakeup. Device can initiate its own resume after suspend."]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 11 - Host: Enable VBUS"]
    #[inline(always)]
    pub fn vbus_en(&mut self) -> VBUS_EN_W {
        VBUS_EN_W { w: self }
    }
    #[doc = "Bit 10 - Host: Enable keep alive packet (for low speed bus)"]
    #[inline(always)]
    pub fn keep_alive_en(&mut self) -> KEEP_ALIVE_EN_W {
        KEEP_ALIVE_EN_W { w: self }
    }
    #[doc = "Bit 9 - Host: Enable SOF generation (for full speed bus)"]
    #[inline(always)]
    pub fn sof_en(&mut self) -> SOF_EN_W {
        SOF_EN_W { w: self }
    }
    #[doc = "Bit 8 - Host: Delay packet(s) until after SOF"]
    #[inline(always)]
    pub fn sof_sync(&mut self) -> SOF_SYNC_W {
        SOF_SYNC_W { w: self }
    }
    #[doc = "Bit 6 - Host: Preable enable for LS device on FS hub"]
    #[inline(always)]
    pub fn preamble_en(&mut self) -> PREAMBLE_EN_W {
        PREAMBLE_EN_W { w: self }
    }
    #[doc = "Bit 4 - Host: Stop transaction"]
    #[inline(always)]
    pub fn stop_trans(&mut self) -> STOP_TRANS_W {
        STOP_TRANS_W { w: self }
    }
    #[doc = "Bit 3 - Host: Receive transaction (IN to host)"]
    #[inline(always)]
    pub fn receive_data(&mut self) -> RECEIVE_DATA_W {
        RECEIVE_DATA_W { w: self }
    }
    #[doc = "Bit 2 - Host: Send transaction (OUT from host)"]
    #[inline(always)]
    pub fn send_data(&mut self) -> SEND_DATA_W {
        SEND_DATA_W { w: self }
    }
    #[doc = "Bit 1 - Host: Send Setup packet"]
    #[inline(always)]
    pub fn send_setup(&mut self) -> SEND_SETUP_W {
        SEND_SETUP_W { w: self }
    }
    #[doc = "Bit 0 - Host: Start transaction"]
    #[inline(always)]
    pub fn start_trans(&mut self) -> START_TRANS_W {
        START_TRANS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SIE control register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sie_ctrl](index.html) module"]
pub struct SIE_CTRL_SPEC;
impl crate::RegisterSpec for SIE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sie_ctrl::R](R) reader structure"]
impl crate::Readable for SIE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sie_ctrl::W](W) writer structure"]
impl crate::Writable for SIE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIE_CTRL to value 0"]
impl crate::Resettable for SIE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
