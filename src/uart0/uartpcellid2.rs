#[doc = "Register `UARTPCELLID2` reader"]
pub type R = crate::R<UARTPCELLID2_SPEC>;
#[doc = "Register `UARTPCELLID2` writer"]
pub type W = crate::W<UARTPCELLID2_SPEC>;
#[doc = "Field `UARTPCELLID2` reader - These bits read back as 0x05"]
pub type UARTPCELLID2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x05"]
    #[inline(always)]
    pub fn uartpcellid2(&self) -> UARTPCELLID2_R {
        UARTPCELLID2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "UARTPCellID2 Register  

You can [`read`](crate::generic::Reg::read) this register and get [`uartpcellid2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartpcellid2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPCELLID2_SPEC;
impl crate::RegisterSpec for UARTPCELLID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartpcellid2::R`](R) reader structure"]
impl crate::Readable for UARTPCELLID2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartpcellid2::W`](W) writer structure"]
impl crate::Writable for UARTPCELLID2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTPCELLID2 to value 0x05"]
impl crate::Resettable for UARTPCELLID2_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
