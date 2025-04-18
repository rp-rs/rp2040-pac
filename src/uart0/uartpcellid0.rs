#[doc = "Register `UARTPCELLID0` reader"]
pub type R = crate::R<UARTPCELLID0_SPEC>;
#[doc = "Register `UARTPCELLID0` writer"]
pub type W = crate::W<UARTPCELLID0_SPEC>;
#[doc = "Field `UARTPCELLID0` reader - These bits read back as 0x0D"]
pub type UARTPCELLID0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x0D"]
    #[inline(always)]
    pub fn uartpcellid0(&self) -> UARTPCELLID0_R {
        UARTPCELLID0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "UARTPCellID0 Register  

You can [`read`](crate::generic::Reg::read) this register and get [`uartpcellid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartpcellid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPCELLID0_SPEC;
impl crate::RegisterSpec for UARTPCELLID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartpcellid0::R`](R) reader structure"]
impl crate::Readable for UARTPCELLID0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartpcellid0::W`](W) writer structure"]
impl crate::Writable for UARTPCELLID0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTPCELLID0 to value 0x0d"]
impl crate::Resettable for UARTPCELLID0_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
