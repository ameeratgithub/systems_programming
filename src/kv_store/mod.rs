use std::collections::HashMap;

use crate::kv_store::lib::{ByteStr, ByteString, KV};

mod lib;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    kv_mem.exe FILE get KEY
    kv_mem.exe FILE delete KEY
    kv_mem.exe FILE insert KEY VALUE
    kv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    kv_mem FILE get KEY
    kv_mem FILE delete KEY
    kv_mem FILE insert KEY VALUE
    kv_mem FILE update KEY VALUE
";

fn store_index_on_disk(a: &mut KV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

pub fn run() {
    const INDEX_KEY: &ByteStr = b"+index";

    // Arguments provided via CLI
    let args: Vec<String> = std::env::args().collect();
    // file name should be first
    let file_name = args.get(1).expect(&USAGE);
    // action: get, insert, delete, update
    let action = args.get(2).expect(&USAGE).as_ref();
    // Key must be specified
    let key = args.get(3).expect(&USAGE).as_ref();
    // Value should be there if action is 'insert' or 'update'
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&file_name);
    let mut store = KV::open(path).expect("Unable to open file");
    store.load().expect("Unable to load data");

    match action {
        "get" => {
            let index_as_bytes = store.get(&INDEX_KEY).unwrap().unwrap();
            let index_decoded = bincode::deserialize(&index_as_bytes);
            let index: HashMap<ByteString, u64> = index_decoded.unwrap();

            match index.get(key) {
                None => eprintln!("{key:?} not found"),
                Some(&i) => {
                    let kv = store.get_at(i).unwrap();
                    println!("{:?}", kv.value);
                }
            }
        }
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        _ => eprintln!("{}", &USAGE),
    }
}
