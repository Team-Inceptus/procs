mod lsproc;

fn main() {
    let procs: Vec<String> = lsproc::enumerate_procids();

    for pid in procs {
        println!("{} - {}", pid, lsproc::get_proc_comm(&pid).trim());
    }
}
