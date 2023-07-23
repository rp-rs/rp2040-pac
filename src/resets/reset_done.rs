#[doc = "Register `RESET_DONE` reader"]
pub struct R(crate::R<RESET_DONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_DONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_DONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_DONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `adc` reader - "]
pub type ADC_R = crate::BitReader<bool>;
#[doc = "Field `busctrl` reader - "]
pub type BUSCTRL_R = crate::BitReader<bool>;
#[doc = "Field `dma` reader - "]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `i2c0` reader - "]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `i2c1` reader - "]
pub type I2C1_R = crate::BitReader<bool>;
#[doc = "Field `io_bank0` reader - "]
pub type IO_BANK0_R = crate::BitReader<bool>;
#[doc = "Field `io_qspi` reader - "]
pub type IO_QSPI_R = crate::BitReader<bool>;
#[doc = "Field `jtag` reader - "]
pub type JTAG_R = crate::BitReader<bool>;
#[doc = "Field `pads_bank0` reader - "]
pub type PADS_BANK0_R = crate::BitReader<bool>;
#[doc = "Field `pads_qspi` reader - "]
pub type PADS_QSPI_R = crate::BitReader<bool>;
#[doc = "Field `pio0` reader - "]
pub type PIO0_R = crate::BitReader<bool>;
#[doc = "Field `pio1` reader - "]
pub type PIO1_R = crate::BitReader<bool>;
#[doc = "Field `pll_sys` reader - "]
pub type PLL_SYS_R = crate::BitReader<bool>;
#[doc = "Field `pll_usb` reader - "]
pub type PLL_USB_R = crate::BitReader<bool>;
#[doc = "Field `pwm` reader - "]
pub type PWM_R = crate::BitReader<bool>;
#[doc = "Field `rtc` reader - "]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `spi0` reader - "]
pub type SPI0_R = crate::BitReader<bool>;
#[doc = "Field `spi1` reader - "]
pub type SPI1_R = crate::BitReader<bool>;
#[doc = "Field `syscfg` reader - "]
pub type SYSCFG_R = crate::BitReader<bool>;
#[doc = "Field `sysinfo` reader - "]
pub type SYSINFO_R = crate::BitReader<bool>;
#[doc = "Field `tbman` reader - "]
pub type TBMAN_R = crate::BitReader<bool>;
#[doc = "Field `timer` reader - "]
pub type TIMER_R = crate::BitReader<bool>;
#[doc = "Field `uart0` reader - "]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `uart1` reader - "]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `usbctrl` reader - "]
pub type USBCTRL_R = crate::BitReader<bool>;
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
#[doc = "Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [reset_done](index.html) module"]
pub struct RESET_DONE_SPEC;
impl crate::RegisterSpec for RESET_DONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_done::R](R) reader structure"]
impl crate::Readable for RESET_DONE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESET_DONE to value 0"]
impl crate::Resettable for RESET_DONE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
