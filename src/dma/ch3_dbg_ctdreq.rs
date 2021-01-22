#[doc = "Reader of register CH3_DBG_CTDREQ"]
pub type R = crate::R<u32, super::CH3_DBG_CTDREQ>;
#[doc = "Reader of field `CH3_DBG_CTDREQ`"]
pub type CH3_DBG_CTDREQ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch3_dbg_ctdreq(&self) -> CH3_DBG_CTDREQ_R {
        CH3_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}
