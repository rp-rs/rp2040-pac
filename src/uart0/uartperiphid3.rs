#[doc = "Register `UARTPERIPHID3` reader"]
pub type R = crate::R<UARTPERIPHID3_SPEC>;
#[doc = "Field `CONFIGURATION` reader - These bits read back as 0x00"]
pub type CONFIGURATION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x00"]
    #[inline(always)]
    pub fn configuration(&self) -> CONFIGURATION_R {
        CONFIGURATION_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPeriphID3 Register  

You can [`read`](crate::Reg::read) this register and get [`uartperiphid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPERIPHID3_SPEC;
impl crate::RegisterSpec for UARTPERIPHID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartperiphid3::R`](R) reader structure"]
impl crate::Readable for UARTPERIPHID3_SPEC {}
#[doc = "`reset()` method sets UARTPERIPHID3 to value 0"]
impl crate::Resettable for UARTPERIPHID3_SPEC {
    const RESET_VALUE: u32 = 0;
}
