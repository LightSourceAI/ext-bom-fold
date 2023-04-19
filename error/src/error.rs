//! Common error space for all LightSource projects.
//!
//! Built off of Google's standard status error-space, this crate provides a lingua franca for errors across services.

use crate::error_details::*;
use std::{convert::Infallible, fmt};

#[derive(Debug, Clone)]
pub struct ErrorPayload<T> {
    pub message: String,
    pub payload: Option<Box<T>>,
}

#[derive(Debug, Clone, Default)]
pub struct AbortedPayload {
    pub message: String,
    pub error_info: Option<ErrorInfo>,
    pub retry_info: Option<RetryInfo>,
}

#[derive(Debug, Clone)]
pub struct UnavailablePayload {
    pub message: String,
    pub debug_info: Option<DebugInfo>,
    pub retry_info: Option<RetryInfo>,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Common error type for all LightSource services.
///
/// Documentation for each subtype is adapted from the gRPC status code spec. These errors are enhanced
/// in that they encoded the googleapi recommended rich error details as part of the error.
#[derive(Debug)]
pub enum Error {
    /// Exists for completeness of mapping to gRPC status but this should typically not be used. At google
    /// they make a distinction between Status and StatusOr, whereas in rust land everything is `Result<T>`,
    /// and the equivalent of status is `Result<()>`.
    #[deprecated(
        note = "Do not use the Error::Ok variant. It exists for completeness but does not make sense in rust code."
    )]
    Ok,

    /// The operation was cancelled, typically by the caller.
    Cancelled(String),

    /// Unknown error. For example, this error may be returned when a Status value received from another
    /// address space belongs to an error space that is not known in this address space. Also errors
    /// raised by APIs that do not return enough error information may be converted to this error.
    Unknown(ErrorPayload<DebugInfo>),

    /// The client specified an invalid argument. Note that this differs from FAILED_PRECONDITION.
    /// INVALID_ARGUMENT indicates arguments that are problematic regardless of the state of the
    /// system (e.g., a malformed file name).
    InvalidArgument(ErrorPayload<BadRequest>),

    /// The deadline expired before the operation could complete. For operations that change the
    /// state of the system, this error may be returned even if the operation has completed
    /// successfully. For example, a successful response from a server could have been delayed long
    DeadlineExceeded(ErrorPayload<DebugInfo>),

    /// Some requested entity (e.g., file or directory) was not found. Note to server developers: if
    /// a request is denied for an entire class of users, such as gradual feature rollout or
    /// undocumented allowlist, NOT_FOUND may be used. If a request is denied for some users within a
    /// class of users, such as user-based access control, PERMISSION_DENIED must be used.
    NotFound(ErrorPayload<ResourceInfo>),

    /// The entity that a client attempted to create (e.g., file or directory) already exists.
    AlreadyExists(ErrorPayload<ResourceInfo>),

    /// The caller does not have permission to execute the specified operation. PERMISSION_DENIED
    /// must not be used for rejections caused by exhausting some resource (use RESOURCE_EXHAUSTED
    /// instead for those errors). PERMISSION_DENIED must not be used if the caller can not be
    /// identified (use UNAUTHENTICATED instead for those errors). This error code does not imply the
    /// request is valid or the requested entity exists or satisfies other pre-conditions.
    PermissionDenied(ErrorPayload<ErrorInfo>),

    /// Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file system
    /// is out of space.
    ResourceExhausted(ErrorPayload<QuotaFailure>),

    /// The operation was rejected because the system is not in a state required for the operation's
    /// execution. For example, the directory to be deleted is non-empty, an rmdir operation is
    /// applied to a non-directory, etc.
    ///
    /// Service implementors can use the following guidelines to decide between FAILED_PRECONDITION,
    /// ABORTED, and UNAVAILABLE:
    ///  (a) Use UNAVAILABLE if the client can retry just the failing call.
    ///  (b) Use ABORTED if the client should retry at a higher level (e.g., when a client-specified
    ///      test-and-set fails, indicating the client should restart a read-modify-write sequence).
    ///  (c) Use FAILED_PRECONDITION if the client should not retry until the system state has been
    ///      explicitly fixed. E.g., if an "rmdir" fails because the directory is non-empty,
    ///      FAILED_PRECONDITION should be returned since the client should not retry unless the
    ///      files are deleted from the directory.
    FailedPrecondition(ErrorPayload<PreconditionFailure>),

    /// The operation was aborted, typically due to a concurrency issue such as a sequencer check
    /// failure or transaction abort. See the guidelines above for deciding between
    /// FAILED_PRECONDITION, ABORTED, and UNAVAILABLE.
    Aborted(Box<AbortedPayload>),

    /// The operation was attempted past the valid range. E.g., seeking or reading past end-of-file.
    /// Unlike INVALID_ARGUMENT, this error indicates a problem that may be fixed if the system state
    /// changes. For example, a 32-bit file system will generate INVALID_ARGUMENT if asked to read at
    /// an offset that is not in the range [0,2^32-1], but it will generate OUT_OF_RANGE if asked to
    /// read from an offset past the current file size. There is a fair bit of overlap between
    /// FAILED_PRECONDITION and OUT_OF_RANGE. We recommend using OUT_OF_RANGE (the more specific
    /// error) when it applies so that callers who are iterating through a space can easily look for
    /// an OUT_OF_RANGE error to detect when they are done.
    OutOfRange(ErrorPayload<BadRequest>),

    /// The operation is not implemented or is not supported/enabled in this service.
    Unimplemented(String),

    /// Internal errors. This means that some invariants expected by the underlying system have been
    /// broken. This error code is reserved for serious errors.
    Internal(ErrorPayload<DebugInfo>),

    /// The service is currently unavailable. This is most likely a transient condition, which can
    /// be corrected by retrying with a backoff. Note that it is not always safe to retry
    /// non-idempotent operations.
    Unavailable(Box<UnavailablePayload>),

    /// Unrecoverable data loss or corruption.
    DataLoss(ErrorPayload<DebugInfo>),

    /// The request does not have valid authentication credentials for the operation.
    Unauthenticated(ErrorPayload<ErrorInfo>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Internal(ref payload) => {
                write!(
                    f,
                    "Internal: msg=\"{}\" detail=\"{}\"",
                    payload.message,
                    payload
                        .payload
                        .as_ref()
                        .and_then(|info| info.detail.as_deref())
                        .unwrap_or("None")
                )
            }
            _ => write!(f, "{:?}", &self),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Code that is provided to client applications.
    pub const fn client_code(&self) -> &'static str {
        match self {
            #[allow(deprecated)]
            Error::Ok => "OK",
            Error::Cancelled(_) => "CANCELLED",
            Error::Unknown(_) => "UNKNOWN",
            Error::InvalidArgument(_) => "INVALID_ARGUMENT",
            Error::DeadlineExceeded(_) => "DEADLINE_EXCEEDED",
            Error::NotFound(_) => "NOT_FOUND",
            Error::AlreadyExists(_) => "ALREADY_EXISTS",
            Error::PermissionDenied(_) => "PERMISSION_DENIED",
            Error::ResourceExhausted(_) => "RESOURCE_EXHAUSTED",
            Error::FailedPrecondition(_) => "FAILED_PRECONDITION",
            Error::Aborted(_) => "ABORTED",
            Error::OutOfRange(_) => "OUT_OF_RANGE",
            Error::Unimplemented(_) => "UNIMPLEMENTED",
            Error::Internal(_) => "INTERNAL",
            Error::Unavailable(_) => "UNAVAILABLE",
            Error::DataLoss(_) => "DATA_LOSS",
            Error::Unauthenticated(_) => "UNAUTHENTICATED",
        }
    }

    pub fn message(&self) -> &str {
        match self {
            #[allow(deprecated)]
            Error::Ok => "",
            Error::Cancelled(message) => message,
            Error::Unknown(inner) => &inner.message,
            Error::InvalidArgument(inner) => &inner.message,
            Error::DeadlineExceeded(inner) => &inner.message,
            Error::NotFound(inner) => &inner.message,
            Error::AlreadyExists(inner) => &inner.message,
            Error::PermissionDenied(inner) => &inner.message,
            Error::ResourceExhausted(inner) => &inner.message,
            Error::FailedPrecondition(inner) => &inner.message,
            Error::Aborted(inner) => &inner.message,
            Error::OutOfRange(inner) => &inner.message,
            Error::Unimplemented(message) => message,
            Error::Internal(inner) => &inner.message,
            Error::Unavailable(inner) => &inner.message,
            Error::DataLoss(inner) => &inner.message,
            Error::Unauthenticated(inner) => &inner.message,
        }
    }
}

impl Error {
    #[inline(always)]
    pub fn cancelled<S: Into<String>>(message: S) -> Self {
        Error::Cancelled(message.into())
    }

    #[inline(always)]
    pub fn unknown<S: Into<String>>(message: S) -> Self {
        Error::Unknown(ErrorPayload {
            message: message.into(),
            payload: DebugInfo::collect().map(Box::new),
        })
    }

    #[inline(always)]
    pub fn unknown_with<S: Into<String>>(message: S, debug_info: Option<DebugInfo>) -> Self {
        Error::Unknown(ErrorPayload { message: message.into(), payload: debug_info.map(Box::new) })
    }

    #[inline(always)]
    pub fn invalid_argument<S: Into<String>>(message: S) -> Self {
        Error::InvalidArgument(ErrorPayload { message: message.into(), payload: None })
    }

    #[inline(always)]
    pub fn invalid_argument_with<S: Into<String>>(
        message: S,
        bad_request: Option<BadRequest>,
    ) -> Self {
        Error::InvalidArgument(ErrorPayload {
            message: message.into(),
            payload: bad_request.map(Box::new),
        })
    }

    #[inline(always)]
    pub fn deadline_exceeded<S: Into<String>>(message: S) -> Self {
        Error::DeadlineExceeded(ErrorPayload {
            message: message.into(),
            payload: DebugInfo::collect().map(Box::new),
        })
    }

    #[inline(always)]
    pub fn deadline_exceeded_with<S: Into<String>>(
        message: S,
        debug_info: Option<DebugInfo>,
    ) -> Self {
        Error::DeadlineExceeded(ErrorPayload {
            message: message.into(),
            payload: debug_info.map(Box::new),
        })
    }

    #[inline(always)]
    pub fn not_found<S: Into<String>>(message: S) -> Self {
        Error::NotFound(ErrorPayload { message: message.into(), payload: None })
    }

    #[inline(always)]
    pub fn not_found_with<S: Into<String>>(
        message: S,
        resource_info: Option<ResourceInfo>,
    ) -> Self {
        Error::NotFound(ErrorPayload {
            message: message.into(),
            payload: resource_info.map(Box::new),
        })
    }

    #[inline(always)]
    pub fn already_exists<S: Into<String>>(message: S) -> Self {
        Error::AlreadyExists(ErrorPayload { message: message.into(), payload: None })
    }

    #[inline(always)]
    pub fn already_exists_with<S: Into<String>>(
        message: S,
        resource_info: Option<ResourceInfo>,
    ) -> Self {
        Error::AlreadyExists(ErrorPayload {
            message: message.into(),
            payload: resource_info.map(Box::new),
        })
    }

    #[inline(always)]
    pub fn permission_denied<S: Into<String>>(message: S) -> Self {
        Error::PermissionDenied(ErrorPayload { message: message.into(), payload: None })
    }

    #[inline(always)]
    pub fn permission_denied_with<S: Into<String>>(
        message: S,
        error_info: Option<ErrorInfo>,
    ) -> Self {
        Error::PermissionDenied(ErrorPayload {
            message: message.into(),
            payload: error_info.map(Box::new),
        })
    }

    #[inline(always)]
    pub fn resource_exhausted<S: Into<String>>(message: S) -> Self {
        Error::ResourceExhausted(ErrorPayload { message: message.into(), payload: None })
    }

    #[inline(always)]
    pub fn resource_exhausted_with<S: Into<String>>(
        message: S,
        quota_failure: Option<QuotaFailure>,
    ) -> Self {
        Error::ResourceExhausted(ErrorPayload {
            message: message.into(),
            payload: quota_failure.map(Box::new),
        })
    }

    #[inline(always)]
    pub fn failed_precondition<S: Into<String>>(message: S) -> Self {
        Error::FailedPrecondition(ErrorPayload { message: message.into(), payload: None })
    }

    #[inline(always)]
    pub fn failed_precondition_with<S: Into<String>>(
        message: S,
        precondition_failure: Option<PreconditionFailure>,
    ) -> Self {
        Error::FailedPrecondition(ErrorPayload {
            message: message.into(),
            payload: precondition_failure.map(Box::new),
        })
    }

    #[inline(always)]
    pub fn aborted<S: Into<String>>(message: S) -> Self {
        Error::Aborted(Box::new(AbortedPayload { message: message.into(), ..Default::default() }))
    }

    #[inline(always)]
    pub fn aborted_with<S: Into<String>>(
        message: S,
        error_info: Option<ErrorInfo>,
        retry_info: Option<RetryInfo>,
    ) -> Self {
        Error::Aborted(Box::new(AbortedPayload { message: message.into(), error_info, retry_info }))
    }

    #[inline(always)]
    pub fn out_of_range<S: Into<String>>(message: S) -> Self {
        Error::OutOfRange(ErrorPayload { message: message.into(), payload: None })
    }

    #[inline(always)]
    pub fn out_of_range_with<S: Into<String>>(message: S, bad_request: Option<BadRequest>) -> Self {
        Error::OutOfRange(ErrorPayload {
            message: message.into(),
            payload: bad_request.map(Box::new),
        })
    }

    #[inline(always)]
    pub fn unimplemented<S: Into<String>>(message: S) -> Self {
        Error::Unimplemented(message.into())
    }

    #[inline(always)]
    pub fn internal<S: Into<String>>(message: S) -> Self {
        Error::Internal(ErrorPayload {
            message: message.into(),
            payload: DebugInfo::collect().map(Box::new),
        })
    }

    #[inline(always)]
    pub fn internal_with<S: Into<String>>(message: S, debug_info: Option<DebugInfo>) -> Self {
        Error::Internal(ErrorPayload { message: message.into(), payload: debug_info.map(Box::new) })
    }

    #[inline(always)]
    pub fn unavailable<S: Into<String>>(message: S) -> Self {
        Error::Unavailable(Box::new(UnavailablePayload {
            message: message.into(),
            debug_info: DebugInfo::collect(),
            retry_info: None,
        }))
    }

    #[inline(always)]
    pub fn unavailable_with<S: Into<String>>(
        message: S,
        debug_info: Option<DebugInfo>,
        retry_info: Option<RetryInfo>,
    ) -> Self {
        Error::Unavailable(Box::new(UnavailablePayload {
            message: message.into(),
            debug_info: debug_info.or_else(DebugInfo::collect),
            retry_info,
        }))
    }

    #[inline(always)]
    pub fn data_loss<S: Into<String>>(message: S) -> Self {
        Error::DataLoss(ErrorPayload {
            message: message.into(),
            payload: DebugInfo::collect().map(Box::new),
        })
    }

    #[inline(always)]
    pub fn data_loss_with<S: Into<String>>(message: S, debug_info: Option<DebugInfo>) -> Self {
        Error::DataLoss(ErrorPayload { message: message.into(), payload: debug_info.map(Box::new) })
    }

    #[inline(always)]
    pub fn unauthenticated<S: Into<String>>(message: S) -> Self {
        Error::Unauthenticated(ErrorPayload { message: message.into(), payload: None })
    }

    #[inline(always)]
    pub fn unauthenticated_with<S: Into<String>>(
        message: S,
        error_info: Option<ErrorInfo>,
    ) -> Self {
        Error::Unauthenticated(ErrorPayload {
            message: message.into(),
            payload: error_info.map(Box::new),
        })
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        panic!("Err(Infallible) should never be returned")
    }
}

/// Macro to construct and error and trace the error detail with `ERROR` level.
#[macro_export]
macro_rules! trace_err {
    ($e:ident, $message:tt) => {{
        let e = Error::$e($message);
        tracing::error!(?e);
        e
    }};
    ($e:ident, $message:tt, $($args:tt),*) => {{
        let e = Error::$e(format!($message, $($args),*));
        tracing::error!(?e);
        e
    }};
}

#[test]
fn test_macro() {
    let e = trace_err!(internal, "foo");
    assert_eq!(e.message(), "foo");
    let e = trace_err!(internal, "Hello {}! {}", "bar", 123);
    assert_eq!(e.message(), "Hello bar! 123");
    assert!(matches!(e, Error::Internal(_)));
}
