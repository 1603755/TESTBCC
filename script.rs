use std::f32::consts::E;
use std::time::{SystemTime, UNIX_EPOCH};
use serial2::SerialPort;
use std::ops::Deref;
use std::io::{Error, ErrorKind};
use serde_json::json;
use reqwest;


pub const PASSWORD : &str = "root_password";
pub const USER : &str = "root";


#[tokio::main]
async fn main() -> Result<(), reqwest::Error>  {
    println!("a");
        // En Windows, el nombre del puerto COM6 es "COM6"
    let port_name = "COM6";
    // Abre el puerto con una velocidad de transmisión de 57600 bps
    let mut port = SerialPort::open(port_name, 57600).unwrap();
    
    let mut registry: (u64, Vec<u8>) = (0, Vec::new());
    println!("b");
    // Bucle principal para leer y escribir continuamente
    loop {
        // Crea un búfer para leer y escribir datos
        let mut buffer = [0; 1];

        // Intenta leer datos desde el puerto
        match port.read(&mut buffer) {
            Ok(read) => {
                println!("c");
                // Si se leyeron datos, escríbelos de vuelta al puerto
                let now = SystemTime::now();
                let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
                println!("{:?}", registry);
                if since_epoch.as_secs() > registry.0 {
                    let hex_string: String = registry.1
                    .iter()
                    .map(|byte| format!("{:02X}", byte))
                    .collect();
                    let post_data = json!({
                        "timestamp": "",
                        "hex": hex_string,
                    });
                    let response = reqwest::Client::new()
                        .post("http://200.234.230.98/cafe")
                        .json(&post_data)
                        .send()
                        .await?;

                    // Verificar si la solicitud fue exitosa (código de estado 2xx)
                    if response.status().is_success() {
                        // Imprimir el cuerpo de la respuesta como una cadena
                        let body = response.text().await?;
                        println!("Response body:\n{}", body);
                    } else {
                        println!("Request failed with status code: {}", response.status());
                    }

                    registry.0 = since_epoch.as_secs();
                    registry.1 = Vec::new();

                } 
                registry.1.extend_from_slice(&buffer);
                println!("{:?}, {:?}",buffer,since_epoch.as_secs());
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // Ignora los errores de timeout y continúa el bucle
                println!("{}",e);
                continue;
            }
            Err(_) => {
                continue;
            }
        
        }
    }
}