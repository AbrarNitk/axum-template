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

### Our Solution
- **Three-Level Error Structure**: User-friendly, technical context, and detailed debugging
- **Single Trait Design**: Simple `ResponseError` trait for all error types
- **Structured Logging**: Rich context for debugging and monitoring
- **Flexible Override**: Services can customize error messages as needed

## Error Structure

```rust
pub struct ApiError {
    trace_id: String,           // For log correlation
    timestamp: DateTime<Utc>,   // When the error occurred
    code: ErrorCode,            // Categorized error type
    status: StatusCode,         // HTTP status code
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

Services implement `ResponseError` directly:

```rust
#[derive(thiserror::Error, Debug)]
pub enum TemplateServiceError {
    #[error("Template not found with ID: {0}")]
    NotFound(String),
    #[error("Invalid request data")]
    BadRequest,
}

impl ResponseError for TemplateServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::BadRequest => StatusCode::BAD_REQUEST,
        }
    }

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
        status: err.status_code(),
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

### 2. Three-Level Information
- **Why**: Different audiences need different levels of detail
- **Benefit**: Better user experience + rich debugging information
- **Control**: Backend decides what technical information to expose

### 3. thiserror Integration
- **Why**: Leverages Rust's excellent error handling ecosystem
- **Benefit**: Automatic error conversion, source tracking, and backtraces
- **Override**: Can provide user-friendly messages when thiserror messages are too technical

### 4. Controller vs Service Errors
- **Service Errors**: Handle domain-specific error cases
- **Controller Errors**: Handle cross-cutting concerns (rate limiting, validation)
- **Flexibility**: Use service errors directly or wrap as needed

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

## Migration Path

1. **Start Simple**: Implement `ResponseError` for your service errors
2. **Add Logging**: Use tracing macros in controllers
3. **Customize Messages**: Override methods when thiserror messages aren't user-friendly
4. **Add Controller Errors**: When you need cross-cutting error handling

This error handling system transforms basic HTTP errors into a comprehensive, user-friendly experience that helps everyone understand and resolve issues quickly. 