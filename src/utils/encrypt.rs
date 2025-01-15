use std::convert::AsRef;
use std::fmt;
use std::num::NonZeroU32;

use argon2::{
    self, password_hash::rand_core::OsRng, password_hash::SaltString, Argon2, PasswordHasher};
use bcrypt::hash;
use rand::RngCore;
use ring::pbkdf2;
use tokio::task;

use crate::errors::{Error, HashPasswordError};

pub enum HashAlgorithm {
    Bcrypt,
    Argon2,
    Pbkdf2,
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HashAlgorithm::Bcrypt => "$2a$",
                HashAlgorithm::Argon2 => "$argon2i$v=19$m=65536,t=3,p=4$",
                HashAlgorithm::Pbkdf2 => "{PBKDF2}",
            }
        )
    }
}

pub async fn hash_password<P>(password: P, algorithm: HashAlgorithm) -> Result<String, Error>
where
    P: AsRef<str> + Send + 'static,
{
    match algorithm {
        HashAlgorithm::Bcrypt => {
            #[cfg(not(test))]
            let cost = bcrypt::DEFAULT_COST;
            #[cfg(test)]
            let cost = 4;
            task::spawn_blocking(move || {
                let hashed =
                    hash(password.as_ref(), cost).map_err(|err| Error::HashPassword{source: HashPasswordError::Bcrypt(err)})?;
                Ok(format!("{}{}", algorithm, hashed))
            })
            .await
            .map_err(Error::RunSyncTask)?
        }
        HashAlgorithm::Argon2 => {
            // 随机生成盐
            let salt = SaltString::generate(&mut OsRng);
            // 使用默认配置创建 Argon2 哈希器
            let argon2 = Argon2::default();
            // 哈希密码
            let result = task::spawn_blocking(move || {
                let hashed = argon2
                    .hash_password(password.as_ref().as_bytes(), &salt)
                    .map(|hash| hash.to_string())
                    .map_err(|err| Error::HashPassword{source:HashPasswordError::Argon2(err)})?;

                Ok(format!("{}{}{}", algorithm, salt.as_str(), hashed))
            })
            .await
            .map_err(Error::RunSyncTask)?;
            result
        }
        HashAlgorithm::Pbkdf2 => {
            // 生成盐
            let mut salt = [0u8; 16];
            OsRng.fill_bytes(&mut salt);
            // PBKDF2 配置
            let iterations = NonZeroU32::new(100_000).unwrap(); // 设置迭代次数
            let mut hash = [0u8; 32]; // PBKDF2 输出的哈希长度
                                      // pbkdf2::derive 没有返回值，它将结果写入传入的缓冲区。哈希计算完毕后，我们将其转换为十六进制字符串。
            let result = task::spawn_blocking(move || {
                pbkdf2::derive(
                    pbkdf2::PBKDF2_HMAC_SHA256,
                    iterations,
                    &salt,
                    password.as_ref().as_bytes(),
                    &mut hash,
                );
                // 将哈希值转换为十六进制字符串
                let hash_hex = hex::encode(hash);
                Ok(format!("{}{}:{}", algorithm, hex::encode(salt), hash_hex))
            })
            .await
            .map_err(Error::RunSyncTask)?;
            result
        }
    }
}

pub fn parse_algorithm_prefix(hashed_password: &str) -> Result<HashAlgorithm, Error> {
    if hashed_password.starts_with("$2a$") {
        Ok(HashAlgorithm::Bcrypt)
    } else if hashed_password.starts_with("$argon2i$") {
        Ok(HashAlgorithm::Argon2)
    } else if hashed_password.starts_with("{PBKDF2}") {
        Ok(HashAlgorithm::Pbkdf2)
    } else { 
        todo!()
        // Err(Error::HashPassword("Unknown algorithm prefix".into()))
    }
}

pub fn strip_algorithm_prefix(hashed_password: &str, algorithm: &HashAlgorithm) -> String {
    match algorithm {
        HashAlgorithm::Bcrypt => hashed_password
            .split("$2a$")
            .nth(1)
            .unwrap_or(hashed_password)
            .to_string(),
        HashAlgorithm::Argon2 => hashed_password
            .split("$argon2i$")
            .nth(1)
            .unwrap_or(hashed_password)
            .to_string(),
        HashAlgorithm::Pbkdf2 => {
            // hashed_password.split("{PBKDF2}").nth(1).unwrap_or(hashed_password).to_string()
            hashed_password
                .split("{PBKDF2}")
                .nth(1)
                .unwrap_or(hashed_password)
                .to_string()
        }
    }
}

pub async fn verify_password<P>(password: P, hashed_password: &str) -> Result<bool, Error>
where
    P: AsRef<str> + Send + 'static,
{
    let owned_password = hashed_password.to_owned(); // 将 &str 转换为 String
    let algorithm = parse_algorithm_prefix(&owned_password)?;
    let stripped_password = strip_algorithm_prefix(hashed_password, &algorithm);
    println!("{}", stripped_password);
    match algorithm {
        HashAlgorithm::Bcrypt => task::spawn_blocking(move || {
            bcrypt::verify(password.as_ref(), stripped_password.as_str())
                .map_err(|err| Error::HashPassword{source:HashPasswordError::Bcrypt(err)})
        })
        .await
        .map_err(Error::RunSyncTask)?,
        HashAlgorithm::Argon2 => {
            todo!()
        
        }
        HashAlgorithm::Pbkdf2 => {
            todo!()
            /*let parts: Vec<&str> = stripped_password.split(':').collect();
            let salt = hex::decode(parts.get(0).ok_or("Invalid format")?)?;
            let hash = *parts.get(1).ok_or("Invalid format")?;
            let iterations = NonZeroU32::new(100_000).unwrap();
            let mut hash_buf = [0u8; 32];
            pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA256, iterations, &salt, password.as_bytes(), &mut hash_buf);
            let is_valid = hex::encode(hash_buf).as_str() == hash;
            Ok(is_valid)*/
        }
    }
}

#[cfg(test)]
mod tests {
    use argon2::PasswordVerifier;
    use tokio::runtime::Runtime;

    use super::*;

    #[test]
    fn test_bcrypt_hash_password() {
        let rt = Runtime::new().unwrap();
        let password = "password123";
        let result = rt.block_on(hash_password(password, HashAlgorithm::Bcrypt));
        assert!(result.is_ok());
        let hashed_password = result.unwrap();
        println!("Bcrypt hashed password: {}", hashed_password);
        assert!(!hashed_password.is_empty());
    }

    #[test]
    fn test_argon2_hash_password() {
        let rt = Runtime::new().unwrap();
        let password = "password123";
        let result = rt.block_on(hash_password(password, HashAlgorithm::Argon2));
        assert!(result.is_ok());
        let hashed_password = result.unwrap();
        println!("Argon2 hashed password: {}", hashed_password);
        assert!(!hashed_password.is_empty());
    }

    #[test]
    fn test_pbkdf2_hash_password() {
        let rt = Runtime::new().unwrap();
        let password = "password123";
        let result = rt.block_on(hash_password(password, HashAlgorithm::Pbkdf2));
        assert!(result.is_ok());
        let hashed_password = result.unwrap();
        println!("PBKDF2 hashed password: {}", hashed_password);
        assert!(!hashed_password.is_empty());
    }

    #[test]
    fn test_bcrypt_verify_password() {
        let rt = Runtime::new().unwrap();
        let password = "password123";
        let result = rt
            .block_on(hash_password(password, HashAlgorithm::Bcrypt))
            .unwrap();
        let is_valid = rt
            .block_on(verify_password(password, result.as_str()))
            .expect("Failed to verify password");
        assert!(is_valid);
        let is_valid = rt
            .block_on(verify_password("wrong password", result.as_str()))
            .expect("Failed to verify password");
        assert!(!is_valid);
    }

    #[test]
    fn test_argon2_verify_password() {
        // let rt = Runtime::new().unwrap();
        let password = "password123";
        // 创建Argon2实例
        let argon2 = Argon2::default();
        // 生成随机盐
        let salt = SaltString::generate(&mut rand::thread_rng());
        // 哈希密码
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();
        println!("Generated Argon2 hash{}", hash);

        // Create a new Argon2 instance and verify
        let parsed_hash =
            argon2::password_hash::PasswordHash::new(&hash).expect("Failed to parse hash");

        // 验证密码（正确密码）
        let is_valid = argon2.verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();
        assert!(is_valid);
        println!("Password verified successfully!");

        // 验证密码（错误密码）
        let is_invalid = argon2
            .verify_password("wrong password".as_bytes(), &parsed_hash)
            .is_err();
        assert!(is_invalid);
        println!("Incorrect password verification failed as expected!");
    }

    #[test]
    fn test_pbkdf2_verify_password() {
        let password = "password123";
        let mut salt = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);
        let salt_hex = hex::encode(&salt);
        let iterations = NonZeroU32::new(100_000).unwrap();
        let mut hash = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            &salt,
            password.as_bytes(),
            &mut hash,
        );
        let hash_hex = hex::encode(hash);

        // Test with the correct password
        let mut hash_buf = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            &salt,
            password.as_bytes(),
            &mut hash_buf,
        );
        let hash_hex_test = hex::encode(hash_buf);
        assert_eq!(hash_hex, hash_hex_test);

        // Test with incorrect password
        let mut hash_buf = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            &salt,
            "wrong password".as_bytes(),
            &mut hash_buf,
        );
        let wrong_hash_hex = hex::encode(hash_buf);
        assert_ne!(hash_hex, wrong_hash_hex);
    }
}
