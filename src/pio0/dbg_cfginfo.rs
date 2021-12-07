#[doc = "Register `DBG_CFGINFO` reader"]
pub struct R(crate::R<DBG_CFGINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_CFGINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_CFGINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_CFGINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IMEM_SIZE` reader - The size of the instruction memory, measured in units of one instruction"]
pub struct IMEM_SIZE_R(crate::FieldReader<u8, u8>);
impl IMEM_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IMEM_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMEM_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM_COUNT` reader - The number of state machines this PIO instance is equipped with."]
pub struct SM_COUNT_R(crate::FieldReader<u8, u8>);
impl SM_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SM_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_DEPTH` reader - The depth of the state machine TX/RX FIFOs, measured in words.  
 Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double  
 this depth."]
pub struct FIFO_DEPTH_R(crate::FieldReader<u8, u8>);
impl FIFO_DEPTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_DEPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_DEPTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:21 - The size of the instruction memory, measured in units of one instruction"]
    #[inline(always)]
    pub fn imem_size(&self) -> IMEM_SIZE_R {
        IMEM_SIZE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - The number of state machines this PIO instance is equipped with."]
    #[inline(always)]
    pub fn sm_count(&self) -> SM_COUNT_R {
        SM_COUNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:5 - The depth of the state machine TX/RX FIFOs, measured in words.  
 Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double  
 this depth."]
    #[inline(always)]
    pub fn fifo_depth(&self) -> FIFO_DEPTH_R {
        FIFO_DEPTH_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "The PIO hardware has some free parameters that may vary between chip products.  
 These should be provided in the chip datasheet, but are also exposed here.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dbg_cfginfo](index.html) module"]
pub struct DBG_CFGINFO_SPEC;
impl crate::RegisterSpec for DBG_CFGINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_cfginfo::R](R) reader structure"]
impl crate::Readable for DBG_CFGINFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBG_CFGINFO to value 0"]
impl crate::Resettable for DBG_CFGINFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
