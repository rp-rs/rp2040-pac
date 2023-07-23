#[doc = "Register `RESET` reader"]
pub struct R(crate::R<RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET` writer"]
pub struct W(crate::W<RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc` reader - "]
pub type ADC_R = crate::BitReader<bool>;
#[doc = "Field `adc` writer - "]
pub type ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `busctrl` reader - "]
pub type BUSCTRL_R = crate::BitReader<bool>;
#[doc = "Field `busctrl` writer - "]
pub type BUSCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `dma` reader - "]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `dma` writer - "]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `i2c0` reader - "]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `i2c0` writer - "]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `i2c1` reader - "]
pub type I2C1_R = crate::BitReader<bool>;
#[doc = "Field `i2c1` writer - "]
pub type I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `io_bank0` reader - "]
pub type IO_BANK0_R = crate::BitReader<bool>;
#[doc = "Field `io_bank0` writer - "]
pub type IO_BANK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `io_qspi` reader - "]
pub type IO_QSPI_R = crate::BitReader<bool>;
#[doc = "Field `io_qspi` writer - "]
pub type IO_QSPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `jtag` reader - "]
pub type JTAG_R = crate::BitReader<bool>;
#[doc = "Field `jtag` writer - "]
pub type JTAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `pads_bank0` reader - "]
pub type PADS_BANK0_R = crate::BitReader<bool>;
#[doc = "Field `pads_bank0` writer - "]
pub type PADS_BANK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `pads_qspi` reader - "]
pub type PADS_QSPI_R = crate::BitReader<bool>;
#[doc = "Field `pads_qspi` writer - "]
pub type PADS_QSPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `pio0` reader - "]
pub type PIO0_R = crate::BitReader<bool>;
#[doc = "Field `pio0` writer - "]
pub type PIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `pio1` reader - "]
pub type PIO1_R = crate::BitReader<bool>;
#[doc = "Field `pio1` writer - "]
pub type PIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `pll_sys` reader - "]
pub type PLL_SYS_R = crate::BitReader<bool>;
#[doc = "Field `pll_sys` writer - "]
pub type PLL_SYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `pll_usb` reader - "]
pub type PLL_USB_R = crate::BitReader<bool>;
#[doc = "Field `pll_usb` writer - "]
pub type PLL_USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `pwm` reader - "]
pub type PWM_R = crate::BitReader<bool>;
#[doc = "Field `pwm` writer - "]
pub type PWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `rtc` reader - "]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `rtc` writer - "]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `spi0` reader - "]
pub type SPI0_R = crate::BitReader<bool>;
#[doc = "Field `spi0` writer - "]
pub type SPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `spi1` reader - "]
pub type SPI1_R = crate::BitReader<bool>;
#[doc = "Field `spi1` writer - "]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `syscfg` reader - "]
pub type SYSCFG_R = crate::BitReader<bool>;
#[doc = "Field `syscfg` writer - "]
pub type SYSCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `sysinfo` reader - "]
pub type SYSINFO_R = crate::BitReader<bool>;
#[doc = "Field `sysinfo` writer - "]
pub type SYSINFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `tbman` reader - "]
pub type TBMAN_R = crate::BitReader<bool>;
#[doc = "Field `tbman` writer - "]
pub type TBMAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `timer` reader - "]
pub type TIMER_R = crate::BitReader<bool>;
#[doc = "Field `timer` writer - "]
pub type TIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `uart0` reader - "]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `uart0` writer - "]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `uart1` reader - "]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `uart1` writer - "]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `usbctrl` reader - "]
pub type USBCTRL_R = crate::BitReader<bool>;
#[doc = "Field `usbctrl` writer - "]
pub type USBCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
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
    pub fn adc(&mut self) -> ADC_W<0> {
        ADC_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn busctrl(&mut self) -> BUSCTRL_W<1> {
        BUSCTRL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<2> {
        DMA_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<3> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<4> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn io_bank0(&mut self) -> IO_BANK0_W<5> {
        IO_BANK0_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn io_qspi(&mut self) -> IO_QSPI_W<6> {
        IO_QSPI_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn jtag(&mut self) -> JTAG_W<7> {
        JTAG_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pads_bank0(&mut self) -> PADS_BANK0_W<8> {
        PADS_BANK0_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pads_qspi(&mut self) -> PADS_QSPI_W<9> {
        PADS_QSPI_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pio0(&mut self) -> PIO0_W<10> {
        PIO0_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pio1(&mut self) -> PIO1_W<11> {
        PIO1_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pll_sys(&mut self) -> PLL_SYS_W<12> {
        PLL_SYS_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pll_usb(&mut self) -> PLL_USB_W<13> {
        PLL_USB_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pwm(&mut self) -> PWM_W<14> {
        PWM_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<15> {
        RTC_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<16> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<17> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn syscfg(&mut self) -> SYSCFG_W<18> {
        SYSCFG_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn sysinfo(&mut self) -> SYSINFO_W<19> {
        SYSINFO_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn tbman(&mut self) -> TBMAN_W<20> {
        TBMAN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<21> {
        TIMER_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<22> {
        UART0_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<23> {
        UART1_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn usbctrl(&mut self) -> USBCTRL_W<24> {
        USBCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset control. If a bit is set it means the peripheral is in reset. 0 means the peripheral's reset is deasserted.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [reset](index.html) module"]
pub struct RESET_SPEC;
impl crate::RegisterSpec for RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset::R](R) reader structure"]
impl crate::Readable for RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset::W](W) writer structure"]
impl crate::Writable for RESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET to value 0x01ff_ffff"]
impl crate::Resettable for RESET_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff_ffff;
}
