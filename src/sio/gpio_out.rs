#[doc = "Reader of register GPIO_OUT"]
pub type R = crate::R<u32, super::GPIO_OUT>;
#[doc = "Writer for register GPIO_OUT"]
pub type W = crate::W<u32, super::GPIO_OUT>;
#[doc = "Register GPIO_OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_OUT`"]
pub type GPIO_OUT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_OUT`"]
pub struct GPIO_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Set output level (1/0 -> high/low) for GPIO0...29.\\n Reading back gives the last value written, NOT the input value from the pins.\\n If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias),\\n the result is as though the write from core 0 took place first,\\n and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_out(&self) -> GPIO_OUT_R {
        GPIO_OUT_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Set output level (1/0 -> high/low) for GPIO0...29.\\n Reading back gives the last value written, NOT the input value from the pins.\\n If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias),\\n the result is as though the write from core 0 took place first,\\n and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_out(&mut self) -> GPIO_OUT_W {
        GPIO_OUT_W { w: self }
    }
}
