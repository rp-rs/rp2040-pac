#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SM0_RXNEMPTY` reader - "]
pub type SM0_RXNEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `SM1_RXNEMPTY` reader - "]
pub type SM1_RXNEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `SM2_RXNEMPTY` reader - "]
pub type SM2_RXNEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `SM3_RXNEMPTY` reader - "]
pub type SM3_RXNEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `SM0_TXNFULL` reader - "]
pub type SM0_TXNFULL_R = crate::BitReader<bool>;
#[doc = "Field `SM1_TXNFULL` reader - "]
pub type SM1_TXNFULL_R = crate::BitReader<bool>;
#[doc = "Field `SM2_TXNFULL` reader - "]
pub type SM2_TXNFULL_R = crate::BitReader<bool>;
#[doc = "Field `SM3_TXNFULL` reader - "]
pub type SM3_TXNFULL_R = crate::BitReader<bool>;
#[doc = "Field `SM0` reader - "]
pub type SM0_R = crate::BitReader<bool>;
#[doc = "Field `SM1` reader - "]
pub type SM1_R = crate::BitReader<bool>;
#[doc = "Field `SM2` reader - "]
pub type SM2_R = crate::BitReader<bool>;
#[doc = "Field `SM3` reader - "]
pub type SM3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sm0_rxnempty(&self) -> SM0_RXNEMPTY_R {
        SM0_RXNEMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sm1_rxnempty(&self) -> SM1_RXNEMPTY_R {
        SM1_RXNEMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sm2_rxnempty(&self) -> SM2_RXNEMPTY_R {
        SM2_RXNEMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sm3_rxnempty(&self) -> SM3_RXNEMPTY_R {
        SM3_RXNEMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sm0_txnfull(&self) -> SM0_TXNFULL_R {
        SM0_TXNFULL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sm1_txnfull(&self) -> SM1_TXNFULL_R {
        SM1_TXNFULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sm2_txnfull(&self) -> SM2_TXNFULL_R {
        SM2_TXNFULL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sm3_txnfull(&self) -> SM3_TXNFULL_R {
        SM3_TXNFULL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sm0(&self) -> SM0_R {
        SM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sm1(&self) -> SM1_R {
        SM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sm2(&self) -> SM2_R {
        SM2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sm3(&self) -> SM3_R {
        SM3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Raw Interrupts  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
