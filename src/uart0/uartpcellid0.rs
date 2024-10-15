#[doc = "Register `UARTPCELLID0` reader"]
pub type R = crate::R<UARTPCELLID0_SPEC>;
#[doc = "Field `UARTPCELLID0` reader - These bits read back as 0x0D"]
pub type UARTPCELLID0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x0D"]
    #[inline(always)]
    pub fn uartpcellid0(&self) -> UARTPCELLID0_R {
        UARTPCELLID0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPCellID0 Register  

You can [`read`](crate::Reg::read) this register and get [`uartpcellid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPCELLID0_SPEC;
impl crate::RegisterSpec for UARTPCELLID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartpcellid0::R`](R) reader structure"]
impl crate::Readable for UARTPCELLID0_SPEC {}
#[doc = "`reset()` method sets UARTPCELLID0 to value 0x0d"]
impl crate::Resettable for UARTPCELLID0_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
