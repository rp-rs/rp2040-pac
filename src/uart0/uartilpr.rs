#[doc = "Register `UARTILPR` reader"]
pub struct R(crate::R<UARTILPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTILPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTILPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTILPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTILPR` writer"]
pub struct W(crate::W<UARTILPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTILPR_SPEC>;
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
impl From<crate::W<UARTILPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTILPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILPDVSR` reader - 8-bit low-power divisor value. These bits are cleared to 0 at reset."]
pub struct ILPDVSR_R(crate::FieldReader<u8, u8>);
impl ILPDVSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ILPDVSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ILPDVSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILPDVSR` writer - 8-bit low-power divisor value. These bits are cleared to 0 at reset."]
pub struct ILPDVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> ILPDVSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value. These bits are cleared to 0 at reset."]
    #[inline(always)]
    pub fn ilpdvsr(&self) -> ILPDVSR_R {
        ILPDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value. These bits are cleared to 0 at reset."]
    #[inline(always)]
    pub fn ilpdvsr(&mut self) -> ILPDVSR_W {
        ILPDVSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IrDA Low-Power Counter Register, UARTILPR  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartilpr](index.html) module"]
pub struct UARTILPR_SPEC;
impl crate::RegisterSpec for UARTILPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartilpr::R](R) reader structure"]
impl crate::Readable for UARTILPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartilpr::W](W) writer structure"]
impl crate::Writable for UARTILPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UARTILPR to value 0"]
impl crate::Resettable for UARTILPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
