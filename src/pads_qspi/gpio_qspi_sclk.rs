#[doc = "Register `GPIO_QSPI_SCLK` reader"]
pub struct R(crate::R<GPIO_QSPI_SCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_QSPI_SCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_QSPI_SCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_QSPI_SCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_QSPI_SCLK` writer"]
pub struct W(crate::W<GPIO_QSPI_SCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_QSPI_SCLK_SPEC>;
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
impl From<crate::W<GPIO_QSPI_SCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_QSPI_SCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEWFAST` reader - Slew rate control. 1 = Fast, 0 = Slow"]
pub type SLEWFAST_R = crate::BitReader<bool>;
#[doc = "Field `SLEWFAST` writer - Slew rate control. 1 = Fast, 0 = Slow"]
pub type SLEWFAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_QSPI_SCLK_SPEC, bool, O>;
#[doc = "Field `SCHMITT` reader - Enable schmitt trigger"]
pub type SCHMITT_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT` writer - Enable schmitt trigger"]
pub type SCHMITT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_QSPI_SCLK_SPEC, bool, O>;
#[doc = "Field `PDE` reader - Pull down enable"]
pub type PDE_R = crate::BitReader<bool>;
#[doc = "Field `PDE` writer - Pull down enable"]
pub type PDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_QSPI_SCLK_SPEC, bool, O>;
#[doc = "Field `PUE` reader - Pull up enable"]
pub type PUE_R = crate::BitReader<bool>;
#[doc = "Field `PUE` writer - Pull up enable"]
pub type PUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_QSPI_SCLK_SPEC, bool, O>;
#[doc = "Field `DRIVE` reader - Drive strength."]
pub type DRIVE_R = crate::FieldReader<u8, DRIVE_A>;
#[doc = "Drive strength.  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRIVE_A {
    #[doc = "0: `0`"]
    _2M_A = 0,
    #[doc = "1: `1`"]
    _4M_A = 1,
    #[doc = "2: `10`"]
    _8M_A = 2,
    #[doc = "3: `11`"]
    _12M_A = 3,
}
impl From<DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_A) -> Self {
        variant as _
    }
}
impl DRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVE_A {
        match self.bits {
            0 => DRIVE_A::_2M_A,
            1 => DRIVE_A::_4M_A,
            2 => DRIVE_A::_8M_A,
            3 => DRIVE_A::_12M_A,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2M_A`"]
    #[inline(always)]
    pub fn is_2m_a(&self) -> bool {
        *self == DRIVE_A::_2M_A
    }
    #[doc = "Checks if the value of the field is `_4M_A`"]
    #[inline(always)]
    pub fn is_4m_a(&self) -> bool {
        *self == DRIVE_A::_4M_A
    }
    #[doc = "Checks if the value of the field is `_8M_A`"]
    #[inline(always)]
    pub fn is_8m_a(&self) -> bool {
        *self == DRIVE_A::_8M_A
    }
    #[doc = "Checks if the value of the field is `_12M_A`"]
    #[inline(always)]
    pub fn is_12m_a(&self) -> bool {
        *self == DRIVE_A::_12M_A
    }
}
#[doc = "Field `DRIVE` writer - Drive strength."]
pub type DRIVE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_QSPI_SCLK_SPEC, u8, DRIVE_A, 2, O>;
impl<'a, const O: u8> DRIVE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _2m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_2M_A)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _4m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_4M_A)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _8m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_8M_A)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _12m_a(self) -> &'a mut W {
        self.variant(DRIVE_A::_12M_A)
    }
}
#[doc = "Field `IE` reader - Input enable"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - Input enable"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_QSPI_SCLK_SPEC, bool, O>;
#[doc = "Field `OD` reader - Output disable. Has priority over output enable from peripherals"]
pub type OD_R = crate::BitReader<bool>;
#[doc = "Field `OD` writer - Output disable. Has priority over output enable from peripherals"]
pub type OD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_QSPI_SCLK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Slew rate control. 1 = Fast, 0 = Slow"]
    #[inline(always)]
    pub fn slewfast(&self) -> SLEWFAST_R {
        SLEWFAST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable schmitt trigger"]
    #[inline(always)]
    pub fn schmitt(&self) -> SCHMITT_R {
        SCHMITT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull down enable"]
    #[inline(always)]
    pub fn pde(&self) -> PDE_R {
        PDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pull up enable"]
    #[inline(always)]
    pub fn pue(&self) -> PUE_R {
        PUE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Drive strength."]
    #[inline(always)]
    pub fn drive(&self) -> DRIVE_R {
        DRIVE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Input enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output disable. Has priority over output enable from peripherals"]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slew rate control. 1 = Fast, 0 = Slow"]
    #[inline(always)]
    #[must_use]
    pub fn slewfast(&mut self) -> SLEWFAST_W<0> {
        SLEWFAST_W::new(self)
    }
    #[doc = "Bit 1 - Enable schmitt trigger"]
    #[inline(always)]
    #[must_use]
    pub fn schmitt(&mut self) -> SCHMITT_W<1> {
        SCHMITT_W::new(self)
    }
    #[doc = "Bit 2 - Pull down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pde(&mut self) -> PDE_W<2> {
        PDE_W::new(self)
    }
    #[doc = "Bit 3 - Pull up enable"]
    #[inline(always)]
    #[must_use]
    pub fn pue(&mut self) -> PUE_W<3> {
        PUE_W::new(self)
    }
    #[doc = "Bits 4:5 - Drive strength."]
    #[inline(always)]
    #[must_use]
    pub fn drive(&mut self) -> DRIVE_W<4> {
        DRIVE_W::new(self)
    }
    #[doc = "Bit 6 - Input enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<6> {
        IE_W::new(self)
    }
    #[doc = "Bit 7 - Output disable. Has priority over output enable from peripherals"]
    #[inline(always)]
    #[must_use]
    pub fn od(&mut self) -> OD_W<7> {
        OD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad control register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_qspi_sclk](index.html) module"]
pub struct GPIO_QSPI_SCLK_SPEC;
impl crate::RegisterSpec for GPIO_QSPI_SCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_qspi_sclk::R](R) reader structure"]
impl crate::Readable for GPIO_QSPI_SCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_sclk::W](W) writer structure"]
impl crate::Writable for GPIO_QSPI_SCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_QSPI_SCLK to value 0x56"]
impl crate::Resettable for GPIO_QSPI_SCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0x56;
}
