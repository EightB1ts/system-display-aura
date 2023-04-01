# System-Display-AURA

![Computer](https://i.imgur.com/4fTDlUM.jpg)

Personal project built to provide real-time ASUS Aura RGB values from memory. Used for an LCD panel inside my custom PC build. First project in Rust, wanted to see how [Tauri](https://tauri.app/) compares to [ElectronJS](https://www.electronjs.org/).

It's impossible to see what RGB values are being sent utilizing the ASUS Aura SDK. I utilized IDA pro to locate memory address for RGB values within ```C:\Program Files (x86)\LightingService\LightingService.exe``` . 

## Structure
**src** contains the web assets. **src-tauri** contains the rust source code.
```
src/
├── index.html
└── style.css
src-tauri/
└── src/
    ├── aura_memory.rs
    └── main.rs
```

## Building

Note: This program accesses the memory of a service. Running this program requires elevated access and a working install of [ASUS Aura](https://www.asus.com/campaign/aura/us/download.php). 

1. Install dependencies [required by Tauri](https://tauri.app/v1/guides/getting-started/prerequisites)
2. Install [Node/NPM](https://nodejs.org/en/)
3. Clone repo
4. Switch to the **stable-x86_64-pc-windows-msvc** toolchain
5. Start by executing ```npm run tauri dev```
   - Alternatively, you can build the project by executing ```npm run tauri build``` . Executable will be located under **./src-tauri/target/release**
