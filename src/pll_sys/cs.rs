#[doc = "Reader of register CS"]
pub type R = crate::R<u32, super::CS>;
#[doc = "Writer for register CS"]
pub type W = crate::W<u32, super::CS>;
#[doc = "Register CS `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Reader of field `REFDIV`"]
pub type REFDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFDIV`"]
pub struct REFDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> REFDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - PLL is locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - Divides the PLL input reference clock.\\n Behaviour is undefined for div=0.\\n PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
    #[inline(always)]
    pub fn refdiv(&self) -> REFDIV_R {
        REFDIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bits 0:5 - Divides the PLL input reference clock.\\n Behaviour is undefined for div=0.\\n PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
    #[inline(always)]
    pub fn refdiv(&mut self) -> REFDIV_W {
        REFDIV_W { w: self }
    }
}
