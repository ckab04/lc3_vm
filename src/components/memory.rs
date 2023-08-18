

fn memory<'a>() -> &'a [u16]{
    const MEMORY_MAX: usize = 1 << 16;
    let vm_memory : &[u16] = &[];
    vm_memory

}

pub(crate) fn mem_read(address : u16) -> u16{
0
}