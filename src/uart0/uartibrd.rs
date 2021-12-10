#[doc = "Register `UARTIBRD` reader"]
pub struct R(crate::R<UARTIBRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTIBRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTIBRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTIBRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTIBRD` writer"]
pub struct W(crate::W<UARTIBRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTIBRD_SPEC>;
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
impl From<crate::W<UARTIBRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTIBRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUD_DIVINT` reader - The integer baud rate divisor. These bits are cleared to 0 on reset."]
pub struct BAUD_DIVINT_R(crate::FieldReader<u16, u16>);
impl BAUD_DIVINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BAUD_DIVINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAUD_DIVINT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAUD_DIVINT` writer - The integer baud rate divisor. These bits are cleared to 0 on reset."]
pub struct BAUD_DIVINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD_DIVINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The integer baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn baud_divint(&self) -> BAUD_DIVINT_R {
        BAUD_DIVINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The integer baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn baud_divint(&mut self) -> BAUD_DIVINT_W {
        BAUD_DIVINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integer Baud Rate Register, UARTIBRD  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartibrd](index.html) module"]
pub struct UARTIBRD_SPEC;
impl crate::RegisterSpec for UARTIBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartibrd::R](R) reader structure"]
impl crate::Readable for UARTIBRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartibrd::W](W) writer structure"]
impl crate::Writable for UARTIBRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UARTIBRD to value 0"]
impl crate::Resettable for UARTIBRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
