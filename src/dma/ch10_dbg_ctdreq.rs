#[doc = "Reader of register CH10_DBG_CTDREQ"]
pub type R = crate::R<u32, super::CH10_DBG_CTDREQ>;
#[doc = "Reader of field `CH10_DBG_CTDREQ`"]
pub type CH10_DBG_CTDREQ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch10_dbg_ctdreq(&self) -> CH10_DBG_CTDREQ_R {
        CH10_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}
