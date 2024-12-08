#[doc = "Register `WDSEL` reader"]
pub type R = crate::R<WDSEL_SPEC>;
#[doc = "Register `WDSEL` writer"]
pub type W = crate::W<WDSEL_SPEC>;
#[doc = "Field `ADC` reader - "]
pub type ADC_R = crate::BitReader;
#[doc = "Field `ADC` writer - "]
pub type ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSCTRL` reader - "]
pub type BUSCTRL_R = crate::BitReader;
#[doc = "Field `BUSCTRL` writer - "]
pub type BUSCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - "]
pub type DMA_R = crate::BitReader;
#[doc = "Field `DMA` writer - "]
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - "]
pub type I2C0_R = crate::BitReader;
#[doc = "Field `I2C0` writer - "]
pub type I2C0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - "]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `I2C1` writer - "]
pub type I2C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_BANK0` reader - "]
pub type IO_BANK0_R = crate::BitReader;
#[doc = "Field `IO_BANK0` writer - "]
pub type IO_BANK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_QSPI` reader - "]
pub type IO_QSPI_R = crate::BitReader;
#[doc = "Field `IO_QSPI` writer - "]
pub type IO_QSPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG` reader - "]
pub type JTAG_R = crate::BitReader;
#[doc = "Field `JTAG` writer - "]
pub type JTAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADS_BANK0` reader - "]
pub type PADS_BANK0_R = crate::BitReader;
#[doc = "Field `PADS_BANK0` writer - "]
pub type PADS_BANK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADS_QSPI` reader - "]
pub type PADS_QSPI_R = crate::BitReader;
#[doc = "Field `PADS_QSPI` writer - "]
pub type PADS_QSPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIO0` reader - "]
pub type PIO0_R = crate::BitReader;
#[doc = "Field `PIO0` writer - "]
pub type PIO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIO1` reader - "]
pub type PIO1_R = crate::BitReader;
#[doc = "Field `PIO1` writer - "]
pub type PIO1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_SYS` reader - "]
pub type PLL_SYS_R = crate::BitReader;
#[doc = "Field `PLL_SYS` writer - "]
pub type PLL_SYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_USB` reader - "]
pub type PLL_USB_R = crate::BitReader;
#[doc = "Field `PLL_USB` writer - "]
pub type PLL_USB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM` reader - "]
pub type PWM_R = crate::BitReader;
#[doc = "Field `PWM` writer - "]
pub type PWM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - "]
pub type RTC_R = crate::BitReader;
#[doc = "Field `RTC` writer - "]
pub type RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0` reader - "]
pub type SPI0_R = crate::BitReader;
#[doc = "Field `SPI0` writer - "]
pub type SPI0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - "]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `SPI1` writer - "]
pub type SPI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG` reader - "]
pub type SYSCFG_R = crate::BitReader;
#[doc = "Field `SYSCFG` writer - "]
pub type SYSCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSINFO` reader - "]
pub type SYSINFO_R = crate::BitReader;
#[doc = "Field `SYSINFO` writer - "]
pub type SYSINFO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBMAN` reader - "]
pub type TBMAN_R = crate::BitReader;
#[doc = "Field `TBMAN` writer - "]
pub type TBMAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER` reader - "]
pub type TIMER_R = crate::BitReader;
#[doc = "Field `TIMER` writer - "]
pub type TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0` reader - "]
pub type UART0_R = crate::BitReader;
#[doc = "Field `UART0` writer - "]
pub type UART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - "]
pub type UART1_R = crate::BitReader;
#[doc = "Field `UART1` writer - "]
pub type UART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCTRL` reader - "]
pub type USBCTRL_R = crate::BitReader;
#[doc = "Field `USBCTRL` writer - "]
pub type USBCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn busctrl(&self) -> BUSCTRL_R {
        BUSCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn io_bank0(&self) -> IO_BANK0_R {
        IO_BANK0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn io_qspi(&self) -> IO_QSPI_R {
        IO_QSPI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn jtag(&self) -> JTAG_R {
        JTAG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pads_bank0(&self) -> PADS_BANK0_R {
        PADS_BANK0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pads_qspi(&self) -> PADS_QSPI_R {
        PADS_QSPI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pio0(&self) -> PIO0_R {
        PIO0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pio1(&self) -> PIO1_R {
        PIO1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pll_sys(&self) -> PLL_SYS_R {
        PLL_SYS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pll_usb(&self) -> PLL_USB_R {
        PLL_USB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pwm(&self) -> PWM_R {
        PWM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn syscfg(&self) -> SYSCFG_R {
        SYSCFG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sysinfo(&self) -> SYSINFO_R {
        SYSINFO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tbman(&self) -> TBMAN_R {
        TBMAN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usbctrl(&self) -> USBCTRL_R {
        USBCTRL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<WDSEL_SPEC> {
        ADC_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn busctrl(&mut self) -> BUSCTRL_W<WDSEL_SPEC> {
        BUSCTRL_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<WDSEL_SPEC> {
        DMA_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<WDSEL_SPEC> {
        I2C0_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<WDSEL_SPEC> {
        I2C1_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn io_bank0(&mut self) -> IO_BANK0_W<WDSEL_SPEC> {
        IO_BANK0_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn io_qspi(&mut self) -> IO_QSPI_W<WDSEL_SPEC> {
        IO_QSPI_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn jtag(&mut self) -> JTAG_W<WDSEL_SPEC> {
        JTAG_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pads_bank0(&mut self) -> PADS_BANK0_W<WDSEL_SPEC> {
        PADS_BANK0_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pads_qspi(&mut self) -> PADS_QSPI_W<WDSEL_SPEC> {
        PADS_QSPI_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pio0(&mut self) -> PIO0_W<WDSEL_SPEC> {
        PIO0_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pio1(&mut self) -> PIO1_W<WDSEL_SPEC> {
        PIO1_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pll_sys(&mut self) -> PLL_SYS_W<WDSEL_SPEC> {
        PLL_SYS_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pll_usb(&mut self) -> PLL_USB_W<WDSEL_SPEC> {
        PLL_USB_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pwm(&mut self) -> PWM_W<WDSEL_SPEC> {
        PWM_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<WDSEL_SPEC> {
        RTC_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<WDSEL_SPEC> {
        SPI0_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<WDSEL_SPEC> {
        SPI1_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn syscfg(&mut self) -> SYSCFG_W<WDSEL_SPEC> {
        SYSCFG_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn sysinfo(&mut self) -> SYSINFO_W<WDSEL_SPEC> {
        SYSINFO_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn tbman(&mut self) -> TBMAN_W<WDSEL_SPEC> {
        TBMAN_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<WDSEL_SPEC> {
        TIMER_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<WDSEL_SPEC> {
        UART0_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<WDSEL_SPEC> {
        UART1_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn usbctrl(&mut self) -> USBCTRL_W<WDSEL_SPEC> {
        USBCTRL_W::new(self, 24)
    }
}
#[doc = "Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires.  

You can [`read`](crate::generic::Reg::read) this register and get [`wdsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDSEL_SPEC;
impl crate::RegisterSpec for WDSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdsel::R`](R) reader structure"]
impl crate::Readable for WDSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdsel::W`](W) writer structure"]
impl crate::Writable for WDSEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDSEL to value 0"]
impl crate::Resettable for WDSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
