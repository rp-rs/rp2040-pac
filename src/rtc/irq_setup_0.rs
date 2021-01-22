#[doc = "Reader of register IRQ_SETUP_0"]
pub type R = crate::R<u32, super::IRQ_SETUP_0>;
#[doc = "Writer for register IRQ_SETUP_0"]
pub type W = crate::W<u32, super::IRQ_SETUP_0>;
#[doc = "Register IRQ_SETUP_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_SETUP_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MATCH_ACTIVE`"]
pub type MATCH_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MATCH_ENA`"]
pub type MATCH_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MATCH_ENA`"]
pub struct MATCH_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_ENA_W<'a> {
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
#[doc = "Reader of field `YEAR_ENA`"]
pub type YEAR_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `YEAR_ENA`"]
pub struct YEAR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `MONTH_ENA`"]
pub type MONTH_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONTH_ENA`"]
pub struct MONTH_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DAY_ENA`"]
pub type DAY_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAY_ENA`"]
pub struct DAY_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `YEAR`"]
pub type YEAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `YEAR`"]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 12)) | (((value as u32) & 0x0fff) << 12);
        self.w
    }
}
#[doc = "Reader of field `MONTH`"]
pub type MONTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MONTH`"]
pub struct MONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DAY`"]
pub type DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAY`"]
pub struct DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn match_active(&self) -> MATCH_ACTIVE_R {
        MATCH_ACTIVE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Global match enable. Don't change any other value while this one is enabled"]
    #[inline(always)]
    pub fn match_ena(&self) -> MATCH_ENA_R {
        MATCH_ENA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable year matching"]
    #[inline(always)]
    pub fn year_ena(&self) -> YEAR_ENA_R {
        YEAR_ENA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable month matching"]
    #[inline(always)]
    pub fn month_ena(&self) -> MONTH_ENA_R {
        MONTH_ENA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable day matching"]
    #[inline(always)]
    pub fn day_ena(&self) -> DAY_ENA_R {
        DAY_ENA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 28 - Global match enable. Don't change any other value while this one is enabled"]
    #[inline(always)]
    pub fn match_ena(&mut self) -> MATCH_ENA_W {
        MATCH_ENA_W { w: self }
    }
    #[doc = "Bit 26 - Enable year matching"]
    #[inline(always)]
    pub fn year_ena(&mut self) -> YEAR_ENA_W {
        YEAR_ENA_W { w: self }
    }
    #[doc = "Bit 25 - Enable month matching"]
    #[inline(always)]
    pub fn month_ena(&mut self) -> MONTH_ENA_W {
        MONTH_ENA_W { w: self }
    }
    #[doc = "Bit 24 - Enable day matching"]
    #[inline(always)]
    pub fn day_ena(&mut self) -> DAY_ENA_W {
        DAY_ENA_W { w: self }
    }
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    pub fn month(&mut self) -> MONTH_W {
        MONTH_W { w: self }
    }
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W {
        DAY_W { w: self }
    }
}
