#[doc = "Register `RANDOMBIT` reader"]
pub struct R(crate::R<RANDOMBIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RANDOMBIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RANDOMBIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RANDOMBIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RANDOMBIT` reader - "]
pub struct RANDOMBIT_R(crate::FieldReader<bool, bool>);
impl RANDOMBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RANDOMBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANDOMBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn randombit(&self) -> RANDOMBIT_R {
        RANDOMBIT_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [randombit](index.html) module"]
pub struct RANDOMBIT_SPEC;
impl crate::RegisterSpec for RANDOMBIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [randombit::R](R) reader structure"]
impl crate::Readable for RANDOMBIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RANDOMBIT to value 0x01"]
impl crate::Resettable for RANDOMBIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
