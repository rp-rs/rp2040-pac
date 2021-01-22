#[doc = "Reader of register IRQ_SETUP_1"]
pub type R = crate::R<u32, super::IRQ_SETUP_1>;
#[doc = "Writer for register IRQ_SETUP_1"]
pub type W = crate::W<u32, super::IRQ_SETUP_1>;
#[doc = "Register IRQ_SETUP_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_SETUP_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOTW_ENA`"]
pub type DOTW_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOTW_ENA`"]
pub struct DOTW_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DOTW_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `HOUR_ENA`"]
pub type HOUR_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOUR_ENA`"]
pub struct HOUR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `MIN_ENA`"]
pub type MIN_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIN_ENA`"]
pub struct MIN_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SEC_ENA`"]
pub type SEC_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC_ENA`"]
pub struct SEC_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DOTW`"]
pub type DOTW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DOTW`"]
pub struct DOTW_W<'a> {
    w: &'a mut W,
}
impl<'a> DOTW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOUR`"]
pub type HOUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOUR`"]
pub struct HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MIN`"]
pub type MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN`"]
pub struct MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEC`"]
pub struct SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Enable day of the week matching"]
    #[inline(always)]
    pub fn dotw_ena(&self) -> DOTW_ENA_R {
        DOTW_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable hour matching"]
    #[inline(always)]
    pub fn hour_ena(&self) -> HOUR_ENA_R {
        HOUR_ENA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable minute matching"]
    #[inline(always)]
    pub fn min_ena(&self) -> MIN_ENA_R {
        MIN_ENA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable second matching"]
    #[inline(always)]
    pub fn sec_ena(&self) -> SEC_ENA_R {
        SEC_ENA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Day of the week"]
    #[inline(always)]
    pub fn dotw(&self) -> DOTW_R {
        DOTW_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Enable day of the week matching"]
    #[inline(always)]
    pub fn dotw_ena(&mut self) -> DOTW_ENA_W {
        DOTW_ENA_W { w: self }
    }
    #[doc = "Bit 30 - Enable hour matching"]
    #[inline(always)]
    pub fn hour_ena(&mut self) -> HOUR_ENA_W {
        HOUR_ENA_W { w: self }
    }
    #[doc = "Bit 29 - Enable minute matching"]
    #[inline(always)]
    pub fn min_ena(&mut self) -> MIN_ENA_W {
        MIN_ENA_W { w: self }
    }
    #[doc = "Bit 28 - Enable second matching"]
    #[inline(always)]
    pub fn sec_ena(&mut self) -> SEC_ENA_W {
        SEC_ENA_W { w: self }
    }
    #[doc = "Bits 24:26 - Day of the week"]
    #[inline(always)]
    pub fn dotw(&mut self) -> DOTW_W {
        DOTW_W { w: self }
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W {
        HOUR_W { w: self }
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W {
        MIN_W { w: self }
    }
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W {
        SEC_W { w: self }
    }
}
