#[doc = "Register `IC_CLR_ACTIVITY` reader"]
pub type R = crate::R<IC_CLR_ACTIVITY_SPEC>;
#[doc = "Field `CLR_ACTIVITY` reader - Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub type CLR_ACTIVITY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_activity(&self) -> CLR_ACTIVITY_R {
        CLR_ACTIVITY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear ACTIVITY Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_activity::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CLR_ACTIVITY_SPEC;
impl crate::RegisterSpec for IC_CLR_ACTIVITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_activity::R`](R) reader structure"]
impl crate::Readable for IC_CLR_ACTIVITY_SPEC {}
#[doc = "`reset()` method sets IC_CLR_ACTIVITY to value 0"]
impl crate::Resettable for IC_CLR_ACTIVITY_SPEC {
    const RESET_VALUE: u32 = 0;
}
