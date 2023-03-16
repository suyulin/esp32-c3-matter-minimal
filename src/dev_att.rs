use matter::data_model::sdm::dev_att::{DataType, DevAttDataFetcher};
use matter::error::Error;

pub struct HardCodedDevAtt {}

impl HardCodedDevAtt {
    pub fn new() -> Self {
        Self {}
    }
}

// credentials/test/attestation/Chip-Test-PAI-FFF1-8000-Cert
const PAI_CERT: [u8; 472] = [
    0x30, 0x82, 0x01, 0xd4, 0x30, 0x82, 0x01, 0x7a, 0xa0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x08, 0x3e,
    0x6c, 0xe6, 0x50, 0x9a, 0xd8, 0x40, 0xcd, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d,
    0x04, 0x03, 0x02, 0x30, 0x30, 0x31, 0x18, 0x30, 0x16, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x0f,
    0x4d, 0x61, 0x74, 0x74, 0x65, 0x72, 0x20, 0x54, 0x65, 0x73, 0x74, 0x20, 0x50, 0x41, 0x41, 0x31,
    0x14, 0x30, 0x12, 0x06, 0x0a, 0x2b, 0x06, 0x01, 0x04, 0x01, 0x82, 0xa2, 0x7c, 0x02, 0x01, 0x0c,
    0x04, 0x46, 0x46, 0x46, 0x31, 0x30, 0x20, 0x17, 0x0d, 0x32, 0x31, 0x30, 0x36, 0x32, 0x38, 0x31,
    0x34, 0x32, 0x33, 0x34, 0x33, 0x5a, 0x18, 0x0f, 0x39, 0x39, 0x39, 0x39, 0x31, 0x32, 0x33, 0x31,
    0x32, 0x33, 0x35, 0x39, 0x35, 0x39, 0x5a, 0x30, 0x46, 0x31, 0x18, 0x30, 0x16, 0x06, 0x03, 0x55,
    0x04, 0x03, 0x0c, 0x0f, 0x4d, 0x61, 0x74, 0x74, 0x65, 0x72, 0x20, 0x54, 0x65, 0x73, 0x74, 0x20,
    0x50, 0x41, 0x49, 0x31, 0x14, 0x30, 0x12, 0x06, 0x0a, 0x2b, 0x06, 0x01, 0x04, 0x01, 0x82, 0xa2,
    0x7c, 0x02, 0x01, 0x0c, 0x04, 0x46, 0x46, 0x46, 0x31, 0x31, 0x14, 0x30, 0x12, 0x06, 0x0a, 0x2b,
    0x06, 0x01, 0x04, 0x01, 0x82, 0xa2, 0x7c, 0x02, 0x02, 0x0c, 0x04, 0x38, 0x30, 0x30, 0x30, 0x30,
    0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86,
    0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00, 0x04, 0x80, 0xdd, 0xf1, 0x1b, 0x22, 0x8f,
    0x3e, 0x31, 0xf6, 0x3b, 0xcf, 0x57, 0x98, 0xda, 0x14, 0x62, 0x3a, 0xeb, 0xbd, 0xe8, 0x2e, 0xf3,
    0x78, 0xee, 0xad, 0xbf, 0xb1, 0x8f, 0xe1, 0xab, 0xce, 0x31, 0xd0, 0x8e, 0xd4, 0xb2, 0x06, 0x04,
    0xb6, 0xcc, 0xc6, 0xd9, 0xb5, 0xfa, 0xb6, 0x4e, 0x7d, 0xe1, 0x0c, 0xb7, 0x4b, 0xe0, 0x17, 0xc9,
    0xec, 0x15, 0x16, 0x05, 0x6d, 0x70, 0xf2, 0xcd, 0x0b, 0x22, 0xa3, 0x66, 0x30, 0x64, 0x30, 0x12,
    0x06, 0x03, 0x55, 0x1d, 0x13, 0x01, 0x01, 0xff, 0x04, 0x08, 0x30, 0x06, 0x01, 0x01, 0xff, 0x02,
    0x01, 0x00, 0x30, 0x0e, 0x06, 0x03, 0x55, 0x1d, 0x0f, 0x01, 0x01, 0xff, 0x04, 0x04, 0x03, 0x02,
    0x01, 0x06, 0x30, 0x1d, 0x06, 0x03, 0x55, 0x1d, 0x0e, 0x04, 0x16, 0x04, 0x14, 0xaf, 0x42, 0xb7,
    0x09, 0x4d, 0xeb, 0xd5, 0x15, 0xec, 0x6e, 0xcf, 0x33, 0xb8, 0x11, 0x15, 0x22, 0x5f, 0x32, 0x52,
    0x88, 0x30, 0x1f, 0x06, 0x03, 0x55, 0x1d, 0x23, 0x04, 0x18, 0x30, 0x16, 0x80, 0x14, 0x6a, 0xfd,
    0x22, 0x77, 0x1f, 0x51, 0x1f, 0xec, 0xbf, 0x16, 0x41, 0x97, 0x67, 0x10, 0xdc, 0xdc, 0x31, 0xa1,
    0x71, 0x7e, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02, 0x03, 0x48,
    0x00, 0x30, 0x45, 0x02, 0x21, 0x00, 0x96, 0xc9, 0xc8, 0xcf, 0x2e, 0x01, 0x88, 0x60, 0x05, 0xd8,
    0xf5, 0xbc, 0x72, 0xc0, 0x7b, 0x75, 0xfd, 0x9a, 0x57, 0x69, 0x5a, 0xc4, 0x91, 0x11, 0x31, 0x13,
    0x8b, 0xea, 0x03, 0x3c, 0xe5, 0x03, 0x02, 0x20, 0x25, 0x54, 0x94, 0x3b, 0xe5, 0x7d, 0x53, 0xd6,
    0xc4, 0x75, 0xf7, 0xd2, 0x3e, 0xbf, 0xcf, 0xc2, 0x03, 0x6c, 0xd2, 0x9b, 0xa6, 0x39, 0x3e, 0xc7,
    0xef, 0xad, 0x87, 0x14, 0xab, 0x71, 0x82, 0x19,
];

// credentials/test/attestation/Chip-Test-DAC-FFF1-8000-0004-Cert
const DAC_CERT: [u8; 494] = [
    0x30, 0x82, 0x01, 0xea, 0x30, 0x82, 0x01, 0x8f, 0xa0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x08, 0x0c,
    0x69, 0x4f, 0x7f, 0x86, 0x60, 0x67, 0xb2, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d,
    0x04, 0x03, 0x02, 0x30, 0x46, 0x31, 0x18, 0x30, 0x16, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x0f,
    0x4d, 0x61, 0x74, 0x74, 0x65, 0x72, 0x20, 0x54, 0x65, 0x73, 0x74, 0x20, 0x50, 0x41, 0x49, 0x31,
    0x14, 0x30, 0x12, 0x06, 0x0a, 0x2b, 0x06, 0x01, 0x04, 0x01, 0x82, 0xa2, 0x7c, 0x02, 0x01, 0x0c,
    0x04, 0x46, 0x46, 0x46, 0x31, 0x31, 0x14, 0x30, 0x12, 0x06, 0x0a, 0x2b, 0x06, 0x01, 0x04, 0x01,
    0x82, 0xa2, 0x7c, 0x02, 0x02, 0x0c, 0x04, 0x38, 0x30, 0x30, 0x30, 0x30, 0x20, 0x17, 0x0d, 0x32,
    0x31, 0x30, 0x36, 0x32, 0x38, 0x31, 0x34, 0x32, 0x33, 0x34, 0x33, 0x5a, 0x18, 0x0f, 0x39, 0x39,
    0x39, 0x39, 0x31, 0x32, 0x33, 0x31, 0x32, 0x33, 0x35, 0x39, 0x35, 0x39, 0x5a, 0x30, 0x4b, 0x31,
    0x1d, 0x30, 0x1b, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x14, 0x4d, 0x61, 0x74, 0x74, 0x65, 0x72,
    0x20, 0x54, 0x65, 0x73, 0x74, 0x20, 0x44, 0x41, 0x43, 0x20, 0x30, 0x30, 0x30, 0x34, 0x31, 0x14,
    0x30, 0x12, 0x06, 0x0a, 0x2b, 0x06, 0x01, 0x04, 0x01, 0x82, 0xa2, 0x7c, 0x02, 0x01, 0x0c, 0x04,
    0x46, 0x46, 0x46, 0x31, 0x31, 0x14, 0x30, 0x12, 0x06, 0x0a, 0x2b, 0x06, 0x01, 0x04, 0x01, 0x82,
    0xa2, 0x7c, 0x02, 0x02, 0x0c, 0x04, 0x38, 0x30, 0x30, 0x30, 0x30, 0x59, 0x30, 0x13, 0x06, 0x07,
    0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01,
    0x07, 0x03, 0x42, 0x00, 0x04, 0x7a, 0x84, 0x58, 0xaf, 0xbb, 0x9b, 0xcd, 0x15, 0xe1, 0x9a, 0xdc,
    0xd2, 0x66, 0xf6, 0x6c, 0x9c, 0x2f, 0x60, 0x7c, 0x74, 0x74, 0x7a, 0x35, 0xf8, 0x0f, 0x37, 0xe1,
    0x18, 0x13, 0x3f, 0x80, 0xf1, 0x76, 0x01, 0x13, 0x27, 0x8f, 0x91, 0xf1, 0x5a, 0xa0, 0xf7, 0xf8,
    0x79, 0x32, 0x09, 0x4f, 0xe6, 0x9f, 0xb7, 0x28, 0x68, 0xa8, 0x1e, 0x26, 0x97, 0x9b, 0x36, 0x8b,
    0x33, 0xb5, 0x54, 0x31, 0x03, 0xa3, 0x60, 0x30, 0x5e, 0x30, 0x0c, 0x06, 0x03, 0x55, 0x1d, 0x13,
    0x01, 0x01, 0xff, 0x04, 0x02, 0x30, 0x00, 0x30, 0x0e, 0x06, 0x03, 0x55, 0x1d, 0x0f, 0x01, 0x01,
    0xff, 0x04, 0x04, 0x03, 0x02, 0x07, 0x80, 0x30, 0x1d, 0x06, 0x03, 0x55, 0x1d, 0x0e, 0x04, 0x16,
    0x04, 0x14, 0xd5, 0xad, 0xb2, 0xb8, 0x83, 0x8e, 0xc8, 0x07, 0x3c, 0x47, 0x72, 0xdc, 0x7e, 0x87,
    0x97, 0xfe, 0xbb, 0x23, 0xb3, 0xae, 0x30, 0x1f, 0x06, 0x03, 0x55, 0x1d, 0x23, 0x04, 0x18, 0x30,
    0x16, 0x80, 0x14, 0xaf, 0x42, 0xb7, 0x09, 0x4d, 0xeb, 0xd5, 0x15, 0xec, 0x6e, 0xcf, 0x33, 0xb8,
    0x11, 0x15, 0x22, 0x5f, 0x32, 0x52, 0x88, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d,
    0x04, 0x03, 0x02, 0x03, 0x49, 0x00, 0x30, 0x46, 0x02, 0x21, 0x00, 0xf6, 0x47, 0x00, 0xa4, 0x17,
    0x4e, 0xe2, 0xa5, 0x11, 0x71, 0x43, 0x13, 0x67, 0xeb, 0x2c, 0x52, 0xbb, 0x78, 0xd3, 0xe0, 0xde,
    0xea, 0x96, 0xe7, 0xcf, 0x6a, 0x36, 0x96, 0xf0, 0xe5, 0xe7, 0xe5, 0x02, 0x21, 0x00, 0xbd, 0x56,
    0x27, 0xec, 0x4e, 0xe5, 0xca, 0x14, 0x31, 0x78, 0x06, 0x28, 0xf3, 0x1d, 0xc1, 0xe3, 0xd7, 0x1e,
    0xff, 0x25, 0x7f, 0x87, 0xb6, 0xa0, 0x08, 0x7e, 0x71, 0x6c, 0xbb, 0x60, 0x61, 0xbf,
];

const DAC_PUBKEY: [u8; 65] = [
    0x04, 0x7a, 0x84, 0x58, 0xaf, 0xbb, 0x9b, 0xcd, 0x15, 0xe1, 0x9a, 0xdc, 0xd2, 0x66, 0xf6, 0x6c,
    0x9c, 0x2f, 0x60, 0x7c, 0x74, 0x74, 0x7a, 0x35, 0xf8, 0x0f, 0x37, 0xe1, 0x18, 0x13, 0x3f, 0x80,
    0xf1, 0x76, 0x01, 0x13, 0x27, 0x8f, 0x91, 0xf1, 0x5a, 0xa0, 0xf7, 0xf8, 0x79, 0x32, 0x09, 0x4f,
    0xe6, 0x9f, 0xb7, 0x28, 0x68, 0xa8, 0x1e, 0x26, 0x97, 0x9b, 0x36, 0x8b, 0x33, 0xb5, 0x54, 0x31,
    0x03,
];

const DAC_PRIVKEY: [u8; 32] = [
    0x05, 0xc6, 0xc3, 0xa8, 0x4d, 0xc6, 0x05, 0xcc, 0x3c, 0xc8, 0x05, 0x80, 0x09, 0xb0, 0x1b, 0x32,
    0x9c, 0xf6, 0x0c, 0xf1, 0x59, 0x70, 0xc6, 0xa9, 0x0e, 0xad, 0xaa, 0xe2, 0xde, 0x49, 0x64, 0x9e,
];

//
const CERT_DECLARATION: [u8; 235] = [
    0x30, 0x81, 0xe8, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x02, 0xa0, 0x81,
    0xda, 0x30, 0x81, 0xd7, 0x02, 0x01, 0x03, 0x31, 0x0d, 0x30, 0x0b, 0x06, 0x09, 0x60, 0x86, 0x48,
    0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x30, 0x45, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d,
    0x01, 0x07, 0x01, 0xa0, 0x38, 0x04, 0x36, 0x15, 0x24, 0x00, 0x01, 0x25, 0x01, 0xf1, 0xff, 0x36,
    0x02, 0x05, 0x00, 0x80, 0x18, 0x25, 0x03, 0x34, 0x12, 0x2c, 0x04, 0x13, 0x5a, 0x49, 0x47, 0x32,
    0x30, 0x31, 0x34, 0x31, 0x5a, 0x42, 0x33, 0x33, 0x30, 0x30, 0x30, 0x31, 0x2d, 0x32, 0x34, 0x24,
    0x05, 0x00, 0x24, 0x06, 0x00, 0x25, 0x07, 0x94, 0x26, 0x24, 0x08, 0x00, 0x18, 0x31, 0x7c, 0x30,
    0x7a, 0x02, 0x01, 0x03, 0x80, 0x14, 0x62, 0xfa, 0x82, 0x33, 0x59, 0xac, 0xfa, 0xa9, 0x96, 0x3e,
    0x1c, 0xfa, 0x14, 0x0a, 0xdd, 0xf5, 0x04, 0xf3, 0x71, 0x60, 0x30, 0x0b, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d,
    0x04, 0x03, 0x02, 0x04, 0x46, 0x30, 0x44, 0x02, 0x20, 0x43, 0xa6, 0x3f, 0x2b, 0x94, 0x3d, 0xf3,
    0x3c, 0x38, 0xb3, 0xe0, 0x2f, 0xca, 0xa7, 0x5f, 0xe3, 0x53, 0x2a, 0xeb, 0xbf, 0x5e, 0x63, 0xf5,
    0xbb, 0xdb, 0xc0, 0xb1, 0xf0, 0x1d, 0x3c, 0x4f, 0x60, 0x02, 0x20, 0x4c, 0x1a, 0xbf, 0x5f, 0x18,
    0x07, 0xb8, 0x18, 0x94, 0xb1, 0x57, 0x6c, 0x47, 0xe4, 0x72, 0x4e, 0x4d, 0x96, 0x6c, 0x61, 0x2e,
    0xd3, 0xfa, 0x25, 0xc1, 0x18, 0xc3, 0xf2, 0xb3, 0xf9, 0x03, 0x69,
];

impl DevAttDataFetcher for HardCodedDevAtt {
    fn get_devatt_data(&self, data_type: DataType, data: &mut [u8]) -> Result<usize, Error> {
        let src = match data_type {
            DataType::CertDeclaration => &CERT_DECLARATION[..],
            DataType::PAI => &PAI_CERT[..],
            DataType::DAC => &DAC_CERT[..],
            DataType::DACPubKey => &DAC_PUBKEY[..],
            DataType::DACPrivKey => &DAC_PRIVKEY[..],
        };
        if src.len() <= data.len() {
            let data = &mut data[0..src.len()];
            data.copy_from_slice(src);
            Ok(src.len())
        } else {
            Err(Error::NoSpace)
        }
    }
}
