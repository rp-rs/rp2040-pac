#[doc = "Register `DORMANT_WAKE_INTE%s` reader"]
pub struct R(crate::R<DORMANT_WAKE_INTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DORMANT_WAKE_INTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DORMANT_WAKE_INTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DORMANT_WAKE_INTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DORMANT_WAKE_INTE%s` writer"]
pub struct W(crate::W<DORMANT_WAKE_INTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DORMANT_WAKE_INTE_SPEC>;
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
impl From<crate::W<DORMANT_WAKE_INTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DORMANT_WAKE_INTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO0_LEVEL_LOW` reader - "]
pub type GPIO0_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO0_LEVEL_LOW` writer - "]
pub type GPIO0_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO0_LEVEL_HIGH` reader - "]
pub type GPIO0_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO0_LEVEL_HIGH` writer - "]
pub type GPIO0_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO0_EDGE_LOW` reader - "]
pub type GPIO0_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO0_EDGE_LOW` writer - "]
pub type GPIO0_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO0_EDGE_HIGH` reader - "]
pub type GPIO0_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO0_EDGE_HIGH` writer - "]
pub type GPIO0_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO1_LEVEL_LOW` reader - "]
pub type GPIO1_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO1_LEVEL_LOW` writer - "]
pub type GPIO1_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO1_LEVEL_HIGH` reader - "]
pub type GPIO1_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO1_LEVEL_HIGH` writer - "]
pub type GPIO1_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO1_EDGE_LOW` reader - "]
pub type GPIO1_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO1_EDGE_LOW` writer - "]
pub type GPIO1_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO1_EDGE_HIGH` reader - "]
pub type GPIO1_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO1_EDGE_HIGH` writer - "]
pub type GPIO1_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO2_LEVEL_LOW` reader - "]
pub type GPIO2_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO2_LEVEL_LOW` writer - "]
pub type GPIO2_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO2_LEVEL_HIGH` reader - "]
pub type GPIO2_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO2_LEVEL_HIGH` writer - "]
pub type GPIO2_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO2_EDGE_LOW` reader - "]
pub type GPIO2_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO2_EDGE_LOW` writer - "]
pub type GPIO2_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO2_EDGE_HIGH` reader - "]
pub type GPIO2_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO2_EDGE_HIGH` writer - "]
pub type GPIO2_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO3_LEVEL_LOW` reader - "]
pub type GPIO3_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO3_LEVEL_LOW` writer - "]
pub type GPIO3_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO3_LEVEL_HIGH` reader - "]
pub type GPIO3_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO3_LEVEL_HIGH` writer - "]
pub type GPIO3_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO3_EDGE_LOW` reader - "]
pub type GPIO3_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO3_EDGE_LOW` writer - "]
pub type GPIO3_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO3_EDGE_HIGH` reader - "]
pub type GPIO3_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO3_EDGE_HIGH` writer - "]
pub type GPIO3_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO4_LEVEL_LOW` reader - "]
pub type GPIO4_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO4_LEVEL_LOW` writer - "]
pub type GPIO4_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO4_LEVEL_HIGH` reader - "]
pub type GPIO4_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO4_LEVEL_HIGH` writer - "]
pub type GPIO4_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO4_EDGE_LOW` reader - "]
pub type GPIO4_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO4_EDGE_LOW` writer - "]
pub type GPIO4_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO4_EDGE_HIGH` reader - "]
pub type GPIO4_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO4_EDGE_HIGH` writer - "]
pub type GPIO4_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO5_LEVEL_LOW` reader - "]
pub type GPIO5_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO5_LEVEL_LOW` writer - "]
pub type GPIO5_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO5_LEVEL_HIGH` reader - "]
pub type GPIO5_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO5_LEVEL_HIGH` writer - "]
pub type GPIO5_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO5_EDGE_LOW` reader - "]
pub type GPIO5_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO5_EDGE_LOW` writer - "]
pub type GPIO5_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO5_EDGE_HIGH` reader - "]
pub type GPIO5_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO5_EDGE_HIGH` writer - "]
pub type GPIO5_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO6_LEVEL_LOW` reader - "]
pub type GPIO6_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO6_LEVEL_LOW` writer - "]
pub type GPIO6_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO6_LEVEL_HIGH` reader - "]
pub type GPIO6_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO6_LEVEL_HIGH` writer - "]
pub type GPIO6_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO6_EDGE_LOW` reader - "]
pub type GPIO6_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO6_EDGE_LOW` writer - "]
pub type GPIO6_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO6_EDGE_HIGH` reader - "]
pub type GPIO6_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO6_EDGE_HIGH` writer - "]
pub type GPIO6_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO7_LEVEL_LOW` reader - "]
pub type GPIO7_LEVEL_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO7_LEVEL_LOW` writer - "]
pub type GPIO7_LEVEL_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO7_LEVEL_HIGH` reader - "]
pub type GPIO7_LEVEL_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO7_LEVEL_HIGH` writer - "]
pub type GPIO7_LEVEL_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO7_EDGE_LOW` reader - "]
pub type GPIO7_EDGE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO7_EDGE_LOW` writer - "]
pub type GPIO7_EDGE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
#[doc = "Field `GPIO7_EDGE_HIGH` reader - "]
pub type GPIO7_EDGE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `GPIO7_EDGE_HIGH` writer - "]
pub type GPIO7_EDGE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DORMANT_WAKE_INTE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio0_level_low(&self) -> GPIO0_LEVEL_LOW_R {
        GPIO0_LEVEL_LOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio0_level_high(&self) -> GPIO0_LEVEL_HIGH_R {
        GPIO0_LEVEL_HIGH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio0_edge_low(&self) -> GPIO0_EDGE_LOW_R {
        GPIO0_EDGE_LOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio0_edge_high(&self) -> GPIO0_EDGE_HIGH_R {
        GPIO0_EDGE_HIGH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio1_level_low(&self) -> GPIO1_LEVEL_LOW_R {
        GPIO1_LEVEL_LOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio1_level_high(&self) -> GPIO1_LEVEL_HIGH_R {
        GPIO1_LEVEL_HIGH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio1_edge_low(&self) -> GPIO1_EDGE_LOW_R {
        GPIO1_EDGE_LOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio1_edge_high(&self) -> GPIO1_EDGE_HIGH_R {
        GPIO1_EDGE_HIGH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio2_level_low(&self) -> GPIO2_LEVEL_LOW_R {
        GPIO2_LEVEL_LOW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio2_level_high(&self) -> GPIO2_LEVEL_HIGH_R {
        GPIO2_LEVEL_HIGH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio2_edge_low(&self) -> GPIO2_EDGE_LOW_R {
        GPIO2_EDGE_LOW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio2_edge_high(&self) -> GPIO2_EDGE_HIGH_R {
        GPIO2_EDGE_HIGH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio3_level_low(&self) -> GPIO3_LEVEL_LOW_R {
        GPIO3_LEVEL_LOW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio3_level_high(&self) -> GPIO3_LEVEL_HIGH_R {
        GPIO3_LEVEL_HIGH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio3_edge_low(&self) -> GPIO3_EDGE_LOW_R {
        GPIO3_EDGE_LOW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio3_edge_high(&self) -> GPIO3_EDGE_HIGH_R {
        GPIO3_EDGE_HIGH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio4_level_low(&self) -> GPIO4_LEVEL_LOW_R {
        GPIO4_LEVEL_LOW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio4_level_high(&self) -> GPIO4_LEVEL_HIGH_R {
        GPIO4_LEVEL_HIGH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio4_edge_low(&self) -> GPIO4_EDGE_LOW_R {
        GPIO4_EDGE_LOW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio4_edge_high(&self) -> GPIO4_EDGE_HIGH_R {
        GPIO4_EDGE_HIGH_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio5_level_low(&self) -> GPIO5_LEVEL_LOW_R {
        GPIO5_LEVEL_LOW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio5_level_high(&self) -> GPIO5_LEVEL_HIGH_R {
        GPIO5_LEVEL_HIGH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio5_edge_low(&self) -> GPIO5_EDGE_LOW_R {
        GPIO5_EDGE_LOW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio5_edge_high(&self) -> GPIO5_EDGE_HIGH_R {
        GPIO5_EDGE_HIGH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn gpio6_level_low(&self) -> GPIO6_LEVEL_LOW_R {
        GPIO6_LEVEL_LOW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpio6_level_high(&self) -> GPIO6_LEVEL_HIGH_R {
        GPIO6_LEVEL_HIGH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio6_edge_low(&self) -> GPIO6_EDGE_LOW_R {
        GPIO6_EDGE_LOW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio6_edge_high(&self) -> GPIO6_EDGE_HIGH_R {
        GPIO6_EDGE_HIGH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio7_level_low(&self) -> GPIO7_LEVEL_LOW_R {
        GPIO7_LEVEL_LOW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio7_level_high(&self) -> GPIO7_LEVEL_HIGH_R {
        GPIO7_LEVEL_HIGH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio7_edge_low(&self) -> GPIO7_EDGE_LOW_R {
        GPIO7_EDGE_LOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio7_edge_high(&self) -> GPIO7_EDGE_HIGH_R {
        GPIO7_EDGE_HIGH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0_level_low(&mut self) -> GPIO0_LEVEL_LOW_W<0> {
        GPIO0_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0_level_high(&mut self) -> GPIO0_LEVEL_HIGH_W<1> {
        GPIO0_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0_edge_low(&mut self) -> GPIO0_EDGE_LOW_W<2> {
        GPIO0_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0_edge_high(&mut self) -> GPIO0_EDGE_HIGH_W<3> {
        GPIO0_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1_level_low(&mut self) -> GPIO1_LEVEL_LOW_W<4> {
        GPIO1_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1_level_high(&mut self) -> GPIO1_LEVEL_HIGH_W<5> {
        GPIO1_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1_edge_low(&mut self) -> GPIO1_EDGE_LOW_W<6> {
        GPIO1_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1_edge_high(&mut self) -> GPIO1_EDGE_HIGH_W<7> {
        GPIO1_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2_level_low(&mut self) -> GPIO2_LEVEL_LOW_W<8> {
        GPIO2_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2_level_high(&mut self) -> GPIO2_LEVEL_HIGH_W<9> {
        GPIO2_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2_edge_low(&mut self) -> GPIO2_EDGE_LOW_W<10> {
        GPIO2_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2_edge_high(&mut self) -> GPIO2_EDGE_HIGH_W<11> {
        GPIO2_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3_level_low(&mut self) -> GPIO3_LEVEL_LOW_W<12> {
        GPIO3_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3_level_high(&mut self) -> GPIO3_LEVEL_HIGH_W<13> {
        GPIO3_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3_edge_low(&mut self) -> GPIO3_EDGE_LOW_W<14> {
        GPIO3_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3_edge_high(&mut self) -> GPIO3_EDGE_HIGH_W<15> {
        GPIO3_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4_level_low(&mut self) -> GPIO4_LEVEL_LOW_W<16> {
        GPIO4_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4_level_high(&mut self) -> GPIO4_LEVEL_HIGH_W<17> {
        GPIO4_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4_edge_low(&mut self) -> GPIO4_EDGE_LOW_W<18> {
        GPIO4_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4_edge_high(&mut self) -> GPIO4_EDGE_HIGH_W<19> {
        GPIO4_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn gpio5_level_low(&mut self) -> GPIO5_LEVEL_LOW_W<20> {
        GPIO5_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn gpio5_level_high(&mut self) -> GPIO5_LEVEL_HIGH_W<21> {
        GPIO5_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn gpio5_edge_low(&mut self) -> GPIO5_EDGE_LOW_W<22> {
        GPIO5_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn gpio5_edge_high(&mut self) -> GPIO5_EDGE_HIGH_W<23> {
        GPIO5_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn gpio6_level_low(&mut self) -> GPIO6_LEVEL_LOW_W<24> {
        GPIO6_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn gpio6_level_high(&mut self) -> GPIO6_LEVEL_HIGH_W<25> {
        GPIO6_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn gpio6_edge_low(&mut self) -> GPIO6_EDGE_LOW_W<26> {
        GPIO6_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn gpio6_edge_high(&mut self) -> GPIO6_EDGE_HIGH_W<27> {
        GPIO6_EDGE_HIGH_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn gpio7_level_low(&mut self) -> GPIO7_LEVEL_LOW_W<28> {
        GPIO7_LEVEL_LOW_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn gpio7_level_high(&mut self) -> GPIO7_LEVEL_HIGH_W<29> {
        GPIO7_LEVEL_HIGH_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn gpio7_edge_low(&mut self) -> GPIO7_EDGE_LOW_W<30> {
        GPIO7_EDGE_LOW_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn gpio7_edge_high(&mut self) -> GPIO7_EDGE_HIGH_W<31> {
        GPIO7_EDGE_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable for dormant_wake  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dormant_wake_inte](index.html) module"]
pub struct DORMANT_WAKE_INTE_SPEC;
impl crate::RegisterSpec for DORMANT_WAKE_INTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dormant_wake_inte::R](R) reader structure"]
impl crate::Readable for DORMANT_WAKE_INTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dormant_wake_inte::W](W) writer structure"]
impl crate::Writable for DORMANT_WAKE_INTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DORMANT_WAKE_INTE%s to value 0"]
impl crate::Resettable for DORMANT_WAKE_INTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
