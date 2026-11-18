use serde::{Deserialize, Serialize};

use crate::CameraInterface;


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Message {
    CameraInterfaceUpdate(CameraInterface),
}