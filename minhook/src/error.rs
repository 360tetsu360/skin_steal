use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::minhook::*;

#[derive(Debug)]
pub enum MhError {
    Unknown,
    AlreadyInitialized,
    NotInitialized,
    AlreadyCreated,
    NotCreated,
    Enabled,
    Disabled,
    NotExecutable,
    UnsupportedFunction,
    MemoryAlloc,
    MemoryProtect,
    ModuleNotFound,
    FunctionNotFound,
}

impl MhError {
    pub(crate) fn from(status: MH_STATUS) -> Self {
        match status {
            MH_STATUS_MH_UNKNOWN => MhError::Unknown,
            MH_STATUS_MH_ERROR_ALREADY_INITIALIZED => MhError::AlreadyInitialized,
            MH_STATUS_MH_ERROR_NOT_INITIALIZED => MhError::NotInitialized,
            MH_STATUS_MH_ERROR_ALREADY_CREATED => MhError::AlreadyCreated,
            MH_STATUS_MH_ERROR_NOT_CREATED => MhError::NotCreated,
            MH_STATUS_MH_ERROR_ENABLED => MhError::Enabled,
            MH_STATUS_MH_ERROR_DISABLED => MhError::Disabled,
            MH_STATUS_MH_ERROR_NOT_EXECUTABLE => MhError::NotExecutable,
            MH_STATUS_MH_ERROR_UNSUPPORTED_FUNCTION => MhError::UnsupportedFunction,
            MH_STATUS_MH_ERROR_MEMORY_ALLOC => MhError::MemoryAlloc,
            MH_STATUS_MH_ERROR_MEMORY_PROTECT => MhError::MemoryProtect,
            MH_STATUS_MH_ERROR_MODULE_NOT_FOUND => MhError::ModuleNotFound,
            MH_STATUS_MH_ERROR_FUNCTION_NOT_FOUND => MhError::FunctionNotFound,
            _ => MhError::Unknown,
        }
    }
}

impl Display for MhError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MhError::Unknown => write!(f, "Unknown"),
            MhError::AlreadyInitialized => write!(f, "AlreadyInitialized"),
            MhError::NotInitialized => write!(f, "NotInitialized"),
            MhError::AlreadyCreated => write!(f, "AlreadyCreated"),
            MhError::NotCreated => write!(f, "NotCreated"),
            MhError::Enabled => write!(f, "Enabled"),
            MhError::Disabled => write!(f, "Disabled"),
            MhError::NotExecutable => write!(f, "NotExecutable"),
            MhError::UnsupportedFunction => write!(f, "UnsupportedFunction"),
            MhError::MemoryAlloc => write!(f, "MemoryAlloc"),
            MhError::MemoryProtect => write!(f, "MemoryProtect"),
            MhError::ModuleNotFound => write!(f, "ModuleNotFound"),
            MhError::FunctionNotFound => write!(f, "FunctionNotFound"),
        }
    }
}

impl Error for MhError {}
