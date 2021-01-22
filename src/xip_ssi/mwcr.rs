#[doc = "Reader of register MWCR"]
pub type R = crate::R<u32, super::MWCR>;
#[doc = "Writer for register MWCR"]
pub type W = crate::W<u32, super::MWCR>;
#[doc = "Register MWCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MWCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MHS`"]
pub type MHS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MHS`"]
pub struct MHS_W<'a> {
    w: &'a mut W,
}
impl<'a> MHS_W<'a> {
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
#[doc = "Reader of field `MDD`"]
pub type MDD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDD`"]
pub struct MDD_W<'a> {
    w: &'a mut W,
}
impl<'a> MDD_W<'a> {
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
#[doc = "Reader of field `MWMOD`"]
pub type MWMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MWMOD`"]
pub struct MWMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MWMOD_W<'a> {
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
    #[doc = "Bit 2 - Microwire handshaking"]
    #[inline(always)]
    pub fn mhs(&self) -> MHS_R {
        MHS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Microwire control"]
    #[inline(always)]
    pub fn mdd(&self) -> MDD_R {
        MDD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Microwire transfer mode"]
    #[inline(always)]
    pub fn mwmod(&self) -> MWMOD_R {
        MWMOD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Microwire handshaking"]
    #[inline(always)]
    pub fn mhs(&mut self) -> MHS_W {
        MHS_W { w: self }
    }
    #[doc = "Bit 1 - Microwire control"]
    #[inline(always)]
    pub fn mdd(&mut self) -> MDD_W {
        MDD_W { w: self }
    }
    #[doc = "Bit 0 - Microwire transfer mode"]
    #[inline(always)]
    pub fn mwmod(&mut self) -> MWMOD_W {
        MWMOD_W { w: self }
    }
}
