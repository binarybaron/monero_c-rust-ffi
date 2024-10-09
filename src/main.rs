
// main.rs

use monero_c_rust_ffi::{MONERO_WalletManagerFactory_getWalletManager, MONERO_WalletManager_createWallet};

fn main() {
    // Initialize the wallet manager pointer
    let wm_ptr: *mut std::os::raw::c_void = unsafe { MONERO_WalletManagerFactory_getWalletManager() };

    // Define the parameters for creating a wallet
    let path = std::ffi::CString::new("/path/to/wallet").expect("CString::new failed");
    let password = std::ffi::CString::new("your_password").expect("CString::new failed");
    let language = std::ffi::CString::new("English").expect("CString::new failed");
    let network_type = NETWORK_MAINNET; // Use the appropriate network type constant

    // Call the MONERO_WalletManager_createWallet function
    let wallet_ptr = unsafe {
        MONERO_WalletManager_createWallet(
            wm_ptr,
            path.as_ptr(),
            password.as_ptr(),
            language.as_ptr(),
            network_type,
        )
    };

    // Check if the wallet was created successfully
    if wallet_ptr.is_null() {
        println!("Failed to create wallet.");
    } else {
        println!("Wallet created successfully.");
    }
}

const NETWORK_MAINNET: std::os::raw::c_int = 0;