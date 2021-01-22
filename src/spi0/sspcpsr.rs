#[doc = "Reader of register SSPCPSR"]
pub type R = crate::R<u32, super::SSPCPSR>;
#[doc = "Writer for register SSPCPSR"]
pub type W = crate::W<u32, super::SSPCPSR>;
#[doc = "Register SSPCPSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SSPCPSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPSDVSR`"]
pub type CPSDVSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPSDVSR`"]
pub struct CPSDVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSDVSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CPSDVSR_R {
        CPSDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    #[inline(always)]
    pub fn cpsdvsr(&mut self) -> CPSDVSR_W {
        CPSDVSR_W { w: self }
    }
}
