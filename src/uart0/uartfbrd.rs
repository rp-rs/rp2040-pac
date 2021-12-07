#[doc = "Register `UARTFBRD` reader"]
pub struct R(crate::R<UARTFBRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTFBRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTFBRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTFBRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTFBRD` writer"]
pub struct W(crate::W<UARTFBRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTFBRD_SPEC>;
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
impl From<crate::W<UARTFBRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTFBRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUD_DIVFRAC` reader - The fractional baud rate divisor. These bits are cleared to 0 on reset."]
pub struct BAUD_DIVFRAC_R(crate::FieldReader<u8, u8>);
impl BAUD_DIVFRAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BAUD_DIVFRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAUD_DIVFRAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAUD_DIVFRAC` writer - The fractional baud rate divisor. These bits are cleared to 0 on reset."]
pub struct BAUD_DIVFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD_DIVFRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - The fractional baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn baud_divfrac(&self) -> BAUD_DIVFRAC_R {
        BAUD_DIVFRAC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The fractional baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn baud_divfrac(&mut self) -> BAUD_DIVFRAC_W {
        BAUD_DIVFRAC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Baud Rate Register, UARTFBRD  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartfbrd](index.html) module"]
pub struct UARTFBRD_SPEC;
impl crate::RegisterSpec for UARTFBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartfbrd::R](R) reader structure"]
impl crate::Readable for UARTFBRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartfbrd::W](W) writer structure"]
impl crate::Writable for UARTFBRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UARTFBRD to value 0"]
impl crate::Resettable for UARTFBRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
