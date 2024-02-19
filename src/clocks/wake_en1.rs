#[doc = "Register `WAKE_EN1` reader"]
pub type R = crate::R<WAKE_EN1_SPEC>;
#[doc = "Register `WAKE_EN1` writer"]
pub type W = crate::W<WAKE_EN1_SPEC>;
#[doc = "Field `clk_sys_sram4` reader - "]
pub type CLK_SYS_SRAM4_R = crate::BitReader;
#[doc = "Field `clk_sys_sram4` writer - "]
pub type CLK_SYS_SRAM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_sys_sram5` reader - "]
pub type CLK_SYS_SRAM5_R = crate::BitReader;
#[doc = "Field `clk_sys_sram5` writer - "]
pub type CLK_SYS_SRAM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_sys_syscfg` reader - "]
pub type CLK_SYS_SYSCFG_R = crate::BitReader;
#[doc = "Field `clk_sys_syscfg` writer - "]
pub type CLK_SYS_SYSCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_sys_sysinfo` reader - "]
pub type CLK_SYS_SYSINFO_R = crate::BitReader;
#[doc = "Field `clk_sys_sysinfo` writer - "]
pub type CLK_SYS_SYSINFO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_sys_tbman` reader - "]
pub type CLK_SYS_TBMAN_R = crate::BitReader;
#[doc = "Field `clk_sys_tbman` writer - "]
pub type CLK_SYS_TBMAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_sys_timer` reader - "]
pub type CLK_SYS_TIMER_R = crate::BitReader;
#[doc = "Field `clk_sys_timer` writer - "]
pub type CLK_SYS_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_peri_uart0` reader - "]
pub type CLK_PERI_UART0_R = crate::BitReader;
#[doc = "Field `clk_peri_uart0` writer - "]
pub type CLK_PERI_UART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_sys_uart0` reader - "]
pub type CLK_SYS_UART0_R = crate::BitReader;
#[doc = "Field `clk_sys_uart0` writer - "]
pub type CLK_SYS_UART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_peri_uart1` reader - "]
pub type CLK_PERI_UART1_R = crate::BitReader;
#[doc = "Field `clk_peri_uart1` writer - "]
pub type CLK_PERI_UART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_sys_uart1` reader - "]
pub type CLK_SYS_UART1_R = crate::BitReader;
#[doc = "Field `clk_sys_uart1` writer - "]
pub type CLK_SYS_UART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_sys_usbctrl` reader - "]
pub type CLK_SYS_USBCTRL_R = crate::BitReader;
#[doc = "Field `clk_sys_usbctrl` writer - "]
pub type CLK_SYS_USBCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_usb_usbctrl` reader - "]
pub type CLK_USB_USBCTRL_R = crate::BitReader;
#[doc = "Field `clk_usb_usbctrl` writer - "]
pub type CLK_USB_USBCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_sys_watchdog` reader - "]
pub type CLK_SYS_WATCHDOG_R = crate::BitReader;
#[doc = "Field `clk_sys_watchdog` writer - "]
pub type CLK_SYS_WATCHDOG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_sys_xip` reader - "]
pub type CLK_SYS_XIP_R = crate::BitReader;
#[doc = "Field `clk_sys_xip` writer - "]
pub type CLK_SYS_XIP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_sys_xosc` reader - "]
pub type CLK_SYS_XOSC_R = crate::BitReader;
#[doc = "Field `clk_sys_xosc` writer - "]
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
    pub fn clk_sys_sram4(&mut self) -> CLK_SYS_SRAM4_W<WAKE_EN1_SPEC> {
        CLK_SYS_SRAM4_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_sram5(&mut self) -> CLK_SYS_SRAM5_W<WAKE_EN1_SPEC> {
        CLK_SYS_SRAM5_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_syscfg(&mut self) -> CLK_SYS_SYSCFG_W<WAKE_EN1_SPEC> {
        CLK_SYS_SYSCFG_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_sysinfo(&mut self) -> CLK_SYS_SYSINFO_W<WAKE_EN1_SPEC> {
        CLK_SYS_SYSINFO_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_tbman(&mut self) -> CLK_SYS_TBMAN_W<WAKE_EN1_SPEC> {
        CLK_SYS_TBMAN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_timer(&mut self) -> CLK_SYS_TIMER_W<WAKE_EN1_SPEC> {
        CLK_SYS_TIMER_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_peri_uart0(&mut self) -> CLK_PERI_UART0_W<WAKE_EN1_SPEC> {
        CLK_PERI_UART0_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_uart0(&mut self) -> CLK_SYS_UART0_W<WAKE_EN1_SPEC> {
        CLK_SYS_UART0_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clk_peri_uart1(&mut self) -> CLK_PERI_UART1_W<WAKE_EN1_SPEC> {
        CLK_PERI_UART1_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_uart1(&mut self) -> CLK_SYS_UART1_W<WAKE_EN1_SPEC> {
        CLK_SYS_UART1_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_usbctrl(&mut self) -> CLK_SYS_USBCTRL_W<WAKE_EN1_SPEC> {
        CLK_SYS_USBCTRL_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb_usbctrl(&mut self) -> CLK_USB_USBCTRL_W<WAKE_EN1_SPEC> {
        CLK_USB_USBCTRL_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_watchdog(&mut self) -> CLK_SYS_WATCHDOG_W<WAKE_EN1_SPEC> {
        CLK_SYS_WATCHDOG_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_xip(&mut self) -> CLK_SYS_XIP_W<WAKE_EN1_SPEC> {
        CLK_SYS_XIP_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_xosc(&mut self) -> CLK_SYS_XOSC_W<WAKE_EN1_SPEC> {
        CLK_SYS_XOSC_W::new(self, 14)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "enable clock in wake mode  

You can [`read`](crate::generic::Reg::read) this register and get [`wake_en1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_en1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKE_EN1_SPEC;
impl crate::RegisterSpec for WAKE_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wake_en1::R`](R) reader structure"]
impl crate::Readable for WAKE_EN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wake_en1::W`](W) writer structure"]
impl crate::Writable for WAKE_EN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKE_EN1 to value 0x7fff"]
impl crate::Resettable for WAKE_EN1_SPEC {
    const RESET_VALUE: u32 = 0x7fff;
}
