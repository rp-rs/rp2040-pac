#[doc = "Register `INTS` reader"]
pub type R = crate::R<INTS_SPEC>;
#[doc = "Field `FIFO` reader - Triggered when the sample FIFO reaches a certain level.  
 This level can be programmed via the FCS_THRESH field."]
pub type FIFO_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Triggered when the sample FIFO reaches a certain level.  
 This level can be programmed via the FCS_THRESH field."]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt status after masking &amp; forcing  

You can [`read`](crate::Reg::read) this register and get [`ints::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints::R`](R) reader structure"]
impl crate::Readable for INTS_SPEC {}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
