#[doc = "Register `UARTPERIPHID0` reader"]
pub type R = crate::R<UARTPERIPHID0_SPEC>;
#[doc = "Register `UARTPERIPHID0` writer"]
pub type W = crate::W<UARTPERIPHID0_SPEC>;
#[doc = "Field `PARTNUMBER0` reader - These bits read back as 0x11"]
pub type PARTNUMBER0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x11"]
    #[inline(always)]
    pub fn partnumber0(&self) -> PARTNUMBER0_R {
        PARTNUMBER0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "UARTPeriphID0 Register  

You can [`read`](crate::generic::Reg::read) this register and get [`uartperiphid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartperiphid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPERIPHID0_SPEC;
impl crate::RegisterSpec for UARTPERIPHID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartperiphid0::R`](R) reader structure"]
impl crate::Readable for UARTPERIPHID0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartperiphid0::W`](W) writer structure"]
impl crate::Writable for UARTPERIPHID0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTPERIPHID0 to value 0x11"]
impl crate::Resettable for UARTPERIPHID0_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
