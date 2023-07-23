#[doc = "Register `ADDR_ENDP10` reader"]
pub struct R(crate::R<ADDR_ENDP10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_ENDP10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_ENDP10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_ENDP10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR_ENDP10` writer"]
pub struct W(crate::W<ADDR_ENDP10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_ENDP10_SPEC>;
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
impl From<crate::W<ADDR_ENDP10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_ENDP10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - Device address"]
pub type ADDRESS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRESS` writer - Device address"]
pub type ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR_ENDP10_SPEC, u8, u8, 7, O>;
#[doc = "Field `ENDPOINT` reader - Endpoint number of the interrupt endpoint"]
pub type ENDPOINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENDPOINT` writer - Endpoint number of the interrupt endpoint"]
pub type ENDPOINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR_ENDP10_SPEC, u8, u8, 4, O>;
#[doc = "Field `INTEP_DIR` reader - Direction of the interrupt endpoint. In=0, Out=1"]
pub type INTEP_DIR_R = crate::BitReader<bool>;
#[doc = "Field `INTEP_DIR` writer - Direction of the interrupt endpoint. In=0, Out=1"]
pub type INTEP_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDR_ENDP10_SPEC, bool, O>;
#[doc = "Field `INTEP_PREAMBLE` reader - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
pub type INTEP_PREAMBLE_R = crate::BitReader<bool>;
#[doc = "Field `INTEP_PREAMBLE` writer - Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
pub type INTEP_PREAMBLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDR_ENDP10_SPEC, bool, O>;
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
#[doc = "Interrupt endpoint 10. Only valid for HOST mode.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [addr_endp10](index.html) module"]
pub struct ADDR_ENDP10_SPEC;
impl crate::RegisterSpec for ADDR_ENDP10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr_endp10::R](R) reader structure"]
impl crate::Readable for ADDR_ENDP10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr_endp10::W](W) writer structure"]
impl crate::Writable for ADDR_ENDP10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR_ENDP10 to value 0"]
impl crate::Resettable for ADDR_ENDP10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
