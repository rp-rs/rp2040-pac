#[doc = "Register `FLEVEL` reader"]
pub type R = crate::R<FLEVEL_SPEC>;
#[doc = "Field `TX0` reader - "]
pub type TX0_R = crate::FieldReader;
#[doc = "Field `RX0` reader - "]
pub type RX0_R = crate::FieldReader;
#[doc = "Field `TX1` reader - "]
pub type TX1_R = crate::FieldReader;
#[doc = "Field `RX1` reader - "]
pub type RX1_R = crate::FieldReader;
#[doc = "Field `TX2` reader - "]
pub type TX2_R = crate::FieldReader;
#[doc = "Field `RX2` reader - "]
pub type RX2_R = crate::FieldReader;
#[doc = "Field `TX3` reader - "]
pub type TX3_R = crate::FieldReader;
#[doc = "Field `RX3` reader - "]
pub type RX3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tx0(&self) -> TX0_R {
        TX0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn rx0(&self) -> RX0_R {
        RX0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn tx1(&self) -> TX1_R {
        TX1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn rx1(&self) -> RX1_R {
        RX1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn tx2(&self) -> TX2_R {
        TX2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn rx2(&self) -> RX2_R {
        RX2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn tx3(&self) -> TX3_R {
        TX3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rx3(&self) -> RX3_R {
        RX3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "FIFO levels  

You can [`read`](crate::Reg::read) this register and get [`flevel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLEVEL_SPEC;
impl crate::RegisterSpec for FLEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flevel::R`](R) reader structure"]
impl crate::Readable for FLEVEL_SPEC {}
#[doc = "`reset()` method sets FLEVEL to value 0"]
impl crate::Resettable for FLEVEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
