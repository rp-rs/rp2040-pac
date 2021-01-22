#[doc = "Reader of register DORMANT_WAKE_INTS"]
pub type R = crate::R<u32, super::DORMANT_WAKE_INTS>;
#[doc = "Reader of field `GPIO_QSPI_SD3_EDGE_HIGH`"]
pub type GPIO_QSPI_SD3_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD3_EDGE_LOW`"]
pub type GPIO_QSPI_SD3_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD3_LEVEL_HIGH`"]
pub type GPIO_QSPI_SD3_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD3_LEVEL_LOW`"]
pub type GPIO_QSPI_SD3_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD2_EDGE_HIGH`"]
pub type GPIO_QSPI_SD2_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD2_EDGE_LOW`"]
pub type GPIO_QSPI_SD2_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD2_LEVEL_HIGH`"]
pub type GPIO_QSPI_SD2_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD2_LEVEL_LOW`"]
pub type GPIO_QSPI_SD2_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD1_EDGE_HIGH`"]
pub type GPIO_QSPI_SD1_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD1_EDGE_LOW`"]
pub type GPIO_QSPI_SD1_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD1_LEVEL_HIGH`"]
pub type GPIO_QSPI_SD1_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD1_LEVEL_LOW`"]
pub type GPIO_QSPI_SD1_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD0_EDGE_HIGH`"]
pub type GPIO_QSPI_SD0_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD0_EDGE_LOW`"]
pub type GPIO_QSPI_SD0_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD0_LEVEL_HIGH`"]
pub type GPIO_QSPI_SD0_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SD0_LEVEL_LOW`"]
pub type GPIO_QSPI_SD0_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SS_EDGE_HIGH`"]
pub type GPIO_QSPI_SS_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SS_EDGE_LOW`"]
pub type GPIO_QSPI_SS_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SS_LEVEL_HIGH`"]
pub type GPIO_QSPI_SS_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SS_LEVEL_LOW`"]
pub type GPIO_QSPI_SS_LEVEL_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SCLK_EDGE_HIGH`"]
pub type GPIO_QSPI_SCLK_EDGE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SCLK_EDGE_LOW`"]
pub type GPIO_QSPI_SCLK_EDGE_LOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SCLK_LEVEL_HIGH`"]
pub type GPIO_QSPI_SCLK_LEVEL_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_QSPI_SCLK_LEVEL_LOW`"]
pub type GPIO_QSPI_SCLK_LEVEL_LOW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> GPIO_QSPI_SD3_EDGE_HIGH_R {
        GPIO_QSPI_SD3_EDGE_HIGH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> GPIO_QSPI_SD3_EDGE_LOW_R {
        GPIO_QSPI_SD3_EDGE_LOW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> GPIO_QSPI_SD3_LEVEL_HIGH_R {
        GPIO_QSPI_SD3_LEVEL_HIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> GPIO_QSPI_SD3_LEVEL_LOW_R {
        GPIO_QSPI_SD3_LEVEL_LOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> GPIO_QSPI_SD2_EDGE_HIGH_R {
        GPIO_QSPI_SD2_EDGE_HIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> GPIO_QSPI_SD2_EDGE_LOW_R {
        GPIO_QSPI_SD2_EDGE_LOW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> GPIO_QSPI_SD2_LEVEL_HIGH_R {
        GPIO_QSPI_SD2_LEVEL_HIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> GPIO_QSPI_SD2_LEVEL_LOW_R {
        GPIO_QSPI_SD2_LEVEL_LOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> GPIO_QSPI_SD1_EDGE_HIGH_R {
        GPIO_QSPI_SD1_EDGE_HIGH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> GPIO_QSPI_SD1_EDGE_LOW_R {
        GPIO_QSPI_SD1_EDGE_LOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> GPIO_QSPI_SD1_LEVEL_HIGH_R {
        GPIO_QSPI_SD1_LEVEL_HIGH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> GPIO_QSPI_SD1_LEVEL_LOW_R {
        GPIO_QSPI_SD1_LEVEL_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> GPIO_QSPI_SD0_EDGE_HIGH_R {
        GPIO_QSPI_SD0_EDGE_HIGH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> GPIO_QSPI_SD0_EDGE_LOW_R {
        GPIO_QSPI_SD0_EDGE_LOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> GPIO_QSPI_SD0_LEVEL_HIGH_R {
        GPIO_QSPI_SD0_LEVEL_HIGH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> GPIO_QSPI_SD0_LEVEL_LOW_R {
        GPIO_QSPI_SD0_LEVEL_LOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> GPIO_QSPI_SS_EDGE_HIGH_R {
        GPIO_QSPI_SS_EDGE_HIGH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> GPIO_QSPI_SS_EDGE_LOW_R {
        GPIO_QSPI_SS_EDGE_LOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> GPIO_QSPI_SS_LEVEL_HIGH_R {
        GPIO_QSPI_SS_LEVEL_HIGH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> GPIO_QSPI_SS_LEVEL_LOW_R {
        GPIO_QSPI_SS_LEVEL_LOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> GPIO_QSPI_SCLK_EDGE_HIGH_R {
        GPIO_QSPI_SCLK_EDGE_HIGH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> GPIO_QSPI_SCLK_EDGE_LOW_R {
        GPIO_QSPI_SCLK_EDGE_LOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> GPIO_QSPI_SCLK_LEVEL_HIGH_R {
        GPIO_QSPI_SCLK_LEVEL_HIGH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> GPIO_QSPI_SCLK_LEVEL_LOW_R {
        GPIO_QSPI_SCLK_LEVEL_LOW_R::new((self.bits & 0x01) != 0)
    }
}
