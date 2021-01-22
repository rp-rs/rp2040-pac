#[doc = "Reader of register GPIO_OUT_SET"]
pub type R = crate::R<u32, super::GPIO_OUT_SET>;
#[doc = "Writer for register GPIO_OUT_SET"]
pub type W = crate::W<u32, super::GPIO_OUT_SET>;
#[doc = "Register GPIO_OUT_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_OUT_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_OUT_SET`"]
pub type GPIO_OUT_SET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_OUT_SET`"]
pub struct GPIO_OUT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OUT_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`"]
    #[inline(always)]
    pub fn gpio_out_set(&self) -> GPIO_OUT_SET_R {
        GPIO_OUT_SET_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`"]
    #[inline(always)]
    pub fn gpio_out_set(&mut self) -> GPIO_OUT_SET_W {
        GPIO_OUT_SET_W { w: self }
    }
}
