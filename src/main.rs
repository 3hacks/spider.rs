use ipfs_api::IpfsClient;
use ipfs_api::TryFromUri;
use http::Uri;
use rsa::{PublicKey, RSAPublicKey, PaddingScheme};
use std::io::Cursor;
use base64;
use rand::rngs::OsRng;

const PUBKEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEArIEzTAaYFFzK75knVvi+
+RqpbDx/iOGtgH6Ott6Bowx9wxN0IerGFbNei/o6a8vrd0RuwcgtYhqstaYq6jyc
fJ+AxOzeO0dikhPJSE+86NtYAfHCm5VgHK8Bq4CtHLQMet9TrDgNkMVs3pi1PiET
OpaCp+Cr0WgR4qg2jeD1/DOpLsxjvQ+C2U1myHvVzyC/zB3QaJglLChf0J2oSE6p
3XzEjCXL4QESom2qbJqueyVLWByJIIgH5Vyk0LI9iMuuAUbtVUtBAK1NYUafSRKl
d3SH/ygnqBKByID435qVYi9G7HZHHQ4bQtSaluLXnOc6UeASoPgYH7+hz4VKswXK
fwIDAQAB
-----END PUBLIC KEY-----"#;

fn encrypt(filename: &str) -> Vec<u8> {
    let pubkey = create_pubkey();
    let mut rng = OsRng;
    //let mut file = File::open(filename).expect("Failed to open the file");
    //let mut buf = Vec::new();
    //let _ = file.read_to_end(&mut buf).expect("Failed to load the file");
    let buf = std::fs::read(filename).expect("Failed to load the file");
    let enc_data = pubkey.encrypt(&mut rng, PaddingScheme::PKCS1v15Encrypt, &buf).expect("Failed to encrypt the data");
    return enc_data;
}
fn create_pubkey() -> RSAPublicKey {
    let der_encoded = PUBKEY
        .lines()
        .filter(|line| !line.starts_with("-"))
        .fold(String::new(), |mut data, line| {
            data.push_str(&line);
            data
        });
    let der_bytes = base64::decode(&der_encoded).expect("failed to decode base64 content");
    let public_key = RSAPublicKey::from_pkcs8(&der_bytes).expect("failed to parse key");
    return public_key;
}

async fn send_to_ipfs(filename: &str) -> String {   
    let uri = "https://ipfs.infura.io:5001/api/v0".parse::<Uri>().unwrap();
    let client = IpfsClient::build_with_base_uri(uri);
    let enc_data = encrypt(filename);
    return client.add(Cursor::new(enc_data)).await.expect("Failed to add file to IPFS").hash;
}
#[tokio::main]
async fn main() {
    println!("{}", send_to_ipfs("hello.txt").await);
}
