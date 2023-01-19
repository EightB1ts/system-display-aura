use vmemory::*;
use std::fmt::Write;

// Offsets to RGB values
const OFFSETS:[usize; 7] = [0x0, 0x0, 0x0, 0x108, 0x0, 0x8, 0x14];

// Initial offset LightingService.exe + START
const START:usize = 0x00399F94;

#[cfg(windows)]
mod windows {
    pub(crate) use windows::Win32::{
        Foundation::{CHAR, MAX_PATH},
        System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
            TH32CS_SNAPPROCESS,
        },
    };
}

#[cfg(windows)]
fn get_pid(process_name: &str) -> u32 {
    /// A helper function to turn a CHAR array to a String
    fn utf8_to_string(bytes: &[windows::CHAR]) -> String {
        use std::ffi::CStr;
        unsafe {
            CStr::from_ptr(bytes.as_ptr() as *const i8)
                .to_string_lossy()
                .into_owned()
        }
    }

    let mut entry = windows::PROCESSENTRY32 {
        dwSize: std::mem::size_of::<windows::PROCESSENTRY32>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [windows::CHAR(0); windows::MAX_PATH as usize],
    };
    unsafe {
        // On Error return 0 as the pid. Maybe this function should instead return itself a Result
        // to indicate if a pid has been found?
        let snapshot = if let Ok(snapshot) =
            windows::CreateToolhelp32Snapshot(windows::TH32CS_SNAPPROCESS, 0)
        {
            snapshot
        } else {
            return 0;
        };
        if windows::Process32First(snapshot, &mut entry) == true {
            while windows::Process32Next(snapshot, &mut entry) == true {
                if utf8_to_string(&entry.szExeFile) == process_name {
                    return entry.th32ProcessID;
                }
            }
        }
    }
    0
}

pub struct Memory {
    hook: vmemory::ProcessMemory,
    address: usize
}

impl Memory {

    // When Memory is instantiated, bind to LightingService and obtain dynamic memory address of RGB pointer
    pub fn new() -> Self {

        let lighting_service = ProcessMemory::attach_process(get_pid("LightingService.exe")).unwrap();
        let mut current_address:usize = START;
        lighting_service.resume();

        // The RGB value consists of a multi-level pointer. 
        // Need to iterate through offset values to obtain the RGB value.
        for (i, x) in OFFSETS.iter().enumerate() {

            // Get Address
            let vmem = lighting_service.read_memory(
                current_address + usize::try_from(*x).unwrap(), 
                4, 
                if i == 0 { true } else { false }
            );
            
            let mut values: [u8; 8] = [0,0,0,0,0,0,0,0];
            for (i, x) in vmem.iter().enumerate() {
                values[i] = *x
            }

            current_address = usize::from_ne_bytes(values);
        }
        
        Self {
            hook: lighting_service,
            address: current_address
        }
    }

    pub fn get(&self) -> String {
        
        let rgb = self.hook.read_memory(
            usize::try_from(self.address).unwrap(), 
            3, 
            false
        );

        let value = format!("{},{},{}", rgb[0].to_string(), rgb[1].to_string(), rgb[2].to_string());

        return value
    }

}