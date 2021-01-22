#[doc = "Reader of register SHCSR"]
pub type R = crate::R<u32, super::SHCSR>;
#[doc = "Writer for register SHCSR"]
pub type W = crate::W<u32, super::SHCSR>;
#[doc = "Register SHCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SHCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SVCALLPENDED`"]
pub type SVCALLPENDED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVCALLPENDED`"]
pub struct SVCALLPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCALLPENDED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Reads as 1 if SVCall is Pending. Write 1 to set pending SVCall, write 0 to clear pending SVCall."]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Reads as 1 if SVCall is Pending. Write 1 to set pending SVCall, write 0 to clear pending SVCall."]
    #[inline(always)]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W {
        SVCALLPENDED_W { w: self }
    }
}
