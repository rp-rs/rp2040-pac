#[doc = "Register `ENABLED0` reader"]
pub struct R(crate::R<ENABLED0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLED0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLED0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLED0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `clk_sys_clocks` reader - "]
pub type CLK_SYS_CLOCKS_R = crate::BitReader<bool>;
#[doc = "Field `clk_adc_adc` reader - "]
pub type CLK_ADC_ADC_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_adc` reader - "]
pub type CLK_SYS_ADC_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_busctrl` reader - "]
pub type CLK_SYS_BUSCTRL_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_busfabric` reader - "]
pub type CLK_SYS_BUSFABRIC_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_dma` reader - "]
pub type CLK_SYS_DMA_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_i2c0` reader - "]
pub type CLK_SYS_I2C0_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_i2c1` reader - "]
pub type CLK_SYS_I2C1_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_io` reader - "]
pub type CLK_SYS_IO_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_jtag` reader - "]
pub type CLK_SYS_JTAG_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_vreg_and_chip_reset` reader - "]
pub type CLK_SYS_VREG_AND_CHIP_RESET_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_pads` reader - "]
pub type CLK_SYS_PADS_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_pio0` reader - "]
pub type CLK_SYS_PIO0_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_pio1` reader - "]
pub type CLK_SYS_PIO1_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_pll_sys` reader - "]
pub type CLK_SYS_PLL_SYS_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_pll_usb` reader - "]
pub type CLK_SYS_PLL_USB_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_psm` reader - "]
pub type CLK_SYS_PSM_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_pwm` reader - "]
pub type CLK_SYS_PWM_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_resets` reader - "]
pub type CLK_SYS_RESETS_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_rom` reader - "]
pub type CLK_SYS_ROM_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_rosc` reader - "]
pub type CLK_SYS_ROSC_R = crate::BitReader<bool>;
#[doc = "Field `clk_rtc_rtc` reader - "]
pub type CLK_RTC_RTC_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_rtc` reader - "]
pub type CLK_SYS_RTC_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_sio` reader - "]
pub type CLK_SYS_SIO_R = crate::BitReader<bool>;
#[doc = "Field `clk_peri_spi0` reader - "]
pub type CLK_PERI_SPI0_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_spi0` reader - "]
pub type CLK_SYS_SPI0_R = crate::BitReader<bool>;
#[doc = "Field `clk_peri_spi1` reader - "]
pub type CLK_PERI_SPI1_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_spi1` reader - "]
pub type CLK_SYS_SPI1_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_sram0` reader - "]
pub type CLK_SYS_SRAM0_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_sram1` reader - "]
pub type CLK_SYS_SRAM1_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_sram2` reader - "]
pub type CLK_SYS_SRAM2_R = crate::BitReader<bool>;
#[doc = "Field `clk_sys_sram3` reader - "]
pub type CLK_SYS_SRAM3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_sys_clocks(&self) -> CLK_SYS_CLOCKS_R {
        CLK_SYS_CLOCKS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_adc_adc(&self) -> CLK_ADC_ADC_R {
        CLK_ADC_ADC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_sys_adc(&self) -> CLK_SYS_ADC_R {
        CLK_SYS_ADC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_sys_busctrl(&self) -> CLK_SYS_BUSCTRL_R {
        CLK_SYS_BUSCTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_sys_busfabric(&self) -> CLK_SYS_BUSFABRIC_R {
        CLK_SYS_BUSFABRIC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_sys_dma(&self) -> CLK_SYS_DMA_R {
        CLK_SYS_DMA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_sys_i2c0(&self) -> CLK_SYS_I2C0_R {
        CLK_SYS_I2C0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_sys_i2c1(&self) -> CLK_SYS_I2C1_R {
        CLK_SYS_I2C1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_sys_io(&self) -> CLK_SYS_IO_R {
        CLK_SYS_IO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_sys_jtag(&self) -> CLK_SYS_JTAG_R {
        CLK_SYS_JTAG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_sys_vreg_and_chip_reset(&self) -> CLK_SYS_VREG_AND_CHIP_RESET_R {
        CLK_SYS_VREG_AND_CHIP_RESET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_sys_pads(&self) -> CLK_SYS_PADS_R {
        CLK_SYS_PADS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_sys_pio0(&self) -> CLK_SYS_PIO0_R {
        CLK_SYS_PIO0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_sys_pio1(&self) -> CLK_SYS_PIO1_R {
        CLK_SYS_PIO1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_sys_pll_sys(&self) -> CLK_SYS_PLL_SYS_R {
        CLK_SYS_PLL_SYS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clk_sys_pll_usb(&self) -> CLK_SYS_PLL_USB_R {
        CLK_SYS_PLL_USB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_sys_psm(&self) -> CLK_SYS_PSM_R {
        CLK_SYS_PSM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clk_sys_pwm(&self) -> CLK_SYS_PWM_R {
        CLK_SYS_PWM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_sys_resets(&self) -> CLK_SYS_RESETS_R {
        CLK_SYS_RESETS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clk_sys_rom(&self) -> CLK_SYS_ROM_R {
        CLK_SYS_ROM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_sys_rosc(&self) -> CLK_SYS_ROSC_R {
        CLK_SYS_ROSC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clk_rtc_rtc(&self) -> CLK_RTC_RTC_R {
        CLK_RTC_RTC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_sys_rtc(&self) -> CLK_SYS_RTC_R {
        CLK_SYS_RTC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn clk_sys_sio(&self) -> CLK_SYS_SIO_R {
        CLK_SYS_SIO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn clk_peri_spi0(&self) -> CLK_PERI_SPI0_R {
        CLK_PERI_SPI0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_sys_spi0(&self) -> CLK_SYS_SPI0_R {
        CLK_SYS_SPI0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn clk_peri_spi1(&self) -> CLK_PERI_SPI1_R {
        CLK_PERI_SPI1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn clk_sys_spi1(&self) -> CLK_SYS_SPI1_R {
        CLK_SYS_SPI1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_sys_sram0(&self) -> CLK_SYS_SRAM0_R {
        CLK_SYS_SRAM0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_sys_sram1(&self) -> CLK_SYS_SRAM1_R {
        CLK_SYS_SRAM1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_sys_sram2(&self) -> CLK_SYS_SRAM2_R {
        CLK_SYS_SRAM2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_sys_sram3(&self) -> CLK_SYS_SRAM3_R {
        CLK_SYS_SRAM3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "indicates the state of the clock enable  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [enabled0](index.html) module"]
pub struct ENABLED0_SPEC;
impl crate::RegisterSpec for ENABLED0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enabled0::R](R) reader structure"]
impl crate::Readable for ENABLED0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ENABLED0 to value 0"]
impl crate::Resettable for ENABLED0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
