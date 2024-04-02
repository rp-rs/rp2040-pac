#[doc = "Register `UARTPCELLID1` reader"]
pub type R = crate::R<UARTPCELLID1_SPEC>;
#[doc = "Field `UARTPCELLID1` reader - These bits read back as 0xF0"]
pub type UARTPCELLID1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xF0"]
    #[inline(always)]
    pub fn uartpcellid1(&self) -> UARTPCELLID1_R {
        UARTPCELLID1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPCellID1 Register  

You can [`read`](crate::Reg::read) this register and get [`uartpcellid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPCELLID1_SPEC;
impl crate::RegisterSpec for UARTPCELLID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartpcellid1::R`](R) reader structure"]
impl crate::Readable for UARTPCELLID1_SPEC {}
#[doc = "`reset()` method sets UARTPCELLID1 to value 0xf0"]
impl crate::Resettable for UARTPCELLID1_SPEC {
    const RESET_VALUE: u32 = 0xf0;
}
