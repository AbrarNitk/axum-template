# Error Handling in Axum with Tracing

A comprehensive guide to implementing robust, user-friendly error handling in Axum applications with structured logging and tracing.

## Overview

This guide explains our error handling architecture that provides three levels of error information while maintaining clean separation of concerns between services and controllers.

## Why This Approach?

### Problems with Traditional Error Handling
- **Poor User Experience**: Technical error messages confuse end users
- **Debugging Difficulty**: Lack of structured error information
- **Inconsistent Responses**: Different error formats across endpoints
- **Security Risks**: Exposing internal system details
- **Boilerplate Code**: Duplicating status code and error code mappings

### Our Solution
- **Three-Level Error Structure**: User-friendly, technical context, and detailed debugging
- **Single Trait Design**: Simple `ResponseError` trait for all error types
- **Automatic Status Code Derivation**: Status codes automatically derived from error codes
- **Structured Logging**: Rich context for debugging and monitoring
- **Flexible Override**: Services can customize error messages and status codes as needed

## Error Structure

```rust
pub struct ApiError {
    trace_id: String,           // For log correlation
    timestamp: DateTime<Utc>,   // When the error occurred
    code: ErrorCode,            // Categorized error type
    status: StatusCode,         // HTTP status code (auto-derived)
    message: String,            // User-friendly message
    description: Option<String>, // Technical context
    details: Option<String>,    // Full debugging details
}
```

### Three Information Levels

1. **`message`** - What users see and act upon
2. **`description`** - Technical context for developers
3. **`details`** - Complete debugging information with backtraces

## Implementation Pattern

### 1. Service Errors

Services implement `ResponseError` with minimal boilerplate:

```rust
#[derive(thiserror::Error, Debug)]
pub enum TemplateServiceError {
    #[error("Template not found with ID: {0}")]
    NotFound(String),
    #[error("Invalid request data")]
    BadRequest,
}

impl ResponseError for TemplateServiceError {
    // Only implement error_code - status_code is automatically derived
    fn error_code(&self) -> ErrorCode {
        match self {
            Self::NotFound(_) => ErrorCode::NotFound,
            Self::BadRequest => ErrorCode::BadRequest,
        }
    }

    // Override only when thiserror message isn't user-friendly
    fn user_message(&self) -> String {
        match self {
            Self::NotFound(_) => "The requested template could not be found".to_string(),
            Self::BadRequest => self.to_string(), // Use thiserror message
        }
    }

    fn technical_description(&self) -> Option<String> {
        match self {
            Self::NotFound(id) => Some(format!("Template with ID '{}' was not found in the database", id)),
            Self::BadRequest => Some("Request validation failed - missing required fields".to_string()),
        }
    }
}
```

### 2. Controller Usage

Controllers can use service errors directly or wrap them:

```rust
// Direct usage (recommended for simple cases)
pub async fn get_template(
    Path(id): Path<String>,
) -> impl IntoResponse {
    match service::template::get(id.clone()).await {
        Ok(template) => response::success(template),
        Err(e) => {
            error!(error = %e, template_id = %id, "Failed to get template");
            response::error::response("template.get", e)
        }
    }
}

// Wrapped usage (when you need controller-specific errors)
#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("TemplateError: {0}")]
    TemplateService(#[from] service::template::ServiceError),
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}
```

### 3. Error Response Function

The `response()` function automatically builds rich error responses:

```rust
pub fn response<Err>(trace_id: &str, err: Err) -> Response
where
    Err: ResponseError,
{
    ApiError {
        trace_id: trace_id.to_string(),
        timestamp: Utc::now(),
        code: err.error_code(),
        status: err.status_code(), // Automatically derived from error_code
        message: err.user_message(),
        description: err.technical_description(),
        details: err.technical_details(),
    }
    .into_response()
}
```

## Key Design Decisions

### 1. Single Trait Approach
- **Why**: Eliminates confusion about which trait to implement
- **Benefit**: Clear, consistent pattern across all error types
- **Alternative**: Could have separate mapping traits, but adds complexity

### 2. Automatic Status Code Derivation
- **Why**: Eliminates boilerplate - no need to implement both methods
- **Benefit**: Single source of truth (error_code determines status_code)
- **Override**: Can still override `status_code()` for special cases
- **Mapping**: Clear 1:1 relationship between error codes and HTTP status codes

### 3. Three-Level Information
- **Why**: Different audiences need different levels of detail
- **Benefit**: Better user experience + rich debugging information
- **Control**: Backend decides what technical information to expose

### 4. thiserror Integration
- **Why**: Leverages Rust's excellent error handling ecosystem
- **Benefit**: Automatic error conversion, source tracking, and backtraces
- **Override**: Can provide user-friendly messages when thiserror messages are too technical

### 5. Controller vs Service Errors
- **Service Errors**: Handle domain-specific error cases
- **Controller Errors**: Handle cross-cutting concerns (rate limiting, validation)
- **Flexibility**: Use service errors directly or wrap as needed

## Status Code Mapping

Status codes are automatically derived from error codes:

```rust
impl From<ErrorCode> for StatusCode {
    fn from(error_code: ErrorCode) -> Self {
        match error_code {
            ErrorCode::NotFound => StatusCode::NOT_FOUND,           // 404
            ErrorCode::BadRequest => StatusCode::BAD_REQUEST,       // 400
            ErrorCode::UnAuthorized => StatusCode::UNAUTHORIZED,    // 401
            ErrorCode::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR, // 500
        }
    }
}
```

### Overriding Status Codes

For special cases, you can still override the status code:

```rust
impl ResponseError for SpecialServiceError {
    fn error_code(&self) -> ErrorCode { ErrorCode::BadRequest }
    
    // Override for special cases
    fn status_code(&self) -> StatusCode {
        match self {
            SpecialServiceError::ValidationError => StatusCode::UNPROCESSABLE_ENTITY, // 422 instead of 400
            _ => self.error_code().into(), // Use default mapping
        }
    }
}
```

## Best Practices

### 1. Error Message Design
```rust
// Good: User-friendly and actionable
"Please provide a valid email address"

// Bad: Technical and confusing
"Email validation failed: regex mismatch"
```

### 2. Technical Context
```rust
// Good: Specific but not overwhelming
"User with ID '123' was not found in the database"

// Bad: Too vague
"Database error occurred"
```

### 3. Logging Strategy
```rust
// Log at controller level with rich context
error!(
    error = %e,
    template_id = %id,
    user_id = %user_id,
    "Failed to get template"
);
```

### 4. Error Code Consistency
```rust
pub enum ErrorCode {
    NotFound,           // 404 errors
    BadRequest,         // 400 errors
    UnAuthorized,       // 401 errors
    InternalServerError, // 500 errors
}
```

## Example API Response

```json
{
  "success": false,
  "error": {
    "trace_id": "template.get.abc123",
    "timestamp": "2024-01-15T10:30:00Z",
    "code": "NotFound",
    "status": 404,
    "message": "The requested template could not be found",
    "description": "Template with ID 'abc123' was not found in the database",
    "details": "Template lookup failed for ID: abc123. Database query returned no results."
  }
}
```

## Benefits

✅ **User Experience**: Clear, actionable error messages  
✅ **Developer Experience**: Rich technical context for debugging  
✅ **Monitoring**: Structured logging with trace IDs  
✅ **Security**: Controlled exposure of technical details  
✅ **Maintainability**: Consistent pattern across all services  
✅ **Flexibility**: Override as much or as little as needed  
✅ **Less Boilerplate**: Only implement `error_code()`, status code is automatic  

## Migration Path

1. **Start Simple**: Implement `ResponseError` for your service errors (only `error_code()`)
2. **Add Logging**: Use tracing macros in controllers
3. **Customize Messages**: Override methods when thiserror messages aren't user-friendly
4. **Add Controller Errors**: When you need cross-cutting error handling
5. **Override Status Codes**: Only when you need different HTTP status codes

## Implementation Checklist

- [ ] Implement `ResponseError` trait with `error_code()` method
- [ ] Override `user_message()` for user-friendly messages when needed
- [ ] Override `technical_description()` for technical context
- [ ] Override `technical_details()` for debugging information (optional)
- [ ] Override `status_code()` only for special cases (rarely needed)
- [ ] Use tracing macros in controllers for structured logging

This error handling system transforms basic HTTP errors into a comprehensive, user-friendly experience while eliminating boilerplate code and maintaining flexibility for special cases. 