use crate::Ptr;

fn safe_get(input: &str) -> Option<String> {
    let (ptr, len) = input.as_wasmptr();
    let (ptr, len) = unsafe { super::get(ptr, len) };
    if len == 0 {
        return None;
    }
    Some(unsafe { String::from_wasmptr(ptr, len) })
}

fn safe_set(key: &str, value: &str) {
    let (key_ptr, key_len) = key.as_wasmptr();
    let (value_ptr, value_len) = value.as_wasmptr();
    unsafe { super::set(key_ptr, key_len, value_ptr, value_len) }
}

pub fn safe_intend(input: &str) -> &str {
    let account: AccountInfo = serde_json::from_str(input).unwrap();
    let stored = safe_get(&account.name).unwrap_or(input.to_string());
    safe_set(&account.name, &stored);
    stored.leak()
}

pub fn safe_done(input: &str) {}

pub fn safe_view(input: &str) -> &str {
    input
}

pub fn safe_transfer(from: &str, to: &str, data: &str) {
    let mut from_acc = serde_json::from_str::<AccountInfo>(from).unwrap();
    let mut to_acc = serde_json::from_str::<AccountInfo>(to).unwrap();
    let diff = serde_json::from_str::<Data>(data).unwrap().amount;

    from_acc.amount -= diff;
    to_acc.amount += diff;

    let solana_key1 = format!("{}:{}", "sol", from_acc.name);
    let solana_key2 = format!("{}:{}", "sol", to_acc.name);

    safe_set(&solana_key1, &serde_json::to_string(&from_acc).unwrap());
    safe_set(&solana_key2, &serde_json::to_string(&to_acc).unwrap());
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct AccountInfo {
    name: String,
    amount: u64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Data {
    amount: u64,
}
