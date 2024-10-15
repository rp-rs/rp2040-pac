#[doc = "Register `HOST_ADDR_ENDP%s` reader"]
pub type R = crate::R<HOST_ADDR_ENDP_SPEC>;
#[doc = "Register `HOST_ADDR_ENDP%s` writer"]
pub type W = crate::W<HOST_ADDR_ENDP_SPEC>;
#[doc = "Field `ADDRESS` reader - Device address"]
pub type ADDRESS_R = crate::FieldReader;
#[doc = "Field `ADDRESS` writer - Device address"]
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ENDPOINT` reader - Endpoint number of the interrupt endpoint"]
pub type ENDPOINT_R = crate::FieldReader;
#[doc = "Field `ENDPOINT` writer - Endpoint number of the interrupt endpoint"]
pub type ENDPOINT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTEP_DIR` reader - Direction of the interrupt endpoint. In=0, Out=1"]
pub type INTEP_DIR_R = crate::BitReader;
#[doc = "Field `INTEP_DIR` writer - Direction of the interrupt endpoint. In=0, Out=1"]
pub type INTEP_DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEP_PREAMBLE` reader - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
pub type INTEP_PREAMBLE_R = crate::BitReader;
#[doc = "Field `INTEP_PREAMBLE` writer - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
pub type INTEP_PREAMBLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - Endpoint number of the interrupt endpoint"]
    #[inline(always)]
    pub fn endpoint(&self) -> ENDPOINT_R {
        ENDPOINT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - Direction of the interrupt endpoint. In=0, Out=1"]
    #[inline(always)]
    pub fn intep_dir(&self) -> INTEP_DIR_R {
        INTEP_DIR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
    #[inline(always)]
    pub fn intep_preamble(&self) -> INTEP_PREAMBLE_R {
        INTEP_PREAMBLE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<HOST_ADDR_ENDP_SPEC> {
        ADDRESS_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Endpoint number of the interrupt endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn endpoint(&mut self) -> ENDPOINT_W<HOST_ADDR_ENDP_SPEC> {
        ENDPOINT_W::new(self, 16)
    }
    #[doc = "Bit 25 - Direction of the interrupt endpoint. In=0, Out=1"]
    #[inline(always)]
    #[must_use]
    pub fn intep_dir(&mut self) -> INTEP_DIR_W<HOST_ADDR_ENDP_SPEC> {
        INTEP_DIR_W::new(self, 25)
    }
    #[doc = "Bit 26 - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
    #[inline(always)]
    #[must_use]
    pub fn intep_preamble(&mut self) -> INTEP_PREAMBLE_W<HOST_ADDR_ENDP_SPEC> {
        INTEP_PREAMBLE_W::new(self, 26)
    }
}
#[doc = "Interrupt endpoints. Only valid in HOST mode.  

You can [`read`](crate::Reg::read) this register and get [`host_addr_endp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_addr_endp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_ADDR_ENDP_SPEC;
impl crate::RegisterSpec for HOST_ADDR_ENDP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_addr_endp::R`](R) reader structure"]
impl crate::Readable for HOST_ADDR_ENDP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_addr_endp::W`](W) writer structure"]
impl crate::Writable for HOST_ADDR_ENDP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_ADDR_ENDP%s to value 0"]
impl crate::Resettable for HOST_ADDR_ENDP_SPEC {
    const RESET_VALUE: u32 = 0;
}
