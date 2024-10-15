#[doc = "Register `TXFLR` reader"]
pub type R = crate::R<TXFLR_SPEC>;
#[doc = "Field `TFTFL` reader - Transmit FIFO level"]
pub type TFTFL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO level"]
    #[inline(always)]
    pub fn tftfl(&self) -> TFTFL_R {
        TFTFL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TX FIFO level  

You can [`read`](crate::Reg::read) this register and get [`txflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFLR_SPEC;
impl crate::RegisterSpec for TXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txflr::R`](R) reader structure"]
impl crate::Readable for TXFLR_SPEC {}
#[doc = "`reset()` method sets TXFLR to value 0"]
impl crate::Resettable for TXFLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
