#[doc = "Register `CHAN_ABORT` reader"]
pub type R = crate::R<CHAN_ABORT_SPEC>;
#[doc = "Register `CHAN_ABORT` writer"]
pub type W = crate::W<CHAN_ABORT_SPEC>;
#[doc = "Field `CHAN_ABORT` reader - Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs.  

 After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
pub type CHAN_ABORT_R = crate::FieldReader<u16>;
#[doc = "Field `CHAN_ABORT` writer - Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs.  

 After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
pub type CHAN_ABORT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs.  

 After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
    #[inline(always)]
    pub fn chan_abort(&self) -> CHAN_ABORT_R {
        CHAN_ABORT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs.  

 After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
    #[inline(always)]
    #[must_use]
    pub fn chan_abort(&mut self) -> CHAN_ABORT_W<CHAN_ABORT_SPEC> {
        CHAN_ABORT_W::new(self, 0)
    }
}
#[doc = "Abort an in-progress transfer sequence on one or more channels  

You can [`read`](crate::Reg::read) this register and get [`chan_abort::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chan_abort::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHAN_ABORT_SPEC;
impl crate::RegisterSpec for CHAN_ABORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_abort::R`](R) reader structure"]
impl crate::Readable for CHAN_ABORT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chan_abort::W`](W) writer structure"]
impl crate::Writable for CHAN_ABORT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHAN_ABORT to value 0"]
impl crate::Resettable for CHAN_ABORT_SPEC {
    const RESET_VALUE: u32 = 0;
}
