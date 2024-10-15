#[doc = "Register `UARTPCELLID3` reader"]
pub type R = crate::R<UARTPCELLID3_SPEC>;
#[doc = "Field `UARTPCELLID3` reader - These bits read back as 0xB1"]
pub type UARTPCELLID3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xB1"]
    #[inline(always)]
    pub fn uartpcellid3(&self) -> UARTPCELLID3_R {
        UARTPCELLID3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPCellID3 Register  

You can [`read`](crate::Reg::read) this register and get [`uartpcellid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPCELLID3_SPEC;
impl crate::RegisterSpec for UARTPCELLID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartpcellid3::R`](R) reader structure"]
impl crate::Readable for UARTPCELLID3_SPEC {}
#[doc = "`reset()` method sets UARTPCELLID3 to value 0xb1"]
impl crate::Resettable for UARTPCELLID3_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
