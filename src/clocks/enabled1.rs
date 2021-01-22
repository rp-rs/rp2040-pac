#[doc = "Reader of register ENABLED1"]
pub type R = crate::R<u32, super::ENABLED1>;
#[doc = "Reader of field `clk_sys_xosc`"]
pub type CLK_SYS_XOSC_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_sys_xip`"]
pub type CLK_SYS_XIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_sys_watchdog`"]
pub type CLK_SYS_WATCHDOG_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_usb_usbctrl`"]
pub type CLK_USB_USBCTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_sys_usbctrl`"]
pub type CLK_SYS_USBCTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_sys_uart1`"]
pub type CLK_SYS_UART1_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_peri_uart1`"]
pub type CLK_PERI_UART1_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_sys_uart0`"]
pub type CLK_SYS_UART0_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_peri_uart0`"]
pub type CLK_PERI_UART0_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_sys_timer`"]
pub type CLK_SYS_TIMER_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_sys_tbman`"]
pub type CLK_SYS_TBMAN_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_sys_sysinfo`"]
pub type CLK_SYS_SYSINFO_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_sys_syscfg`"]
pub type CLK_SYS_SYSCFG_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_sys_sram5`"]
pub type CLK_SYS_SRAM5_R = crate::R<bool, bool>;
#[doc = "Reader of field `clk_sys_sram4`"]
pub type CLK_SYS_SRAM4_R = crate::R<bool, bool>;
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
