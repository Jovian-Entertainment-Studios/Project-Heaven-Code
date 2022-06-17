use glam::f32::{Quat, Vec3A};
use rend3::util::typedefs::FastHashMap;
use std::{collections::HashMap, hash::BuildHasher};

mod platform;

fn button_pressed<Hash: BuildHasher>(map: &HashMap<u32, bool, Hash>, key: u32) -> bool {
    map.get(&key).map_or(false, |b| *b)
}

pub struct SpaceCam {
    pub camera_yaw: f32,
    pub camera_pitch: f32,
    pub camera_roll: f32,

    pub rotation: Quat,

    pub side: Vec3A,
    pub up: Vec3A,
    pub forward: Vec3A,

    pub run_speed: f32,
    pub walk_speed: f32,

    pub delta_time: std::time::Duration,

    pub camera_location: Vec3A,
}
pub fn space_cam(
    mut data: SpaceCam,
    scancode_status: &FastHashMap<u32, bool>,
) -> (Quat, Vec3A, f32) {
    let quaternion_new = Quat::from_euler(
        glam::EulerRot::YXZ,
        data.camera_yaw,
        data.camera_pitch,
        data.camera_roll,
    );

    data.camera_roll = 0.;
    data.camera_pitch = 0.;
    data.camera_yaw = 0.;

    data.rotation = Quat::mul_quat(quaternion_new, data.rotation).normalize();

    data.side = Quat::mul_vec3a(data.rotation.inverse(), Vec3A::X);
    data.up = Quat::mul_vec3a(data.rotation.inverse(), Vec3A::Y);
    data.forward = Quat::mul_vec3a(data.rotation.inverse(), Vec3A::Z);

    let velocity = if button_pressed(scancode_status, platform::Scancodes::SHIFT) {
        data.run_speed
    } else {
        data.walk_speed
    };
    if button_pressed(scancode_status, platform::Scancodes::W) {
        data.camera_location += data.forward * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::S) {
        data.camera_location -= data.forward * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::A) {
        data.camera_location -= data.side * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::D) {
        data.camera_location += data.side * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::SPACE) {
        data.camera_location += data.up * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::COMMA) {
        data.camera_location -= data.up * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::Q) {
        data.camera_roll -= 1. * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::E) {
        data.camera_roll += 1. * data.delta_time.as_secs_f32();
    }

    (data.rotation, data.camera_location, data.camera_roll)
}

pub struct ShipCam {
    pub camera_yaw: f32,
    pub camera_pitch: f32,

    pub ship_yaw: f32,
    pub ship_pitch: f32,
    pub ship_roll: f32,

    pub camera_side: Vec3A,
    pub camera_up: Vec3A,
    pub camera_forward: Vec3A,

    pub ship_side: Vec3A,
    pub ship_up: Vec3A,
    pub ship_forward: Vec3A,

    pub acceleration_max: f32,
    pub acceleration: f32,
    pub velocity_vec: Vec3A,

    pub delta_time: std::time::Duration,

    pub camera_location: Vec3A,
    pub camera_relative_rotation: Quat,
    pub camera_rotation: Quat,

    pub ship_location: Vec3A,
    pub ship_rotation: Quat,
}

pub fn ship_cam(
    mut data: ShipCam,
    scancode_status: &FastHashMap<u32, bool>,
) -> (f32, f32, f32, f32, Quat, Vec3A, Quat, Vec3A) {
    let ship_new_rotation_quaternion = Quat::from_euler(
        glam::EulerRot::YXZ,
        data.ship_yaw,
        data.ship_pitch,
        data.ship_roll,
    );

    data.ship_pitch = 0.;
    data.ship_yaw = 0.;
    data.ship_roll = 0.;

    let old_rot = data.ship_rotation;

    data.ship_rotation =
        Quat::mul_quat(data.ship_rotation, ship_new_rotation_quaternion).normalize();

    data.camera_rotation = data.ship_rotation * old_rot.inverse() * data.camera_rotation;

    data.ship_side = Quat::mul_vec3a(data.ship_rotation.inverse(), Vec3A::X);
    data.ship_up = Quat::mul_vec3a(data.ship_rotation.inverse(), Vec3A::Y);
    data.ship_forward = Quat::mul_vec3a(data.ship_rotation.inverse(), Vec3A::Z);

    if button_pressed(scancode_status, platform::Scancodes::PLUS_NUM) {
        data.acceleration = data.acceleration + (0.1 * data.delta_time.as_secs_f32());
        if data.acceleration > data.acceleration_max {
            data.acceleration = data.acceleration_max;
        }
    }
    if button_pressed(scancode_status, platform::Scancodes::MINUS_NUM) {
        data.acceleration = data.acceleration - (0.1 * data.delta_time.as_secs_f32());
        if data.acceleration < 0. {
            data.acceleration = 0.;
        }
    }

    data.velocity_vec += data.delta_time.as_secs_f32() * data.ship_forward * data.acceleration;

    data.ship_location += data.delta_time.as_secs_f32() * data.velocity_vec;

    let velocity = 10.;

    if button_pressed(scancode_status, platform::Scancodes::W) {
        data.velocity_vec += data.ship_forward * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::S) {
        data.velocity_vec -= data.ship_forward * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::A) {
        data.velocity_vec -= data.ship_side * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::D) {
        data.velocity_vec += data.ship_side * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::SPACE) {
        data.velocity_vec += data.ship_up * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::COMMA) {
        data.velocity_vec -= data.ship_up * velocity * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::Q) {
        data.ship_roll += 1. * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::E) {
        data.ship_roll -= 1. * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::UP) {
        data.ship_pitch -= 1. * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::DOWN) {
        data.ship_pitch += 1. * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::LEFT) {
        data.ship_yaw -= 1. * data.delta_time.as_secs_f32();
    }
    if button_pressed(scancode_status, platform::Scancodes::RIGHT) {
        data.ship_yaw += 1. * data.delta_time.as_secs_f32();
    }

    (
        data.acceleration,
        data.ship_yaw,
        data.ship_pitch,
        data.ship_roll,
        data.ship_rotation,
        data.ship_location,
        data.camera_rotation,
        data.velocity_vec,
    )
}
