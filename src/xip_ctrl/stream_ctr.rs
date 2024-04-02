#[doc = "Register `STREAM_CTR` reader"]
pub type R = crate::R<STREAM_CTR_SPEC>;
#[doc = "Register `STREAM_CTR` writer"]
pub type W = crate::W<STREAM_CTR_SPEC>;
#[doc = "Field `STREAM_CTR` reader - Write a nonzero value to start a streaming read. This will then  
 progress in the background, using flash idle cycles to transfer  
 a linear data block from flash to the streaming FIFO.  
 Decrements automatically (1 at a time) as the stream  
 progresses, and halts on reaching 0.  
 Write 0 to halt an in-progress stream, and discard any in-flight  
 read, so that a new stream can immediately be started (after  
 draining the FIFO and reinitialising STREAM_ADDR)"]
pub type STREAM_CTR_R = crate::FieldReader<u32>;
#[doc = "Field `STREAM_CTR` writer - Write a nonzero value to start a streaming read. This will then  
 progress in the background, using flash idle cycles to transfer  
 a linear data block from flash to the streaming FIFO.  
 Decrements automatically (1 at a time) as the stream  
 progresses, and halts on reaching 0.  
 Write 0 to halt an in-progress stream, and discard any in-flight  
 read, so that a new stream can immediately be started (after  
 draining the FIFO and reinitialising STREAM_ADDR)"]
pub type STREAM_CTR_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Write a nonzero value to start a streaming read. This will then  
 progress in the background, using flash idle cycles to transfer  
 a linear data block from flash to the streaming FIFO.  
 Decrements automatically (1 at a time) as the stream  
 progresses, and halts on reaching 0.  
 Write 0 to halt an in-progress stream, and discard any in-flight  
 read, so that a new stream can immediately be started (after  
 draining the FIFO and reinitialising STREAM_ADDR)"]
    #[inline(always)]
    pub fn stream_ctr(&self) -> STREAM_CTR_R {
        STREAM_CTR_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Write a nonzero value to start a streaming read. This will then  
 progress in the background, using flash idle cycles to transfer  
 a linear data block from flash to the streaming FIFO.  
 Decrements automatically (1 at a time) as the stream  
 progresses, and halts on reaching 0.  
 Write 0 to halt an in-progress stream, and discard any in-flight  
 read, so that a new stream can immediately be started (after  
 draining the FIFO and reinitialising STREAM_ADDR)"]
    #[inline(always)]
    #[must_use]
    pub fn stream_ctr(&mut self) -> STREAM_CTR_W<STREAM_CTR_SPEC> {
        STREAM_CTR_W::new(self, 0)
    }
}
#[doc = "FIFO stream control  

You can [`read`](crate::Reg::read) this register and get [`stream_ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stream_ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STREAM_CTR_SPEC;
impl crate::RegisterSpec for STREAM_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stream_ctr::R`](R) reader structure"]
impl crate::Readable for STREAM_CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stream_ctr::W`](W) writer structure"]
impl crate::Writable for STREAM_CTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STREAM_CTR to value 0"]
impl crate::Resettable for STREAM_CTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
