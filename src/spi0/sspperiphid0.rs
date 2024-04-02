#[doc = "Register `SSPPERIPHID0` reader"]
pub type R = crate::R<SSPPERIPHID0_SPEC>;
#[doc = "Field `PARTNUMBER0` reader - These bits read back as 0x22"]
pub type PARTNUMBER0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x22"]
    #[inline(always)]
    pub fn partnumber0(&self) -> PARTNUMBER0_R {
        PARTNUMBER0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

You can [`read`](crate::Reg::read) this register and get [`sspperiphid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPERIPHID0_SPEC;
impl crate::RegisterSpec for SSPPERIPHID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspperiphid0::R`](R) reader structure"]
impl crate::Readable for SSPPERIPHID0_SPEC {}
#[doc = "`reset()` method sets SSPPERIPHID0 to value 0x22"]
impl crate::Resettable for SSPPERIPHID0_SPEC {
    const RESET_VALUE: u32 = 0x22;
}
