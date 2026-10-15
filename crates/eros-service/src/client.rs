use eros_core::{Reply, Request, MAX_FRAME};
use serde::de::DeserializeOwned;
use std::io::{self, BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::time::Duration;

pub fn socket_path() -> Result<PathBuf, String> {
    super::paths::socket_path().map_err(|error| error.to_string())
}

pub fn connect() -> Result<UnixStream, String> {
    UnixStream::connect(socket_path()?).map_err(|_| {
        "O serviço local do Eros não está disponível. Feche e abra o aplicativo para reconectar.".into()
    })
}

pub fn request<T: DeserializeOwned>(request: Request) -> Result<T, String> {
    let mut stream = connect()?;
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .and_then(|_| stream.set_write_timeout(Some(Duration::from_secs(30))))
        .map_err(|_| "Não foi possível configurar o canal local.".to_string())?;
    write_frame(&mut stream, &request)?;
    let mut reader = BufReader::new(stream);
    let frame = read_frame(&mut reader)
        .map_err(|_| "Não foi possível ler a resposta do serviço local.".to_string())?;
    decode_reply(frame)
}

pub fn open_subscription(conversation_id: String) -> Result<BufReader<UnixStream>, String> {
    open_event_subscription(Request::Subscribe { conversation_id })
}

pub fn open_activity_subscription() -> Result<BufReader<UnixStream>, String> {
    open_event_subscription(Request::SubscribeActivity)
}

fn open_event_subscription(request: Request) -> Result<BufReader<UnixStream>, String> {
    let stream = connect()?;
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .and_then(|_| stream.set_write_timeout(Some(Duration::from_secs(30))))
        .map_err(|_| "Não foi possível configurar o canal local.".to_string())?;
    let mut reader = BufReader::new(stream);
    write_frame(reader.get_mut(), &request)?;
    let frame = read_frame(&mut reader)
        .map_err(|_| "Não foi possível ler a resposta do serviço local.".to_string())?;
    let reply: Reply = serde_json::from_slice(&frame)
        .map_err(|_| "O serviço local respondeu em um formato inválido.".to_string())?;
    if !reply.ok {
        return Err(reply
            .error
            .unwrap_or_else(|| "Não foi possível conectar ao terminal.".into()));
    }
    reader
        .get_ref()
        .set_read_timeout(None)
        .map_err(|_| "Não foi possível acompanhar o terminal.".to_string())?;
    Ok(reader)
}

pub fn read_frame(reader: &mut impl BufRead) -> io::Result<Vec<u8>> {
    let mut frame = Vec::with_capacity(1024);
    loop {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "IPC stream closed",
            ));
        }
        let consumed = match available.iter().position(|byte| *byte == b'\n') {
            Some(index) => index + 1,
            None => available.len(),
        };
        let complete = available[..consumed].last() == Some(&b'\n');
        let payload_bytes = consumed - if complete { 1 } else { 0 };
        if frame.len().saturating_add(payload_bytes) > MAX_FRAME {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "IPC frame too large",
            ));
        }
        frame.extend_from_slice(&available[..consumed]);
        reader.consume(consumed);
        if complete {
            frame.pop();
            return Ok(frame);
        }
    }
}

pub fn write_frame(writer: &mut impl Write, value: &impl serde::Serialize) -> Result<(), String> {
    let mut frame = serde_json::to_vec(value).map_err(|error| error.to_string())?;
    if frame.len() > MAX_FRAME {
        return Err("A mensagem excede o limite local de IPC.".into());
    }
    frame.push(b'\n');
    writer
        .write_all(&frame)
        .and_then(|_| writer.flush())
        .map_err(|_| "Não foi possível falar com o serviço local.".into())
}

fn decode_reply<T: DeserializeOwned>(frame: Vec<u8>) -> Result<T, String> {
    let reply: Reply = serde_json::from_slice(&frame)
        .map_err(|_| "O serviço local respondeu em um formato inválido.".to_string())?;
    if !reply.ok {
        return Err(reply
            .error
            .unwrap_or_else(|| "A operação não foi concluída.".into()));
    }
    serde_json::from_value(reply.data)
        .map_err(|_| "O serviço local respondeu com dados incompatíveis.".to_string())
}
