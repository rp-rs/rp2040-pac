#[doc = "Register `RXFLR` reader"]
pub type R = crate::R<RXFLR_SPEC>;
#[doc = "Field `RXTFL` reader - Receive FIFO level"]
pub type RXTFL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO level"]
    #[inline(always)]
    pub fn rxtfl(&self) -> RXTFL_R {
        RXTFL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO level  

You can [`read`](crate::generic::Reg::read) this register and get [`rxflr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFLR_SPEC;
impl crate::RegisterSpec for RXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxflr::R`](R) reader structure"]
impl crate::Readable for RXFLR_SPEC {}
#[doc = "`reset()` method sets RXFLR to value 0"]
impl crate::Resettable for RXFLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
