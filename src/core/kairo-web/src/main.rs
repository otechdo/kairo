use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;
use std::path::PathBuf;
use std::thread;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let public_dir: &Path = Path::new("web"); // Dossier à servir

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        let public_dir: &Path = public_dir;

        thread::spawn(move || {
            handle_connection(stream, &public_dir);
        });
    }
}

fn handle_connection(mut stream: TcpStream, public_dir: &Path) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let file_path: PathBuf = public_dir.join(filename);
    let contents: String = fs::read_to_string(file_path)
        .unwrap_or_else(|_| format!("Erreur : Fichier '{}' introuvable", filename));

    let response: String = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
