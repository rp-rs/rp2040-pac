#[doc = "Register `FIFO_ST` reader"]
pub type R = crate::R<FIFO_ST_SPEC>;
#[doc = "Register `FIFO_ST` writer"]
pub type W = crate::W<FIFO_ST_SPEC>;
#[doc = "Field `VLD` reader - Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)"]
pub type VLD_R = crate::BitReader;
#[doc = "Field `RDY` reader - Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)"]
pub type RDY_R = crate::BitReader;
#[doc = "Field `WOF` reader - Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
pub type WOF_R = crate::BitReader;
#[doc = "Field `WOF` writer - Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
pub type WOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ROE` reader - Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
pub type ROE_R = crate::BitReader;
#[doc = "Field `ROE` writer - Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
pub type ROE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
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
    pub fn wof(&mut self) -> WOF_W<FIFO_ST_SPEC> {
        WOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn roe(&mut self) -> ROE_W<FIFO_ST_SPEC> {
        ROE_W::new(self, 3)
    }
}
#[doc = "Status register for inter-core FIFOs (mailboxes).  
 There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep.  
 Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX).  
 Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX).  
 The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register.  

You can [`read`](crate::Reg::read) this register and get [`fifo_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_ST_SPEC;
impl crate::RegisterSpec for FIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_st::R`](R) reader structure"]
impl crate::Readable for FIFO_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_st::W`](W) writer structure"]
impl crate::Writable for FIFO_ST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0c;
}
#[doc = "`reset()` method sets FIFO_ST to value 0x02"]
impl crate::Resettable for FIFO_ST_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
