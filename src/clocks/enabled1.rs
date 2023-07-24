#[doc = "Register `ENABLED1` reader"]
pub struct R(crate::R<ENABLED1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLED1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLED1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLED1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `clk_sys_sram4` reader - "]
pub type CLK_SYS_SRAM4_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_sram5` reader - "]
pub type CLK_SYS_SRAM5_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_syscfg` reader - "]
pub type CLK_SYS_SYSCFG_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_sysinfo` reader - "]
pub type CLK_SYS_SYSINFO_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_tbman` reader - "]
pub type CLK_SYS_TBMAN_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_timer` reader - "]
pub type CLK_SYS_TIMER_R = crate::BitReader<bool>;
#[doc = "Field `clk_peri_uart0` reader - "]
pub type CLK_PERI_UART0_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_uart0` reader - "]
pub type CLK_SYS_UART0_R = crate::BitReader<bool>;
#[doc = "Field `clk_peri_uart1` reader - "]
pub type CLK_PERI_UART1_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_uart1` reader - "]
pub type CLK_SYS_UART1_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_usbctrl` reader - "]
pub type CLK_SYS_USBCTRL_R = crate::BitReader<bool>;
#[doc = "Field `clk_usb_usbctrl` reader - "]
pub type CLK_USB_USBCTRL_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_watchdog` reader - "]
pub type CLK_SYS_WATCHDOG_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_xip` reader - "]
pub type CLK_SYS_XIP_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_xosc` reader - "]
pub type CLK_SYS_XOSC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_sys_sram4(&self) -> CLK_SYS_SRAM4_R {
        CLK_SYS_SRAM4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_sys_sram5(&self) -> CLK_SYS_SRAM5_R {
        CLK_SYS_SRAM5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_sys_syscfg(&self) -> CLK_SYS_SYSCFG_R {
        CLK_SYS_SYSCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_sys_sysinfo(&self) -> CLK_SYS_SYSINFO_R {
        CLK_SYS_SYSINFO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_sys_tbman(&self) -> CLK_SYS_TBMAN_R {
        CLK_SYS_TBMAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_sys_timer(&self) -> CLK_SYS_TIMER_R {
        CLK_SYS_TIMER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_peri_uart0(&self) -> CLK_PERI_UART0_R {
        CLK_PERI_UART0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_sys_uart0(&self) -> CLK_SYS_UART0_R {
        CLK_SYS_UART0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_peri_uart1(&self) -> CLK_PERI_UART1_R {
        CLK_PERI_UART1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_sys_uart1(&self) -> CLK_SYS_UART1_R {
        CLK_SYS_UART1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_sys_usbctrl(&self) -> CLK_SYS_USBCTRL_R {
        CLK_SYS_USBCTRL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_usb_usbctrl(&self) -> CLK_USB_USBCTRL_R {
        CLK_USB_USBCTRL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_sys_watchdog(&self) -> CLK_SYS_WATCHDOG_R {
        CLK_SYS_WATCHDOG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_sys_xip(&self) -> CLK_SYS_XIP_R {
        CLK_SYS_XIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_sys_xosc(&self) -> CLK_SYS_XOSC_R {
        CLK_SYS_XOSC_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "indicates the state of the clock enable  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [enabled1](index.html) module"]
pub struct ENABLED1_SPEC;
impl crate::RegisterSpec for ENABLED1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enabled1::R](R) reader structure"]
impl crate::Readable for ENABLED1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ENABLED1 to value 0"]
impl crate::Resettable for ENABLED1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
