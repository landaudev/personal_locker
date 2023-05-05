#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write, SeekFrom, Seek, BufWriter};
use std::path::Path;
use serde_json::json;
use serde::{Deserialize, Serialize};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce // Or `Aes128Gcm`
};
use hex;
use sha2::{Sha256, Digest};
use generic_array::GenericArray;
use generic_array::typenum::U32;
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    data: String,
    file_name: String,
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_pass,create_password,chek_password,write_data, create_file_user, delete_file_user])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}

#[tauri::command]
fn delete_file_user(file_name: &str) -> String {
    // Open data.json and read its contents
    let path = Path::new("data.json");
    let file = OpenOptions::new().read(true).write(true).create(true).open(path).expect("Could not open file");
    let reader = BufReader::new(&file);

    // Deserialize the contents of data.json and retrieve the array of stored files
    let data: serde_json::Value = serde_json::from_reader(reader).expect("Could not parse JSON");
    let mut array = data.as_array().unwrap().clone();

    // Encrypt the file name using the user's password hash and remove the corresponding file from the array
    let pass = get_pass();
    let file_name = hex::encode(encrypt(file_name, &password_to_hash(&pass))).to_string();
    array.retain(|item| item["file_name"].as_str() != Some(&file_name));

    // Write the updated array of files to data.json and return its contents
    let file = OpenOptions::new().write(true).truncate(true).open(path).expect("Could not open file");
    serde_json::to_writer_pretty(file, &array).expect("Could not write JSON to file");

    read_file_data("data.json")
}

#[tauri::command]
fn create_file_user(file_name: &str) -> String {
    let path = Path::new("data.json");
    let file = OpenOptions::new().read(true).write(true).create(true).open(path).expect("Could not open file");
    let mut reader = BufReader::new(&file);
    let mut records: Vec<Record> = Vec::new();

    // Try to read the existing records from the file
    match serde_json::from_reader(&mut reader) {
        Ok(existing_records) => records = existing_records,
        Err(_) => println!("No existing records found in file"),
    }

    let pass = get_pass();
    let file_name = hex::encode(encrypt(file_name, &password_to_hash(&pass))).to_string();
    // Create a new record
    let new_record = Record {
        data: "".to_owned(),
        file_name: file_name.to_owned(),
    };

    // Add the new record to the records vector
    records.push(new_record);

    // Reset the file position to the beginning
    reader.seek(SeekFrom::Start(0)).expect("Could not seek reader");

    // Serialize the updated records vector back to JSON
    let serialized = serde_json::to_string(&records).expect("Could not serialize data");

    // Write the serialized JSON back to the file
    let mut writer = BufWriter::new(&file);
    writer.write_all(serialized.as_bytes()).expect("Could not write file");
    writer.flush().expect("Could not flush writer");

    read_file_data("data.json")

}

#[tauri::command]
fn write_data(data: &str) {
    let pass = get_pass();

    // Open the file in read-write mode, create a new file if it does not already exist
    let path = Path::new("data.json");
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    // Parse the json data as a serde_json::Value
    let mut json_data: serde_json::Value = serde_json::from_str(data).unwrap();

    // Encrypt the "data" and "file_name" fields in each element of the json array
    if let serde_json::Value::Array(ref mut array) = json_data {
        for item in array.iter_mut() {
            if let Some(data_value) = item.get_mut("data") {
                if let Some(data_str) = data_value.as_str() {
                    *data_value = JsonValue::String(hex::encode(encrypt(data_str, &password_to_hash(&pass))).to_string());
                }
            }
            if let Some(file_name_value) = item.get_mut("file_name") {
                if let Some(file_name_str) = file_name_value.as_str() {
                    *file_name_value = JsonValue::String(hex::encode(encrypt(file_name_str, &password_to_hash(&pass))).to_string());
                }
            }
        }
    }

    // Write the encrypted json data to the file
    match file.write_all(json_data.to_string().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
        Ok(_) => println!("successfully wrote to {}", path.display()),
    }
}

fn chek_file_return_data() -> String {
    // Create file data and get status message
    let status_data = create_file_data("data.json");

    // Check status message and act accordingly
    match status_data.as_str() {
        "File data was created." => {
            let data = read_file_data("data.json");
            return data;
        },
        "File data exists." => {
            let data = read_file_data("data.json");
            return data;
        },
        _ => return "File data was not created".to_string(),
    }

}

fn read_file_data(file_path: &str) -> String {
     // Open the file and create a file object
    let path = Path::new(file_path);
    let file = File::open(&path).expect("no such file");
    let reader = BufReader::new(file);
    let mut data = String::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        data.push_str(&line);
    }

    // If no data was read, return an empty string
    if data.is_empty() {
        return String::new();
    }

    // Parse the read data as JSON into a vector of Record structs
    let encrypted_data: Vec<Record> = serde_json::from_str(&data).unwrap();
    let pass = get_pass();
    let mut decrypted_data = Vec::new();

    // Iterate through each Record struct in the encrypted data vector
    for record in encrypted_data {
        let decrypted_record = Record {
            data: if record.data.is_empty() {
                "".to_string()
            } else {
                decrypt(&hex::decode(record.data).unwrap(), &password_to_hash(&pass))
            },
            file_name: decrypt(&hex::decode(record.file_name).unwrap(), &password_to_hash(&pass)),
        };
        decrypted_data.push(decrypted_record);
    }

    return serde_json::to_string(&decrypted_data).unwrap();
}

// This function creates a new file at the specified path with no data, if the file doesn't already exist.
// If the file already exists, it returns a message indicating that the file data already exists.
// If the file cannot be created, it returns a message indicating that the file data was not created.
fn create_file_data(file_path: &str) -> String {
    let path = Path::new(file_path);
    if !path.exists() {
        match File::create(&path) {
            Ok(_) => return "File data was created.".to_string(),
            Err(_e) => return "File data was not created".to_string(),
        }
    }
    return "File data exists.".to_string();
}

#[tauri::command]
fn chek_password(password: &str) -> String {
    let pass = password_to_hash(password);

    // Read password file
    let contents = fs::read_to_string("ps.json").expect("Something went wrong reading the file");
    let data: serde_json::Value = serde_json::from_str(&contents).unwrap();

    // Compare hashed password with password from file
    let passoword_file = data["password_login"].as_str().unwrap();
    if pass == passoword_file {
        return chek_file_return_data();
    } else {
        return "Password incorrect.".to_string()
    }
}

#[tauri::command]
fn get_pass() -> String {
    // First, we check if the file with the password exists. If it doesn't, we print an error message and return a default string.
    if let Err(e) = create_file_chek_pass("ps.json") {
        println!("Error: {}", e);
        return "Password does not exist.".to_string();
    // If the file exists, we return a message indicating that the password is available.
    } else {
        return "Password exists.".to_string();
    }
}

#[tauri::command]
fn create_file_chek_pass(file_path: &str) -> Result<(), Error> {
    let path = Path::new(file_path);

    // Check if the file does not already exist, then create it
    if !path.exists() {
        match File::create(&path) {
            Ok(_) => println!("The file pass was created successfully."),
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        }
    }

    // Open the file in the given path and return an error if it fails
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    // Read each line of the opened file and look for the password_login key in JSON format
    let mut password = String::new();
    for line in reader.lines() {
        let line = line?;
        let data: Result<serde_json::Value, _> = serde_json::from_str(&line);
        if let Ok(json) = data {
            if let Some(password_value) = json.get("password_login").and_then(|v| v.as_str()) {
                password = password_value.to_string();
                break;
            }
        }
    }
    if password.is_empty() {
        return Err(Error::new(ErrorKind::NotFound, "Password not found"));
    }
    Ok(())
}

#[tauri::command]
fn create_password(password: &str) -> String {
    // Hash the input password using a hash function.
    let pass = password_to_hash(password);

    // Define a new file path for the output file.
    let path = Path::new("ps.json");

    // Open the output file in write-only mode.
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why), // Handle any errors that occur while creating the file.
        Ok(file) => file, // Return the opened file handle if successful.
    };

    // Create a JSON object with the hashed password.
    let json = json!({"password_login": pass});

    // Write the JSON object to the output file.
    match file.write_all(json.to_string().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why), // Handle any errors that occur while writing to the file.
        Ok(_) => println!("successfully wrote to {}", path.display()), // Print a success message if the write operation is successful.
    }
    "Password exists.".to_string() // Return a success message indicating that the password has been created.
}

fn password_to_hash(pass: &str) -> String {
    // Create a new SHA-256 hasher instance.
    let mut hasher = Sha256::new();
    // Update the hasher with the password bytes.
    hasher.update(pass);
    // Finalize the hashing process and retrieve the raw hash bytes.
    let hash_result = hasher.finalize();
    // Convert the raw hash bytes to a hexadecimal string representation.
    let hash_string = format!("{:x}", hash_result);
    // Return the resulting hash string.
    hash_string
}

fn encrypt(plaintext: &str, pass: &str) -> Vec<u8> {
    // Convert password to bytes
    let key_bytes = pass.as_bytes();

    // Truncate or pad key_bytes to a length of 32 bytes to match the 256-bit key size for AES-256-GCM
    let mut key: [u8; 32] = [0; 32];
    key.clone_from_slice(&key_bytes[..std::cmp::min(key_bytes.len(), 32)]);

    // Convert 32-byte key into a generic array for AES encryption
    let key_arr = GenericArray::<u8, U32>::from_exact_iter(key.iter().cloned()).unwrap();

    // Create an AES-256-GCM cipher instance with the given key
    let cipher = Aes256Gcm::new(&key_arr);

    // Generate a unique 96-bit nonce for this message
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    // Encrypt the plaintext using AES-256-GCM and the generated nonce
    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).unwrap();
    ciphertext
}

fn decrypt(ciphertext: &[u8], pass: &str) -> String {
    // Convert password to bytes
    let key_bytes = pass.as_bytes();

    // Truncate or pad key_bytes to be 32 bytes long
    let mut key: [u8; 32] = [0; 32];
    key.clone_from_slice(&key_bytes[..std::cmp::min(key_bytes.len(), 32)]);

    // Convert key bytes into a GenericArray for use with AES256-GCM
    let key_arr = GenericArray::<u8, U32>::from_exact_iter(key.iter().cloned()).unwrap();

    // Create an AES256-GCM cipher using the key
    let cipher = Aes256Gcm::new(&key_arr);

    // Create a unique nonce for the message (nonce must be 96 bits)
    let nonce = Nonce::from_slice(b"unique nonce");

    // Decrypt the ciphertext using the cipher and nonce
    let plaintext = cipher.decrypt(nonce, ciphertext).unwrap();

    // Convert the decrypted plaintext to a String and return it
    String::from_utf8(plaintext).unwrap()
}