#[doc = "Register `UARTPERIPHID0` reader"]
pub type R = crate::R<UARTPERIPHID0_SPEC>;
#[doc = "Field `PARTNUMBER0` reader - These bits read back as 0x11"]
pub type PARTNUMBER0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x11"]
    #[inline(always)]
    pub fn partnumber0(&self) -> PARTNUMBER0_R {
        PARTNUMBER0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPeriphID0 Register  

You can [`read`](crate::Reg::read) this register and get [`uartperiphid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPERIPHID0_SPEC;
impl crate::RegisterSpec for UARTPERIPHID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartperiphid0::R`](R) reader structure"]
impl crate::Readable for UARTPERIPHID0_SPEC {}
#[doc = "`reset()` method sets UARTPERIPHID0 to value 0x11"]
impl crate::Resettable for UARTPERIPHID0_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
