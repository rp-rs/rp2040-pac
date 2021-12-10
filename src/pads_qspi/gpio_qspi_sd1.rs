#[doc = "Register `GPIO_QSPI_SD1` reader"]
pub struct R(crate::R<GPIO_QSPI_SD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_QSPI_SD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_QSPI_SD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_QSPI_SD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_QSPI_SD1` writer"]
pub struct W(crate::W<GPIO_QSPI_SD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_QSPI_SD1_SPEC>;
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
impl From<crate::W<GPIO_QSPI_SD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_QSPI_SD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OD` reader - Output disable. Has priority over output enable from peripherals"]
pub struct OD_R(crate::FieldReader<bool, bool>);
impl OD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD` writer - Output disable. Has priority over output enable from peripherals"]
pub struct OD_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_W<'a> {
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
#[doc = "Field `IE` reader - Input enable"]
pub struct IE_R(crate::FieldReader<bool, bool>);
impl IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE` writer - Input enable"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
#[doc = "Drive strength.  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVE_A {
    #[doc = "0: `0`"]
    _2MA = 0,
    #[doc = "1: `1`"]
    _4MA = 1,
    #[doc = "2: `10`"]
    _8MA = 2,
    #[doc = "3: `11`"]
    _12MA = 3,
}
impl From<DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DRIVE` reader - Drive strength."]
pub struct DRIVE_R(crate::FieldReader<u8, DRIVE_A>);
impl DRIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVE_A {
        match self.bits {
            0 => DRIVE_A::_2MA,
            1 => DRIVE_A::_4MA,
            2 => DRIVE_A::_8MA,
            3 => DRIVE_A::_12MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2MA`"]
    #[inline(always)]
    pub fn is_2m_a(&self) -> bool {
        **self == DRIVE_A::_2MA
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline(always)]
    pub fn is_4m_a(&self) -> bool {
        **self == DRIVE_A::_4MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline(always)]
    pub fn is_8m_a(&self) -> bool {
        **self == DRIVE_A::_8MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline(always)]
    pub fn is_12m_a(&self) -> bool {
        **self == DRIVE_A::_12MA
    }
}
impl core::ops::Deref for DRIVE_R {
    type Target = crate::FieldReader<u8, DRIVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIVE` writer - Drive strength."]
pub struct DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIVE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _2m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_2MA)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _4m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_4MA)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _8m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_8MA)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _12m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_12MA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `PUE` reader - Pull up enable"]
pub struct PUE_R(crate::FieldReader<bool, bool>);
impl PUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUE` writer - Pull up enable"]
pub struct PUE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUE_W<'a> {
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
#[doc = "Field `PDE` reader - Pull down enable"]
pub struct PDE_R(crate::FieldReader<bool, bool>);
impl PDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDE` writer - Pull down enable"]
pub struct PDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDE_W<'a> {
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
#[doc = "Field `SCHMITT` reader - Enable schmitt trigger"]
pub struct SCHMITT_R(crate::FieldReader<bool, bool>);
impl SCHMITT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT` writer - Enable schmitt trigger"]
pub struct SCHMITT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT_W<'a> {
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
#[doc = "Field `SLEWFAST` reader - Slew rate control. 1 = Fast, 0 = Slow"]
pub struct SLEWFAST_R(crate::FieldReader<bool, bool>);
impl SLEWFAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEWFAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEWFAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEWFAST` writer - Slew rate control. 1 = Fast, 0 = Slow"]
pub struct SLEWFAST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEWFAST_W<'a> {
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
    #[doc = "Bit 7 - Output disable. Has priority over output enable from peripherals"]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Input enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Drive strength."]
    #[inline(always)]
    pub fn drive(&self) -> DRIVE_R {
        DRIVE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Pull up enable"]
    #[inline(always)]
    pub fn pue(&self) -> PUE_R {
        PUE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pull down enable"]
    #[inline(always)]
    pub fn pde(&self) -> PDE_R {
        PDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable schmitt trigger"]
    #[inline(always)]
    pub fn schmitt(&self) -> SCHMITT_R {
        SCHMITT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Slew rate control. 1 = Fast, 0 = Slow"]
    #[inline(always)]
    pub fn slewfast(&self) -> SLEWFAST_R {
        SLEWFAST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Output disable. Has priority over output enable from peripherals"]
    #[inline(always)]
    pub fn od(&mut self) -> OD_W {
        OD_W { w: self }
    }
    #[doc = "Bit 6 - Input enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bits 4:5 - Drive strength."]
    #[inline(always)]
    pub fn drive(&mut self) -> DRIVE_W {
        DRIVE_W { w: self }
    }
    #[doc = "Bit 3 - Pull up enable"]
    #[inline(always)]
    pub fn pue(&mut self) -> PUE_W {
        PUE_W { w: self }
    }
    #[doc = "Bit 2 - Pull down enable"]
    #[inline(always)]
    pub fn pde(&mut self) -> PDE_W {
        PDE_W { w: self }
    }
    #[doc = "Bit 1 - Enable schmitt trigger"]
    #[inline(always)]
    pub fn schmitt(&mut self) -> SCHMITT_W {
        SCHMITT_W { w: self }
    }
    #[doc = "Bit 0 - Slew rate control. 1 = Fast, 0 = Slow"]
    #[inline(always)]
    pub fn slewfast(&mut self) -> SLEWFAST_W {
        SLEWFAST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad control register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_qspi_sd1](index.html) module"]
pub struct GPIO_QSPI_SD1_SPEC;
impl crate::RegisterSpec for GPIO_QSPI_SD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_qspi_sd1::R](R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sd1::W](W) writer structure"]
impl crate::Writable for GPIO_QSPI_SD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_QSPI_SD1 to value 0x52"]
impl crate::Resettable for GPIO_QSPI_SD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x52
    }
}
