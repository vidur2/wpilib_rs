use cxx::UniquePtr;

#[cxx::bridge]
mod ffi{
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
    unsafe extern "C++"{
        include!("wpilib_rs/header_files/frc/Spark.h");

        type Spark;
        fn spark(channel: i64) -> UniquePtr<Spark>;
        fn set(&self, value: f64);
    }

}

use crate::ffi::Spark;
use crate::ffi::spark;

const MOTOR1: UniquePtr<Spark> = spark(1);
const MOTOR2: UniquePtr<Spark> = spark(2);

fn robot_init(){
    MOTOR1.set(0.5);
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