#[doc = "Register `GPIO_OE_XOR` writer"]
pub struct W(crate::W<GPIO_OE_XOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_OE_XOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPIO_OE_XOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_OE_XOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_OE_XOR` writer - Perform an atomic bitwise XOR on GPIO_OE, i.e. `GPIO_OE ^= wdata`"]
pub struct GPIO_OE_XOR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OE_XOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bitwise XOR on GPIO_OE, i.e. `GPIO_OE ^= wdata`"]
    #[inline(always)]
    pub fn gpio_oe_xor(&mut self) -> GPIO_OE_XOR_W {
        GPIO_OE_XOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO output enable XOR  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_oe_xor](index.html) module"]
pub struct GPIO_OE_XOR_SPEC;
impl crate::RegisterSpec for GPIO_OE_XOR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpio_oe_xor::W](W) writer structure"]
impl crate::Writable for GPIO_OE_XOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_OE_XOR to value 0"]
impl crate::Resettable for GPIO_OE_XOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
