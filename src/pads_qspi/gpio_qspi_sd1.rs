#[doc = "Register `GPIO_QSPI_SD1` reader"]
pub type R = crate::R<GPIO_QSPI_SD1_SPEC>;
#[doc = "Register `GPIO_QSPI_SD1` writer"]
pub type W = crate::W<GPIO_QSPI_SD1_SPEC>;
#[doc = "Field `SLEWFAST` reader - Slew rate control. 1 = Fast, 0 = Slow"]
pub type SLEWFAST_R = crate::BitReader;
#[doc = "Field `SLEWFAST` writer - Slew rate control. 1 = Fast, 0 = Slow"]
pub type SLEWFAST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCHMITT` reader - Enable schmitt trigger"]
pub type SCHMITT_R = crate::BitReader;
#[doc = "Field `SCHMITT` writer - Enable schmitt trigger"]
pub type SCHMITT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDE` reader - Pull down enable"]
pub type PDE_R = crate::BitReader;
#[doc = "Field `PDE` writer - Pull down enable"]
pub type PDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PUE` reader - Pull up enable"]
pub type PUE_R = crate::BitReader;
#[doc = "Field `PUE` writer - Pull up enable"]
pub type PUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRIVE` reader - Drive strength."]
pub type DRIVE_R = crate::FieldReader<DRIVE_A>;
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
impl crate::FieldSpec for DRIVE_A {
    type Ux = u8;
}
impl DRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRIVE_A {
        match self.bits {
            0 => DRIVE_A::_2M_A,
            1 => DRIVE_A::_4M_A,
            2 => DRIVE_A::_8M_A,
            3 => DRIVE_A::_12M_A,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_2m_a(&self) -> bool {
        *self == DRIVE_A::_2M_A
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_4m_a(&self) -> bool {
        *self == DRIVE_A::_4M_A
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_8m_a(&self) -> bool {
        *self == DRIVE_A::_8M_A
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_12m_a(&self) -> bool {
        *self == DRIVE_A::_12M_A
    }
}
#[doc = "Field `DRIVE` writer - Drive strength."]
pub type DRIVE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DRIVE_A>;
impl<'a, REG, const O: u8> DRIVE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _2m_a(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVE_A::_2M_A)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _4m_a(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVE_A::_4M_A)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _8m_a(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVE_A::_8M_A)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _12m_a(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVE_A::_12M_A)
    }
}
#[doc = "Field `IE` reader - Input enable"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - Input enable"]
pub type IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD` reader - Output disable. Has priority over output enable from peripherals"]
pub type OD_R = crate::BitReader;
#[doc = "Field `OD` writer - Output disable. Has priority over output enable from peripherals"]
pub type OD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn slewfast(&mut self) -> SLEWFAST_W<GPIO_QSPI_SD1_SPEC, 0> {
        SLEWFAST_W::new(self)
    }
    #[doc = "Bit 1 - Enable schmitt trigger"]
    #[inline(always)]
    #[must_use]
    pub fn schmitt(&mut self) -> SCHMITT_W<GPIO_QSPI_SD1_SPEC, 1> {
        SCHMITT_W::new(self)
    }
    #[doc = "Bit 2 - Pull down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pde(&mut self) -> PDE_W<GPIO_QSPI_SD1_SPEC, 2> {
        PDE_W::new(self)
    }
    #[doc = "Bit 3 - Pull up enable"]
    #[inline(always)]
    #[must_use]
    pub fn pue(&mut self) -> PUE_W<GPIO_QSPI_SD1_SPEC, 3> {
        PUE_W::new(self)
    }
    #[doc = "Bits 4:5 - Drive strength."]
    #[inline(always)]
    #[must_use]
    pub fn drive(&mut self) -> DRIVE_W<GPIO_QSPI_SD1_SPEC, 4> {
        DRIVE_W::new(self)
    }
    #[doc = "Bit 6 - Input enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<GPIO_QSPI_SD1_SPEC, 6> {
        IE_W::new(self)
    }
    #[doc = "Bit 7 - Output disable. Has priority over output enable from peripherals"]
    #[inline(always)]
    #[must_use]
    pub fn od(&mut self) -> OD_W<GPIO_QSPI_SD1_SPEC, 7> {
        OD_W::new(self)
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
#[doc = "Pad control register  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_qspi_sd1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_qspi_sd1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_QSPI_SD1_SPEC;
impl crate::RegisterSpec for GPIO_QSPI_SD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_qspi_sd1::R`](R) reader structure"]
impl crate::Readable for GPIO_QSPI_SD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_qspi_sd1::W`](W) writer structure"]
impl crate::Writable for GPIO_QSPI_SD1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_QSPI_SD1 to value 0x52"]
impl crate::Resettable for GPIO_QSPI_SD1_SPEC {
    const RESET_VALUE: Self::Ux = 0x52;
}
