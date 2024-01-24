#[doc = "Register `CH_READ_ADDR` reader"]
pub type R = crate::R<CH_READ_ADDR_SPEC>;
#[doc = "Register `CH_READ_ADDR` writer"]
pub type W = crate::W<CH_READ_ADDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CH_READ_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Channel 0 Read Address pointer  
 This register updates automatically each time a read completes. The current value is the next address to be read by this channel.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_read_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_read_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_READ_ADDR_SPEC;
impl crate::RegisterSpec for CH_READ_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_read_addr::R`](R) reader structure"]
impl crate::Readable for CH_READ_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_read_addr::W`](W) writer structure"]
impl crate::Writable for CH_READ_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_READ_ADDR to value 0"]
impl crate::Resettable for CH_READ_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
