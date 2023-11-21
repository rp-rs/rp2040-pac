#[doc = "Register `SETUP_PACKET_LOW` reader"]
pub type R = crate::R<SETUP_PACKET_LOW_SPEC>;
#[doc = "Register `SETUP_PACKET_LOW` writer"]
pub type W = crate::W<SETUP_PACKET_LOW_SPEC>;
#[doc = "Field `BMREQUESTTYPE` reader - "]
pub type BMREQUESTTYPE_R = crate::FieldReader;
#[doc = "Field `BMREQUESTTYPE` writer - "]
pub type BMREQUESTTYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `BREQUEST` reader - "]
pub type BREQUEST_R = crate::FieldReader;
#[doc = "Field `BREQUEST` writer - "]
pub type BREQUEST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WVALUE` reader - "]
pub type WVALUE_R = crate::FieldReader<u16>;
#[doc = "Field `WVALUE` writer - "]
pub type WVALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bmrequesttype(&self) -> BMREQUESTTYPE_R {
        BMREQUESTTYPE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn brequest(&self) -> BREQUEST_R {
        BREQUEST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wvalue(&self) -> WVALUE_R {
        WVALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn bmrequesttype(&mut self) -> BMREQUESTTYPE_W<SETUP_PACKET_LOW_SPEC, 0> {
        BMREQUESTTYPE_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn brequest(&mut self) -> BREQUEST_W<SETUP_PACKET_LOW_SPEC, 8> {
        BREQUEST_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn wvalue(&mut self) -> WVALUE_W<SETUP_PACKET_LOW_SPEC, 16> {
        WVALUE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bytes 0-3 of the SETUP packet from the host.  

You can [`read`](crate::generic::Reg::read) this register and get [`setup_packet_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup_packet_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SETUP_PACKET_LOW_SPEC;
impl crate::RegisterSpec for SETUP_PACKET_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setup_packet_low::R`](R) reader structure"]
impl crate::Readable for SETUP_PACKET_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`setup_packet_low::W`](W) writer structure"]
impl crate::Writable for SETUP_PACKET_LOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETUP_PACKET_LOW to value 0"]
impl crate::Resettable for SETUP_PACKET_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
