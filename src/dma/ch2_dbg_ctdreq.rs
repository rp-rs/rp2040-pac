#[doc = "Register `CH2_DBG_CTDREQ` reader"]
pub type R = crate::R<CH2_DBG_CTDREQ_SPEC>;
#[doc = "Register `CH2_DBG_CTDREQ` writer"]
pub type W = crate::W<CH2_DBG_CTDREQ_SPEC>;
#[doc = "Field `CH2_DBG_CTDREQ` reader - "]
pub type CH2_DBG_CTDREQ_R = crate::FieldReader;
#[doc = "Field `CH2_DBG_CTDREQ` writer - "]
pub type CH2_DBG_CTDREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch2_dbg_ctdreq(&self) -> CH2_DBG_CTDREQ_R {
        CH2_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_dbg_ctdreq(&mut self) -> CH2_DBG_CTDREQ_W<CH2_DBG_CTDREQ_SPEC, 0> {
        CH2_DBG_CTDREQ_W::new(self)
    }
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
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch2_dbg_ctdreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dbg_ctdreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_DBG_CTDREQ_SPEC;
impl crate::RegisterSpec for CH2_DBG_CTDREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_dbg_ctdreq::R`](R) reader structure"]
impl crate::Readable for CH2_DBG_CTDREQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2_dbg_ctdreq::W`](W) writer structure"]
impl crate::Writable for CH2_DBG_CTDREQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x3f;
}
#[doc = "`reset()` method sets CH2_DBG_CTDREQ to value 0"]
impl crate::Resettable for CH2_DBG_CTDREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
