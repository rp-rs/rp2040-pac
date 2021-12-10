#[doc = "Register `PROC1_INTS%s` reader"]
pub struct R(crate::R<PROC1_INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC1_INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC1_INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC1_INTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO7_EDGE_HIGH` reader - "]
pub struct GPIO7_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO7_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO7_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO7_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO7_EDGE_LOW` reader - "]
pub struct GPIO7_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO7_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO7_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO7_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO7_LEVEL_HIGH` reader - "]
pub struct GPIO7_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO7_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO7_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO7_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO7_LEVEL_LOW` reader - "]
pub struct GPIO7_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO7_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO7_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO7_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO6_EDGE_HIGH` reader - "]
pub struct GPIO6_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO6_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO6_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO6_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO6_EDGE_LOW` reader - "]
pub struct GPIO6_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO6_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO6_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO6_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO6_LEVEL_HIGH` reader - "]
pub struct GPIO6_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO6_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO6_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO6_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO6_LEVEL_LOW` reader - "]
pub struct GPIO6_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO6_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO6_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO6_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5_EDGE_HIGH` reader - "]
pub struct GPIO5_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO5_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO5_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5_EDGE_LOW` reader - "]
pub struct GPIO5_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO5_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO5_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5_LEVEL_HIGH` reader - "]
pub struct GPIO5_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO5_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO5_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5_LEVEL_LOW` reader - "]
pub struct GPIO5_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO5_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO5_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4_EDGE_HIGH` reader - "]
pub struct GPIO4_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO4_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO4_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4_EDGE_LOW` reader - "]
pub struct GPIO4_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO4_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO4_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4_LEVEL_HIGH` reader - "]
pub struct GPIO4_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO4_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO4_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4_LEVEL_LOW` reader - "]
pub struct GPIO4_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO4_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO4_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3_EDGE_HIGH` reader - "]
pub struct GPIO3_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO3_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO3_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3_EDGE_LOW` reader - "]
pub struct GPIO3_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO3_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO3_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3_LEVEL_HIGH` reader - "]
pub struct GPIO3_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO3_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO3_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3_LEVEL_LOW` reader - "]
pub struct GPIO3_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO3_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO3_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2_EDGE_HIGH` reader - "]
pub struct GPIO2_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO2_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO2_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2_EDGE_LOW` reader - "]
pub struct GPIO2_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO2_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO2_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2_LEVEL_HIGH` reader - "]
pub struct GPIO2_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO2_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO2_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2_LEVEL_LOW` reader - "]
pub struct GPIO2_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO2_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO2_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1_EDGE_HIGH` reader - "]
pub struct GPIO1_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO1_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO1_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1_EDGE_LOW` reader - "]
pub struct GPIO1_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO1_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO1_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1_LEVEL_HIGH` reader - "]
pub struct GPIO1_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO1_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO1_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1_LEVEL_LOW` reader - "]
pub struct GPIO1_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO1_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO1_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0_EDGE_HIGH` reader - "]
pub struct GPIO0_EDGE_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO0_EDGE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_EDGE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO0_EDGE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0_EDGE_LOW` reader - "]
pub struct GPIO0_EDGE_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO0_EDGE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_EDGE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO0_EDGE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0_LEVEL_HIGH` reader - "]
pub struct GPIO0_LEVEL_HIGH_R(crate::FieldReader<bool, bool>);
impl GPIO0_LEVEL_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_LEVEL_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO0_LEVEL_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0_LEVEL_LOW` reader - "]
pub struct GPIO0_LEVEL_LOW_R(crate::FieldReader<bool, bool>);
impl GPIO0_LEVEL_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_LEVEL_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO0_LEVEL_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio7_edge_high(&self) -> GPIO7_EDGE_HIGH_R {
        GPIO7_EDGE_HIGH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio7_edge_low(&self) -> GPIO7_EDGE_LOW_R {
        GPIO7_EDGE_LOW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio7_level_high(&self) -> GPIO7_LEVEL_HIGH_R {
        GPIO7_LEVEL_HIGH_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio7_level_low(&self) -> GPIO7_LEVEL_LOW_R {
        GPIO7_LEVEL_LOW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio6_edge_high(&self) -> GPIO6_EDGE_HIGH_R {
        GPIO6_EDGE_HIGH_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio6_edge_low(&self) -> GPIO6_EDGE_LOW_R {
        GPIO6_EDGE_LOW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpio6_level_high(&self) -> GPIO6_LEVEL_HIGH_R {
        GPIO6_LEVEL_HIGH_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn gpio6_level_low(&self) -> GPIO6_LEVEL_LOW_R {
        GPIO6_LEVEL_LOW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio5_edge_high(&self) -> GPIO5_EDGE_HIGH_R {
        GPIO5_EDGE_HIGH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio5_edge_low(&self) -> GPIO5_EDGE_LOW_R {
        GPIO5_EDGE_LOW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio5_level_high(&self) -> GPIO5_LEVEL_HIGH_R {
        GPIO5_LEVEL_HIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio5_level_low(&self) -> GPIO5_LEVEL_LOW_R {
        GPIO5_LEVEL_LOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio4_edge_high(&self) -> GPIO4_EDGE_HIGH_R {
        GPIO4_EDGE_HIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio4_edge_low(&self) -> GPIO4_EDGE_LOW_R {
        GPIO4_EDGE_LOW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio4_level_high(&self) -> GPIO4_LEVEL_HIGH_R {
        GPIO4_LEVEL_HIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio4_level_low(&self) -> GPIO4_LEVEL_LOW_R {
        GPIO4_LEVEL_LOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio3_edge_high(&self) -> GPIO3_EDGE_HIGH_R {
        GPIO3_EDGE_HIGH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio3_edge_low(&self) -> GPIO3_EDGE_LOW_R {
        GPIO3_EDGE_LOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio3_level_high(&self) -> GPIO3_LEVEL_HIGH_R {
        GPIO3_LEVEL_HIGH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio3_level_low(&self) -> GPIO3_LEVEL_LOW_R {
        GPIO3_LEVEL_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio2_edge_high(&self) -> GPIO2_EDGE_HIGH_R {
        GPIO2_EDGE_HIGH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio2_edge_low(&self) -> GPIO2_EDGE_LOW_R {
        GPIO2_EDGE_LOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio2_level_high(&self) -> GPIO2_LEVEL_HIGH_R {
        GPIO2_LEVEL_HIGH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio2_level_low(&self) -> GPIO2_LEVEL_LOW_R {
        GPIO2_LEVEL_LOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio1_edge_high(&self) -> GPIO1_EDGE_HIGH_R {
        GPIO1_EDGE_HIGH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio1_edge_low(&self) -> GPIO1_EDGE_LOW_R {
        GPIO1_EDGE_LOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio1_level_high(&self) -> GPIO1_LEVEL_HIGH_R {
        GPIO1_LEVEL_HIGH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio1_level_low(&self) -> GPIO1_LEVEL_LOW_R {
        GPIO1_LEVEL_LOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio0_edge_high(&self) -> GPIO0_EDGE_HIGH_R {
        GPIO0_EDGE_HIGH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio0_edge_low(&self) -> GPIO0_EDGE_LOW_R {
        GPIO0_EDGE_LOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio0_level_high(&self) -> GPIO0_LEVEL_HIGH_R {
        GPIO0_LEVEL_HIGH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio0_level_low(&self) -> GPIO0_LEVEL_LOW_R {
        GPIO0_LEVEL_LOW_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt status after masking & forcing for proc1  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [proc1_ints](index.html) module"]
pub struct PROC1_INTS_SPEC;
impl crate::RegisterSpec for PROC1_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc1_ints::R](R) reader structure"]
impl crate::Readable for PROC1_INTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PROC1_INTS%s to value 0"]
impl crate::Resettable for PROC1_INTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
