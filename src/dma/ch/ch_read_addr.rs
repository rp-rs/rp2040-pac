#[doc = "Register `CH_READ_ADDR` reader"]
pub type R = crate::R<CH_READ_ADDR_SPEC>;
#[doc = "Register `CH_READ_ADDR` writer"]
pub type W = crate::W<CH_READ_ADDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 0 Read Address pointer  
 This register updates automatically each time a read completes. The current value is the next address to be read by this channel.  

You can [`read`](crate::Reg::read) this register and get [`ch_read_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_read_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_READ_ADDR_SPEC;
impl crate::RegisterSpec for CH_READ_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_read_addr::R`](R) reader structure"]
impl crate::Readable for CH_READ_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_read_addr::W`](W) writer structure"]
impl crate::Writable for CH_READ_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_READ_ADDR to value 0"]
impl crate::Resettable for CH_READ_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
