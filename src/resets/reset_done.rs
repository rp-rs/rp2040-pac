#[doc = "Register `RESET_DONE` reader"]
pub type R = crate::R<RESET_DONE_SPEC>;
#[doc = "Register `RESET_DONE` writer"]
pub type W = crate::W<RESET_DONE_SPEC>;
#[doc = "Field `ADC` reader - "]
pub type ADC_R = crate::BitReader;
#[doc = "Field `BUSCTRL` reader - "]
pub type BUSCTRL_R = crate::BitReader;
#[doc = "Field `DMA` reader - "]
pub type DMA_R = crate::BitReader;
#[doc = "Field `I2C0` reader - "]
pub type I2C0_R = crate::BitReader;
#[doc = "Field `I2C1` reader - "]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `IO_BANK0` reader - "]
pub type IO_BANK0_R = crate::BitReader;
#[doc = "Field `IO_QSPI` reader - "]
pub type IO_QSPI_R = crate::BitReader;
#[doc = "Field `JTAG` reader - "]
pub type JTAG_R = crate::BitReader;
#[doc = "Field `PADS_BANK0` reader - "]
pub type PADS_BANK0_R = crate::BitReader;
#[doc = "Field `PADS_QSPI` reader - "]
pub type PADS_QSPI_R = crate::BitReader;
#[doc = "Field `PIO0` reader - "]
pub type PIO0_R = crate::BitReader;
#[doc = "Field `PIO1` reader - "]
pub type PIO1_R = crate::BitReader;
#[doc = "Field `PLL_SYS` reader - "]
pub type PLL_SYS_R = crate::BitReader;
#[doc = "Field `PLL_USB` reader - "]
pub type PLL_USB_R = crate::BitReader;
#[doc = "Field `PWM` reader - "]
pub type PWM_R = crate::BitReader;
#[doc = "Field `RTC` reader - "]
pub type RTC_R = crate::BitReader;
#[doc = "Field `SPI0` reader - "]
pub type SPI0_R = crate::BitReader;
#[doc = "Field `SPI1` reader - "]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `SYSCFG` reader - "]
pub type SYSCFG_R = crate::BitReader;
#[doc = "Field `SYSINFO` reader - "]
pub type SYSINFO_R = crate::BitReader;
#[doc = "Field `TBMAN` reader - "]
pub type TBMAN_R = crate::BitReader;
#[doc = "Field `TIMER` reader - "]
pub type TIMER_R = crate::BitReader;
#[doc = "Field `UART0` reader - "]
pub type UART0_R = crate::BitReader;
#[doc = "Field `UART1` reader - "]
pub type UART1_R = crate::BitReader;
#[doc = "Field `USBCTRL` reader - "]
pub type USBCTRL_R = crate::BitReader;
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
impl W {}
#[doc = "Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed.  

You can [`read`](crate::generic::Reg::read) this register and get [`reset_done::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_done::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_DONE_SPEC;
impl crate::RegisterSpec for RESET_DONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_done::R`](R) reader structure"]
impl crate::Readable for RESET_DONE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_done::W`](W) writer structure"]
impl crate::Writable for RESET_DONE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET_DONE to value 0"]
impl crate::Resettable for RESET_DONE_SPEC {
    const RESET_VALUE: u32 = 0;
}
