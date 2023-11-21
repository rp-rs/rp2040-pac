#[doc = "Register `IC_TX_TL` reader"]
pub type R = crate::R<IC_TX_TL_SPEC>;
#[doc = "Register `IC_TX_TL` writer"]
pub type W = crate::W<IC_TX_TL_SPEC>;
#[doc = "Field `TX_TL` reader - Transmit FIFO Threshold Level.  

 Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
pub type TX_TL_R = crate::FieldReader;
#[doc = "Field `TX_TL` writer - Transmit FIFO Threshold Level.  

 Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
pub type TX_TL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO Threshold Level.  

 Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
    #[inline(always)]
    pub fn tx_tl(&self) -> TX_TL_R {
        TX_TL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit FIFO Threshold Level.  

 Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
    #[inline(always)]
    #[must_use]
    pub fn tx_tl(&mut self) -> TX_TL_W<IC_TX_TL_SPEC, 0> {
        TX_TL_W::new(self)
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
#[doc = "I2C Transmit FIFO Threshold Register  

You can [`read`](crate::generic::Reg::read) this register and get [`ic_tx_tl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic_tx_tl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_TX_TL_SPEC;
impl crate::RegisterSpec for IC_TX_TL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_tx_tl::R`](R) reader structure"]
impl crate::Readable for IC_TX_TL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_tx_tl::W`](W) writer structure"]
impl crate::Writable for IC_TX_TL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IC_TX_TL to value 0"]
impl crate::Resettable for IC_TX_TL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
