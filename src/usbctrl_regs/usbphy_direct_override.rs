#[doc = "Register `USBPHY_DIRECT_OVERRIDE` reader"]
pub struct R(crate::R<USBPHY_DIRECT_OVERRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHY_DIRECT_OVERRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHY_DIRECT_OVERRIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHY_DIRECT_OVERRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHY_DIRECT_OVERRIDE` writer"]
pub struct W(crate::W<USBPHY_DIRECT_OVERRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHY_DIRECT_OVERRIDE_SPEC>;
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
impl From<crate::W<USBPHY_DIRECT_OVERRIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHY_DIRECT_OVERRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DP_PULLUP_HISEL_OVERRIDE_EN` reader - "]
pub type DP_PULLUP_HISEL_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DP_PULLUP_HISEL_OVERRIDE_EN` writer - "]
pub type DP_PULLUP_HISEL_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `DM_PULLUP_HISEL_OVERRIDE_EN` reader - "]
pub type DM_PULLUP_HISEL_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DM_PULLUP_HISEL_OVERRIDE_EN` writer - "]
pub type DM_PULLUP_HISEL_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `DP_PULLUP_EN_OVERRIDE_EN` reader - "]
pub type DP_PULLUP_EN_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DP_PULLUP_EN_OVERRIDE_EN` writer - "]
pub type DP_PULLUP_EN_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `DP_PULLDN_EN_OVERRIDE_EN` reader - "]
pub type DP_PULLDN_EN_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DP_PULLDN_EN_OVERRIDE_EN` writer - "]
pub type DP_PULLDN_EN_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `DM_PULLDN_EN_OVERRIDE_EN` reader - "]
pub type DM_PULLDN_EN_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DM_PULLDN_EN_OVERRIDE_EN` writer - "]
pub type DM_PULLDN_EN_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `TX_DP_OE_OVERRIDE_EN` reader - "]
pub type TX_DP_OE_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_DP_OE_OVERRIDE_EN` writer - "]
pub type TX_DP_OE_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `TX_DM_OE_OVERRIDE_EN` reader - "]
pub type TX_DM_OE_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_DM_OE_OVERRIDE_EN` writer - "]
pub type TX_DM_OE_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `TX_DP_OVERRIDE_EN` reader - "]
pub type TX_DP_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_DP_OVERRIDE_EN` writer - "]
pub type TX_DP_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `TX_DM_OVERRIDE_EN` reader - "]
pub type TX_DM_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_DM_OVERRIDE_EN` writer - "]
pub type TX_DM_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `RX_PD_OVERRIDE_EN` reader - "]
pub type RX_PD_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_PD_OVERRIDE_EN` writer - "]
pub type RX_PD_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `TX_PD_OVERRIDE_EN` reader - "]
pub type TX_PD_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_PD_OVERRIDE_EN` writer - "]
pub type TX_PD_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `TX_FSSLEW_OVERRIDE_EN` reader - "]
pub type TX_FSSLEW_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_FSSLEW_OVERRIDE_EN` writer - "]
pub type TX_FSSLEW_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `DM_PULLUP_OVERRIDE_EN` reader - "]
pub type DM_PULLUP_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DM_PULLUP_OVERRIDE_EN` writer - "]
pub type DM_PULLUP_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `TX_DIFFMODE_OVERRIDE_EN` reader - "]
pub type TX_DIFFMODE_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_DIFFMODE_OVERRIDE_EN` writer - "]
pub type TX_DIFFMODE_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBPHY_DIRECT_OVERRIDE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dp_pullup_hisel_override_en(&self) -> DP_PULLUP_HISEL_OVERRIDE_EN_R {
        DP_PULLUP_HISEL_OVERRIDE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dm_pullup_hisel_override_en(&self) -> DM_PULLUP_HISEL_OVERRIDE_EN_R {
        DM_PULLUP_HISEL_OVERRIDE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dp_pullup_en_override_en(&self) -> DP_PULLUP_EN_OVERRIDE_EN_R {
        DP_PULLUP_EN_OVERRIDE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dp_pulldn_en_override_en(&self) -> DP_PULLDN_EN_OVERRIDE_EN_R {
        DP_PULLDN_EN_OVERRIDE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dm_pulldn_en_override_en(&self) -> DM_PULLDN_EN_OVERRIDE_EN_R {
        DM_PULLDN_EN_OVERRIDE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_dp_oe_override_en(&self) -> TX_DP_OE_OVERRIDE_EN_R {
        TX_DP_OE_OVERRIDE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_dm_oe_override_en(&self) -> TX_DM_OE_OVERRIDE_EN_R {
        TX_DM_OE_OVERRIDE_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_dp_override_en(&self) -> TX_DP_OVERRIDE_EN_R {
        TX_DP_OVERRIDE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_dm_override_en(&self) -> TX_DM_OVERRIDE_EN_R {
        TX_DM_OVERRIDE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_pd_override_en(&self) -> RX_PD_OVERRIDE_EN_R {
        RX_PD_OVERRIDE_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_pd_override_en(&self) -> TX_PD_OVERRIDE_EN_R {
        TX_PD_OVERRIDE_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_fsslew_override_en(&self) -> TX_FSSLEW_OVERRIDE_EN_R {
        TX_FSSLEW_OVERRIDE_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dm_pullup_override_en(&self) -> DM_PULLUP_OVERRIDE_EN_R {
        DM_PULLUP_OVERRIDE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_diffmode_override_en(&self) -> TX_DIFFMODE_OVERRIDE_EN_R {
        TX_DIFFMODE_OVERRIDE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pullup_hisel_override_en(&mut self) -> DP_PULLUP_HISEL_OVERRIDE_EN_W<0> {
        DP_PULLUP_HISEL_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pullup_hisel_override_en(&mut self) -> DM_PULLUP_HISEL_OVERRIDE_EN_W<1> {
        DM_PULLUP_HISEL_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pullup_en_override_en(&mut self) -> DP_PULLUP_EN_OVERRIDE_EN_W<2> {
        DP_PULLUP_EN_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pulldn_en_override_en(&mut self) -> DP_PULLDN_EN_OVERRIDE_EN_W<3> {
        DP_PULLDN_EN_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pulldn_en_override_en(&mut self) -> DM_PULLDN_EN_OVERRIDE_EN_W<4> {
        DM_PULLDN_EN_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dp_oe_override_en(&mut self) -> TX_DP_OE_OVERRIDE_EN_W<5> {
        TX_DP_OE_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dm_oe_override_en(&mut self) -> TX_DM_OE_OVERRIDE_EN_W<6> {
        TX_DM_OE_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dp_override_en(&mut self) -> TX_DP_OVERRIDE_EN_W<7> {
        TX_DP_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dm_override_en(&mut self) -> TX_DM_OVERRIDE_EN_W<8> {
        TX_DM_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pd_override_en(&mut self) -> RX_PD_OVERRIDE_EN_W<9> {
        RX_PD_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pd_override_en(&mut self) -> TX_PD_OVERRIDE_EN_W<10> {
        TX_PD_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fsslew_override_en(&mut self) -> TX_FSSLEW_OVERRIDE_EN_W<11> {
        TX_FSSLEW_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pullup_override_en(&mut self) -> DM_PULLUP_OVERRIDE_EN_W<12> {
        DM_PULLUP_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tx_diffmode_override_en(&mut self) -> TX_DIFFMODE_OVERRIDE_EN_W<15> {
        TX_DIFFMODE_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Override enable for each control in usbphy_direct  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [usbphy_direct_override](index.html) module"]
pub struct USBPHY_DIRECT_OVERRIDE_SPEC;
impl crate::RegisterSpec for USBPHY_DIRECT_OVERRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbphy_direct_override::R](R) reader structure"]
impl crate::Readable for USBPHY_DIRECT_OVERRIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphy_direct_override::W](W) writer structure"]
impl crate::Writable for USBPHY_DIRECT_OVERRIDE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPHY_DIRECT_OVERRIDE to value 0"]
impl crate::Resettable for USBPHY_DIRECT_OVERRIDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
