#[doc = "Register `WAKE_EN0` reader"]
pub struct R(crate::R<WAKE_EN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKE_EN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKE_EN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKE_EN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKE_EN0` writer"]
pub struct W(crate::W<WAKE_EN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKE_EN0_SPEC>;
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
impl From<crate::W<WAKE_EN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKE_EN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clk_sys_sram3` reader - "]
pub struct CLK_SYS_SRAM3_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_SRAM3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_SRAM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_SRAM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_sram3` writer - "]
pub struct CLK_SYS_SRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_SRAM3_W<'a> {
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
#[doc = "Field `clk_sys_sram2` reader - "]
pub struct CLK_SYS_SRAM2_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_SRAM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_SRAM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_SRAM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_sram2` writer - "]
pub struct CLK_SYS_SRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_SRAM2_W<'a> {
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
#[doc = "Field `clk_sys_sram1` reader - "]
pub struct CLK_SYS_SRAM1_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_SRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_SRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_SRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_sram1` writer - "]
pub struct CLK_SYS_SRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_SRAM1_W<'a> {
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
#[doc = "Field `clk_sys_sram0` reader - "]
pub struct CLK_SYS_SRAM0_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_SRAM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_SRAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_SRAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_sram0` writer - "]
pub struct CLK_SYS_SRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_SRAM0_W<'a> {
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
#[doc = "Field `clk_sys_spi1` reader - "]
pub struct CLK_SYS_SPI1_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_SPI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_SPI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_SPI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_spi1` writer - "]
pub struct CLK_SYS_SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_SPI1_W<'a> {
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
#[doc = "Field `clk_peri_spi1` reader - "]
pub struct CLK_PERI_SPI1_R(crate::FieldReader<bool, bool>);
impl CLK_PERI_SPI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_PERI_SPI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_PERI_SPI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_peri_spi1` writer - "]
pub struct CLK_PERI_SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_PERI_SPI1_W<'a> {
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
#[doc = "Field `clk_sys_spi0` reader - "]
pub struct CLK_SYS_SPI0_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_SPI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_SPI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_SPI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_spi0` writer - "]
pub struct CLK_SYS_SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_SPI0_W<'a> {
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
#[doc = "Field `clk_peri_spi0` reader - "]
pub struct CLK_PERI_SPI0_R(crate::FieldReader<bool, bool>);
impl CLK_PERI_SPI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_PERI_SPI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_PERI_SPI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_peri_spi0` writer - "]
pub struct CLK_PERI_SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_PERI_SPI0_W<'a> {
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
#[doc = "Field `clk_sys_sio` reader - "]
pub struct CLK_SYS_SIO_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_SIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_SIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_SIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_sio` writer - "]
pub struct CLK_SYS_SIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_SIO_W<'a> {
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
#[doc = "Field `clk_sys_rtc` reader - "]
pub struct CLK_SYS_RTC_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_RTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_RTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_rtc` writer - "]
pub struct CLK_SYS_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_RTC_W<'a> {
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
#[doc = "Field `clk_rtc_rtc` reader - "]
pub struct CLK_RTC_RTC_R(crate::FieldReader<bool, bool>);
impl CLK_RTC_RTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_RTC_RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_RTC_RTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_rtc_rtc` writer - "]
pub struct CLK_RTC_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_RTC_RTC_W<'a> {
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
#[doc = "Field `clk_sys_rosc` reader - "]
pub struct CLK_SYS_ROSC_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_ROSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_ROSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_ROSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_rosc` writer - "]
pub struct CLK_SYS_ROSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_ROSC_W<'a> {
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
#[doc = "Field `clk_sys_rom` reader - "]
pub struct CLK_SYS_ROM_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_ROM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_ROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_ROM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_rom` writer - "]
pub struct CLK_SYS_ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_ROM_W<'a> {
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
#[doc = "Field `clk_sys_resets` reader - "]
pub struct CLK_SYS_RESETS_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_RESETS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_RESETS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_RESETS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_resets` writer - "]
pub struct CLK_SYS_RESETS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_RESETS_W<'a> {
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
#[doc = "Field `clk_sys_pwm` reader - "]
pub struct CLK_SYS_PWM_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_PWM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_PWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_PWM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_pwm` writer - "]
pub struct CLK_SYS_PWM_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_PWM_W<'a> {
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
#[doc = "Field `clk_sys_psm` reader - "]
pub struct CLK_SYS_PSM_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_PSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_PSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_PSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_psm` writer - "]
pub struct CLK_SYS_PSM_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_PSM_W<'a> {
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
#[doc = "Field `clk_sys_pll_usb` reader - "]
pub struct CLK_SYS_PLL_USB_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_PLL_USB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_PLL_USB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_PLL_USB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_pll_usb` writer - "]
pub struct CLK_SYS_PLL_USB_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_PLL_USB_W<'a> {
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
#[doc = "Field `clk_sys_pll_sys` reader - "]
pub struct CLK_SYS_PLL_SYS_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_PLL_SYS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_PLL_SYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_PLL_SYS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_pll_sys` writer - "]
pub struct CLK_SYS_PLL_SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_PLL_SYS_W<'a> {
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
#[doc = "Field `clk_sys_pio1` reader - "]
pub struct CLK_SYS_PIO1_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_PIO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_PIO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_PIO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_pio1` writer - "]
pub struct CLK_SYS_PIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_PIO1_W<'a> {
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
#[doc = "Field `clk_sys_pio0` reader - "]
pub struct CLK_SYS_PIO0_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_PIO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_PIO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_PIO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_pio0` writer - "]
pub struct CLK_SYS_PIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_PIO0_W<'a> {
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
#[doc = "Field `clk_sys_pads` reader - "]
pub struct CLK_SYS_PADS_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_PADS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_PADS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_PADS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_pads` writer - "]
pub struct CLK_SYS_PADS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_PADS_W<'a> {
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
#[doc = "Field `clk_sys_vreg_and_chip_reset` reader - "]
pub struct CLK_SYS_VREG_AND_CHIP_RESET_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_VREG_AND_CHIP_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_VREG_AND_CHIP_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_VREG_AND_CHIP_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_vreg_and_chip_reset` writer - "]
pub struct CLK_SYS_VREG_AND_CHIP_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_VREG_AND_CHIP_RESET_W<'a> {
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
#[doc = "Field `clk_sys_jtag` reader - "]
pub struct CLK_SYS_JTAG_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_JTAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_JTAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_jtag` writer - "]
pub struct CLK_SYS_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_JTAG_W<'a> {
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
#[doc = "Field `clk_sys_io` reader - "]
pub struct CLK_SYS_IO_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_IO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_IO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_io` writer - "]
pub struct CLK_SYS_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_IO_W<'a> {
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
#[doc = "Field `clk_sys_i2c1` reader - "]
pub struct CLK_SYS_I2C1_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_I2C1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_I2C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_I2C1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_i2c1` writer - "]
pub struct CLK_SYS_I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_I2C1_W<'a> {
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
#[doc = "Field `clk_sys_i2c0` reader - "]
pub struct CLK_SYS_I2C0_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_I2C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_I2C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_I2C0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_i2c0` writer - "]
pub struct CLK_SYS_I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_I2C0_W<'a> {
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
#[doc = "Field `clk_sys_dma` reader - "]
pub struct CLK_SYS_DMA_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_dma` writer - "]
pub struct CLK_SYS_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_DMA_W<'a> {
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
#[doc = "Field `clk_sys_busfabric` reader - "]
pub struct CLK_SYS_BUSFABRIC_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_BUSFABRIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_BUSFABRIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_BUSFABRIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_busfabric` writer - "]
pub struct CLK_SYS_BUSFABRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_BUSFABRIC_W<'a> {
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
#[doc = "Field `clk_sys_busctrl` reader - "]
pub struct CLK_SYS_BUSCTRL_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_BUSCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_BUSCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_BUSCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_busctrl` writer - "]
pub struct CLK_SYS_BUSCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_BUSCTRL_W<'a> {
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
#[doc = "Field `clk_sys_adc` reader - "]
pub struct CLK_SYS_ADC_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_adc` writer - "]
pub struct CLK_SYS_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_ADC_W<'a> {
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
#[doc = "Field `clk_adc_adc` reader - "]
pub struct CLK_ADC_ADC_R(crate::FieldReader<bool, bool>);
impl CLK_ADC_ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_ADC_ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_ADC_ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_adc_adc` writer - "]
pub struct CLK_ADC_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_ADC_ADC_W<'a> {
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
#[doc = "Field `clk_sys_clocks` reader - "]
pub struct CLK_SYS_CLOCKS_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_CLOCKS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_CLOCKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_CLOCKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_clocks` writer - "]
pub struct CLK_SYS_CLOCKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_CLOCKS_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_sys_sram3(&self) -> CLK_SYS_SRAM3_R {
        CLK_SYS_SRAM3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_sys_sram2(&self) -> CLK_SYS_SRAM2_R {
        CLK_SYS_SRAM2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_sys_sram1(&self) -> CLK_SYS_SRAM1_R {
        CLK_SYS_SRAM1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_sys_sram0(&self) -> CLK_SYS_SRAM0_R {
        CLK_SYS_SRAM0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn clk_sys_spi1(&self) -> CLK_SYS_SPI1_R {
        CLK_SYS_SPI1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn clk_peri_spi1(&self) -> CLK_PERI_SPI1_R {
        CLK_PERI_SPI1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_sys_spi0(&self) -> CLK_SYS_SPI0_R {
        CLK_SYS_SPI0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn clk_peri_spi0(&self) -> CLK_PERI_SPI0_R {
        CLK_PERI_SPI0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn clk_sys_sio(&self) -> CLK_SYS_SIO_R {
        CLK_SYS_SIO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_sys_rtc(&self) -> CLK_SYS_RTC_R {
        CLK_SYS_RTC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clk_rtc_rtc(&self) -> CLK_RTC_RTC_R {
        CLK_RTC_RTC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_sys_rosc(&self) -> CLK_SYS_ROSC_R {
        CLK_SYS_ROSC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clk_sys_rom(&self) -> CLK_SYS_ROM_R {
        CLK_SYS_ROM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_sys_resets(&self) -> CLK_SYS_RESETS_R {
        CLK_SYS_RESETS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clk_sys_pwm(&self) -> CLK_SYS_PWM_R {
        CLK_SYS_PWM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_sys_psm(&self) -> CLK_SYS_PSM_R {
        CLK_SYS_PSM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clk_sys_pll_usb(&self) -> CLK_SYS_PLL_USB_R {
        CLK_SYS_PLL_USB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_sys_pll_sys(&self) -> CLK_SYS_PLL_SYS_R {
        CLK_SYS_PLL_SYS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_sys_pio1(&self) -> CLK_SYS_PIO1_R {
        CLK_SYS_PIO1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_sys_pio0(&self) -> CLK_SYS_PIO0_R {
        CLK_SYS_PIO0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_sys_pads(&self) -> CLK_SYS_PADS_R {
        CLK_SYS_PADS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_sys_vreg_and_chip_reset(&self) -> CLK_SYS_VREG_AND_CHIP_RESET_R {
        CLK_SYS_VREG_AND_CHIP_RESET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_sys_jtag(&self) -> CLK_SYS_JTAG_R {
        CLK_SYS_JTAG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_sys_io(&self) -> CLK_SYS_IO_R {
        CLK_SYS_IO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_sys_i2c1(&self) -> CLK_SYS_I2C1_R {
        CLK_SYS_I2C1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_sys_i2c0(&self) -> CLK_SYS_I2C0_R {
        CLK_SYS_I2C0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_sys_dma(&self) -> CLK_SYS_DMA_R {
        CLK_SYS_DMA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_sys_busfabric(&self) -> CLK_SYS_BUSFABRIC_R {
        CLK_SYS_BUSFABRIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_sys_busctrl(&self) -> CLK_SYS_BUSCTRL_R {
        CLK_SYS_BUSCTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_sys_adc(&self) -> CLK_SYS_ADC_R {
        CLK_SYS_ADC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_adc_adc(&self) -> CLK_ADC_ADC_R {
        CLK_ADC_ADC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_sys_clocks(&self) -> CLK_SYS_CLOCKS_R {
        CLK_SYS_CLOCKS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_sys_sram3(&mut self) -> CLK_SYS_SRAM3_W {
        CLK_SYS_SRAM3_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_sys_sram2(&mut self) -> CLK_SYS_SRAM2_W {
        CLK_SYS_SRAM2_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_sys_sram1(&mut self) -> CLK_SYS_SRAM1_W {
        CLK_SYS_SRAM1_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_sys_sram0(&mut self) -> CLK_SYS_SRAM0_W {
        CLK_SYS_SRAM0_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn clk_sys_spi1(&mut self) -> CLK_SYS_SPI1_W {
        CLK_SYS_SPI1_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn clk_peri_spi1(&mut self) -> CLK_PERI_SPI1_W {
        CLK_PERI_SPI1_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_sys_spi0(&mut self) -> CLK_SYS_SPI0_W {
        CLK_SYS_SPI0_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn clk_peri_spi0(&mut self) -> CLK_PERI_SPI0_W {
        CLK_PERI_SPI0_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn clk_sys_sio(&mut self) -> CLK_SYS_SIO_W {
        CLK_SYS_SIO_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_sys_rtc(&mut self) -> CLK_SYS_RTC_W {
        CLK_SYS_RTC_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clk_rtc_rtc(&mut self) -> CLK_RTC_RTC_W {
        CLK_RTC_RTC_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_sys_rosc(&mut self) -> CLK_SYS_ROSC_W {
        CLK_SYS_ROSC_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clk_sys_rom(&mut self) -> CLK_SYS_ROM_W {
        CLK_SYS_ROM_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_sys_resets(&mut self) -> CLK_SYS_RESETS_W {
        CLK_SYS_RESETS_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clk_sys_pwm(&mut self) -> CLK_SYS_PWM_W {
        CLK_SYS_PWM_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_sys_psm(&mut self) -> CLK_SYS_PSM_W {
        CLK_SYS_PSM_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clk_sys_pll_usb(&mut self) -> CLK_SYS_PLL_USB_W {
        CLK_SYS_PLL_USB_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_sys_pll_sys(&mut self) -> CLK_SYS_PLL_SYS_W {
        CLK_SYS_PLL_SYS_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_sys_pio1(&mut self) -> CLK_SYS_PIO1_W {
        CLK_SYS_PIO1_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_sys_pio0(&mut self) -> CLK_SYS_PIO0_W {
        CLK_SYS_PIO0_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_sys_pads(&mut self) -> CLK_SYS_PADS_W {
        CLK_SYS_PADS_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_sys_vreg_and_chip_reset(&mut self) -> CLK_SYS_VREG_AND_CHIP_RESET_W {
        CLK_SYS_VREG_AND_CHIP_RESET_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_sys_jtag(&mut self) -> CLK_SYS_JTAG_W {
        CLK_SYS_JTAG_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_sys_io(&mut self) -> CLK_SYS_IO_W {
        CLK_SYS_IO_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_sys_i2c1(&mut self) -> CLK_SYS_I2C1_W {
        CLK_SYS_I2C1_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_sys_i2c0(&mut self) -> CLK_SYS_I2C0_W {
        CLK_SYS_I2C0_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_sys_dma(&mut self) -> CLK_SYS_DMA_W {
        CLK_SYS_DMA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_sys_busfabric(&mut self) -> CLK_SYS_BUSFABRIC_W {
        CLK_SYS_BUSFABRIC_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_sys_busctrl(&mut self) -> CLK_SYS_BUSCTRL_W {
        CLK_SYS_BUSCTRL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_sys_adc(&mut self) -> CLK_SYS_ADC_W {
        CLK_SYS_ADC_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_adc_adc(&mut self) -> CLK_ADC_ADC_W {
        CLK_ADC_ADC_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_sys_clocks(&mut self) -> CLK_SYS_CLOCKS_W {
        CLK_SYS_CLOCKS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "enable clock in wake mode  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [wake_en0](index.html) module"]
pub struct WAKE_EN0_SPEC;
impl crate::RegisterSpec for WAKE_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wake_en0::R](R) reader structure"]
impl crate::Readable for WAKE_EN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wake_en0::W](W) writer structure"]
impl crate::Writable for WAKE_EN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKE_EN0 to value 0xffff_ffff"]
impl crate::Resettable for WAKE_EN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
