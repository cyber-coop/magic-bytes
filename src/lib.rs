pub struct MagicBytes;

impl MagicBytes {
    pub const BITCOIN_MAINNET: [u8; 4] = [0xF9, 0xBE, 0xB4, 0xD9];
    pub const BITCOIN_TESTNET: [u8; 4] = [0xFA, 0xBF, 0xB5, 0xDA];
    pub const BITCOIN_REGTEST: [u8; 4] = [0x0b, 0x11, 0x09, 0x07];
    pub const BITCOIN_SIGNET: [u8; 4] = [0x0A, 0x03, 0xCF, 0x40];
    pub const DOGECOIN_MAINNET: [u8; 4] = [0xc0, 0xc0, 0xc0, 0xc0]; //https://github.com/dogecoin/dogecoin/blob/master/src/chainparams.cpp#L148
    pub const DOGECOIN_TESTNET: [u8; 4] = [0xFC, 0xC1, 0xB7, 0xDC]; //https://github.com/dogecoin/dogecoin/blob/master/src/chainparams.cpp#L306
    pub const DOGECOIN_REGTEST: [u8; 4] = [0xFA, 0xBF, 0xB5, 0xDA]; //https://github.com/dogecoin/dogecoin/blob/master/src/chainparams.cpp#L436
    pub const LITECOIN_MAINNET: [u8; 4] = [0xfb, 0xc0, 0xb6, 0xdb];
    pub const LITECOIN_TESTNET: [u8; 4] = [0xfd, 0xd2, 0xc8, 0xf1];
    pub const LITECOIN_REGTEST: [u8; 4] = [0xfa, 0xbf, 0xb5, 0xda];
    pub const NAMECOIN_MAINNET: [u8; 4] = [0xF9, 0xBE, 0xB4, 0xFE];
}
