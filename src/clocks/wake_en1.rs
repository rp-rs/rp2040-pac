#[doc = "Register `WAKE_EN1` reader"]
pub struct R(crate::R<WAKE_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKE_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKE_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKE_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKE_EN1` writer"]
pub struct W(crate::W<WAKE_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKE_EN1_SPEC>;
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
impl From<crate::W<WAKE_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKE_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clk_sys_xosc` reader - "]
pub struct CLK_SYS_XOSC_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_XOSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_XOSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_XOSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_xosc` writer - "]
pub struct CLK_SYS_XOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_XOSC_W<'a> {
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
#[doc = "Field `clk_sys_xip` reader - "]
pub struct CLK_SYS_XIP_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_XIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_XIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_XIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_xip` writer - "]
pub struct CLK_SYS_XIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_XIP_W<'a> {
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
#[doc = "Field `clk_sys_watchdog` reader - "]
pub struct CLK_SYS_WATCHDOG_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_WATCHDOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_WATCHDOG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_WATCHDOG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_watchdog` writer - "]
pub struct CLK_SYS_WATCHDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_WATCHDOG_W<'a> {
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
#[doc = "Field `clk_usb_usbctrl` reader - "]
pub struct CLK_USB_USBCTRL_R(crate::FieldReader<bool, bool>);
impl CLK_USB_USBCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_USB_USBCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_USB_USBCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_usb_usbctrl` writer - "]
pub struct CLK_USB_USBCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_USB_USBCTRL_W<'a> {
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
#[doc = "Field `clk_sys_usbctrl` reader - "]
pub struct CLK_SYS_USBCTRL_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_USBCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_USBCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_USBCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_usbctrl` writer - "]
pub struct CLK_SYS_USBCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_USBCTRL_W<'a> {
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
#[doc = "Field `clk_sys_uart1` reader - "]
pub struct CLK_SYS_UART1_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_UART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_UART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_UART1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_uart1` writer - "]
pub struct CLK_SYS_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_UART1_W<'a> {
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
#[doc = "Field `clk_peri_uart1` reader - "]
pub struct CLK_PERI_UART1_R(crate::FieldReader<bool, bool>);
impl CLK_PERI_UART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_PERI_UART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_PERI_UART1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_peri_uart1` writer - "]
pub struct CLK_PERI_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_PERI_UART1_W<'a> {
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
#[doc = "Field `clk_sys_uart0` reader - "]
pub struct CLK_SYS_UART0_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_UART0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_UART0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_UART0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_uart0` writer - "]
pub struct CLK_SYS_UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_UART0_W<'a> {
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
#[doc = "Field `clk_peri_uart0` reader - "]
pub struct CLK_PERI_UART0_R(crate::FieldReader<bool, bool>);
impl CLK_PERI_UART0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_PERI_UART0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_PERI_UART0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_peri_uart0` writer - "]
pub struct CLK_PERI_UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_PERI_UART0_W<'a> {
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
#[doc = "Field `clk_sys_timer` reader - "]
pub struct CLK_SYS_TIMER_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_TIMER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_timer` writer - "]
pub struct CLK_SYS_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_TIMER_W<'a> {
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
#[doc = "Field `clk_sys_tbman` reader - "]
pub struct CLK_SYS_TBMAN_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_TBMAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_TBMAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_TBMAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_tbman` writer - "]
pub struct CLK_SYS_TBMAN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_TBMAN_W<'a> {
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
#[doc = "Field `clk_sys_sysinfo` reader - "]
pub struct CLK_SYS_SYSINFO_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_SYSINFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_SYSINFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_SYSINFO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_sysinfo` writer - "]
pub struct CLK_SYS_SYSINFO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_SYSINFO_W<'a> {
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
#[doc = "Field `clk_sys_syscfg` reader - "]
pub struct CLK_SYS_SYSCFG_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_SYSCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_SYSCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_SYSCFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_syscfg` writer - "]
pub struct CLK_SYS_SYSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_SYSCFG_W<'a> {
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
#[doc = "Field `clk_sys_sram5` reader - "]
pub struct CLK_SYS_SRAM5_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_SRAM5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_SRAM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_SRAM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_sram5` writer - "]
pub struct CLK_SYS_SRAM5_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_SRAM5_W<'a> {
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
#[doc = "Field `clk_sys_sram4` reader - "]
pub struct CLK_SYS_SRAM4_R(crate::FieldReader<bool, bool>);
impl CLK_SYS_SRAM4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SYS_SRAM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SYS_SRAM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_sys_sram4` writer - "]
pub struct CLK_SYS_SRAM4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SYS_SRAM4_W<'a> {
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
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_sys_xosc(&self) -> CLK_SYS_XOSC_R {
        CLK_SYS_XOSC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_sys_xip(&self) -> CLK_SYS_XIP_R {
        CLK_SYS_XIP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_sys_watchdog(&self) -> CLK_SYS_WATCHDOG_R {
        CLK_SYS_WATCHDOG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_usb_usbctrl(&self) -> CLK_USB_USBCTRL_R {
        CLK_USB_USBCTRL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_sys_usbctrl(&self) -> CLK_SYS_USBCTRL_R {
        CLK_SYS_USBCTRL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_sys_uart1(&self) -> CLK_SYS_UART1_R {
        CLK_SYS_UART1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_peri_uart1(&self) -> CLK_PERI_UART1_R {
        CLK_PERI_UART1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_sys_uart0(&self) -> CLK_SYS_UART0_R {
        CLK_SYS_UART0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_peri_uart0(&self) -> CLK_PERI_UART0_R {
        CLK_PERI_UART0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_sys_timer(&self) -> CLK_SYS_TIMER_R {
        CLK_SYS_TIMER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_sys_tbman(&self) -> CLK_SYS_TBMAN_R {
        CLK_SYS_TBMAN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_sys_sysinfo(&self) -> CLK_SYS_SYSINFO_R {
        CLK_SYS_SYSINFO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_sys_syscfg(&self) -> CLK_SYS_SYSCFG_R {
        CLK_SYS_SYSCFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_sys_sram5(&self) -> CLK_SYS_SRAM5_R {
        CLK_SYS_SRAM5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_sys_sram4(&self) -> CLK_SYS_SRAM4_R {
        CLK_SYS_SRAM4_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_sys_xosc(&mut self) -> CLK_SYS_XOSC_W {
        CLK_SYS_XOSC_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_sys_xip(&mut self) -> CLK_SYS_XIP_W {
        CLK_SYS_XIP_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_sys_watchdog(&mut self) -> CLK_SYS_WATCHDOG_W {
        CLK_SYS_WATCHDOG_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_usb_usbctrl(&mut self) -> CLK_USB_USBCTRL_W {
        CLK_USB_USBCTRL_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_sys_usbctrl(&mut self) -> CLK_SYS_USBCTRL_W {
        CLK_SYS_USBCTRL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_sys_uart1(&mut self) -> CLK_SYS_UART1_W {
        CLK_SYS_UART1_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_peri_uart1(&mut self) -> CLK_PERI_UART1_W {
        CLK_PERI_UART1_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_sys_uart0(&mut self) -> CLK_SYS_UART0_W {
        CLK_SYS_UART0_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_peri_uart0(&mut self) -> CLK_PERI_UART0_W {
        CLK_PERI_UART0_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_sys_timer(&mut self) -> CLK_SYS_TIMER_W {
        CLK_SYS_TIMER_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_sys_tbman(&mut self) -> CLK_SYS_TBMAN_W {
        CLK_SYS_TBMAN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_sys_sysinfo(&mut self) -> CLK_SYS_SYSINFO_W {
        CLK_SYS_SYSINFO_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_sys_syscfg(&mut self) -> CLK_SYS_SYSCFG_W {
        CLK_SYS_SYSCFG_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_sys_sram5(&mut self) -> CLK_SYS_SRAM5_W {
        CLK_SYS_SRAM5_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_sys_sram4(&mut self) -> CLK_SYS_SRAM4_W {
        CLK_SYS_SRAM4_W { w: self }
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

For information about available fields see [wake_en1](index.html) module"]
pub struct WAKE_EN1_SPEC;
impl crate::RegisterSpec for WAKE_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wake_en1::R](R) reader structure"]
impl crate::Readable for WAKE_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wake_en1::W](W) writer structure"]
impl crate::Writable for WAKE_EN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKE_EN1 to value 0x7fff"]
impl crate::Resettable for WAKE_EN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff
    }
}
