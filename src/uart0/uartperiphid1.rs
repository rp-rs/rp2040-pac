#[doc = "Register `UARTPERIPHID1` reader"]
pub type R = crate::R<UARTPERIPHID1_SPEC>;
#[doc = "Field `PARTNUMBER1` reader - These bits read back as 0x0"]
pub type PARTNUMBER1_R = crate::FieldReader;
#[doc = "Field `DESIGNER0` reader - These bits read back as 0x1"]
pub type DESIGNER0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - These bits read back as 0x0"]
    #[inline(always)]
    pub fn partnumber1(&self) -> PARTNUMBER1_R {
        PARTNUMBER1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - These bits read back as 0x1"]
    #[inline(always)]
    pub fn designer0(&self) -> DESIGNER0_R {
        DESIGNER0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "UARTPeriphID1 Register  

You can [`read`](crate::Reg::read) this register and get [`uartperiphid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPERIPHID1_SPEC;
impl crate::RegisterSpec for UARTPERIPHID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartperiphid1::R`](R) reader structure"]
impl crate::Readable for UARTPERIPHID1_SPEC {}
#[doc = "`reset()` method sets UARTPERIPHID1 to value 0x10"]
impl crate::Resettable for UARTPERIPHID1_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
