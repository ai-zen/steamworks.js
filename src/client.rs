use std::ptr::addr_of_mut;
use std::sync::Mutex;
use steamworks::Client;
use steamworks::SingleClient;

lazy_static! {
    static ref STEAM_CLIENT: Mutex<Option<Client>> = Mutex::new(None);
}

static mut STEAM_SINGLE: Option<SingleClient> = None;

pub fn has_client() -> bool {
    STEAM_CLIENT.lock().unwrap().is_some()
}

pub fn get_client() -> Client {
    let option = STEAM_CLIENT.lock().unwrap().to_owned();
    option.unwrap()
}

pub fn set_client(client: Client) {
    let mut client_ref = STEAM_CLIENT.lock().unwrap();
    *client_ref = Some(client);
}

pub fn drop_client() {
    let mut client_ref = STEAM_CLIENT.lock().unwrap();
    *client_ref = None;
}

pub fn get_single() -> &'static SingleClient {
    unsafe {
        match *addr_of_mut!(STEAM_SINGLE) {
            Some(ref single) => single,
            None => panic!("Steam single not initialized"),
        }
    }
}

pub fn set_single(single: SingleClient) {
    unsafe {
        STEAM_SINGLE = Some(single);
    }
}

pub fn drop_single() {
    unsafe {
        STEAM_SINGLE = None;
    }
}
