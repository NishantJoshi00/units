use super::*;

fn safe_intend(path: &str) -> String {
    let (path_ptr, path_len) = path.as_wasmptr();
    let (ptr, len) = unsafe { intend(path_ptr, path_len) };

    unsafe { String::from_wasmptr(ptr, len) }
}

fn safe_done(path: &str) {
    let (path_ptr, path_len) = path.as_wasmptr();
    unsafe { done(path_ptr, path_len) };
}

fn safe_transfer(path1: &str, path2: &str, data: &str) {
    let (path_ptr1, path_len1) = path1.as_wasmptr();
    let (path_ptr2, path_len2) = path2.as_wasmptr();
    let (data_ptr, data_len) = data.as_wasmptr();
    unsafe {
        transfer(
            path_ptr1, path_len1, path_ptr2, path_len2, data_ptr, data_len,
        )
    };
}

fn safe_view(path: &str) -> String {
    let (path_ptr, path_len) = path.as_wasmptr();
    let (ptr, len) = unsafe { view(path_ptr, path_len) };

    unsafe { String::from_wasmptr(ptr, len) }
}

#[cfg(feature = "transfer")]
pub fn safe_main(input: &str) -> &str {
    let input: Input = serde_json::from_str(input).expect("invalid input");
    let p1 = safe_intend(&input.path1);
    let p2 = safe_intend(&input.path2);

    let data = Data {
        amount: input.amount,
    };
    let data = serde_json::to_string(&data).expect("invalid data");

    safe_transfer(&p1, &p2, &data);

    safe_done(&p1);
    safe_done(&p2);


    "done"
}

#[cfg(feature = "view")]
pub fn safe_main(input: &str) -> &str {
    let input: OnlyPath = serde_json::from_str(input).expect("invalid input");
    let p1 = safe_intend(&input.path);

    let p1_data = safe_view(&p1);

    let p1_d = serde_json::from_str::<MoreData>(&p1_data).expect("invalid data");

    let data = ViewData { path1: p1_d };

    let data = serde_json::to_string(&data).expect("invalid data");

    safe_done(&p1);

    data.leak()
}

#[derive(serde::Serialize)]
struct ViewData {
    path1: MoreData,
}

#[derive(serde::Deserialize)]
struct OnlyPath {
    path: String,
}

#[derive(serde::Deserialize)]
struct Input {
    path1: String,
    path2: String,
    amount: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    amount: u64,
}


#[derive(serde::Serialize, serde::Deserialize)]
struct MoreData {
    name: String,
    amount: u64,
}
