use crate::pb3::{
    err::{guard, Err, Result},
    typ::trytes::sizeof_ntrytes,
};
use crate::spongos::Spongos;
use crate::trits::{TritSlice, TritSliceMut, Trits};

/// Size of `MsgId` link is 27 trytes.
pub fn sizeof_join() -> usize {
    sizeof_ntrytes(27)
}

/// Put `msgid` link into buffer `b` and `join` `slink` into `s`.
///
/// Note, `msgid` link itself is not `absorb`ed into `s`.
pub fn wrap_join(msgid: TritSlice, slink: &mut Spongos, s: &mut Spongos, b: &mut TritSliceMut) {
    assert_eq!(sizeof_join(), msgid.size());
    s.join(slink);
    msgid.copy(b.advance(msgid.size()));
}

/// Get `msgid` link from buffer `b`, lookup `slink` spongos instance and `join` `slink` into `s`.
///
/// Note, messages may be "spammed", ie. multiple messages can have the same `msgid`.
/// Behaviour to deal with such cases must be implemented in the `lookup_link`.
pub fn unwrap_join<T>(
    lookup_link: impl Fn(TritSlice) -> Option<(Spongos, T)>,
    s: &mut Spongos,
    b: &mut TritSlice,
) -> Result<T> {
    guard(sizeof_join() <= b.size(), Err::Eof)?;
    let mut msgid = Trits::zero(sizeof_join());
    b.advance(msgid.size()).copy(msgid.slice_mut());
    let (mut slink, info) = lookup_link(msgid.slice()).ok_or(Err::LinkNotFound)?;
    s.join(&mut slink);
    Ok(info)
}
