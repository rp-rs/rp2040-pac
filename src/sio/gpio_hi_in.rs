#[doc = "Register `GPIO_HI_IN` reader"]
pub struct R(crate::R<GPIO_HI_IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_HI_IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_HI_IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_HI_IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO_HI_IN` reader - Input value on QSPI IO in order 0..5: SCLK, SSn, SD0, SD1, SD2, SD3"]
pub struct GPIO_HI_IN_R(crate::FieldReader<u8, u8>);
impl GPIO_HI_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_HI_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_HI_IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Input value on QSPI IO in order 0..5: SCLK, SSn, SD0, SD1, SD2, SD3"]
    #[inline(always)]
    pub fn gpio_hi_in(&self) -> GPIO_HI_IN_R {
        GPIO_HI_IN_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Input value for QSPI pins  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_hi_in](index.html) module"]
pub struct GPIO_HI_IN_SPEC;
impl crate::RegisterSpec for GPIO_HI_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_hi_in::R](R) reader structure"]
impl crate::Readable for GPIO_HI_IN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_HI_IN to value 0"]
impl crate::Resettable for GPIO_HI_IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
