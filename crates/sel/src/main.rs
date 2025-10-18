fn main() {
    let mut vm = roscasel::VirtualMachine::new(1024, 4096);
    vm.load_program(todo!());
    vm.run().unwrap();
}
