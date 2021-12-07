#[doc = "Register `FRCE_OFF` reader"]
pub struct R(crate::R<FRCE_OFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRCE_OFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRCE_OFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRCE_OFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRCE_OFF` writer"]
pub struct W(crate::W<FRCE_OFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRCE_OFF_SPEC>;
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
impl From<crate::W<FRCE_OFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRCE_OFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `proc1` reader - "]
pub struct PROC1_R(crate::FieldReader<bool, bool>);
impl PROC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `proc1` writer - "]
pub struct PROC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_W<'a> {
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
#[doc = "Field `proc0` reader - "]
pub struct PROC0_R(crate::FieldReader<bool, bool>);
impl PROC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `proc0` writer - "]
pub struct PROC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_W<'a> {
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
#[doc = "Field `sio` reader - "]
pub struct SIO_R(crate::FieldReader<bool, bool>);
impl SIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sio` writer - "]
pub struct SIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `vreg_and_chip_reset` reader - "]
pub struct VREG_AND_CHIP_RESET_R(crate::FieldReader<bool, bool>);
impl VREG_AND_CHIP_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VREG_AND_CHIP_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREG_AND_CHIP_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vreg_and_chip_reset` writer - "]
pub struct VREG_AND_CHIP_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG_AND_CHIP_RESET_W<'a> {
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
#[doc = "Field `xip` reader - "]
pub struct XIP_R(crate::FieldReader<bool, bool>);
impl XIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xip` writer - "]
pub struct XIP_W<'a> {
    w: &'a mut W,
}
impl<'a> XIP_W<'a> {
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
#[doc = "Field `sram5` reader - "]
pub struct SRAM5_R(crate::FieldReader<bool, bool>);
impl SRAM5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram5` writer - "]
pub struct SRAM5_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM5_W<'a> {
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
#[doc = "Field `sram4` reader - "]
pub struct SRAM4_R(crate::FieldReader<bool, bool>);
impl SRAM4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram4` writer - "]
pub struct SRAM4_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM4_W<'a> {
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
#[doc = "Field `sram3` reader - "]
pub struct SRAM3_R(crate::FieldReader<bool, bool>);
impl SRAM3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram3` writer - "]
pub struct SRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3_W<'a> {
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
#[doc = "Field `sram2` reader - "]
pub struct SRAM2_R(crate::FieldReader<bool, bool>);
impl SRAM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram2` writer - "]
pub struct SRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2_W<'a> {
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
#[doc = "Field `sram1` reader - "]
pub struct SRAM1_R(crate::FieldReader<bool, bool>);
impl SRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram1` writer - "]
pub struct SRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1_W<'a> {
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
#[doc = "Field `sram0` reader - "]
pub struct SRAM0_R(crate::FieldReader<bool, bool>);
impl SRAM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram0` writer - "]
pub struct SRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0_W<'a> {
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
#[doc = "Field `rom` reader - "]
pub struct ROM_R(crate::FieldReader<bool, bool>);
impl ROM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rom` writer - "]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
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
#[doc = "Field `busfabric` reader - "]
pub struct BUSFABRIC_R(crate::FieldReader<bool, bool>);
impl BUSFABRIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSFABRIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSFABRIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `busfabric` writer - "]
pub struct BUSFABRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFABRIC_W<'a> {
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
#[doc = "Field `resets` reader - "]
pub struct RESETS_R(crate::FieldReader<bool, bool>);
impl RESETS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESETS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESETS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `resets` writer - "]
pub struct RESETS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETS_W<'a> {
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
#[doc = "Field `clocks` reader - "]
pub struct CLOCKS_R(crate::FieldReader<bool, bool>);
impl CLOCKS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOCKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clocks` writer - "]
pub struct CLOCKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKS_W<'a> {
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
#[doc = "Field `xosc` reader - "]
pub struct XOSC_R(crate::FieldReader<bool, bool>);
impl XOSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xosc` writer - "]
pub struct XOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_W<'a> {
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
#[doc = "Field `rosc` reader - "]
pub struct ROSC_R(crate::FieldReader<bool, bool>);
impl ROSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosc` writer - "]
pub struct ROSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSC_W<'a> {
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
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn proc1(&self) -> PROC1_R {
        PROC1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn proc0(&self) -> PROC0_R {
        PROC0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sio(&self) -> SIO_R {
        SIO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn vreg_and_chip_reset(&self) -> VREG_AND_CHIP_RESET_R {
        VREG_AND_CHIP_RESET_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xip(&self) -> XIP_R {
        XIP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sram5(&self) -> SRAM5_R {
        SRAM5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sram4(&self) -> SRAM4_R {
        SRAM4_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sram0(&self) -> SRAM0_R {
        SRAM0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busfabric(&self) -> BUSFABRIC_R {
        BUSFABRIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn resets(&self) -> RESETS_R {
        RESETS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clocks(&self) -> CLOCKS_R {
        CLOCKS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xosc(&self) -> XOSC_R {
        XOSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rosc(&self) -> ROSC_R {
        ROSC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn proc1(&mut self) -> PROC1_W {
        PROC1_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn proc0(&mut self) -> PROC0_W {
        PROC0_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sio(&mut self) -> SIO_W {
        SIO_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn vreg_and_chip_reset(&mut self) -> VREG_AND_CHIP_RESET_W {
        VREG_AND_CHIP_RESET_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xip(&mut self) -> XIP_W {
        XIP_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sram5(&mut self) -> SRAM5_W {
        SRAM5_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sram4(&mut self) -> SRAM4_W {
        SRAM4_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sram3(&mut self) -> SRAM3_W {
        SRAM3_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sram2(&mut self) -> SRAM2_W {
        SRAM2_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sram1(&mut self) -> SRAM1_W {
        SRAM1_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sram0(&mut self) -> SRAM0_W {
        SRAM0_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busfabric(&mut self) -> BUSFABRIC_W {
        BUSFABRIC_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn resets(&mut self) -> RESETS_W {
        RESETS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clocks(&mut self) -> CLOCKS_W {
        CLOCKS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xosc(&mut self) -> XOSC_W {
        XOSC_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rosc(&mut self) -> ROSC_W {
        ROSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force into reset (i.e. power it off)  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [frce_off](index.html) module"]
pub struct FRCE_OFF_SPEC;
impl crate::RegisterSpec for FRCE_OFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frce_off::R](R) reader structure"]
impl crate::Readable for FRCE_OFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frce_off::W](W) writer structure"]
impl crate::Writable for FRCE_OFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRCE_OFF to value 0"]
impl crate::Resettable for FRCE_OFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
