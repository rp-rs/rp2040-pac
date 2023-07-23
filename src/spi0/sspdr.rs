#[doc = "Register `SSPDR` reader"]
pub struct R(crate::R<SSPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPDR` writer"]
pub struct W(crate::W<SSPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPDR_SPEC>;
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
impl From<crate::W<SSPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSPDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register, SSPDR on page 3-6  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspdr](index.html) module"]
pub struct SSPDR_SPEC;
impl crate::RegisterSpec for SSPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspdr::R](R) reader structure"]
impl crate::Readable for SSPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspdr::W](W) writer structure"]
impl crate::Writable for SSPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSPDR to value 0"]
impl crate::Resettable for SSPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
