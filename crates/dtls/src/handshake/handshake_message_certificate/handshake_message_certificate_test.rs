use super::*;

use std::io::{BufReader, BufWriter};

#[test]
fn test_handshake_message_certificate() -> Result<(), Error> {
    let raw_certificate = vec![
        0x00, 0x01, 0x8c, 0x00, 0x01, 0x89, 0x30, 0x82, 0x01, 0x85, 0x30, 0x82, 0x01, 0x2b, 0x02,
        0x14, 0x7d, 0x00, 0xcf, 0x07, 0xfc, 0xe2, 0xb6, 0xb8, 0x3f, 0x72, 0xeb, 0x11, 0x36, 0x1b,
        0xf6, 0x39, 0xf1, 0x3c, 0x33, 0x41, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d,
        0x04, 0x03, 0x02, 0x30, 0x45, 0x31, 0x0b, 0x30, 0x09, 0x06, 0x03, 0x55, 0x04, 0x06, 0x13,
        0x02, 0x41, 0x55, 0x31, 0x13, 0x30, 0x11, 0x06, 0x03, 0x55, 0x04, 0x08, 0x0c, 0x0a, 0x53,
        0x6f, 0x6d, 0x65, 0x2d, 0x53, 0x74, 0x61, 0x74, 0x65, 0x31, 0x21, 0x30, 0x1f, 0x06, 0x03,
        0x55, 0x04, 0x0a, 0x0c, 0x18, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x65, 0x74, 0x20, 0x57,
        0x69, 0x64, 0x67, 0x69, 0x74, 0x73, 0x20, 0x50, 0x74, 0x79, 0x20, 0x4c, 0x74, 0x64, 0x30,
        0x1e, 0x17, 0x0d, 0x31, 0x38, 0x31, 0x30, 0x32, 0x35, 0x30, 0x38, 0x35, 0x31, 0x31, 0x32,
        0x5a, 0x17, 0x0d, 0x31, 0x39, 0x31, 0x30, 0x32, 0x35, 0x30, 0x38, 0x35, 0x31, 0x31, 0x32,
        0x5a, 0x30, 0x45, 0x31, 0x0b, 0x30, 0x09, 0x06, 0x03, 0x55, 0x04, 0x06, 0x13, 0x02, 0x41,
        0x55, 0x31, 0x13, 0x30, 0x11, 0x06, 0x03, 0x55, 0x04, 0x08, 0x0c, 0x0a, 0x53, 0x6f, 0x6d,
        0x65, 0x2d, 0x53, 0x74, 0x61, 0x74, 0x65, 0x31, 0x21, 0x30, 0x1f, 0x06, 0x03, 0x55, 0x04,
        0x0a, 0x0c, 0x18, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x65, 0x74, 0x20, 0x57, 0x69, 0x64,
        0x67, 0x69, 0x74, 0x73, 0x20, 0x50, 0x74, 0x79, 0x20, 0x4c, 0x74, 0x64, 0x30, 0x59, 0x30,
        0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48,
        0xce, 0x3d, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00, 0x04, 0xf9, 0xb1, 0x62, 0xd6, 0x07, 0xae,
        0xc3, 0x36, 0x34, 0xf5, 0xa3, 0x09, 0x39, 0x86, 0xe7, 0x3b, 0x59, 0xf7, 0x4a, 0x1d, 0xf4,
        0x97, 0x4f, 0x91, 0x40, 0x56, 0x1b, 0x3d, 0x6c, 0x5a, 0x38, 0x10, 0x15, 0x58, 0xf5, 0xa4,
        0xcc, 0xdf, 0xd5, 0xf5, 0x4a, 0x35, 0x40, 0x0f, 0x9f, 0x54, 0xb7, 0xe9, 0xe2, 0xae, 0x63,
        0x83, 0x6a, 0x4c, 0xfc, 0xc2, 0x5f, 0x78, 0xa0, 0xbb, 0x46, 0x54, 0xa4, 0xda, 0x30, 0x0a,
        0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02, 0x03, 0x48, 0x00, 0x30, 0x45,
        0x02, 0x20, 0x47, 0x1a, 0x5f, 0x58, 0x2a, 0x74, 0x33, 0x6d, 0xed, 0xac, 0x37, 0x21, 0xfa,
        0x76, 0x5a, 0x4d, 0x78, 0x68, 0x1a, 0xdd, 0x80, 0xa4, 0xd4, 0xb7, 0x7f, 0x7d, 0x78, 0xb3,
        0xfb, 0xf3, 0x95, 0xfb, 0x02, 0x21, 0x00, 0xc0, 0x73, 0x30, 0xda, 0x2b, 0xc0, 0x0c, 0x9e,
        0xb2, 0x25, 0x0d, 0x46, 0xb0, 0xbc, 0x66, 0x7f, 0x71, 0x66, 0xbf, 0x16, 0xb3, 0x80, 0x78,
        0xd0, 0x0c, 0xef, 0xcc, 0xf5, 0xc1, 0x15, 0x0f, 0x58,
    ];

    let mut reader = BufReader::new(raw_certificate.as_slice());
    let c = HandshakeMessageCertificate::unmarshal(&mut reader)?;
    //TODO: add x509 parse
    //  certificate, err := x509.ParseCertificate(c.certificate[0])
    //  if err != nil {
    //  	t.Error(err)
    //  }
    //  copyCertificatePrivateMembers(certificate, parsedCertificate)
    //  if !reflect.DeepEqual(certificate, parsedCertificate) {
    //  	t.Errorf("handshakeMessageCertificate unmarshal: got %#v, want %#v", c, parsedCertificate)
    //  }

    let mut raw = vec![];
    {
        let mut writer = BufWriter::<&mut Vec<u8>>::new(raw.as_mut());
        c.marshal(&mut writer)?;
    }
    assert_eq!(
        raw, raw_certificate,
        "handshakeMessageCertificate marshal: got {:?}, want {:?}",
        raw, raw_certificate
    );

    Ok(())
}

#[test]
fn test_empty_handshake_message_certificate() -> Result<(), Error> {
    let raw_certificate = vec![0x00, 0x00, 0x00];

    let expected_certificate = HandshakeMessageCertificate {
        certificate: vec![],
    };

    let mut reader = BufReader::new(raw_certificate.as_slice());
    let c = HandshakeMessageCertificate::unmarshal(&mut reader)?;

    assert_eq!(
        c, expected_certificate,
        "handshakeMessageCertificate unmarshal: got {:?}, want {:?}",
        c, expected_certificate,
    );

    Ok(())
}
