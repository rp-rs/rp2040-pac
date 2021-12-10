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
#[doc = "Field `usbctrl` reader - "]
pub struct USBCTRL_R(crate::FieldReader<bool, bool>);
impl USBCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart1` reader - "]
pub struct UART1_R(crate::FieldReader<bool, bool>);
impl UART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart0` reader - "]
pub struct UART0_R(crate::FieldReader<bool, bool>);
impl UART0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer` reader - "]
pub struct TIMER_R(crate::FieldReader<bool, bool>);
impl TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbman` reader - "]
pub struct TBMAN_R(crate::FieldReader<bool, bool>);
impl TBMAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TBMAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBMAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sysinfo` reader - "]
pub struct SYSINFO_R(crate::FieldReader<bool, bool>);
impl SYSINFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSINFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSINFO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `syscfg` reader - "]
pub struct SYSCFG_R(crate::FieldReader<bool, bool>);
impl SYSCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spi1` reader - "]
pub struct SPI1_R(crate::FieldReader<bool, bool>);
impl SPI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spi0` reader - "]
pub struct SPI0_R(crate::FieldReader<bool, bool>);
impl SPI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtc` reader - "]
pub struct RTC_R(crate::FieldReader<bool, bool>);
impl RTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm` reader - "]
pub struct PWM_R(crate::FieldReader<bool, bool>);
impl PWM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pll_usb` reader - "]
pub struct PLL_USB_R(crate::FieldReader<bool, bool>);
impl PLL_USB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_USB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_USB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pll_sys` reader - "]
pub struct PLL_SYS_R(crate::FieldReader<bool, bool>);
impl PLL_SYS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_SYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_SYS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pio1` reader - "]
pub struct PIO1_R(crate::FieldReader<bool, bool>);
impl PIO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pio0` reader - "]
pub struct PIO0_R(crate::FieldReader<bool, bool>);
impl PIO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pads_qspi` reader - "]
pub struct PADS_QSPI_R(crate::FieldReader<bool, bool>);
impl PADS_QSPI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PADS_QSPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADS_QSPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pads_bank0` reader - "]
pub struct PADS_BANK0_R(crate::FieldReader<bool, bool>);
impl PADS_BANK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PADS_BANK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADS_BANK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `jtag` reader - "]
pub struct JTAG_R(crate::FieldReader<bool, bool>);
impl JTAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `io_qspi` reader - "]
pub struct IO_QSPI_R(crate::FieldReader<bool, bool>);
impl IO_QSPI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_QSPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_QSPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `io_bank0` reader - "]
pub struct IO_BANK0_R(crate::FieldReader<bool, bool>);
impl IO_BANK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_BANK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_BANK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c1` reader - "]
pub struct I2C1_R(crate::FieldReader<bool, bool>);
impl I2C1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c0` reader - "]
pub struct I2C0_R(crate::FieldReader<bool, bool>);
impl I2C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma` reader - "]
pub struct DMA_R(crate::FieldReader<bool, bool>);
impl DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `busctrl` reader - "]
pub struct BUSCTRL_R(crate::FieldReader<bool, bool>);
impl BUSCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc` reader - "]
pub struct ADC_R(crate::FieldReader<bool, bool>);
impl ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
