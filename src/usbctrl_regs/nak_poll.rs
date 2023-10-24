#[doc = "Register `NAK_POLL` reader"]
pub type R = crate::R<NAK_POLL_SPEC>;
#[doc = "Register `NAK_POLL` writer"]
pub type W = crate::W<NAK_POLL_SPEC>;
#[doc = "Field `DELAY_LS` reader - NAK polling interval for a low speed device"]
pub type DELAY_LS_R = crate::FieldReader<u16>;
#[doc = "Field `DELAY_LS` writer - NAK polling interval for a low speed device"]
pub type DELAY_LS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `DELAY_FS` reader - NAK polling interval for a full speed device"]
pub type DELAY_FS_R = crate::FieldReader<u16>;
#[doc = "Field `DELAY_FS` writer - NAK polling interval for a full speed device"]
pub type DELAY_FS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - NAK polling interval for a low speed device"]
    #[inline(always)]
    pub fn delay_ls(&self) -> DELAY_LS_R {
        DELAY_LS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - NAK polling interval for a full speed device"]
    #[inline(always)]
    pub fn delay_fs(&self) -> DELAY_FS_R {
        DELAY_FS_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - NAK polling interval for a low speed device"]
    #[inline(always)]
    #[must_use]
    pub fn delay_ls(&mut self) -> DELAY_LS_W<NAK_POLL_SPEC, 0> {
        DELAY_LS_W::new(self)
    }
    #[doc = "Bits 16:25 - NAK polling interval for a full speed device"]
    #[inline(always)]
    #[must_use]
    pub fn delay_fs(&mut self) -> DELAY_FS_W<NAK_POLL_SPEC, 16> {
        DELAY_FS_W::new(self)
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
#[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK.  

You can [`read`](crate::generic::Reg::read) this register and get [`nak_poll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nak_poll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAK_POLL_SPEC;
impl crate::RegisterSpec for NAK_POLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nak_poll::R`](R) reader structure"]
impl crate::Readable for NAK_POLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nak_poll::W`](W) writer structure"]
impl crate::Writable for NAK_POLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NAK_POLL to value 0x0010_0010"]
impl crate::Resettable for NAK_POLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0010;
}
