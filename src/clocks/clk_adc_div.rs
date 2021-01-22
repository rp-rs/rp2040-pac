#[doc = "Reader of register CLK_ADC_DIV"]
pub type R = crate::R<u32, super::CLK_ADC_DIV>;
#[doc = "Writer for register CLK_ADC_DIV"]
pub type W = crate::W<u32, super::CLK_ADC_DIV>;
#[doc = "Register CLK_ADC_DIV `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::CLK_ADC_DIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT`"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - Integer component of the divisor, 0 -> divide by 2^16"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Integer component of the divisor, 0 -> divide by 2^16"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
}
