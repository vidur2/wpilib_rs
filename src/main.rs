trait TimedRobot{
    fn robot_init();
    fn robot_periodic();
    fn autonomous_init();
    fn autonomous_periodic();
    fn teleop_init();
    fn teleop_periodic();
}

pub struct Robot{}

#[cxx::bridge]
mod ffi{
    unsafe extern "C++"{
        include!("wpilib_rs/Robot.h");

        type TimedRobot;
        pub fn robot_init();
        pub fn robot_periodic();
        pub fn autonomous_init();
        pub fn autonomous_periodic();
        pub fn teleop_init();
        pub fn teleop_periodic();

    }
}

impl TimedRobot for Robot {
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
