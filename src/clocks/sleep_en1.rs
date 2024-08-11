#[doc = "Register `SLEEP_EN1` reader"]
pub type R = crate::R<SLEEP_EN1_SPEC>;
#[doc = "Register `SLEEP_EN1` writer"]
pub type W = crate::W<SLEEP_EN1_SPEC>;
#[doc = "Field `CLK_SYS_SRAM4` reader - "]
pub type CLK_SYS_SRAM4_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM4` writer - "]
pub type CLK_SYS_SRAM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_SRAM5` reader - "]
pub type CLK_SYS_SRAM5_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM5` writer - "]
pub type CLK_SYS_SRAM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_SYSCFG` reader - "]
pub type CLK_SYS_SYSCFG_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SYSCFG` writer - "]
pub type CLK_SYS_SYSCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_SYSINFO` reader - "]
pub type CLK_SYS_SYSINFO_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SYSINFO` writer - "]
pub type CLK_SYS_SYSINFO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_TBMAN` reader - "]
pub type CLK_SYS_TBMAN_R = crate::BitReader;
#[doc = "Field `CLK_SYS_TBMAN` writer - "]
pub type CLK_SYS_TBMAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_TIMER` reader - "]
pub type CLK_SYS_TIMER_R = crate::BitReader;
#[doc = "Field `CLK_SYS_TIMER` writer - "]
pub type CLK_SYS_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PERI_UART0` reader - "]
pub type CLK_PERI_UART0_R = crate::BitReader;
#[doc = "Field `CLK_PERI_UART0` writer - "]
pub type CLK_PERI_UART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_UART0` reader - "]
pub type CLK_SYS_UART0_R = crate::BitReader;
#[doc = "Field `CLK_SYS_UART0` writer - "]
pub type CLK_SYS_UART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PERI_UART1` reader - "]
pub type CLK_PERI_UART1_R = crate::BitReader;
#[doc = "Field `CLK_PERI_UART1` writer - "]
pub type CLK_PERI_UART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_UART1` reader - "]
pub type CLK_SYS_UART1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_UART1` writer - "]
pub type CLK_SYS_UART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_USBCTRL` reader - "]
pub type CLK_SYS_USBCTRL_R = crate::BitReader;
#[doc = "Field `CLK_SYS_USBCTRL` writer - "]
pub type CLK_SYS_USBCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_USB_USBCTRL` reader - "]
pub type CLK_USB_USBCTRL_R = crate::BitReader;
#[doc = "Field `CLK_USB_USBCTRL` writer - "]
pub type CLK_USB_USBCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_WATCHDOG` reader - "]
pub type CLK_SYS_WATCHDOG_R = crate::BitReader;
#[doc = "Field `CLK_SYS_WATCHDOG` writer - "]
pub type CLK_SYS_WATCHDOG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_XIP` reader - "]
pub type CLK_SYS_XIP_R = crate::BitReader;
#[doc = "Field `CLK_SYS_XIP` writer - "]
pub type CLK_SYS_XIP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_XOSC` reader - "]
pub type CLK_SYS_XOSC_R = crate::BitReader;
#[doc = "Field `CLK_SYS_XOSC` writer - "]
pub type CLK_SYS_XOSC_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_sram4(&mut self) -> CLK_SYS_SRAM4_W<SLEEP_EN1_SPEC> {
        CLK_SYS_SRAM4_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_sram5(&mut self) -> CLK_SYS_SRAM5_W<SLEEP_EN1_SPEC> {
        CLK_SYS_SRAM5_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_syscfg(&mut self) -> CLK_SYS_SYSCFG_W<SLEEP_EN1_SPEC> {
        CLK_SYS_SYSCFG_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_sysinfo(&mut self) -> CLK_SYS_SYSINFO_W<SLEEP_EN1_SPEC> {
        CLK_SYS_SYSINFO_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_tbman(&mut self) -> CLK_SYS_TBMAN_W<SLEEP_EN1_SPEC> {
        CLK_SYS_TBMAN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_timer(&mut self) -> CLK_SYS_TIMER_W<SLEEP_EN1_SPEC> {
        CLK_SYS_TIMER_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_peri_uart0(&mut self) -> CLK_PERI_UART0_W<SLEEP_EN1_SPEC> {
        CLK_PERI_UART0_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_uart0(&mut self) -> CLK_SYS_UART0_W<SLEEP_EN1_SPEC> {
        CLK_SYS_UART0_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clk_peri_uart1(&mut self) -> CLK_PERI_UART1_W<SLEEP_EN1_SPEC> {
        CLK_PERI_UART1_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_uart1(&mut self) -> CLK_SYS_UART1_W<SLEEP_EN1_SPEC> {
        CLK_SYS_UART1_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_usbctrl(&mut self) -> CLK_SYS_USBCTRL_W<SLEEP_EN1_SPEC> {
        CLK_SYS_USBCTRL_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb_usbctrl(&mut self) -> CLK_USB_USBCTRL_W<SLEEP_EN1_SPEC> {
        CLK_USB_USBCTRL_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_watchdog(&mut self) -> CLK_SYS_WATCHDOG_W<SLEEP_EN1_SPEC> {
        CLK_SYS_WATCHDOG_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_xip(&mut self) -> CLK_SYS_XIP_W<SLEEP_EN1_SPEC> {
        CLK_SYS_XIP_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_xosc(&mut self) -> CLK_SYS_XOSC_W<SLEEP_EN1_SPEC> {
        CLK_SYS_XOSC_W::new(self, 14)
    }
}
#[doc = "enable clock in sleep mode  

You can [`read`](crate::generic::Reg::read) this register and get [`sleep_en1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_en1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEP_EN1_SPEC;
impl crate::RegisterSpec for SLEEP_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_en1::R`](R) reader structure"]
impl crate::Readable for SLEEP_EN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleep_en1::W`](W) writer structure"]
impl crate::Writable for SLEEP_EN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLEEP_EN1 to value 0x7fff"]
impl crate::Resettable for SLEEP_EN1_SPEC {
    const RESET_VALUE: u32 = 0x7fff;
}
