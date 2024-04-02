#[doc = "Register `IC_RX_TL` reader"]
pub type R = crate::R<IC_RX_TL_SPEC>;
#[doc = "Register `IC_RX_TL` writer"]
pub type W = crate::W<IC_RX_TL_SPEC>;
#[doc = "Field `RX_TL` reader - Receive FIFO Threshold Level.  

 Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries."]
pub type RX_TL_R = crate::FieldReader;
#[doc = "Field `RX_TL` writer - Receive FIFO Threshold Level.  

 Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries."]
pub type RX_TL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO Threshold Level.  

 Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries."]
    #[inline(always)]
    pub fn rx_tl(&self) -> RX_TL_R {
        RX_TL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive FIFO Threshold Level.  

 Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tl(&mut self) -> RX_TL_W<IC_RX_TL_SPEC> {
        RX_TL_W::new(self, 0)
    }
}
#[doc = "I2C Receive FIFO Threshold Register  

You can [`read`](crate::Reg::read) this register and get [`ic_rx_tl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_rx_tl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_RX_TL_SPEC;
impl crate::RegisterSpec for IC_RX_TL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_rx_tl::R`](R) reader structure"]
impl crate::Readable for IC_RX_TL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_rx_tl::W`](W) writer structure"]
impl crate::Writable for IC_RX_TL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_RX_TL to value 0"]
impl crate::Resettable for IC_RX_TL_SPEC {
    const RESET_VALUE: u32 = 0;
}
