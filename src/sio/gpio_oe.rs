#[doc = "Reader of register GPIO_OE"]
pub type R = crate::R<u32, super::GPIO_OE>;
#[doc = "Writer for register GPIO_OE"]
pub type W = crate::W<u32, super::GPIO_OE>;
#[doc = "Register GPIO_OE `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_OE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_OE`"]
pub type GPIO_OE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_OE`"]
pub struct GPIO_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Set output enable (1/0 -> output/input) for GPIO0...29.\\n Reading back gives the last value written.\\n If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias),\\n the result is as though the write from core 0 took place first,\\n and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_oe(&self) -> GPIO_OE_R {
        GPIO_OE_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Set output enable (1/0 -> output/input) for GPIO0...29.\\n Reading back gives the last value written.\\n If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias),\\n the result is as though the write from core 0 took place first,\\n and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_oe(&mut self) -> GPIO_OE_W {
        GPIO_OE_W { w: self }
    }
}
