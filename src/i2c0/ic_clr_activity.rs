#[doc = "Register `IC_CLR_ACTIVITY` reader"]
pub struct R(crate::R<IC_CLR_ACTIVITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_CLR_ACTIVITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_CLR_ACTIVITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_CLR_ACTIVITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLR_ACTIVITY` reader - Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
pub struct CLR_ACTIVITY_R(crate::FieldReader<bool, bool>);
impl CLR_ACTIVITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_ACTIVITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_ACTIVITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_activity(&self) -> CLR_ACTIVITY_R {
        CLR_ACTIVITY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Clear ACTIVITY Interrupt Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_clr_activity](index.html) module"]
pub struct IC_CLR_ACTIVITY_SPEC;
impl crate::RegisterSpec for IC_CLR_ACTIVITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_clr_activity::R](R) reader structure"]
impl crate::Readable for IC_CLR_ACTIVITY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_CLR_ACTIVITY to value 0"]
impl crate::Resettable for IC_CLR_ACTIVITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
