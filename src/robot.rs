use crate::{Error, ErrorCode, ComSock};


pub struct DobotCR {

    comsock: ComSock,

    acc_ratio: usize,
    vel_ratio: usize,

}


impl DobotCR {

    pub fn connect(address: Option<String>) -> Result<Self, Error> {

        Ok(DobotCR {
            comsock: match ComSock::new(address) {
                Ok(comsock) => comsock,
                Err(error) => return Err(error),
            },
            acc_ratio: 50,
            vel_ratio: 50,
        })

    }

    fn get_vec6(&mut self, name: &str) -> Result<[f64; 6], Error> {

        let command = name;
        let arguments = format!("");

        let (error_id, values) = match self.comsock.command(
            command, &arguments) {
            Ok(result) => result, Err(error) => return Err(error),
        };
        if error_id != 0 {
            return Err(Error::DobotError(ErrorCode::from(error_id)));
        }
        if values.len() != 6 {
            return Err(Error::MessageError);
        }

        let mut result = [0.; 6];
        for index in 0..6 {
            result[index] = match values[index].parse::<f64>() {
                Ok(v) => v, Err(_) => return Err(Error::MessageError),
            }
        }

        Ok(result)

    }

    fn just_command(&mut self, command: &str) -> Result<(), Error> {
        
        let arguments = format!("");

        let (error_id, _) = match self.comsock.command(
            command, &arguments) {
            Ok(result) => result, Err(error) => return Err(error),
        };
        if error_id != 0 {
            return Err(Error::DobotError(ErrorCode::from(error_id)));
        }

        Ok(())

    }

    pub fn get_pose(&mut self) -> Result<[f64; 6], Error> {

        self.get_vec6("GetPose")

    }

    pub fn get_joints(&mut self) -> Result<[f64; 6], Error> {

        self.get_vec6("GetAngle")

    }

    pub fn set_acc_ratio(&mut self, acc_ratio: f64) -> Result<&Self, Error> {

        if acc_ratio < 0. || acc_ratio > 1. {
            return Err(Error::ArgumentError)
        }

        self.acc_ratio = (acc_ratio*100.) as usize;

        Ok(self)

    }

    pub fn set_vel_ratio(&mut self, vel_ratio: f64) -> Result<&Self, Error> {

        if vel_ratio < 0. || vel_ratio > 1. {
            return Err(Error::ArgumentError)
        }

        self.vel_ratio = (vel_ratio*100.) as usize;

        Ok(self)

    }

    pub fn stop(&mut self) -> Result<(), Error> {

        self.just_command("Stop")

    }

    pub fn pause(&mut self) -> Result<(), Error> {

        self.just_command("Pause")

    }

    pub fn resume(&mut self) -> Result<(), Error> {

        self.just_command("Continue")

    }

    pub fn move_j(&mut self, joints: [f64; 6]) -> Result<(), Error> {

        let command = "MovJ";
        let arguments = format!("joint={{{},{},{},{},{},{}}},a={},v={}",
            joints[0], joints[1], joints[2], joints[3], joints[4], joints[5],
            self.acc_ratio, self.vel_ratio);

        let (error_id, _) = match self.comsock.command(
            command, &arguments) {
            Ok(result) => result, Err(error) => return Err(error),
        };
        if error_id != 0 {
            return Err(Error::DobotError(ErrorCode::from(error_id)));
        }

        Ok(())

    }

    pub fn move_l(&mut self, pose: [f64; 6], speed: Option<f64>) -> Result<(), Error> {

        let v_speed = match speed {
            Some(speed) => format!("speed={}", speed),
            None => format!("v={}", self.vel_ratio),
        };

        let command = "MovL";
        let arguments = format!("pose={{{},{},{},{},{},{}}},a={},{}",
            pose[0], pose[1], pose[2], pose[3], pose[4], pose[5],
            self.acc_ratio, v_speed);

        let (error_id, _) = match self.comsock.command(
            command, &arguments) {
            Ok(result) => result, Err(error) => return Err(error),
        };
        if error_id != 0 {
            return Err(Error::DobotError(ErrorCode::from(error_id)));
        }

        Ok(())

    }

}