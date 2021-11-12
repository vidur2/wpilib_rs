// Get smart pointer for C++ into Rust code so it is usable
use cxx::UniquePtr;

// C++ <-> Rust Bridge
#[cxx::bridge]
mod ffi{
    
    // Rust functions which go to C++ header
    extern "Rust"{
        fn robot_init();
        fn robot_periodic();
        fn autonomous_init();
        fn autonomous_periodic();
        fn teleop_init();
        fn teleop_periodic();
        fn test_init();
        fn test_periodic();
    }
    
    // C++ objects/methods brought into Rust scope
    unsafe extern "C++"{
        include!("wpilib_rs/header_files/frc/Spark.h");

        type Spark;
        fn spark(channel: i64) -> UniquePtr<Spark>;
        fn Set(&self, value: f64);
    }

}


// Import nessescary C++ objects/methods from bridge
use crate::ffi::Spark;
use crate::ffi::spark;

// Motors declared as constants so they will be available the whole scope
const MOTOR1: UniquePtr<Spark> = spark(1);
const MOTOR2: UniquePtr<Spark> = spark(2);

// Robot functions go here
fn robot_init(){
    MOTOR1.Set(0.5);
}

fn robot_periodic(){

}

fn autonomous_init(){

}

fn autonomous_periodic(){

}

fn teleop_init(){

}

fn teleop_periodic(){

}

fn test_init(){

}

fn test_periodic(){

}