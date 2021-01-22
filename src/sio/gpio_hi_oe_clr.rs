#[doc = "Reader of register GPIO_HI_OE_CLR"]
pub type R = crate::R<u32, super::GPIO_HI_OE_CLR>;
#[doc = "Writer for register GPIO_HI_OE_CLR"]
pub type W = crate::W<u32, super::GPIO_HI_OE_CLR>;
#[doc = "Register GPIO_HI_OE_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_HI_OE_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_HI_OE_CLR`"]
pub type GPIO_HI_OE_CLR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_HI_OE_CLR`"]
pub struct GPIO_HI_OE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_HI_OE_CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Perform an atomic bit-clear on GPIO_HI_OE, i.e. `GPIO_HI_OE &= ~wdata`"]
    #[inline(always)]
    pub fn gpio_hi_oe_clr(&self) -> GPIO_HI_OE_CLR_R {
        GPIO_HI_OE_CLR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Perform an atomic bit-clear on GPIO_HI_OE, i.e. `GPIO_HI_OE &= ~wdata`"]
    #[inline(always)]
    pub fn gpio_hi_oe_clr(&mut self) -> GPIO_HI_OE_CLR_W {
        GPIO_HI_OE_CLR_W { w: self }
    }
}
