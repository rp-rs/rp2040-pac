#[doc = "Reader of register GPIO_HI_OE"]
pub type R = crate::R<u32, super::GPIO_HI_OE>;
#[doc = "Writer for register GPIO_HI_OE"]
pub type W = crate::W<u32, super::GPIO_HI_OE>;
#[doc = "Register GPIO_HI_OE `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_HI_OE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_HI_OE`"]
pub type GPIO_HI_OE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_HI_OE`"]
pub struct GPIO_HI_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_HI_OE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Set output enable (1/0 -> output/input) for QSPI IO0...5.\\n Reading back gives the last value written.\\n If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias),\\n the result is as though the write from core 0 took place first,\\n and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_hi_oe(&self) -> GPIO_HI_OE_R {
        GPIO_HI_OE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set output enable (1/0 -> output/input) for QSPI IO0...5.\\n Reading back gives the last value written.\\n If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias),\\n the result is as though the write from core 0 took place first,\\n and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_hi_oe(&mut self) -> GPIO_HI_OE_W {
        GPIO_HI_OE_W { w: self }
    }
}
