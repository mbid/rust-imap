use imap_proto::UidSetMember;

/// An IMAP [`COPYUID` response](https://tools.ietf.org/html/rfc2359#page-4).
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct CopyUid {
    /// The UIDVALIDITY of the appended-to mailbox.
    pub uid_validity: u32,
    /// The UIDs of the messages copied to the destination mailbox, in the order they were copied.
    pub old_uids: Vec<UidSetMember>,
    /// The UIDs assigned to the copied messages, in the order they were assigned.
    pub new_uids: Vec<UidSetMember>,
}
