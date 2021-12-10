#[doc = "Register `FLEVEL` reader"]
pub struct R(crate::R<FLEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX3` reader - "]
pub struct RX3_R(crate::FieldReader<u8, u8>);
impl RX3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX3` reader - "]
pub struct TX3_R(crate::FieldReader<u8, u8>);
impl TX3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX2` reader - "]
pub struct RX2_R(crate::FieldReader<u8, u8>);
impl RX2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX2` reader - "]
pub struct TX2_R(crate::FieldReader<u8, u8>);
impl TX2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX1` reader - "]
pub struct RX1_R(crate::FieldReader<u8, u8>);
impl RX1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX1` reader - "]
pub struct TX1_R(crate::FieldReader<u8, u8>);
impl TX1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX0` reader - "]
pub struct RX0_R(crate::FieldReader<u8, u8>);
impl RX0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX0` reader - "]
pub struct TX0_R(crate::FieldReader<u8, u8>);
impl TX0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rx3(&self) -> RX3_R {
        RX3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn tx3(&self) -> TX3_R {
        TX3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn rx2(&self) -> RX2_R {
        RX2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn tx2(&self) -> TX2_R {
        TX2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn rx1(&self) -> RX1_R {
        RX1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn tx1(&self) -> TX1_R {
        TX1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn rx0(&self) -> RX0_R {
        RX0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tx0(&self) -> TX0_R {
        TX0_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "FIFO levels  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [flevel](index.html) module"]
pub struct FLEVEL_SPEC;
impl crate::RegisterSpec for FLEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flevel::R](R) reader structure"]
impl crate::Readable for FLEVEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLEVEL to value 0"]
impl crate::Resettable for FLEVEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
