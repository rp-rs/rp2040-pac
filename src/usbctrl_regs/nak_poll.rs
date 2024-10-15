#[doc = "Register `NAK_POLL` reader"]
pub type R = crate::R<NAK_POLL_SPEC>;
#[doc = "Register `NAK_POLL` writer"]
pub type W = crate::W<NAK_POLL_SPEC>;
#[doc = "Field `DELAY_LS` reader - NAK polling interval for a low speed device"]
pub type DELAY_LS_R = crate::FieldReader<u16>;
#[doc = "Field `DELAY_LS` writer - NAK polling interval for a low speed device"]
pub type DELAY_LS_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DELAY_FS` reader - NAK polling interval for a full speed device"]
pub type DELAY_FS_R = crate::FieldReader<u16>;
#[doc = "Field `DELAY_FS` writer - NAK polling interval for a full speed device"]
pub type DELAY_FS_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
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
    pub fn delay_ls(&mut self) -> DELAY_LS_W<NAK_POLL_SPEC> {
        DELAY_LS_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - NAK polling interval for a full speed device"]
    #[inline(always)]
    #[must_use]
    pub fn delay_fs(&mut self) -> DELAY_FS_W<NAK_POLL_SPEC> {
        DELAY_FS_W::new(self, 16)
    }
}
#[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK.  

You can [`read`](crate::Reg::read) this register and get [`nak_poll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nak_poll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAK_POLL_SPEC;
impl crate::RegisterSpec for NAK_POLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nak_poll::R`](R) reader structure"]
impl crate::Readable for NAK_POLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nak_poll::W`](W) writer structure"]
impl crate::Writable for NAK_POLL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NAK_POLL to value 0x0010_0010"]
impl crate::Resettable for NAK_POLL_SPEC {
    const RESET_VALUE: u32 = 0x0010_0010;
}
