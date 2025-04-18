#[doc = "Register `ENABLED1` reader"]
pub type R = crate::R<ENABLED1_SPEC>;
#[doc = "Register `ENABLED1` writer"]
pub type W = crate::W<ENABLED1_SPEC>;
#[doc = "Field `CLK_SYS_SRAM4` reader - "]
pub type CLK_SYS_SRAM4_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM5` reader - "]
pub type CLK_SYS_SRAM5_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SYSCFG` reader - "]
pub type CLK_SYS_SYSCFG_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SYSINFO` reader - "]
pub type CLK_SYS_SYSINFO_R = crate::BitReader;
#[doc = "Field `CLK_SYS_TBMAN` reader - "]
pub type CLK_SYS_TBMAN_R = crate::BitReader;
#[doc = "Field `CLK_SYS_TIMER` reader - "]
pub type CLK_SYS_TIMER_R = crate::BitReader;
#[doc = "Field `CLK_PERI_UART0` reader - "]
pub type CLK_PERI_UART0_R = crate::BitReader;
#[doc = "Field `CLK_SYS_UART0` reader - "]
pub type CLK_SYS_UART0_R = crate::BitReader;
#[doc = "Field `CLK_PERI_UART1` reader - "]
pub type CLK_PERI_UART1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_UART1` reader - "]
pub type CLK_SYS_UART1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_USBCTRL` reader - "]
pub type CLK_SYS_USBCTRL_R = crate::BitReader;
#[doc = "Field `CLK_USB_USBCTRL` reader - "]
pub type CLK_USB_USBCTRL_R = crate::BitReader;
#[doc = "Field `CLK_SYS_WATCHDOG` reader - "]
pub type CLK_SYS_WATCHDOG_R = crate::BitReader;
#[doc = "Field `CLK_SYS_XIP` reader - "]
pub type CLK_SYS_XIP_R = crate::BitReader;
#[doc = "Field `CLK_SYS_XOSC` reader - "]
pub type CLK_SYS_XOSC_R = crate::BitReader;
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
impl W {}
#[doc = "indicates the state of the clock enable  

You can [`read`](crate::generic::Reg::read) this register and get [`enabled1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enabled1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLED1_SPEC;
impl crate::RegisterSpec for ENABLED1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enabled1::R`](R) reader structure"]
impl crate::Readable for ENABLED1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enabled1::W`](W) writer structure"]
impl crate::Writable for ENABLED1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLED1 to value 0"]
impl crate::Resettable for ENABLED1_SPEC {
    const RESET_VALUE: u32 = 0;
}
