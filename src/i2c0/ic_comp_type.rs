#[doc = "Register `IC_COMP_TYPE` reader"]
pub type R = crate::R<IC_COMP_TYPE_SPEC>;
#[doc = "Field `IC_COMP_TYPE` reader - Designware Component Type number = 0x44_57_01_40. This assigned unique hex value is constant and is derived from the two ASCII letters 'DW' followed by a 16-bit unsigned number."]
pub type IC_COMP_TYPE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Designware Component Type number = 0x44_57_01_40. This assigned unique hex value is constant and is derived from the two ASCII letters 'DW' followed by a 16-bit unsigned number."]
    #[inline(always)]
    pub fn ic_comp_type(&self) -> IC_COMP_TYPE_R {
        IC_COMP_TYPE_R::new(self.bits)
    }
}
#[doc = "I2C Component Type Register  

You can [`read`](crate::Reg::read) this register and get [`ic_comp_type::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_COMP_TYPE_SPEC;
impl crate::RegisterSpec for IC_COMP_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_comp_type::R`](R) reader structure"]
impl crate::Readable for IC_COMP_TYPE_SPEC {}
#[doc = "`reset()` method sets IC_COMP_TYPE to value 0x4457_0140"]
impl crate::Resettable for IC_COMP_TYPE_SPEC {
    const RESET_VALUE: u32 = 0x4457_0140;
}
