mod rust_implementation;

use cxx::{type_id, ExternType};

unsafe impl ExternType for rust_implementation::TimedRobot{
    type Id = type_id!("wpilib_rust_impl:TimedRobot");
    type Kind = cxx::Kind::Trivial;
}

#[cxx::bridge(namespace="wpilib_rust_impl")]
mod ffi{
    unsafe extern "C++"{
        include!("wpilib_rs/header_files/Robot.h");

        type TimedRobot = crate::rust_implementation::TimedRobot;
    }

}

use ffi::TimedRobot;

impl TimedRobot{
    fn robot_init(){
        println!("Shit")
    }
    fn robot_periodic(){
        println!("Shit")
    }
}
fn main() {
    println!("Hello World")
}