#[doc = "Reader of register SLEEP_EN1"]
pub type R = crate::R<u32, super::SLEEP_EN1>;
#[doc = "Writer for register SLEEP_EN1"]
pub type W = crate::W<u32, super::SLEEP_EN1>;
#[doc = "Register SLEEP_EN1 `reset()`'s with value 0x7fff"]
impl crate::ResetValue for super::SLEEP_EN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fff
    }
}
#[doc = "Reader of field `clk_sys_xosc`"]
pub type CLK_SYS_XOSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_xosc`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `clk_sys_xip`"]
pub type CLK_SYS_XIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_xip`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `clk_sys_watchdog`"]
pub type CLK_SYS_WATCHDOG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_watchdog`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `clk_usb_usbctrl`"]
pub type CLK_USB_USBCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_usb_usbctrl`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `clk_sys_usbctrl`"]
pub type CLK_SYS_USBCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_usbctrl`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `clk_sys_uart1`"]
pub type CLK_SYS_UART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_uart1`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `clk_peri_uart1`"]
pub type CLK_PERI_UART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_peri_uart1`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `clk_sys_uart0`"]
pub type CLK_SYS_UART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_uart0`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `clk_peri_uart0`"]
pub type CLK_PERI_UART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_peri_uart0`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `clk_sys_timer`"]
pub type CLK_SYS_TIMER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_timer`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `clk_sys_tbman`"]
pub type CLK_SYS_TBMAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_tbman`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `clk_sys_sysinfo`"]
pub type CLK_SYS_SYSINFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_sysinfo`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `clk_sys_syscfg`"]
pub type CLK_SYS_SYSCFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_syscfg`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `clk_sys_sram5`"]
pub type CLK_SYS_SRAM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_sram5`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `clk_sys_sram4`"]
pub type CLK_SYS_SRAM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_sys_sram4`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
}
