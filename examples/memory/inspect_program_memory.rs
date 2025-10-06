use winapi::{
    DWORD, HANDLE, LPSYSTEM_INFO, LPVOID, MEMORY_BASIC_INFORMATION as MEM_INFO, PVOID, SIZE_T,
    SYSTEM_INFO,
};

///
/// NOTE: This examples only runs on windows operating system because it's easier to
/// work with
///
fn main() {
    // DWORD is u32 in Rust
    let this_pid: DWORD;
    // A pointer to some opaque resource within Windows.
    let this_proc: HANDLE;
    // LPVOID is windows naming convention and stands for Long Pointer Void, where void is the
    // data type, means no associated data type.
    let min_addr: LPVOID;
    let max_addr: LPVOID;
    // PVOID stands for Pointer Void, where void is the data type
    let mut base_addr: PVOID;
    // Both following structs are defined by windows internally
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEM_INFO;

    const MEM_INFO_SIZE: usize = std::mem::size_of::<MEM_INFO>();

    // This block guarantees that all memory is initialized
    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    // System calls to get the data
    unsafe {
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();
        kernel32::GetSystemInfo(&mut proc_info as LPSYSTEM_INFO);
    }

    min_addr = proc_info.lpMinimumApplicationAddress;
    max_addr = proc_info.lpMaximumApplicationAddress;

    println!("{this_pid:?} @ {this_proc:p}");
    println!("{proc_info:?}");
    println!("min: {min_addr:p}, max: {max_addr:p}");

    // This loop does the work of scanning through the address space
    loop {
        // SIZE_T is u64 which will be usize on 64-bit machines
        let rc: SIZE_T = unsafe {
            // Provides information about a specific segment of the running program's
            // memory address space, starting at base_addr
            kernel32::VirtualQueryEx(this_proc, base_addr, &mut mem_info, MEM_INFO_SIZE as SIZE_T)
        };

        if rc == 0 {
            break;
        }

        println!("{mem_info:#?}");
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;
    }
}
