#[doc = "Register `IC_DMA_CR` reader"]
pub type R = crate::R<IC_DMA_CR_SPEC>;
#[doc = "Register `IC_DMA_CR` writer"]
pub type W = crate::W<IC_DMA_CR_SPEC>;
#[doc = "Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDMAE_A {
    #[doc = "0: Receive FIFO DMA channel disabled"]
    DISABLED = 0,
    #[doc = "1: Receive FIFO DMA channel enabled"]
    ENABLED = 1,
}
impl From<RDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDMAE` reader - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0"]
pub type RDMAE_R = crate::BitReader<RDMAE_A>;
impl RDMAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDMAE_A {
        match self.bits {
            false => RDMAE_A::DISABLED,
            true => RDMAE_A::ENABLED,
        }
    }
    #[doc = "Receive FIFO DMA channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDMAE_A::DISABLED
    }
    #[doc = "Receive FIFO DMA channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDMAE_A::ENABLED
    }
}
#[doc = "Field `RDMAE` writer - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0"]
pub type RDMAE_W<'a, REG> = crate::BitWriter<'a, REG, RDMAE_A>;
impl<'a, REG> RDMAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO DMA channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDMAE_A::DISABLED)
    }
    #[doc = "Receive FIFO DMA channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDMAE_A::ENABLED)
    }
}
#[doc = "Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDMAE_A {
    #[doc = "0: transmit FIFO DMA channel disabled"]
    DISABLED = 0,
    #[doc = "1: Transmit FIFO DMA channel enabled"]
    ENABLED = 1,
}
impl From<TDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: TDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDMAE` reader - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0"]
pub type TDMAE_R = crate::BitReader<TDMAE_A>;
impl TDMAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDMAE_A {
        match self.bits {
            false => TDMAE_A::DISABLED,
            true => TDMAE_A::ENABLED,
        }
    }
    #[doc = "transmit FIFO DMA channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDMAE_A::DISABLED
    }
    #[doc = "Transmit FIFO DMA channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDMAE_A::ENABLED
    }
}
#[doc = "Field `TDMAE` writer - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0"]
pub type TDMAE_W<'a, REG> = crate::BitWriter<'a, REG, TDMAE_A>;
impl<'a, REG> TDMAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "transmit FIFO DMA channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TDMAE_A::DISABLED)
    }
    #[doc = "Transmit FIFO DMA channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TDMAE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0"]
    #[inline(always)]
    pub fn rdmae(&self) -> RDMAE_R {
        RDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0"]
    #[inline(always)]
    pub fn tdmae(&self) -> TDMAE_R {
        TDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn rdmae(&mut self) -> RDMAE_W<IC_DMA_CR_SPEC> {
        RDMAE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn tdmae(&mut self) -> TDMAE_W<IC_DMA_CR_SPEC> {
        TDMAE_W::new(self, 1)
    }
}
#[doc = "DMA Control Register  

 The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE.  

You can [`read`](crate::Reg::read) this register and get [`ic_dma_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_DMA_CR_SPEC;
impl crate::RegisterSpec for IC_DMA_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_dma_cr::R`](R) reader structure"]
impl crate::Readable for IC_DMA_CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_dma_cr::W`](W) writer structure"]
impl crate::Writable for IC_DMA_CR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_DMA_CR to value 0"]
impl crate::Resettable for IC_DMA_CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
