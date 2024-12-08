#[doc = "Register `FIFO` reader"]
pub type R = crate::R<FIFO_SPEC>;
#[doc = "Register `FIFO` writer"]
pub type W = crate::W<FIFO_SPEC>;
#[doc = "Field `VAL` reader -   

The field is **modified** in some way after a read operation."]
pub type VAL_R = crate::FieldReader<u16>;
#[doc = "Field `ERR` reader - 1 if this particular sample experienced a conversion error. Remains in the same location if the sample is shifted.  

The field is **modified** in some way after a read operation."]
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
impl W {}
#[doc = "Conversion result FIFO  

You can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo::W`](W) writer structure"]
impl crate::Writable for FIFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FIFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
