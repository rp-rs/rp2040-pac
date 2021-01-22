#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Reader of field `SM3`"]
pub type SM3_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM2`"]
pub type SM2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM1`"]
pub type SM1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM0`"]
pub type SM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM3_TXNFULL`"]
pub type SM3_TXNFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM2_TXNFULL`"]
pub type SM2_TXNFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM1_TXNFULL`"]
pub type SM1_TXNFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM0_TXNFULL`"]
pub type SM0_TXNFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM3_RXNEMPTY`"]
pub type SM3_RXNEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM2_RXNEMPTY`"]
pub type SM2_RXNEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM1_RXNEMPTY`"]
pub type SM1_RXNEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM0_RXNEMPTY`"]
pub type SM0_RXNEMPTY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sm3(&self) -> SM3_R {
        SM3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sm2(&self) -> SM2_R {
        SM2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sm1(&self) -> SM1_R {
        SM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sm0(&self) -> SM0_R {
        SM0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sm3_txnfull(&self) -> SM3_TXNFULL_R {
        SM3_TXNFULL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sm2_txnfull(&self) -> SM2_TXNFULL_R {
        SM2_TXNFULL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sm1_txnfull(&self) -> SM1_TXNFULL_R {
        SM1_TXNFULL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sm0_txnfull(&self) -> SM0_TXNFULL_R {
        SM0_TXNFULL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sm3_rxnempty(&self) -> SM3_RXNEMPTY_R {
        SM3_RXNEMPTY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sm2_rxnempty(&self) -> SM2_RXNEMPTY_R {
        SM2_RXNEMPTY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sm1_rxnempty(&self) -> SM1_RXNEMPTY_R {
        SM1_RXNEMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sm0_rxnempty(&self) -> SM0_RXNEMPTY_R {
        SM0_RXNEMPTY_R::new((self.bits & 0x01) != 0)
    }
}
