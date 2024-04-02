#[doc = "Register `FDEBUG` reader"]
pub type R = crate::R<FDEBUG_SPEC>;
#[doc = "Register `FDEBUG` writer"]
pub type W = crate::W<FDEBUG_SPEC>;
#[doc = "Field `RXSTALL` reader - State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear."]
pub type RXSTALL_R = crate::FieldReader;
#[doc = "Field `RXSTALL` writer - State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear."]
pub type RXSTALL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RXUNDER` reader - RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error."]
pub type RXUNDER_R = crate::FieldReader;
#[doc = "Field `RXUNDER` writer - RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error."]
pub type RXUNDER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXOVER` reader - TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor."]
pub type TXOVER_R = crate::FieldReader;
#[doc = "Field `TXOVER` writer - TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor."]
pub type TXOVER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXSTALL` reader - State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear."]
pub type TXSTALL_R = crate::FieldReader;
#[doc = "Field `TXSTALL` writer - State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear."]
pub type TXSTALL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear."]
    #[inline(always)]
    pub fn rxstall(&self) -> RXSTALL_R {
        RXSTALL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error."]
    #[inline(always)]
    pub fn rxunder(&self) -> RXUNDER_R {
        RXUNDER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor."]
    #[inline(always)]
    pub fn txover(&self) -> TXOVER_R {
        TXOVER_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear."]
    #[inline(always)]
    pub fn txstall(&self) -> TXSTALL_R {
        TXSTALL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn rxstall(&mut self) -> RXSTALL_W<FDEBUG_SPEC> {
        RXSTALL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error."]
    #[inline(always)]
    #[must_use]
    pub fn rxunder(&mut self) -> RXUNDER_W<FDEBUG_SPEC> {
        RXUNDER_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor."]
    #[inline(always)]
    #[must_use]
    pub fn txover(&mut self) -> TXOVER_W<FDEBUG_SPEC> {
        TXOVER_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn txstall(&mut self) -> TXSTALL_W<FDEBUG_SPEC> {
        TXSTALL_W::new(self, 24)
    }
}
#[doc = "FIFO debug register  

You can [`read`](crate::Reg::read) this register and get [`fdebug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdebug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDEBUG_SPEC;
impl crate::RegisterSpec for FDEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdebug::R`](R) reader structure"]
impl crate::Readable for FDEBUG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdebug::W`](W) writer structure"]
impl crate::Writable for FDEBUG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f0f_0f0f;
}
#[doc = "`reset()` method sets FDEBUG to value 0"]
impl crate::Resettable for FDEBUG_SPEC {
    const RESET_VALUE: u32 = 0;
}
