#[doc = "Reader of register RESET_DONE"]
pub type R = crate::R<u32, super::RESET_DONE>;
#[doc = "Reader of field `usbctrl`"]
pub type USBCTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `uart1`"]
pub type UART1_R = crate::R<bool, bool>;
#[doc = "Reader of field `uart0`"]
pub type UART0_R = crate::R<bool, bool>;
#[doc = "Reader of field `timer`"]
pub type TIMER_R = crate::R<bool, bool>;
#[doc = "Reader of field `tbman`"]
pub type TBMAN_R = crate::R<bool, bool>;
#[doc = "Reader of field `sysinfo`"]
pub type SYSINFO_R = crate::R<bool, bool>;
#[doc = "Reader of field `syscfg`"]
pub type SYSCFG_R = crate::R<bool, bool>;
#[doc = "Reader of field `spi1`"]
pub type SPI1_R = crate::R<bool, bool>;
#[doc = "Reader of field `spi0`"]
pub type SPI0_R = crate::R<bool, bool>;
#[doc = "Reader of field `rtc`"]
pub type RTC_R = crate::R<bool, bool>;
#[doc = "Reader of field `pwm`"]
pub type PWM_R = crate::R<bool, bool>;
#[doc = "Reader of field `pll_usb`"]
pub type PLL_USB_R = crate::R<bool, bool>;
#[doc = "Reader of field `pll_sys`"]
pub type PLL_SYS_R = crate::R<bool, bool>;
#[doc = "Reader of field `pio1`"]
pub type PIO1_R = crate::R<bool, bool>;
#[doc = "Reader of field `pio0`"]
pub type PIO0_R = crate::R<bool, bool>;
#[doc = "Reader of field `pads_qspi`"]
pub type PADS_QSPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `pads_bank0`"]
pub type PADS_BANK0_R = crate::R<bool, bool>;
#[doc = "Reader of field `jtag`"]
pub type JTAG_R = crate::R<bool, bool>;
#[doc = "Reader of field `io_qspi`"]
pub type IO_QSPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `io_bank0`"]
pub type IO_BANK0_R = crate::R<bool, bool>;
#[doc = "Reader of field `i2c1`"]
pub type I2C1_R = crate::R<bool, bool>;
#[doc = "Reader of field `i2c0`"]
pub type I2C0_R = crate::R<bool, bool>;
#[doc = "Reader of field `dma`"]
pub type DMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `busctrl`"]
pub type BUSCTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `adc`"]
pub type ADC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usbctrl(&self) -> USBCTRL_R {
        USBCTRL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tbman(&self) -> TBMAN_R {
        TBMAN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sysinfo(&self) -> SYSINFO_R {
        SYSINFO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn syscfg(&self) -> SYSCFG_R {
        SYSCFG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pwm(&self) -> PWM_R {
        PWM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pll_usb(&self) -> PLL_USB_R {
        PLL_USB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pll_sys(&self) -> PLL_SYS_R {
        PLL_SYS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pio1(&self) -> PIO1_R {
        PIO1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pio0(&self) -> PIO0_R {
        PIO0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pads_qspi(&self) -> PADS_QSPI_R {
        PADS_QSPI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pads_bank0(&self) -> PADS_BANK0_R {
        PADS_BANK0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn jtag(&self) -> JTAG_R {
        JTAG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn io_qspi(&self) -> IO_QSPI_R {
        IO_QSPI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn io_bank0(&self) -> IO_BANK0_R {
        IO_BANK0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn busctrl(&self) -> BUSCTRL_R {
        BUSCTRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 0x01) != 0)
    }
}
