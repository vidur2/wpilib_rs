#[cxx::bridge]
mod ffi{
    unsafe extern "C++"{
        include!("wpilib_rs/header_files/Robot.h");

        type TimedRobot;
    }
}

use crate::ffi::TimedRobot;

impl TimedRobot{
    fn robot_init(){
        // Robot init
    }
    fn robot_periodic(){
        // Robot Periodic
    }
    fn autonomous_init(){
        // autonomous init
    }
    fn autonomous_periodic(){
        // autonomous periodic
    }
    fn teleop_init(){
        // teleop init
    }
    fn teleop_periodic(){
        // teleop periodic
    }
}

// impl TimedRobot for Robot {
//     fn robot_init(){
//         println!("Shit")
//     }
//     fn robot_periodic(){
//         println!("Shit")
//     }
// }
fn main() {
    println!("Hello World")
}
