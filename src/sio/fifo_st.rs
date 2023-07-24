#[doc = "Register `FIFO_ST` reader"]
pub struct R(crate::R<FIFO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_ST` writer"]
pub struct W(crate::W<FIFO_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FIFO_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLD` reader - Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)"]
pub type VLD_R = crate::BitReader<bool>;
#[doc = "Field `RDY` reader - Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)"]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `WOF` reader - Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
pub type WOF_R = crate::BitReader<bool>;
#[doc = "Field `WOF` writer - Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
pub type WOF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FIFO_ST_SPEC, bool, O>;
#[doc = "Field `ROE` reader - Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
pub type ROE_R = crate::BitReader<bool>;
#[doc = "Field `ROE` writer - Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
pub type ROE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FIFO_ST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
    #[inline(always)]
    pub fn wof(&self) -> WOF_R {
        WOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
    #[inline(always)]
    pub fn roe(&self) -> ROE_R {
        ROE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn wof(&mut self) -> WOF_W<2> {
        WOF_W::new(self)
    }
    #[doc = "Bit 3 - Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn roe(&mut self) -> ROE_W<3> {
        ROE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register for inter-core FIFOs (mailboxes).  
 There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep.  
 Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX).  
 Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX).  
 The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fifo_st](index.html) module"]
pub struct FIFO_ST_SPEC;
impl crate::RegisterSpec for FIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_st::R](R) reader structure"]
impl crate::Readable for FIFO_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_st::W](W) writer structure"]
impl crate::Writable for FIFO_ST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0c;
}
#[doc = "`reset()` method sets FIFO_ST to value 0x02"]
impl crate::Resettable for FIFO_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
