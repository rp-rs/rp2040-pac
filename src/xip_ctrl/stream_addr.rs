#[doc = "Register `STREAM_ADDR` reader"]
pub type R = crate::R<STREAM_ADDR_SPEC>;
#[doc = "Register `STREAM_ADDR` writer"]
pub type W = crate::W<STREAM_ADDR_SPEC>;
#[doc = "Field `STREAM_ADDR` reader - The address of the next word to be streamed from flash to the streaming FIFO.  
 Increments automatically after each flash access.  
 Write the initial access address here before starting a streaming read."]
pub type STREAM_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `STREAM_ADDR` writer - The address of the next word to be streamed from flash to the streaming FIFO.  
 Increments automatically after each flash access.  
 Write the initial access address here before starting a streaming read."]
pub type STREAM_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - The address of the next word to be streamed from flash to the streaming FIFO.  
 Increments automatically after each flash access.  
 Write the initial access address here before starting a streaming read."]
    #[inline(always)]
    pub fn stream_addr(&self) -> STREAM_ADDR_R {
        STREAM_ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - The address of the next word to be streamed from flash to the streaming FIFO.  
 Increments automatically after each flash access.  
 Write the initial access address here before starting a streaming read."]
    #[inline(always)]
    #[must_use]
    pub fn stream_addr(&mut self) -> STREAM_ADDR_W<STREAM_ADDR_SPEC> {
        STREAM_ADDR_W::new(self, 2)
    }
}
#[doc = "FIFO stream address  

You can [`read`](crate::Reg::read) this register and get [`stream_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stream_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STREAM_ADDR_SPEC;
impl crate::RegisterSpec for STREAM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stream_addr::R`](R) reader structure"]
impl crate::Readable for STREAM_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stream_addr::W`](W) writer structure"]
impl crate::Writable for STREAM_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STREAM_ADDR to value 0"]
impl crate::Resettable for STREAM_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
