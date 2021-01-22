#[doc = "Reader of register DBGPAUSE"]
pub type R = crate::R<u32, super::DBGPAUSE>;
#[doc = "Writer for register DBGPAUSE"]
pub type W = crate::W<u32, super::DBGPAUSE>;
#[doc = "Register DBGPAUSE `reset()`'s with value 0x07"]
impl crate::ResetValue for super::DBGPAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Reader of field `DBG1`"]
pub type DBG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG1`"]
pub struct DBG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG1_W<'a> {
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
#[doc = "Reader of field `DBG0`"]
pub type DBG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG0`"]
pub struct DBG0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG0_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Pause when processor 1 is in debug mode"]
    #[inline(always)]
    pub fn dbg1(&self) -> DBG1_R {
        DBG1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pause when processor 0 is in debug mode"]
    #[inline(always)]
    pub fn dbg0(&self) -> DBG0_R {
        DBG0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Pause when processor 1 is in debug mode"]
    #[inline(always)]
    pub fn dbg1(&mut self) -> DBG1_W {
        DBG1_W { w: self }
    }
    #[doc = "Bit 1 - Pause when processor 0 is in debug mode"]
    #[inline(always)]
    pub fn dbg0(&mut self) -> DBG0_W {
        DBG0_W { w: self }
    }
}
