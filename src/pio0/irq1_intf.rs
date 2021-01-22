#[doc = "Reader of register IRQ1_INTF"]
pub type R = crate::R<u32, super::IRQ1_INTF>;
#[doc = "Writer for register IRQ1_INTF"]
pub type W = crate::W<u32, super::IRQ1_INTF>;
#[doc = "Register IRQ1_INTF `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ1_INTF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SM3`"]
pub type SM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM3`"]
pub struct SM3_W<'a> {
    w: &'a mut W,
}
impl<'a> SM3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SM2`"]
pub type SM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM2`"]
pub struct SM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SM2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SM1`"]
pub type SM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM1`"]
pub struct SM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SM0`"]
pub type SM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM0`"]
pub struct SM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SM3_TXNFULL`"]
pub type SM3_TXNFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM3_TXNFULL`"]
pub struct SM3_TXNFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SM3_TXNFULL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SM2_TXNFULL`"]
pub type SM2_TXNFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM2_TXNFULL`"]
pub struct SM2_TXNFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SM2_TXNFULL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SM1_TXNFULL`"]
pub type SM1_TXNFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM1_TXNFULL`"]
pub struct SM1_TXNFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_TXNFULL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SM0_TXNFULL`"]
pub type SM0_TXNFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM0_TXNFULL`"]
pub struct SM0_TXNFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_TXNFULL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SM3_RXNEMPTY`"]
pub type SM3_RXNEMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM3_RXNEMPTY`"]
pub struct SM3_RXNEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SM3_RXNEMPTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SM2_RXNEMPTY`"]
pub type SM2_RXNEMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM2_RXNEMPTY`"]
pub struct SM2_RXNEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SM2_RXNEMPTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SM1_RXNEMPTY`"]
pub type SM1_RXNEMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM1_RXNEMPTY`"]
pub struct SM1_RXNEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_RXNEMPTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SM0_RXNEMPTY`"]
pub type SM0_RXNEMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM0_RXNEMPTY`"]
pub struct SM0_RXNEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_RXNEMPTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
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
impl W {
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sm3(&mut self) -> SM3_W {
        SM3_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sm2(&mut self) -> SM2_W {
        SM2_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sm1(&mut self) -> SM1_W {
        SM1_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sm0(&mut self) -> SM0_W {
        SM0_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sm3_txnfull(&mut self) -> SM3_TXNFULL_W {
        SM3_TXNFULL_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sm2_txnfull(&mut self) -> SM2_TXNFULL_W {
        SM2_TXNFULL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sm1_txnfull(&mut self) -> SM1_TXNFULL_W {
        SM1_TXNFULL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sm0_txnfull(&mut self) -> SM0_TXNFULL_W {
        SM0_TXNFULL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sm3_rxnempty(&mut self) -> SM3_RXNEMPTY_W {
        SM3_RXNEMPTY_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sm2_rxnempty(&mut self) -> SM2_RXNEMPTY_W {
        SM2_RXNEMPTY_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sm1_rxnempty(&mut self) -> SM1_RXNEMPTY_W {
        SM1_RXNEMPTY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sm0_rxnempty(&mut self) -> SM0_RXNEMPTY_W {
        SM0_RXNEMPTY_W { w: self }
    }
}
