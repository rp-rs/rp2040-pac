#[doc = "Register `SSPPERIPHID3` reader"]
pub type R = crate::R<SSPPERIPHID3_SPEC>;
#[doc = "Field `CONFIGURATION` reader - These bits read back as 0x00"]
pub type CONFIGURATION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x00"]
    #[inline(always)]
    pub fn configuration(&self) -> CONFIGURATION_R {
        CONFIGURATION_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

You can [`read`](crate::Reg::read) this register and get [`sspperiphid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPERIPHID3_SPEC;
impl crate::RegisterSpec for SSPPERIPHID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspperiphid3::R`](R) reader structure"]
impl crate::Readable for SSPPERIPHID3_SPEC {}
#[doc = "`reset()` method sets SSPPERIPHID3 to value 0"]
impl crate::Resettable for SSPPERIPHID3_SPEC {
    const RESET_VALUE: u32 = 0;
}
