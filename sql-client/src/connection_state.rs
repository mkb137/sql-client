bitflags::bitflags! {
    /// The current connection state.
    struct ConnectionState: u8 {
        const CLOSED = 0x00;
        const OPEN = 0x01;
        const CONNECTING = 0x02;
        const EXECUTING = 0x04;
        const FETCHING = 0x08;
        const BROKEN = 0x16;
    }
}
