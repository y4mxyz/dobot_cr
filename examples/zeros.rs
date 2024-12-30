use dobot_cr::*;


fn main() {

    let mut cr = DobotCR::connect(None).unwrap();
    // let mut cr = DobotCR::connect(Some(String::from("192.168.5.1:29999"))).unwrap();

    cr.set_vel_ratio(1./4.).unwrap();

    cr.move_j([0., 0., 0., 0., 0., 0.]).unwrap();

}