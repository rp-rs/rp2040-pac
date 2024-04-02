#[doc = "Register `FIFO` reader"]
pub type R = crate::R<FIFO_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u16>;
#[doc = "Field `ERR` reader - 1 if this particular sample experienced a conversion error. Remains in the same location if the sample is shifted."]
pub type ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - 1 if this particular sample experienced a conversion error. Remains in the same location if the sample is shifted."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Conversion result FIFO  

You can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FIFO_SPEC {}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FIFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
