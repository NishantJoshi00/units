#[allow(warnings)]
mod bindings;

use bindings::{component::units::{http, storage}, exports::component::units::driver};
use std::error::Error;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use urlencoding::encode;


struct Component;

impl driver::Guest for Component {
    fn intend(input: String) -> Result<String, driver::DriverError> {
        let mut callback_details: CallabckInfo = serde_json::from_str(&input).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?;
        let config = Config::new();
        verify_vpa(&config, &mut callback_details).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?;
        let callback_details_str = serde_json::to_string(&callback_details)
        .map_err(|e| driver::DriverError::SystemError(e.to_string()))?;
        Ok(callback_details_str)
    }

    fn done(_input: String) -> Result<(), driver::DriverError> {
        Ok(())
    }

    fn transfer(fro: String, to: String, value: String) -> Result<(), driver::DriverError> {
        let mut from_acc = serde_json::from_str::<CallabckInfo>(&fro).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?;
        let mut to_acc = serde_json::from_str::<CallabckInfo>(&to).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?;
        let diff = serde_json::from_str::<Data>(&value).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?.amount;

        let config = Config::new();
        send_money_p2p(&config, &mut from_acc, &mut to_acc,&diff.to_string()).map_err(|e| {
            driver::DriverError::SystemError(e.to_string())
        })?;
        Ok(())
    }

    fn view(input: String) -> Result<String, driver::DriverError> {
        let mut callback_details: CallabckInfo = serde_json::from_str(&input).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?;
        let config = Config::new();
        check_balance(&config, &mut callback_details).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?;
        let callback_details_str = serde_json::to_string(&callback_details)
        .map_err(|e| driver::DriverError::SystemError(e.to_string()))?;
    
        Ok(callback_details_str)
    }

    fn bind(input: String, _existing: Option<String>) -> Result<String, driver::DriverError> {
        let mut callback_details: CallabckInfo = serde_json::from_str(&input).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?;
        let config=Config::new();
    
        get_sms_token(&config,&mut callback_details).map_err(|e| {
            driver::DriverError::SystemError(e.to_string())
        })?;

        sms_proxy_callback(&config, &callback_details).map_err(|e| {
            driver::DriverError::SystemError(e.to_string())
        })?;

        bind_device(&config, &mut callback_details).map_err(|e| {
            driver::DriverError::SystemError(e.to_string())
        })?;

        get_npci_token(&config, &mut callback_details).map_err(|e| {
            driver::DriverError::SystemError(e.to_string())
        })?;

        let mut acc_and_unique_id_pair: HashMap<String, String> = HashMap::new();
        let type_prefix = "test";
        fetch_accounts(&config,&mut callback_details,&mut acc_and_unique_id_pair,&type_prefix).map_err(|e| {
            driver::DriverError::SystemError(e.to_string())
        })?;
        add_accounts(&config, &mut callback_details, &mut acc_and_unique_id_pair, &type_prefix).map_err(|e| {
            driver::DriverError::SystemError(e.to_string())
        })?;
        
        let callback_details_str = serde_json::to_string(&callback_details)
        .map_err(|e| driver::DriverError::SystemError(e.to_string()))?;
    
        Ok(callback_details_str)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Data {
    amount: u64,
}

#[warn(non_snake_case)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct CallabckInfo{
    merchantCustomerId: String,
    raw_message: String,
    device_id: String,
    num: String,
    mobile_num: String,
    device_finger_print: String,
    upi_request_id: String,
    bank_account_unique_id: String,
    customer_vpa: String,
    balance: String,
}

// Configuration struct to hold environment variables
struct Config {
    host: String,
    uri: String,
    merchant_id: String,
    channel_id: String,
    sandbox_id: String,
    bank_code: String,
}

// Implement a method to create default configuration
impl Config {
    fn new() -> Self {
        Config {
            host: format!("http://{}:8030", env!("UPI_HOST")),
            uri: "n8".to_string(),
            merchant_id: "INTERNALTESTUAT".to_string(),
            channel_id: "INTERNALTESTUATAPP".to_string(),
            sandbox_id: "YESBACQ".to_string(),
            bank_code: "500004".to_string(),
        }
    }
}


// Get SMS Token
#[derive(Serialize, Deserialize, Debug)]
#[warn(non_snake_case)]
struct SmsTokenRequest {
    merchantCustomerId: String,
    provider: String,
    deviceId: String,
    udfParameters: String,
}
fn get_sms_token(config: &Config,callback_details:&mut CallabckInfo) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/api/{}/merchants/customer/getSmsToken", config.host, config.uri);
    
    let device_id = format!("test-device-{}", Uuid::new_v4().to_string()[..6].to_lowercase());
    
    let request_body = SmsTokenRequest {
        merchantCustomerId: format!("FLP.test.{}",device_id.clone()),
        provider: "jio".to_string(),
        deviceId: device_id.clone(),
        udfParameters: "{}".to_string(),
    };

    let request = http::Request {
        method: http::Method::Post,
        url: url.to_string(),
        headers: vec![
            ("x-sandbox-id".to_string(), config.sandbox_id.to_string()),
            ("x-merchant-channel-id".to_string(), config.channel_id.to_string()),
            ("x-merchant-id".to_string(), config.merchant_id.to_string()),
            ("Content-Type".to_string(),"application/json".to_string()),
        ],
        body :Some(serde_json::to_string(&request_body).expect("Failed to serialize request body")),
    };
    let response = http::send_request(&request);

    let body = response.body;

    let parsed: Value = serde_json::from_str(&body).expect("Failed to parse JSON");

    callback_details.device_id=device_id.clone();
    callback_details.merchantCustomerId=format!("FLP.test.{}",device_id.clone());

    // Extract `smsContent` from the JSON response
    if let Some(sms_content) = parsed["payload"]["smsContent"].as_str() {
        callback_details.raw_message = sms_content.to_string();
        // println!("{}",sms_content.to_string());
    } else {
        eprintln!("smsContent not found in the response.");
    }
    Ok(())
}

#[derive(Serialize)]
struct TokenRequest {
    merchantCustomerId: String,
    upiRequestId: String,
    deviceFingerPrint: String,
    tokenRequestType: String,
    tokenChallenge: String,
    udfParameters: String,
}

fn get_npci_token(config: &Config, callback_details: &mut CallabckInfo) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/api/{}/merchants/npci/token", config.host, config.uri);
    
    let upi_request_id = format!("MUL{}", Uuid::new_v4().to_string().replace("-", ""))
        .chars()
        .take(35)
        .collect::<String>();

    let request_body = TokenRequest {
        merchantCustomerId: callback_details.merchantCustomerId.clone(),
        upiRequestId: upi_request_id.clone(),
        deviceFingerPrint: callback_details.device_finger_print.clone(),
        tokenRequestType: "initial".to_string(),
        tokenChallenge: "DUMMYL2tCqBgHYcmvWPHgojpbSEQ==".to_string(),
        udfParameters: "{}".to_string(),
    };

    let request = http::Request {
        method: http::Method::Post,
        url: url.clone(),
        headers: vec![
            ("x-sandbox-id".to_string(), config.sandbox_id.clone()),
            ("x-merchant-channel-id".to_string(), config.channel_id.clone()),
            ("x-merchant-id".to_string(), config.merchant_id.clone()),
            ("Content-Type".to_string(), "application/json".to_string()),
        ],
        body: Some(serde_json::to_string(&request_body).expect("Failed to serialize request body")),
    };

    let response = http::send_request(&request);
    let body = response.body;
    callback_details.upi_request_id = upi_request_id;
    
    Ok(())
}

#[derive(Serialize)]
struct BindDeviceRequest {
    merchantCustomerId: String,
    smsContent: String,
    deviceId: String,
    manufacturer: String,
    model: String,
    version: String,
    os: String,
    ssid: String,
    mobileNumber: String,
    deregisterOldCustomer: String,
    packageName: String,
    udfParameters: String,
}
fn bind_device(config: &Config, callback_details: &mut CallabckInfo) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/api/{}/merchants/customer/bindDevice", config.host, config.uri);
    
    let request_body = BindDeviceRequest {
        merchantCustomerId: callback_details.merchantCustomerId.clone(),
        smsContent: callback_details.raw_message.clone(),
        deviceId: callback_details.device_id.clone(),
        manufacturer: "Samsu".to_string(),
        model: "Galaxy".to_string(),
        version: "11".to_string(),
        os: "ANDROID".to_string(),
        ssid: "2336592311".to_string(),
        mobileNumber: "919503008141".to_string(),
        deregisterOldCustomer: "true".to_string(),
        packageName: "com.juspay.in".to_string(),
        udfParameters: "{}".to_string(),
    };

    let request = http::Request {
        method: http::Method::Post,
        url: url.clone(),
        headers: vec![
            ("x-sandbox-id".to_string(), config.sandbox_id.clone()),
            ("x-merchant-channel-id".to_string(), config.channel_id.clone()),
            ("x-merchant-id".to_string(), config.merchant_id.clone()),
            ("Content-Type".to_string(), "application/json".to_string()),
        ],
        body: Some(serde_json::to_string(&request_body).expect("Failed to serialize request body")),
    };

    let response = http::send_request(&request);
    let body = response.body;

    let parsed: Value = serde_json::from_str(&body).expect("Failed to parse JSON");

    if let Some(device_finger_print) = parsed["payload"]["deviceFingerPrint"].as_str() {
        callback_details.device_finger_print = device_finger_print.to_string();
    } else {
       
    }

    Ok(())
}

fn sms_proxy_callback(config: &Config, callback_details: &CallabckInfo) -> Result<(), Box<dyn Error>> {
    let sms_content = encode(&callback_details.raw_message);
    let url = format!(
        "{}/api/{}/sms/inbound?Rawmessage={}&Send=919503008141",
        config.host, config.uri,sms_content
    );

    let request = http::Request {
        method: http::Method::Get,
        url: url.to_string(),
        headers: vec![
            ("x-sandbox-id".to_string(), config.sandbox_id.clone()),
            ("x-inbound-sms-gateway".to_string(), "TRUE".to_string()),
            ("Content-Type".to_string(), "application/json".to_string()),
        ],
        body: None,
    };

    let response = http::send_request(&request);
    Ok(())
}


fn process_response(response_body: &str, env: &mut HashMap<String, String>, type_prefix: &str) {
    // Parse the JSON response
    let parsed: Value = serde_json::from_str(response_body).expect("Failed to parse JSON");

    if let Some(accounts) = parsed["payload"]["accounts"].as_array() {
        let mut total_acc=0;

        for (index, account) in accounts.iter().enumerate() {
            if let Some(bank_account_unique_id) = account["bankAccountUniqueId"].as_str() {
                let ind = index + 1;
                let key = format!("{}-bankAccountUniqueId-{}", type_prefix, ind);
                env.insert(key.clone(), bank_account_unique_id.to_string());
                total_acc+=1;
            }
        }

        let number_of_accounts_key = format!("{}-numberOfAccounts", type_prefix);
        env.insert(number_of_accounts_key, total_acc.to_string());
    } else {
        eprintln!("Accounts array not found in the response.");
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FetchAccountsRequest {
    merchantCustomerId: String,
    upiRequestId: String,
    deviceFingerPrint: String,
    bankCode: String,
    udfParameters: String,
}

fn fetch_accounts(
    config: &Config,
    callback_details: &mut CallabckInfo,
    acc_and_unique_id: &mut HashMap<String, String>,
    type_prefix: &str,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/api/{}/merchants/accounts/fetch", config.host, config.uri);

    let request_body = FetchAccountsRequest {
        merchantCustomerId: callback_details.merchantCustomerId.to_string(),
        upiRequestId: callback_details.upi_request_id.to_string(),
        deviceFingerPrint: callback_details.device_finger_print.to_string(),
        bankCode: config.bank_code.clone(),
        udfParameters: "{}".to_string(),
    };

    let request = http::Request {
        method: http::Method::Post,
        url: url.clone(),
        headers: vec![
            ("x-merchant-id".to_string(), config.merchant_id.clone()),
            ("x-merchant-channel-id".to_string(), config.channel_id.clone()),
            ("x-sandbox-id".to_string(), config.sandbox_id.clone()),
            ("Content-Type".to_string(), "application/json".to_string()),
        ],
        body: Some(serde_json::to_string(&request_body).expect("Failed to serialize request body")),
    };

    let response = http::send_request(&request);
    let body = response.body;

    process_response(&body, acc_and_unique_id, type_prefix);
    Ok(())
}


fn getRandomIntInclusive(min: usize, max: usize) -> usize {
    let range = max - min + 1;
    let random = rand::random::<usize>() % range + min;
    random
}

#[derive(Serialize)]
struct AddAccountRequest {
    merchantCustomerId: String,
    bankAccountUniqueId: String,
    customerVpa: String,
    setAsDefaultBank: String,
    udfParameters: String,
}

fn add_accounts(
    config: &Config,
    callback_details: &mut CallabckInfo,
    acc_and_unique_id: &mut HashMap<String, String>,
    type_prefix: &str,
) -> Result<(), Box<dyn Error>> {
    let number_of_accounts_key = format!("{}-numberOfAccounts", type_prefix);
    let number_of_accounts: usize = acc_and_unique_id
        .get(&number_of_accounts_key)
        .ok_or("Number of accounts not found")?
        .parse()?;

    if number_of_accounts == 0 {
        return Err("No accounts found to add".into());
    }

    let random_index = getRandomIntInclusive(1, number_of_accounts) - 1;
    let selected_key = format!("{}-bankAccountUniqueId-{}", type_prefix, random_index + 1);

    let bank_account_unique_id = acc_and_unique_id
        .get(&selected_key)
        .ok_or("Bank account unique ID not found")?;

    callback_details.bank_account_unique_id = bank_account_unique_id.to_string();

    let random_vpa_suffix = getRandomIntInclusive(1000, 9999);
    let customer_vpa = format!("{}@ypay", random_vpa_suffix);

    callback_details.customer_vpa = customer_vpa.to_string();

    let request_body = AddAccountRequest {
        merchantCustomerId: callback_details.merchantCustomerId.to_string(),
        bankAccountUniqueId: bank_account_unique_id.to_string(),
        customerVpa: customer_vpa.to_string(),
        setAsDefaultBank: "True".to_string(),
        udfParameters: "{}".to_string(),
    };

    let url = format!("{}/api/{}/merchants/accounts/add", config.host, config.uri);

    let request = http::Request {
        method: http::Method::Post,
        url: url.clone(),
        headers: vec![
            ("x-merchant-id".to_string(), config.merchant_id.clone()),
            ("x-merchant-channel-id".to_string(), config.channel_id.clone()),
            ("x-sandbox-id".to_string(), config.sandbox_id.clone()),
            ("Content-Type".to_string(), "application/json".to_string()),
        ],
        body: Some(serde_json::to_string(&request_body).expect("Failed to serialize request body")),
    };

    let response = http::send_request(&request);
    let body = response.body;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct CheckBalanceRequest {
    merchantCustomerId: String,
    deviceFingerPrint: String,
    upiRequestId: String,
    bankAccountUniqueId: String,
    customerVpa: String,
    credBlock: String,
    udfParameters: String,
}

fn check_balance(config: &Config, callback_details: &mut CallabckInfo) -> Result<(), Box<dyn Error>> {
    let request_body = CheckBalanceRequest {
        merchantCustomerId: callback_details.merchantCustomerId.to_string(),
        deviceFingerPrint: callback_details.device_finger_print.to_string(),
        upiRequestId: callback_details.upi_request_id.to_string(),
        bankAccountUniqueId: callback_details.bank_account_unique_id.to_string(),
        customerVpa: callback_details.customer_vpa.to_string(),
        credBlock: "{\"mpincred\":{\"type\":\"PIN\",\"subType\":\"MPIN\",\"data\":{\"type\":\"\",\"skey\":\"\",\"pid\":\"\",\"ki\":\"20150822\",\"hmac\":\"\",\"encryptedBase64String\":\"2.0|DUMMYDTk03wPTLNb5lIi5GtYaKy84MndAFA==\",\"code\":\"NPCI\"}}}".to_string(),
        udfParameters: "{}".to_string(),
    };

    let url = format!("{}/api/{}/merchants/accounts/balance", config.host, config.uri);

    let request = http::Request {
        method: http::Method::Post,
        url: url.clone(),
        headers: vec![
            ("x-merchant-id".to_string(), config.merchant_id.clone()),
            ("x-merchant-channel-id".to_string(), config.channel_id.clone()),
            ("x-sandbox-id".to_string(), config.sandbox_id.clone()),
            ("Content-Type".to_string(), "application/json".to_string()),
        ],
        body: Some(serde_json::to_string(&request_body).expect("Failed to serialize request body")),
    };
    
    let response = http::send_request(&request);
    let body = response.body;
    let parsed: Value = serde_json::from_str(&body).expect("Failed to parse JSON");
    if let Some(balance) = parsed["payload"]["balance"].as_str() {
        callback_details.balance = balance.to_string();
    } else {
    }

    Ok(())
}


#[derive(Serialize, Deserialize, Debug)]
struct VerifyVpaRequest {
    customerVpa: String,
    udfParameters: String,
}
fn verify_vpa(config: &Config, callback_details: &mut CallabckInfo) -> Result<(), Box<dyn Error>> {
    let request_body = VerifyVpaRequest {
        customerVpa: "cust495@yapl".to_string(),
        udfParameters: "{}".to_string(),
    };

    let url = format!("{}/api/{}/merchants/vpas/validity", config.host, config.uri);

    let request = http::Request {
        method: http::Method::Post,
        url: url.clone(),
        headers: vec![
            ("x-merchant-id".to_string(), config.merchant_id.clone()),
            ("x-merchant-channel-id".to_string(), config.channel_id.clone()),
            ("x-sandbox-id".to_string(), config.sandbox_id.clone()),
            ("Content-Type".to_string(), "application/json".to_string()),
        ],
        body: Some(serde_json::to_string(&request_body).expect("Failed to serialize request body")),
    };

    let response = http::send_request(&request);

    Ok(())
}


#[derive(Serialize, Deserialize, Debug)]
struct SendMoneyP2PRequest {
    merchantCustomerId: String,
    deviceFingerPrint: String,
    merchantRequestId: String,
    bankAccountUniqueId: String,
    upiRequestId: String,
    payerVpa: String,
    remarks: String,
    payeeVpa: String,
    payeeName: String,
    mcc: String,
    transactionType: String,
    refCategory: String,
    merchantSignature: String,
    currency: String,
    credBlock: String,
    amount: String,
    udfParameters: String,
}
fn send_money_p2p(config: &Config, callback_details_of_from_acc: &mut CallabckInfo, callback_details_of_to_acc: &mut CallabckInfo,amt:&String) -> Result<(), Box<dyn Error>> {
    let merchant_request_id = format!(
        "MUL{}",
        Uuid::new_v4().to_string().replace("-", "")
    )
    .chars()
    .take(35)
    .collect::<String>();

    let request_body = SendMoneyP2PRequest {
        merchantCustomerId: callback_details_of_from_acc.merchantCustomerId.to_string(),
        deviceFingerPrint: callback_details_of_from_acc.device_finger_print.to_string(),
        merchantRequestId: merchant_request_id.to_string(),
        bankAccountUniqueId: callback_details_of_from_acc.bank_account_unique_id.to_string(),
        upiRequestId: callback_details_of_from_acc.upi_request_id.to_string(),
        payerVpa: callback_details_of_from_acc.customer_vpa.to_string(),
        remarks: "UPI".to_string(),
        payeeVpa: callback_details_of_to_acc.customer_vpa.to_string(),
        payeeName: "BruceWayne".to_string(),
        mcc: "5262".to_string(),
        transactionType: "P2P_PAY".to_string(),
        refCategory: "01".to_string(),
        merchantSignature: "merchantSignature".to_string(),
        currency: "INR".to_string(),
        credBlock: "{\"mpincred\":{\"type\":\"PIN\",\"subType\":\"MPIN\",\"data\":{\"type\":\"\",\"skey\":\"\",\"pid\":\"\",\"ki\":\"20150822\",\"hmac\":\"\",\"encryptedBase64String\":\"2.0|DUMMYDTk03wPTLNb5lIi5GtYaKy84MndAFA==\",\"code\":\"NPCI\"}}}".to_string(),
        amount: amt.to_string(),
        udfParameters: "{}".to_string(),
    };

    let url = format!("{}/api/{}/merchants/transactions/sendMoney", config.host, config.uri);

    let request = http::Request {
        method: http::Method::Post,
        url: url.clone(),
        headers: vec![
            ("x-merchant-id".to_string(), config.merchant_id.clone()),
            ("x-merchant-channel-id".to_string(), config.channel_id.clone()),
            ("x-sandbox-id".to_string(), config.sandbox_id.clone()),
            ("Content-Type".to_string(), "application/json".to_string()),
        ],
        body: Some(serde_json::to_string(&request_body).expect("Failed to serialize request body")),
    };

    let response = http::send_request(&request);
    let body = response.body;
    callback_details_of_from_acc.balance = 
    (callback_details_of_from_acc.balance.parse::<i32>().unwrap() - amt.parse::<i32>().unwrap()).to_string();
    callback_details_of_from_acc.balance = 
    (callback_details_of_to_acc.balance.parse::<i32>().unwrap() + amt.parse::<i32>().unwrap()).to_string();
    Ok(())
}


bindings::export!(Component with_types_in bindings);
