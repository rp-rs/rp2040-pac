#[doc = "Register `HOST_ADDR_ENDP%s` reader"]
pub struct R(crate::R<HOST_ADDR_ENDP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_ADDR_ENDP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_ADDR_ENDP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_ADDR_ENDP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_ADDR_ENDP%s` writer"]
pub struct W(crate::W<HOST_ADDR_ENDP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_ADDR_ENDP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HOST_ADDR_ENDP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_ADDR_ENDP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - Device address"]
pub type ADDRESS_R = crate::FieldReader;
#[doc = "Field `ADDRESS` writer - Device address"]
pub type ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, HOST_ADDR_ENDP_SPEC, 7, O>;
#[doc = "Field `ENDPOINT` reader - Endpoint number of the interrupt endpoint"]
pub type ENDPOINT_R = crate::FieldReader;
#[doc = "Field `ENDPOINT` writer - Endpoint number of the interrupt endpoint"]
pub type ENDPOINT_W<'a, const O: u8> = crate::FieldWriter<'a, HOST_ADDR_ENDP_SPEC, 4, O>;
#[doc = "Field `INTEP_DIR` reader - Direction of the interrupt endpoint. In=0, Out=1"]
pub type INTEP_DIR_R = crate::BitReader;
#[doc = "Field `INTEP_DIR` writer - Direction of the interrupt endpoint. In=0, Out=1"]
pub type INTEP_DIR_W<'a, const O: u8> = crate::BitWriter<'a, HOST_ADDR_ENDP_SPEC, O>;
#[doc = "Field `INTEP_PREAMBLE` reader - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
pub type INTEP_PREAMBLE_R = crate::BitReader;
#[doc = "Field `INTEP_PREAMBLE` writer - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
pub type INTEP_PREAMBLE_W<'a, const O: u8> = crate::BitWriter<'a, HOST_ADDR_ENDP_SPEC, O>;
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
    pub fn address(&mut self) -> ADDRESS_W<0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Bits 16:19 - Endpoint number of the interrupt endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn endpoint(&mut self) -> ENDPOINT_W<16> {
        ENDPOINT_W::new(self)
    }
    #[doc = "Bit 25 - Direction of the interrupt endpoint. In=0, Out=1"]
    #[inline(always)]
    #[must_use]
    pub fn intep_dir(&mut self) -> INTEP_DIR_W<25> {
        INTEP_DIR_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
    #[inline(always)]
    #[must_use]
    pub fn intep_preamble(&mut self) -> INTEP_PREAMBLE_W<26> {
        INTEP_PREAMBLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt endpoints. Only valid in HOST mode.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [host_addr_endp](index.html) module"]
pub struct HOST_ADDR_ENDP_SPEC;
impl crate::RegisterSpec for HOST_ADDR_ENDP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_addr_endp::R](R) reader structure"]
impl crate::Readable for HOST_ADDR_ENDP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_addr_endp::W](W) writer structure"]
impl crate::Writable for HOST_ADDR_ENDP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_ADDR_ENDP%s to value 0"]
impl crate::Resettable for HOST_ADDR_ENDP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
