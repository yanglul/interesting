use std::io;
use std::mem::{size_of, MaybeUninit};
use std::ptr::NonNull;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, FALSE, HMODULE, LPCVOID, LPVOID, TRUE};
use winapi::um::winnt;
use winapi::um::winnt::MEMORY_BASIC_INFORMATION;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;
const MAX_PROC_NAME_LEN: usize = 1024;
pub struct Process {
    pid: u32,
    handle: NonNull<c_void>,
}

pub struct ProcessItem {
    pub pid: u32,
    pub name: String,
}

pub fn enum_proc() -> io::Result<Vec<u32>> {
    let mut size = 0;
    let mut pids = Vec::<DWORD>::with_capacity(2048);
    if unsafe {

        winapi::um::psapi::EnumProcesses(
            pids.as_mut_ptr(),
            (pids.capacity() * std::mem::size_of::<DWORD>()) as u32,
            &mut size,
        )
    } == FALSE
    {
        return Err(io::Error::last_os_error());
    }

    let count = size as usize / std::mem::size_of::<DWORD>();
    unsafe { pids.set_len(count) }
    Ok(pids)
}

impl Process {
    pub fn write_memory(&self, address: usize, value: &[u8]) {
        let mut write = 0;
        let mut old_protect: u32 = 0;
        if unsafe {
            /*
            BOOL VirtualProtectEx(
                [in]  HANDLE hProcess,
                [in]  LPVOID lpAddress,
                [in]  SIZE_T dwSize,
                [in]  DWORD  flNewProtect,
                [out] PDWORD lpflOldProtect
                );
            */
            winapi::um::memoryapi::VirtualProtectEx(
                self.handle.as_ptr(),
                address as LPVOID,
                value.len(),
                PAGE_EXECUTE_READWRITE,
                &mut old_protect,
            )
        } == FALSE
        {
            return;
        }
        if unsafe {
            winapi::um::memoryapi::WriteProcessMemory(
                self.handle.as_ptr(),
                address as LPVOID,
                value.as_ptr().cast(),
                value.len(),
                &mut write,
            )
        } == FALSE
        {
            return;
        }
    }
    pub fn read_memory(&self, address: usize, n: usize) -> io::Result<Vec<u8>> {
        let mut buffer: Vec<u8> = Vec::with_capacity(n);
        let mut read = 0;
        if unsafe {
            winapi::um::memoryapi::ReadProcessMemory(
                self.handle.as_ptr(),
                address as LPCVOID,
                buffer.as_ptr() as LPVOID,
                n,
                &mut read,
            )
        } == FALSE
        {
            return Err(io::Error::last_os_error());
        }
        unsafe { buffer.set_len(read as usize) };
        Ok(buffer)
    }
    pub fn memory_regions(&self) -> Vec<MEMORY_BASIC_INFORMATION> {
        let mut base = 0;

        let mut regions = Vec::new();
        let mut info = MaybeUninit::uninit();

        loop {
            /*
             SIZE_T VirtualQueryEx(
                [in]           HANDLE                    hProcess,
                [in, optional] LPCVOID                   lpAddress,
                [out]          PMEMORY_BASIC_INFORMATION lpBuffer,
                [in]           SIZE_T                    dwLength
             );
             */
            let written = unsafe {
                winapi::um::memoryapi::VirtualQueryEx(
                    self.handle.as_ptr(),
                    base as *const _,
                    info.as_mut_ptr(),
                    size_of::<MEMORY_BASIC_INFORMATION>(),
                )
            };

            //结束
            if written == 0 {
                break;
            }
            let info = unsafe { info.assume_init() };

            //计算下一块区域base
            base = info.BaseAddress as usize + info.RegionSize;
            
            //保存
            regions.push(info);
        }

        let mask = winnt::PAGE_EXECUTE_READWRITE
        | winnt::PAGE_EXECUTE_WRITECOPY
        | winnt::PAGE_READWRITE
        | winnt::PAGE_WRITECOPY;

        //过滤掉系统模块，无效区域
        return regions.into_iter().filter(|x|!(x.BaseAddress as u32 > 0x70000000 
        && (x.BaseAddress as u32) < 0x80000000)).filter(|p| (p.Protect & mask) != 0).collect();
    }
    pub fn name(&self) -> io::Result<String> {
        let mut module = MaybeUninit::<HMODULE>::uninit();
        let mut size = 0;
        if unsafe {
            winapi::um::psapi::EnumProcessModules(
                self.handle.as_ptr(),
                module.as_mut_ptr(),
                size_of::<HMODULE>() as u32,
                &mut size,
            )
        } == FALSE
        {
            return Err(io::Error::last_os_error());
        }
        let module = unsafe { module.assume_init() };

        let mut buffer = Vec::<u8>::with_capacity(MAX_PROC_NAME_LEN);
        let length = unsafe {

            winapi::um::psapi::GetModuleBaseNameA(
                self.handle.as_ptr(),
                module,
                buffer.as_mut_ptr().cast(),
                buffer.capacity() as u32,
            )
        };
        if length == 0 {
            return Err(io::Error::last_os_error());
        }

        unsafe { buffer.set_len(length as usize) };
        return match String::from_utf8(buffer) {
            Ok(s) => Ok(s),
            Err(e) => Ok("".to_string()),
        };
    }

    pub fn pid(&self) -> u32 {
        return self.pid;
    }

    pub fn open(pid: u32) -> io::Result<Self> {
        NonNull::new(unsafe {

            winapi::um::processthreadsapi::OpenProcess(
                winnt::PROCESS_QUERY_INFORMATION
                    | winnt::PROCESS_VM_READ
                    | winnt::PROCESS_VM_WRITE
                    | winnt::PROCESS_VM_OPERATION,
                FALSE,
                pid,
            )
        })
        .map(|handle| Self { pid, handle })
        .ok_or_else(io::Error::last_os_error)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        let ret = unsafe { winapi::um::handleapi::CloseHandle(self.handle.as_mut()) };
        assert_ne!(ret, FALSE);
    }
}


use std::io::stdin;


#[test]
fn run() {
    println!("MyCheatEngine");
    println!("all process:");
    let pids = enum_proc()
        .unwrap()
        .into_iter()
        .flat_map(Process::open)
        .flat_map(|proc| match proc.name() {
            Ok(name) => Ok(ProcessItem {
                pid: proc.pid(),
                name,
            }),
            Err(err) => Err(err),
        })
        .collect::<Vec<_>>();

    for ele in pids {
        println!("{} {}", ele.pid, ele.name)
    }
    println!("Pid:");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let pid = input.trim().parse::<u32>().unwrap();
    let process = Process::open(pid).unwrap();

    let regions:Vec<MEMORY_BASIC_INFORMATION> = process.memory_regions();
    println!("Scanning {} memory regions", regions.len());


    println!("Which exact value to scan for?");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let target: u32 = input.trim().parse::<u32>().unwrap();
    let target = target.to_ne_bytes();

    let mut locations = Vec::new();
    for region in regions {
        match process.read_memory(region.BaseAddress as _, region.RegionSize){
            Ok(mem)=>{
                mem.windows(target.len()).enumerate().for_each(|(offset, window)| {
                    if window == target {
                        locations.push(region.BaseAddress as usize + offset);
                    }
                })
            },
            Err(e)=>continue
        }
    }
    println!("The number of value is {:?}",locations.len());
    while locations.len() != 1 {
        println!("Next Scan value:");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let target: u32 = input.trim().parse::<u32>().unwrap();
        let target = target.to_ne_bytes();

        locations.retain(|addr| match process.read_memory(*addr, target.len()) {
            Ok(memory) => memory == target,
            Err(_) => false,
        });
    }

    println!("Scan Finished! this address is :{:?},Type the new value:",locations[0]);

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let new_value: i32 = input.trim().parse::<i32>().unwrap();
    let new_value = new_value.to_ne_bytes();
    process.write_memory(locations[0], &new_value);

    println!("The new value is {:?}",new_value);

}