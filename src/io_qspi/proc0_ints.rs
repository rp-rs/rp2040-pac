#[doc = "Register `PROC0_INTS` reader"]
pub struct R(crate::R<PROC0_INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC0_INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC0_INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC0_INTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO_QSPI_SCLK_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SCLK_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SCLK_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SCLK_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SCLK_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SCLK_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SCLK_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SCLK_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SS_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SS_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SS_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SS_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SS_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SS_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SS_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SS_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD0_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SD0_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD0_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SD0_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD0_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SD0_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD0_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SD0_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD1_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SD1_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD1_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SD1_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD1_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SD1_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD1_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SD1_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD2_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SD2_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD2_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SD2_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD2_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SD2_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD2_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SD2_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD3_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SD3_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD3_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SD3_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD3_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SD3_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD3_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SD3_EDGE_HIGH_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> GPIO_QSPI_SCLK_LEVEL_LOW_R {
        GPIO_QSPI_SCLK_LEVEL_LOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> GPIO_QSPI_SCLK_LEVEL_HIGH_R {
        GPIO_QSPI_SCLK_LEVEL_HIGH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> GPIO_QSPI_SCLK_EDGE_LOW_R {
        GPIO_QSPI_SCLK_EDGE_LOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> GPIO_QSPI_SCLK_EDGE_HIGH_R {
        GPIO_QSPI_SCLK_EDGE_HIGH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> GPIO_QSPI_SS_LEVEL_LOW_R {
        GPIO_QSPI_SS_LEVEL_LOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> GPIO_QSPI_SS_LEVEL_HIGH_R {
        GPIO_QSPI_SS_LEVEL_HIGH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> GPIO_QSPI_SS_EDGE_LOW_R {
        GPIO_QSPI_SS_EDGE_LOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> GPIO_QSPI_SS_EDGE_HIGH_R {
        GPIO_QSPI_SS_EDGE_HIGH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> GPIO_QSPI_SD0_LEVEL_LOW_R {
        GPIO_QSPI_SD0_LEVEL_LOW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> GPIO_QSPI_SD0_LEVEL_HIGH_R {
        GPIO_QSPI_SD0_LEVEL_HIGH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> GPIO_QSPI_SD0_EDGE_LOW_R {
        GPIO_QSPI_SD0_EDGE_LOW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> GPIO_QSPI_SD0_EDGE_HIGH_R {
        GPIO_QSPI_SD0_EDGE_HIGH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> GPIO_QSPI_SD1_LEVEL_LOW_R {
        GPIO_QSPI_SD1_LEVEL_LOW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> GPIO_QSPI_SD1_LEVEL_HIGH_R {
        GPIO_QSPI_SD1_LEVEL_HIGH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> GPIO_QSPI_SD1_EDGE_LOW_R {
        GPIO_QSPI_SD1_EDGE_LOW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> GPIO_QSPI_SD1_EDGE_HIGH_R {
        GPIO_QSPI_SD1_EDGE_HIGH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> GPIO_QSPI_SD2_LEVEL_LOW_R {
        GPIO_QSPI_SD2_LEVEL_LOW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> GPIO_QSPI_SD2_LEVEL_HIGH_R {
        GPIO_QSPI_SD2_LEVEL_HIGH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> GPIO_QSPI_SD2_EDGE_LOW_R {
        GPIO_QSPI_SD2_EDGE_LOW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> GPIO_QSPI_SD2_EDGE_HIGH_R {
        GPIO_QSPI_SD2_EDGE_HIGH_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> GPIO_QSPI_SD3_LEVEL_LOW_R {
        GPIO_QSPI_SD3_LEVEL_LOW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> GPIO_QSPI_SD3_LEVEL_HIGH_R {
        GPIO_QSPI_SD3_LEVEL_HIGH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> GPIO_QSPI_SD3_EDGE_LOW_R {
        GPIO_QSPI_SD3_EDGE_LOW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> GPIO_QSPI_SD3_EDGE_HIGH_R {
        GPIO_QSPI_SD3_EDGE_HIGH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Interrupt status after masking & forcing for proc0  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [proc0_ints](index.html) module"]
pub struct PROC0_INTS_SPEC;
impl crate::RegisterSpec for PROC0_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc0_ints::R](R) reader structure"]
impl crate::Readable for PROC0_INTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PROC0_INTS to value 0"]
impl crate::Resettable for PROC0_INTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
