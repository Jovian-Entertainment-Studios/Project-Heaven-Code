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
pub fn space_cam(mut data: SpaceCam, scancode_status: &FastHashMap<u32, bool>) -> (Quat, Vec3A, f32) {
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
