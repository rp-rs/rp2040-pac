#[doc = "Register `UARTPCELLID2` reader"]
pub type R = crate::R<UARTPCELLID2_SPEC>;
#[doc = "Field `UARTPCELLID2` reader - These bits read back as 0x05"]
pub type UARTPCELLID2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x05"]
    #[inline(always)]
    pub fn uartpcellid2(&self) -> UARTPCELLID2_R {
        UARTPCELLID2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPCellID2 Register  

You can [`read`](crate::Reg::read) this register and get [`uartpcellid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPCELLID2_SPEC;
impl crate::RegisterSpec for UARTPCELLID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartpcellid2::R`](R) reader structure"]
impl crate::Readable for UARTPCELLID2_SPEC {}
#[doc = "`reset()` method sets UARTPCELLID2 to value 0x05"]
impl crate::Resettable for UARTPCELLID2_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
