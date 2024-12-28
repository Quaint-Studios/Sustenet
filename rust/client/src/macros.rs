/// This calls loop and select! in a single macro.
#[macro_export]
macro_rules! lselect {
    ($($select_body:tt)*) => {
        loop {
            tokio::select! {
                $($select_body)*
            }
        }
    };
}

/// This reads a u8 for the length and then calls read_exact
/// to read the bytes then converts it to a string.
/// 
/// This is used in a loop to continue if an error occurs.
#[macro_export]
macro_rules! lread_string {
    ($reader:expr, $error:expr, $name:expr) => {
        {
        let len = match $reader.read_u8().await {
            Ok(len) => len,
            Err(e) => {
                $error(format!("Failed to read the {} len. {:?}", $name, e).as_str());
                continue;
            }
        } as usize;

        let mut val = vec![0u8; len];
        match $reader.read_exact(&mut val).await {
            Ok(_) => (),
            Err(e) => {
                $error(format!("Failed to read the {}. {:?}", $name, e).as_str());
                continue;
            }
        };

        match String::from_utf8(val) {
            Ok(val) => val,
            Err(e) => {
                $error(format!("Failed to convert the {} to a String. {:?}", $name, e).as_str());
                continue;
            }
        }
        }
    };
}
