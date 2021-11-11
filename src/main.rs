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
        fn spark(channel: u8) -> UniquePtr<Spark>;
    }

}

use crate::ffi::Spark;

const MOTOR1: UniquePtr<Spark> = spark(1);
const MOTOR2: UniquePtr<Spark> = spark(2);

fn robot_init(){

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