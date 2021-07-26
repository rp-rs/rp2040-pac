#[doc = "Register `GPIO_HI_OUT_XOR` reader"]
pub struct R(crate::R<GPIO_HI_OUT_XOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_HI_OUT_XOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_HI_OUT_XOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_HI_OUT_XOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_HI_OUT_XOR` writer"]
pub struct W(crate::W<GPIO_HI_OUT_XOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_HI_OUT_XOR_SPEC>;
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
impl From<crate::W<GPIO_HI_OUT_XOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_HI_OUT_XOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_HI_OUT_XOR` reader - Perform an atomic bitwise XOR on GPIO_HI_OUT, i.e. `GPIO_HI_OUT ^= wdata`"]
pub struct GPIO_HI_OUT_XOR_R(crate::FieldReader<u8, u8>);
impl GPIO_HI_OUT_XOR_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_HI_OUT_XOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_HI_OUT_XOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_HI_OUT_XOR` writer - Perform an atomic bitwise XOR on GPIO_HI_OUT, i.e. `GPIO_HI_OUT ^= wdata`"]
pub struct GPIO_HI_OUT_XOR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_HI_OUT_XOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Perform an atomic bitwise XOR on GPIO_HI_OUT, i.e. `GPIO_HI_OUT ^= wdata`"]
    #[inline(always)]
    pub fn gpio_hi_out_xor(&self) -> GPIO_HI_OUT_XOR_R {
        GPIO_HI_OUT_XOR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Perform an atomic bitwise XOR on GPIO_HI_OUT, i.e. `GPIO_HI_OUT ^= wdata`"]
    #[inline(always)]
    pub fn gpio_hi_out_xor(&mut self) -> GPIO_HI_OUT_XOR_W {
        GPIO_HI_OUT_XOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI output value XOR  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_hi_out_xor](index.html) module"]
pub struct GPIO_HI_OUT_XOR_SPEC;
impl crate::RegisterSpec for GPIO_HI_OUT_XOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_hi_out_xor::R](R) reader structure"]
impl crate::Readable for GPIO_HI_OUT_XOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_hi_out_xor::W](W) writer structure"]
impl crate::Writable for GPIO_HI_OUT_XOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_HI_OUT_XOR to value 0"]
impl crate::Resettable for GPIO_HI_OUT_XOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
