use wasm_bindgen::prelude::*;
use identity::account::Account;

#[wasm_bindgen(js_name = Account)]
pub struct WasmAccount(Account);

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
    #[wasm_bindgen(js_name = testAccount)]
    pub fn test_account(&self) -> String{
        return String::from("test success");
    }
}

impl From<Account> for WasmAccount {
    fn from(account: Account) -> WasmAccount {
        WasmAccount(account)
    }
}
