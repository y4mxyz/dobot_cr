use crate::Error;
use std::io::{Read, Write};
use std::net::TcpStream;


pub struct ComSock {

    socket: TcpStream,
    buffer: [u8; 1024],

}

impl ComSock {

    pub fn new(address: Option<String>) -> Result<Self, Error> {
        
        let address = match address {
            Some(address) => address,
            None => String::from("192.168.5.1:29999"),
        };
        
        let stream = match TcpStream::connect(address) {
            Ok(stream) => stream,
            Err(_) => return Err(Error::CommunictionError),
        };

        Ok(ComSock {
            socket: stream,
            buffer: [0; 1024],
        })

    }

    pub fn command(&mut self, command: &str, arguments_list: &String) -> Result<(isize, Vec<String>), Error> {
        
        let command = format!("{}({})", command, arguments_list);
        
        match self.socket.write_all(command.as_bytes()) {
            Ok(_) => {}, Err(_) => return Err(Error::CommunictionError),
        };

        self.buffer.fill(0);
        
        match self.socket.read(&mut self.buffer) {
            Ok(_) => {}, Err(_) => return Err(Error::CommunictionError),
        };

        let response = match String::from_utf8(self.buffer.to_vec()) {
            Ok(string) => string, Err(_) => return Err(Error::CommunictionError),
        };

        let response = match response.find(format!(",{};", command).as_str()) {
            Some(find) => response[0..find].to_string(), None => return Err(Error::CommunictionError),
        };
        
        let (error_id, values) = match response.find(",") {
            Some(comma) => {
                (response[0..comma].to_string(), response[comma..response.len()].to_string())
            }, None => return Err(Error::CommunictionError),
        };
        if values.len() < 2 {
            return Err(Error::CommunictionError);
        }
        let splited_values: Vec<&str> = values[2..values.len()-1].split(",").collect();
        let splited_values: Vec<String> = splited_values.iter().map(|v| v.to_string()).collect();
        Ok((match error_id.parse::<isize>() {
            Ok(error_id) => error_id,  Err(_) => return Err(Error::CommunictionError),
        }, splited_values))

    }
    
}