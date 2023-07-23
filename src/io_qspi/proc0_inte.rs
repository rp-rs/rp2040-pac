#[doc = "Register `PROC0_INTE` reader"]
pub struct R(crate::R<PROC0_INTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC0_INTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC0_INTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC0_INTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROC0_INTE` writer"]
pub struct W(crate::W<PROC0_INTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROC0_INTE_SPEC>;
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
impl From<crate::W<PROC0_INTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROC0_INTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_QSPI_SCLK_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SCLK_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SCLK_LEVEL_LOW` writer - "]
pub type GPIO_QSPI_SCLK_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SCLK_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SCLK_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SCLK_LEVEL_HIGH` writer - "]
pub type GPIO_QSPI_SCLK_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SCLK_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SCLK_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SCLK_EDGE_LOW` writer - "]
pub type GPIO_QSPI_SCLK_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SCLK_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SCLK_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SCLK_EDGE_HIGH` writer - "]
pub type GPIO_QSPI_SCLK_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SS_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SS_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SS_LEVEL_LOW` writer - "]
pub type GPIO_QSPI_SS_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SS_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SS_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SS_LEVEL_HIGH` writer - "]
pub type GPIO_QSPI_SS_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SS_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SS_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SS_EDGE_LOW` writer - "]
pub type GPIO_QSPI_SS_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SS_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SS_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SS_EDGE_HIGH` writer - "]
pub type GPIO_QSPI_SS_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD0_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SD0_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD0_LEVEL_LOW` writer - "]
pub type GPIO_QSPI_SD0_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD0_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SD0_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD0_LEVEL_HIGH` writer - "]
pub type GPIO_QSPI_SD0_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD0_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SD0_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD0_EDGE_LOW` writer - "]
pub type GPIO_QSPI_SD0_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD0_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SD0_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD0_EDGE_HIGH` writer - "]
pub type GPIO_QSPI_SD0_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD1_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SD1_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD1_LEVEL_LOW` writer - "]
pub type GPIO_QSPI_SD1_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD1_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SD1_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD1_LEVEL_HIGH` writer - "]
pub type GPIO_QSPI_SD1_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD1_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SD1_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD1_EDGE_LOW` writer - "]
pub type GPIO_QSPI_SD1_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD1_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SD1_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD1_EDGE_HIGH` writer - "]
pub type GPIO_QSPI_SD1_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD2_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SD2_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD2_LEVEL_LOW` writer - "]
pub type GPIO_QSPI_SD2_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD2_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SD2_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD2_LEVEL_HIGH` writer - "]
pub type GPIO_QSPI_SD2_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD2_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SD2_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD2_EDGE_LOW` writer - "]
pub type GPIO_QSPI_SD2_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD2_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SD2_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD2_EDGE_HIGH` writer - "]
pub type GPIO_QSPI_SD2_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD3_LEVEL_LOW` reader - "]
pub type GPIO_QSPI_SD3_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD3_LEVEL_LOW` writer - "]
pub type GPIO_QSPI_SD3_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD3_LEVEL_HIGH` reader - "]
pub type GPIO_QSPI_SD3_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD3_LEVEL_HIGH` writer - "]
pub type GPIO_QSPI_SD3_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD3_EDGE_LOW` reader - "]
pub type GPIO_QSPI_SD3_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD3_EDGE_LOW` writer - "]
pub type GPIO_QSPI_SD3_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO_QSPI_SD3_EDGE_HIGH` reader - "]
pub type GPIO_QSPI_SD3_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_QSPI_SD3_EDGE_HIGH` writer - "]
pub type GPIO_QSPI_SD3_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROC0_INTE_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sclk_level_low(&mut self) -> GPIO_QSPI_SCLK_LEVEL_LOW_W<0> {
        GPIO_QSPI_SCLK_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sclk_level_high(&mut self) -> GPIO_QSPI_SCLK_LEVEL_HIGH_W<1> {
        GPIO_QSPI_SCLK_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sclk_edge_low(&mut self) -> GPIO_QSPI_SCLK_EDGE_LOW_W<2> {
        GPIO_QSPI_SCLK_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sclk_edge_high(&mut self) -> GPIO_QSPI_SCLK_EDGE_HIGH_W<3> {
        GPIO_QSPI_SCLK_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_ss_level_low(&mut self) -> GPIO_QSPI_SS_LEVEL_LOW_W<4> {
        GPIO_QSPI_SS_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_ss_level_high(&mut self) -> GPIO_QSPI_SS_LEVEL_HIGH_W<5> {
        GPIO_QSPI_SS_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_ss_edge_low(&mut self) -> GPIO_QSPI_SS_EDGE_LOW_W<6> {
        GPIO_QSPI_SS_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_ss_edge_high(&mut self) -> GPIO_QSPI_SS_EDGE_HIGH_W<7> {
        GPIO_QSPI_SS_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd0_level_low(&mut self) -> GPIO_QSPI_SD0_LEVEL_LOW_W<8> {
        GPIO_QSPI_SD0_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd0_level_high(&mut self) -> GPIO_QSPI_SD0_LEVEL_HIGH_W<9> {
        GPIO_QSPI_SD0_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd0_edge_low(&mut self) -> GPIO_QSPI_SD0_EDGE_LOW_W<10> {
        GPIO_QSPI_SD0_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd0_edge_high(&mut self) -> GPIO_QSPI_SD0_EDGE_HIGH_W<11> {
        GPIO_QSPI_SD0_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd1_level_low(&mut self) -> GPIO_QSPI_SD1_LEVEL_LOW_W<12> {
        GPIO_QSPI_SD1_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd1_level_high(&mut self) -> GPIO_QSPI_SD1_LEVEL_HIGH_W<13> {
        GPIO_QSPI_SD1_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd1_edge_low(&mut self) -> GPIO_QSPI_SD1_EDGE_LOW_W<14> {
        GPIO_QSPI_SD1_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd1_edge_high(&mut self) -> GPIO_QSPI_SD1_EDGE_HIGH_W<15> {
        GPIO_QSPI_SD1_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd2_level_low(&mut self) -> GPIO_QSPI_SD2_LEVEL_LOW_W<16> {
        GPIO_QSPI_SD2_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd2_level_high(&mut self) -> GPIO_QSPI_SD2_LEVEL_HIGH_W<17> {
        GPIO_QSPI_SD2_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd2_edge_low(&mut self) -> GPIO_QSPI_SD2_EDGE_LOW_W<18> {
        GPIO_QSPI_SD2_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd2_edge_high(&mut self) -> GPIO_QSPI_SD2_EDGE_HIGH_W<19> {
        GPIO_QSPI_SD2_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd3_level_low(&mut self) -> GPIO_QSPI_SD3_LEVEL_LOW_W<20> {
        GPIO_QSPI_SD3_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd3_level_high(&mut self) -> GPIO_QSPI_SD3_LEVEL_HIGH_W<21> {
        GPIO_QSPI_SD3_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd3_edge_low(&mut self) -> GPIO_QSPI_SD3_EDGE_LOW_W<22> {
        GPIO_QSPI_SD3_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_qspi_sd3_edge_high(&mut self) -> GPIO_QSPI_SD3_EDGE_HIGH_W<23> {
        GPIO_QSPI_SD3_EDGE_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable for proc0  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [proc0_inte](index.html) module"]
pub struct PROC0_INTE_SPEC;
impl crate::RegisterSpec for PROC0_INTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc0_inte::R](R) reader structure"]
impl crate::Readable for PROC0_INTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [proc0_inte::W](W) writer structure"]
impl crate::Writable for PROC0_INTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROC0_INTE to value 0"]
impl crate::Resettable for PROC0_INTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
