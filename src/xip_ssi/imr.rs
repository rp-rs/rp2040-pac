#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Writer for register IMR"]
pub type W = crate::W<u32, super::IMR>;
#[doc = "Register IMR `reset()`'s with value 0"]
impl crate::ResetValue for super::IMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSTIM`"]
pub type MSTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSTIM`"]
pub struct MSTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTIM_W<'a> {
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
#[doc = "Reader of field `RXFIM`"]
pub type RXFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIM`"]
pub struct RXFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIM_W<'a> {
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
#[doc = "Reader of field `RXOIM`"]
pub type RXOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOIM`"]
pub struct RXOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOIM_W<'a> {
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
#[doc = "Reader of field `RXUIM`"]
pub type RXUIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUIM`"]
pub struct RXUIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUIM_W<'a> {
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
#[doc = "Reader of field `TXOIM`"]
pub type TXOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOIM`"]
pub struct TXOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOIM_W<'a> {
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
#[doc = "Reader of field `TXEIM`"]
pub type TXEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXEIM`"]
pub struct TXEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIM_W<'a> {
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
    #[doc = "Bit 5 - Multi-master contention interrupt mask"]
    #[inline(always)]
    pub fn mstim(&self) -> MSTIM_R {
        MSTIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt mask"]
    #[inline(always)]
    pub fn rxfim(&self) -> RXFIM_R {
        RXFIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn rxoim(&self) -> RXOIM_R {
        RXOIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn rxuim(&self) -> RXUIM_R {
        RXUIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn txoim(&self) -> TXOIM_R {
        TXOIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit FIFO empty interrupt mask"]
    #[inline(always)]
    pub fn txeim(&self) -> TXEIM_R {
        TXEIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Multi-master contention interrupt mask"]
    #[inline(always)]
    pub fn mstim(&mut self) -> MSTIM_W {
        MSTIM_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt mask"]
    #[inline(always)]
    pub fn rxfim(&mut self) -> RXFIM_W {
        RXFIM_W { w: self }
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn rxoim(&mut self) -> RXOIM_W {
        RXOIM_W { w: self }
    }
    #[doc = "Bit 2 - Receive FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn rxuim(&mut self) -> RXUIM_W {
        RXUIM_W { w: self }
    }
    #[doc = "Bit 1 - Transmit FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn txoim(&mut self) -> TXOIM_W {
        TXOIM_W { w: self }
    }
    #[doc = "Bit 0 - Transmit FIFO empty interrupt mask"]
    #[inline(always)]
    pub fn txeim(&mut self) -> TXEIM_W {
        TXEIM_W { w: self }
    }
}
