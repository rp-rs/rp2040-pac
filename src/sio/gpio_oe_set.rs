#[doc = "Reader of register GPIO_OE_SET"]
pub type R = crate::R<u32, super::GPIO_OE_SET>;
#[doc = "Writer for register GPIO_OE_SET"]
pub type W = crate::W<u32, super::GPIO_OE_SET>;
#[doc = "Register GPIO_OE_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_OE_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_OE_SET`"]
pub type GPIO_OE_SET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_OE_SET`"]
pub struct GPIO_OE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OE_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Perform an atomic bit-set on GPIO_OE, i.e. `GPIO_OE |= wdata`"]
    #[inline(always)]
    pub fn gpio_oe_set(&self) -> GPIO_OE_SET_R {
        GPIO_OE_SET_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bit-set on GPIO_OE, i.e. `GPIO_OE |= wdata`"]
    #[inline(always)]
    pub fn gpio_oe_set(&mut self) -> GPIO_OE_SET_W {
        GPIO_OE_SET_W { w: self }
    }
}
