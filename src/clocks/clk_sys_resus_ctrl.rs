#[doc = "Reader of register CLK_SYS_RESUS_CTRL"]
pub type R = crate::R<u32, super::CLK_SYS_RESUS_CTRL>;
#[doc = "Writer for register CLK_SYS_RESUS_CTRL"]
pub type W = crate::W<u32, super::CLK_SYS_RESUS_CTRL>;
#[doc = "Register CLK_SYS_RESUS_CTRL `reset()`'s with value 0xff"]
impl crate::ResetValue for super::CLK_SYS_RESUS_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `CLEAR`"]
pub type CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLEAR`"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `FRCE`"]
pub type FRCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRCE`"]
pub struct FRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMEOUT`"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - For clearing the resus after the fault that triggered it has been corrected"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Force a resus, for test purposes only"]
    #[inline(always)]
    pub fn frce(&self) -> FRCE_R {
        FRCE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable resus"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - This is expressed as a number of clk_ref cycles\\n and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - For clearing the resus after the fault that triggered it has been corrected"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
    #[doc = "Bit 12 - Force a resus, for test purposes only"]
    #[inline(always)]
    pub fn frce(&mut self) -> FRCE_W {
        FRCE_W { w: self }
    }
    #[doc = "Bit 8 - Enable resus"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 0:7 - This is expressed as a number of clk_ref cycles\\n and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
}
