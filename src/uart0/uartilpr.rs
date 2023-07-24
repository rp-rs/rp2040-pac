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
pub type ILPDVSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILPDVSR` writer - 8-bit low-power divisor value. These bits are cleared to 0 at reset."]
pub type ILPDVSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UARTILPR_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn ilpdvsr(&mut self) -> ILPDVSR_W<0> {
        ILPDVSR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UARTILPR to value 0"]
impl crate::Resettable for UARTILPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
