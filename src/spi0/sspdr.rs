#[doc = "Register `SSPDR` reader"]
pub type R = crate::R<SSPDR_SPEC>;
#[doc = "Register `SSPDR` writer"]
pub type W = crate::W<SSPDR_SPEC>;
#[doc = "Field `DATA` reader - Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn data(&mut self) -> DATA_W<SSPDR_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "Data register, SSPDR on page 3-6  

You can [`read`](crate::Reg::read) this register and get [`sspdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPDR_SPEC;
impl crate::RegisterSpec for SSPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspdr::R`](R) reader structure"]
impl crate::Readable for SSPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspdr::W`](W) writer structure"]
impl crate::Writable for SSPDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPDR to value 0"]
impl crate::Resettable for SSPDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
