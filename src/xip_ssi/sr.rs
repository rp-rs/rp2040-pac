#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `BUSY` reader - SSI busy flag"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `TFNF` reader - Transmit FIFO not full"]
pub type TFNF_R = crate::BitReader;
#[doc = "Field `TFE` reader - Transmit FIFO empty"]
pub type TFE_R = crate::BitReader;
#[doc = "Field `RFNE` reader - Receive FIFO not empty"]
pub type RFNE_R = crate::BitReader;
#[doc = "Field `RFF` reader - Receive FIFO full"]
pub type RFF_R = crate::BitReader;
#[doc = "Field `TXE` reader - Transmission error"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `DCOL` reader - Data collision error"]
pub type DCOL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SSI busy flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full"]
    #[inline(always)]
    pub fn tfnf(&self) -> TFNF_R {
        TFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO not empty"]
    #[inline(always)]
    pub fn rfne(&self) -> RFNE_R {
        RFNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmission error"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data collision error"]
    #[inline(always)]
    pub fn dcol(&self) -> DCOL_R {
        DCOL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {}
#[doc = "Status register  

You can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
