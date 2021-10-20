use std::{future::Future, path::PathBuf};

#[cfg(feature = "async_tokio")]
pub fn spawn<T>(future: T) -> tokio::task::JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::task::spawn(future)
}

pub fn normalize_path<S: AsRef<str>>(path: S) -> String {
    use normpath::BasePathBuf;
    use normpath::PathExt;
    use std::str::FromStr;
    let p = PathBuf::from_str(path.as_ref()).unwrap();
    let p = p.normalize().unwrap();
    p.as_path().to_string_lossy().to_string()
}
