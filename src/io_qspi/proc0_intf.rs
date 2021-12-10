#[doc = "Register `PROC0_INTF` reader"]
pub struct R(crate::R<PROC0_INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC0_INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC0_INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC0_INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROC0_INTF` writer"]
pub struct W(crate::W<PROC0_INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROC0_INTF_SPEC>;
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
impl From<crate::W<PROC0_INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROC0_INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_QSPI_SD3_EDGE_HIGH` reader - "]
pub struct GPIO_QSPI_SD3_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD3_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD3_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD3_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD3_EDGE_HIGH` writer - "]
pub struct GPIO_QSPI_SD3_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD3_EDGE_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `GPIO_QSPI_SD3_EDGE_LOW` reader - "]
pub struct GPIO_QSPI_SD3_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD3_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD3_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD3_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD3_EDGE_LOW` writer - "]
pub struct GPIO_QSPI_SD3_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD3_EDGE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `GPIO_QSPI_SD3_LEVEL_HIGH` reader - "]
pub struct GPIO_QSPI_SD3_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD3_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD3_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD3_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD3_LEVEL_HIGH` writer - "]
pub struct GPIO_QSPI_SD3_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD3_LEVEL_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `GPIO_QSPI_SD3_LEVEL_LOW` reader - "]
pub struct GPIO_QSPI_SD3_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD3_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD3_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD3_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD3_LEVEL_LOW` writer - "]
pub struct GPIO_QSPI_SD3_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD3_LEVEL_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `GPIO_QSPI_SD2_EDGE_HIGH` reader - "]
pub struct GPIO_QSPI_SD2_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD2_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD2_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD2_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD2_EDGE_HIGH` writer - "]
pub struct GPIO_QSPI_SD2_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD2_EDGE_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `GPIO_QSPI_SD2_EDGE_LOW` reader - "]
pub struct GPIO_QSPI_SD2_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD2_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD2_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD2_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD2_EDGE_LOW` writer - "]
pub struct GPIO_QSPI_SD2_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD2_EDGE_LOW_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SD2_LEVEL_HIGH` reader - "]
pub struct GPIO_QSPI_SD2_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD2_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD2_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD2_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD2_LEVEL_HIGH` writer - "]
pub struct GPIO_QSPI_SD2_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD2_LEVEL_HIGH_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SD2_LEVEL_LOW` reader - "]
pub struct GPIO_QSPI_SD2_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD2_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD2_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD2_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD2_LEVEL_LOW` writer - "]
pub struct GPIO_QSPI_SD2_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD2_LEVEL_LOW_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SD1_EDGE_HIGH` reader - "]
pub struct GPIO_QSPI_SD1_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD1_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD1_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD1_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD1_EDGE_HIGH` writer - "]
pub struct GPIO_QSPI_SD1_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD1_EDGE_HIGH_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SD1_EDGE_LOW` reader - "]
pub struct GPIO_QSPI_SD1_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD1_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD1_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD1_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD1_EDGE_LOW` writer - "]
pub struct GPIO_QSPI_SD1_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD1_EDGE_LOW_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SD1_LEVEL_HIGH` reader - "]
pub struct GPIO_QSPI_SD1_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD1_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD1_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD1_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD1_LEVEL_HIGH` writer - "]
pub struct GPIO_QSPI_SD1_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD1_LEVEL_HIGH_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SD1_LEVEL_LOW` reader - "]
pub struct GPIO_QSPI_SD1_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD1_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD1_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD1_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD1_LEVEL_LOW` writer - "]
pub struct GPIO_QSPI_SD1_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD1_LEVEL_LOW_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SD0_EDGE_HIGH` reader - "]
pub struct GPIO_QSPI_SD0_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD0_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD0_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD0_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD0_EDGE_HIGH` writer - "]
pub struct GPIO_QSPI_SD0_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD0_EDGE_HIGH_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SD0_EDGE_LOW` reader - "]
pub struct GPIO_QSPI_SD0_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD0_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD0_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD0_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD0_EDGE_LOW` writer - "]
pub struct GPIO_QSPI_SD0_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD0_EDGE_LOW_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SD0_LEVEL_HIGH` reader - "]
pub struct GPIO_QSPI_SD0_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD0_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD0_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD0_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD0_LEVEL_HIGH` writer - "]
pub struct GPIO_QSPI_SD0_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD0_LEVEL_HIGH_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SD0_LEVEL_LOW` reader - "]
pub struct GPIO_QSPI_SD0_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SD0_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SD0_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SD0_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SD0_LEVEL_LOW` writer - "]
pub struct GPIO_QSPI_SD0_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SD0_LEVEL_LOW_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SS_EDGE_HIGH` reader - "]
pub struct GPIO_QSPI_SS_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SS_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SS_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SS_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SS_EDGE_HIGH` writer - "]
pub struct GPIO_QSPI_SS_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SS_EDGE_HIGH_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SS_EDGE_LOW` reader - "]
pub struct GPIO_QSPI_SS_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SS_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SS_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SS_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SS_EDGE_LOW` writer - "]
pub struct GPIO_QSPI_SS_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SS_EDGE_LOW_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SS_LEVEL_HIGH` reader - "]
pub struct GPIO_QSPI_SS_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SS_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SS_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SS_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SS_LEVEL_HIGH` writer - "]
pub struct GPIO_QSPI_SS_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SS_LEVEL_HIGH_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SS_LEVEL_LOW` reader - "]
pub struct GPIO_QSPI_SS_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SS_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SS_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SS_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SS_LEVEL_LOW` writer - "]
pub struct GPIO_QSPI_SS_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SS_LEVEL_LOW_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SCLK_EDGE_HIGH` reader - "]
pub struct GPIO_QSPI_SCLK_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SCLK_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SCLK_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SCLK_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SCLK_EDGE_HIGH` writer - "]
pub struct GPIO_QSPI_SCLK_EDGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SCLK_EDGE_HIGH_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SCLK_EDGE_LOW` reader - "]
pub struct GPIO_QSPI_SCLK_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SCLK_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SCLK_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SCLK_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SCLK_EDGE_LOW` writer - "]
pub struct GPIO_QSPI_SCLK_EDGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SCLK_EDGE_LOW_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SCLK_LEVEL_HIGH` reader - "]
pub struct GPIO_QSPI_SCLK_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SCLK_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SCLK_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SCLK_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SCLK_LEVEL_HIGH` writer - "]
pub struct GPIO_QSPI_SCLK_LEVEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SCLK_LEVEL_HIGH_W<'a> {
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
#[doc = "Field `GPIO_QSPI_SCLK_LEVEL_LOW` reader - "]
pub struct GPIO_QSPI_SCLK_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO_QSPI_SCLK_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_QSPI_SCLK_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_QSPI_SCLK_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_QSPI_SCLK_LEVEL_LOW` writer - "]
pub struct GPIO_QSPI_SCLK_LEVEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_QSPI_SCLK_LEVEL_LOW_W<'a> {
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
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> GPIO_QSPI_SD3_EDGE_HIGH_R {
        GPIO_QSPI_SD3_EDGE_HIGH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> GPIO_QSPI_SD3_EDGE_LOW_R {
        GPIO_QSPI_SD3_EDGE_LOW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> GPIO_QSPI_SD3_LEVEL_HIGH_R {
        GPIO_QSPI_SD3_LEVEL_HIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> GPIO_QSPI_SD3_LEVEL_LOW_R {
        GPIO_QSPI_SD3_LEVEL_LOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> GPIO_QSPI_SD2_EDGE_HIGH_R {
        GPIO_QSPI_SD2_EDGE_HIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> GPIO_QSPI_SD2_EDGE_LOW_R {
        GPIO_QSPI_SD2_EDGE_LOW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> GPIO_QSPI_SD2_LEVEL_HIGH_R {
        GPIO_QSPI_SD2_LEVEL_HIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> GPIO_QSPI_SD2_LEVEL_LOW_R {
        GPIO_QSPI_SD2_LEVEL_LOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> GPIO_QSPI_SD1_EDGE_HIGH_R {
        GPIO_QSPI_SD1_EDGE_HIGH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> GPIO_QSPI_SD1_EDGE_LOW_R {
        GPIO_QSPI_SD1_EDGE_LOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> GPIO_QSPI_SD1_LEVEL_HIGH_R {
        GPIO_QSPI_SD1_LEVEL_HIGH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> GPIO_QSPI_SD1_LEVEL_LOW_R {
        GPIO_QSPI_SD1_LEVEL_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> GPIO_QSPI_SD0_EDGE_HIGH_R {
        GPIO_QSPI_SD0_EDGE_HIGH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> GPIO_QSPI_SD0_EDGE_LOW_R {
        GPIO_QSPI_SD0_EDGE_LOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> GPIO_QSPI_SD0_LEVEL_HIGH_R {
        GPIO_QSPI_SD0_LEVEL_HIGH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> GPIO_QSPI_SD0_LEVEL_LOW_R {
        GPIO_QSPI_SD0_LEVEL_LOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> GPIO_QSPI_SS_EDGE_HIGH_R {
        GPIO_QSPI_SS_EDGE_HIGH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> GPIO_QSPI_SS_EDGE_LOW_R {
        GPIO_QSPI_SS_EDGE_LOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> GPIO_QSPI_SS_LEVEL_HIGH_R {
        GPIO_QSPI_SS_LEVEL_HIGH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> GPIO_QSPI_SS_LEVEL_LOW_R {
        GPIO_QSPI_SS_LEVEL_LOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> GPIO_QSPI_SCLK_EDGE_HIGH_R {
        GPIO_QSPI_SCLK_EDGE_HIGH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> GPIO_QSPI_SCLK_EDGE_LOW_R {
        GPIO_QSPI_SCLK_EDGE_LOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> GPIO_QSPI_SCLK_LEVEL_HIGH_R {
        GPIO_QSPI_SCLK_LEVEL_HIGH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> GPIO_QSPI_SCLK_LEVEL_LOW_R {
        GPIO_QSPI_SCLK_LEVEL_LOW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&mut self) -> GPIO_QSPI_SD3_EDGE_HIGH_W {
        GPIO_QSPI_SD3_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&mut self) -> GPIO_QSPI_SD3_EDGE_LOW_W {
        GPIO_QSPI_SD3_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&mut self) -> GPIO_QSPI_SD3_LEVEL_HIGH_W {
        GPIO_QSPI_SD3_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&mut self) -> GPIO_QSPI_SD3_LEVEL_LOW_W {
        GPIO_QSPI_SD3_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&mut self) -> GPIO_QSPI_SD2_EDGE_HIGH_W {
        GPIO_QSPI_SD2_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&mut self) -> GPIO_QSPI_SD2_EDGE_LOW_W {
        GPIO_QSPI_SD2_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&mut self) -> GPIO_QSPI_SD2_LEVEL_HIGH_W {
        GPIO_QSPI_SD2_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&mut self) -> GPIO_QSPI_SD2_LEVEL_LOW_W {
        GPIO_QSPI_SD2_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&mut self) -> GPIO_QSPI_SD1_EDGE_HIGH_W {
        GPIO_QSPI_SD1_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&mut self) -> GPIO_QSPI_SD1_EDGE_LOW_W {
        GPIO_QSPI_SD1_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&mut self) -> GPIO_QSPI_SD1_LEVEL_HIGH_W {
        GPIO_QSPI_SD1_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&mut self) -> GPIO_QSPI_SD1_LEVEL_LOW_W {
        GPIO_QSPI_SD1_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&mut self) -> GPIO_QSPI_SD0_EDGE_HIGH_W {
        GPIO_QSPI_SD0_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&mut self) -> GPIO_QSPI_SD0_EDGE_LOW_W {
        GPIO_QSPI_SD0_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&mut self) -> GPIO_QSPI_SD0_LEVEL_HIGH_W {
        GPIO_QSPI_SD0_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&mut self) -> GPIO_QSPI_SD0_LEVEL_LOW_W {
        GPIO_QSPI_SD0_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&mut self) -> GPIO_QSPI_SS_EDGE_HIGH_W {
        GPIO_QSPI_SS_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&mut self) -> GPIO_QSPI_SS_EDGE_LOW_W {
        GPIO_QSPI_SS_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&mut self) -> GPIO_QSPI_SS_LEVEL_HIGH_W {
        GPIO_QSPI_SS_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&mut self) -> GPIO_QSPI_SS_LEVEL_LOW_W {
        GPIO_QSPI_SS_LEVEL_LOW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&mut self) -> GPIO_QSPI_SCLK_EDGE_HIGH_W {
        GPIO_QSPI_SCLK_EDGE_HIGH_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&mut self) -> GPIO_QSPI_SCLK_EDGE_LOW_W {
        GPIO_QSPI_SCLK_EDGE_LOW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&mut self) -> GPIO_QSPI_SCLK_LEVEL_HIGH_W {
        GPIO_QSPI_SCLK_LEVEL_HIGH_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&mut self) -> GPIO_QSPI_SCLK_LEVEL_LOW_W {
        GPIO_QSPI_SCLK_LEVEL_LOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Force for proc0  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [proc0_intf](index.html) module"]
pub struct PROC0_INTF_SPEC;
impl crate::RegisterSpec for PROC0_INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc0_intf::R](R) reader structure"]
impl crate::Readable for PROC0_INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [proc0_intf::W](W) writer structure"]
impl crate::Writable for PROC0_INTF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROC0_INTF to value 0"]
impl crate::Resettable for PROC0_INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
