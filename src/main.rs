use serialport;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;

fn main() {

    let ports = serialport::available_ports().expect("No ports found!");

    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/ttyUSB0", 19200)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .parity(Parity::None)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    // Receiver code snippet
    let mut buffer: Vec<u8> = vec![0;10];

    let mut output_file =
        File::create("resources/received.txt").expect("unable to create a file");

    // Read in a loop1..

    loop {
        port.clear(serialport::ClearBuffer::Input)
            .expect("unable to clear the buffer");

        //port.flush().expect("unable to flush");

        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let received_data = &buffer[..bytes_read];

                    if std::str::from_utf8(received_data).is_ok() {
                        let utf_8_data = String::from_utf8_lossy(received_data);
                        output_file
                            .write_all(&received_data)
                            .expect("unable to write data");

                        println!("Received: {:?}", utf_8_data);

                        port.flush().expect("unable to flush");
                        //std::thread::sleep(Duration::from_millis(100)); // Adjust sleep duration as needed
                    } else {
                        eprintln!("Received non-UTF-8 data");
                    }
                    //         let received_data = &buffer[..bytes_read];
                    //         output_file
                    //             .write_all(&received_data)
                    //             .expect("unable to write data ");
                    //
                    //         let utf_8_data = String::from_utf8_lossy(received_data);
                    //
                    //         println!("Received: {:?}", utf_8_data);
                    //         std::thread::sleep(Duration::from_millis(10)); // Adjust sleep duration as needed
                    //     }
                    // }
                }
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                println!("Timeout occurred. Waiting for more data...");
                std::thread::sleep(Duration::from_millis(1000)); // Adjust sleep duration as needed
            }
            Err(e) => {
                eprintln!("Error reading from serial port: {}", e);
                 break; // Break out of the loop on error
            }
        }
    }

}

