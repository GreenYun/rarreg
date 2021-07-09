use core::fmt;
use core::ops::Index;

use crc32fast::Hasher;

use super::tools::keygen;
use super::tools::sign::sign;

pub struct RegInfo {
    username: String,
    lic_type: String,
    uid: String,
    data: String,
}

impl RegInfo {
    pub fn new(username: String, lic_type: String) -> RegInfo {
        // Hardcoded Vec instead of re-generating every time
        let rar_priv_key = // keygen::gen_priv_key("".to_string());
        vec![
            0x5e, 0xd6, 0x61, 0xfd, 0xb7, 0x44, 0x90, 0xae, 0xc1, 0x50, 0x74, 0x46, 0x1f, 0xf1,
            0xb9, 0x5f, 0xa8, 0x1f, 0x27, 0x05, 0x01, 0x5f, 0xb9, 0xbd, 0x90, 0xca, 0xbc, 0x6a,
            0xfe, 0x59,
        ];

        let user_priv_key = keygen::gen_priv_key(username.to_owned());
        let user_pub_key = keygen::calc_pub_key(user_priv_key);
        let user_compressed_pub_key = keygen::compress_pub_key(user_pub_key);

        let tmp = format!("{:064x}", user_compressed_pub_key);
        let data3 = format!("60{}", tmp.index(0..48));

        let d3_priv_key = keygen::gen_priv_key(data3.clone());
        let d3_pub_key = keygen::calc_pub_key(d3_priv_key);
        let d3_compressed_pub_key = keygen::compress_pub_key(d3_pub_key);

        let data0 = format!("{:064x}", d3_compressed_pub_key);

        let uid = format!("{:}{:}", tmp.index(48..), data0.index(0..4));

        let (r, s) = sign(lic_type.clone(), rar_priv_key.clone());
        let data1 = format!("60{:060x}{:060x}", s, r);

        let tmp = format!("{}{}", username, data0);
        let (r, s) = sign(tmp, rar_priv_key);
        let data2 = format!("60{:060x}{:060x}", s, r);

        let tmp = format!(
            "{}{}{}{}{}{}",
            lic_type, username, data0, data1, data2, data3
        );

        let mut hasher = Hasher::new();
        hasher.update(tmp.as_bytes());
        let checksum = !hasher.finalize();
        let checksum = format!("{:010}", checksum);

        RegInfo {
            username,
            lic_type,
            uid,
            data: format!(
                "{}{}{}{}{}{}{}{}{}",
                data0.len(),
                data1.len(),
                data2.len(),
                data3.len(),
                data0,
                data1,
                data2,
                data3,
                checksum
            ),
        }
    }
}

impl fmt::Display for RegInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = [
            self.data.index(0..54),
            self.data.index(54..108),
            self.data.index(108..162),
            self.data.index(162..216),
            self.data.index(216..270),
            self.data.index(270..324),
            self.data.index(324..),
        ];

        write!(
            f,
            "RAR registration data
{}
{}
UID={}
{}
{}
{}
{}
{}
{}
{}",
            self.username,
            self.lic_type,
            self.uid,
            data[0],
            data[1],
            data[2],
            data[3],
            data[4],
            data[5],
            data[6],
        )?;

        Ok(())
    }
}
