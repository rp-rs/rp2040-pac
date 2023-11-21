#[doc = "Register `CH_AL3_READ_ADDR_TRIG` reader"]
pub type R = crate::R<CH_AL3_READ_ADDR_TRIG_SPEC>;
#[doc = "Register `CH_AL3_READ_ADDR_TRIG` writer"]
pub type W = crate::W<CH_AL3_READ_ADDR_TRIG_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CH_AL3_READ_ADDR_TRIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
#[doc = "Alias for channel 0 READ_ADDR register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_al3_read_addr_trig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_al3_read_addr_trig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_AL3_READ_ADDR_TRIG_SPEC;
impl crate::RegisterSpec for CH_AL3_READ_ADDR_TRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_al3_read_addr_trig::R`](R) reader structure"]
impl crate::Readable for CH_AL3_READ_ADDR_TRIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_al3_read_addr_trig::W`](W) writer structure"]
impl crate::Writable for CH_AL3_READ_ADDR_TRIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH_AL3_READ_ADDR_TRIG to value 0"]
impl crate::Resettable for CH_AL3_READ_ADDR_TRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
