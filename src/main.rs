/*
 * Shrunner
 * Shell Code Runner inspired by Sylvain Kerkour (kerkour.com)
 *
 */

const SHELLCODE_BYTES: &[u8] = include_bytes!("..\\shellcode.bin");
const SHELLCODE_LEN: usize   = SHELLCODE_BYTES.len();

#[no_mangle]
#[link_section = ".text"]
static SHELLCODE: [u8; SHELLCODE_LEN] = *include_bytes!("..\\shellcode.bin");

fn main() 
{
    unsafe
    {
        // Get the address of the shellcode first byte
        // Convert it to a ptr
        // Then cast to a function ptr
        let shellc = &(SHELLCODE.as_ptr() as usize) as *const usize as *const fn();
        // Exec shell code
        (*shellc)()
    }
}

