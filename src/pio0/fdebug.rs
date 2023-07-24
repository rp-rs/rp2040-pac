#[doc = "Register `FDEBUG` reader"]
pub struct R(crate::R<FDEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDEBUG` writer"]
pub struct W(crate::W<FDEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDEBUG_SPEC>;
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
impl From<crate::W<FDEBUG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDEBUG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXSTALL` reader - State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear."]
pub type RXSTALL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXSTALL` writer - State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear."]
pub type RXSTALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDEBUG_SPEC, u8, u8, 4, O>;
#[doc = "Field `RXUNDER` reader - RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error."]
pub type RXUNDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXUNDER` writer - RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error."]
pub type RXUNDER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDEBUG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXOVER` reader - TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor."]
pub type TXOVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXOVER` writer - TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor."]
pub type TXOVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDEBUG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXSTALL` reader - State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear."]
pub type TXSTALL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXSTALL` writer - State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear."]
pub type TXSTALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDEBUG_SPEC, u8, u8, 4, O>;
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
    pub fn rxstall(&mut self) -> RXSTALL_W<0> {
        RXSTALL_W::new(self)
    }
    #[doc = "Bits 8:11 - RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error."]
    #[inline(always)]
    #[must_use]
    pub fn rxunder(&mut self) -> RXUNDER_W<8> {
        RXUNDER_W::new(self)
    }
    #[doc = "Bits 16:19 - TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor."]
    #[inline(always)]
    #[must_use]
    pub fn txover(&mut self) -> TXOVER_W<16> {
        TXOVER_W::new(self)
    }
    #[doc = "Bits 24:27 - State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn txstall(&mut self) -> TXSTALL_W<24> {
        TXSTALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO debug register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fdebug](index.html) module"]
pub struct FDEBUG_SPEC;
impl crate::RegisterSpec for FDEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdebug::R](R) reader structure"]
impl crate::Readable for FDEBUG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdebug::W](W) writer structure"]
impl crate::Writable for FDEBUG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f0f_0f0f;
}
#[doc = "`reset()` method sets FDEBUG to value 0"]
impl crate::Resettable for FDEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
