#[doc = "Register `CLK_SYS_RESUS_STATUS` reader"]
pub struct R(crate::R<CLK_SYS_RESUS_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SYS_RESUS_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SYS_RESUS_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SYS_RESUS_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESUSSED` reader - Clock has been resuscitated, correct the error then send ctrl_clear=1"]
pub struct RESUSSED_R(crate::FieldReader<bool, bool>);
impl RESUSSED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESUSSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUSSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Clock has been resuscitated, correct the error then send ctrl_clear=1"]
    #[inline(always)]
    pub fn resussed(&self) -> RESUSSED_R {
        RESUSSED_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [clk_sys_resus_status](index.html) module"]
pub struct CLK_SYS_RESUS_STATUS_SPEC;
impl crate::RegisterSpec for CLK_SYS_RESUS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_sys_resus_status::R](R) reader structure"]
impl crate::Readable for CLK_SYS_RESUS_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLK_SYS_RESUS_STATUS to value 0"]
impl crate::Resettable for CLK_SYS_RESUS_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
