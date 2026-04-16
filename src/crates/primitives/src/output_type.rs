use bitcoin::Script;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputType {
    P2pkh,
    P2sh,
    P2wpkh,
    P2wsh,
    P2tr,
    OpReturn,
    NonStandard,
    // TODO: pay2anchor
}

/// Classify a scriptPubKey by type from raw bytes.
pub fn classify_script_pubkey(spk: &[u8]) -> OutputType {
    let script = Script::from_bytes(spk);

    if script.is_op_return() {
        return OutputType::OpReturn;
    }
    if script.is_p2pkh() {
        return OutputType::P2pkh;
    }
    if script.is_p2sh() {
        return OutputType::P2sh;
    }
    if script.is_p2wpkh() {
        return OutputType::P2wpkh;
    }
    if script.is_p2wsh() {
        return OutputType::P2wsh;
    }
    if script.is_p2tr() {
        return OutputType::P2tr;
    }

    OutputType::NonStandard
}
