#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>AWS CodePipeline</fullname>
//! <p>
//! <b>Overview</b>
//! </p>
//! <p>This is the AWS CodePipeline API Reference. This guide provides descriptions of the
//! actions and data types for AWS CodePipeline. Some functionality for your pipeline can
//! only be configured through the API. For more information, see the <a href="https://docs.aws.amazon.com/codepipeline/latest/userguide/welcome.html">AWS
//! CodePipeline User Guide</a>.</p>
//! <p>You can use the AWS CodePipeline API to work with pipelines, stages, actions, and
//! transitions.</p>
//! <p>
//! <i>Pipelines</i> are models of automated release processes. Each pipeline
//! is uniquely named, and consists of stages, actions, and transitions. </p>
//! <p>You can work with pipelines by calling:</p>
//! <ul>
//! <li>
//! <p>
//! <a>CreatePipeline</a>, which creates a uniquely named
//! pipeline.</p>
//! </li>
//! <li>
//! <p>
//! <a>DeletePipeline</a>, which deletes the specified
//! pipeline.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetPipeline</a>, which returns information about the pipeline
//! structure and pipeline metadata, including the pipeline Amazon Resource Name
//! (ARN).</p>
//! </li>
//! <li>
//! <p>
//! <a>GetPipelineExecution</a>, which returns information about a
//! specific execution of a pipeline.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetPipelineState</a>, which returns information about the current
//! state of the stages and actions of a pipeline.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListActionExecutions</a>, which returns action-level details
//! for past executions. The details include full stage and action-level details,
//! including individual action duration, status, any errors that occurred during
//! the execution, and input and output artifact location details.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListPipelines</a>, which gets a summary of all of the pipelines
//! associated with your account.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListPipelineExecutions</a>, which gets a summary of the most
//! recent executions for a pipeline.</p>
//! </li>
//! <li>
//! <p>
//! <a>StartPipelineExecution</a>, which runs the most recent revision of
//! an artifact through the pipeline.</p>
//! </li>
//! <li>
//! <p>
//! <a>StopPipelineExecution</a>, which stops the specified pipeline
//! execution from continuing through the pipeline.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdatePipeline</a>, which updates a pipeline with edits or changes
//! to the structure of the pipeline.</p>
//! </li>
//! </ul>
//! <p>Pipelines include <i>stages</i>. Each stage contains one or more
//! actions that must complete before the next stage begins. A stage results in success or
//! failure. If a stage fails, the pipeline stops at that stage and remains stopped until
//! either a new version of an artifact appears in the source location, or a user takes
//! action to rerun the most recent artifact through the pipeline. You can call <a>GetPipelineState</a>, which displays the status of a pipeline, including the
//! status of stages in the pipeline, or <a>GetPipeline</a>, which returns the
//! entire structure of the pipeline, including the stages of that pipeline. For more
//! information about the structure of stages and actions, see <a href="https://docs.aws.amazon.com/codepipeline/latest/userguide/pipeline-structure.html">AWS CodePipeline
//! Pipeline Structure Reference</a>.</p>
//! <p>Pipeline stages include <i>actions</i> that are categorized into
//! categories such as source or build actions performed in a stage of a pipeline. For
//! example, you can use a source action to import artifacts into a pipeline from a source
//! such as Amazon S3. Like stages, you do not work with actions directly in most cases, but
//! you do define and interact with actions when working with pipeline operations such as
//! <a>CreatePipeline</a> and <a>GetPipelineState</a>. Valid
//! action categories are:</p>
//! <ul>
//! <li>
//! <p>Source</p>
//! </li>
//! <li>
//! <p>Build</p>
//! </li>
//! <li>
//! <p>Test</p>
//! </li>
//! <li>
//! <p>Deploy</p>
//! </li>
//! <li>
//! <p>Approval</p>
//! </li>
//! <li>
//! <p>Invoke</p>
//! </li>
//! </ul>
//! <p>Pipelines also include <i>transitions</i>, which allow the transition
//! of artifacts from one stage to the next in a pipeline after the actions in one stage
//! complete.</p>
//! <p>You can work with transitions by calling:</p>
//! <ul>
//! <li>
//! <p>
//! <a>DisableStageTransition</a>, which prevents artifacts from
//! transitioning to the next stage in a pipeline.</p>
//! </li>
//! <li>
//! <p>
//! <a>EnableStageTransition</a>, which enables transition of artifacts
//! between stages in a pipeline. </p>
//! </li>
//! </ul>
//! <p>
//! <b>Using the API to integrate with AWS CodePipeline</b>
//! </p>
//! <p>For third-party integrators or developers who want to create their own integrations
//! with AWS CodePipeline, the expected sequence varies from the standard API user. To
//! integrate with AWS CodePipeline, developers need to work with the following
//! items:</p>
//! <p>
//! <b>Jobs</b>, which are instances of an action. For
//! example, a job for a source action might import a revision of an artifact from a source. </p>
//! <p>You can work with jobs by calling:</p>
//! <ul>
//! <li>
//! <p>
//! <a>AcknowledgeJob</a>, which confirms whether a job worker has
//! received the specified job.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetJobDetails</a>, which returns the details of a job.</p>
//! </li>
//! <li>
//! <p>
//! <a>PollForJobs</a>, which determines whether there are any jobs to
//! act on.</p>
//! </li>
//! <li>
//! <p>
//! <a>PutJobFailureResult</a>, which provides details of a job failure.
//! </p>
//! </li>
//! <li>
//! <p>
//! <a>PutJobSuccessResult</a>, which provides details of a job
//! success.</p>
//! </li>
//! </ul>
//! <p>
//! <b>Third party jobs</b>, which are instances of an action
//! created by a partner action and integrated into AWS CodePipeline. Partner actions are
//! created by members of the AWS Partner Network.</p>
//! <p>You can work with third party jobs by calling:</p>
//! <ul>
//! <li>
//! <p>
//! <a>AcknowledgeThirdPartyJob</a>, which confirms whether a job worker
//! has received the specified job.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetThirdPartyJobDetails</a>, which requests the details of a job
//! for a partner action.</p>
//! </li>
//! <li>
//! <p>
//! <a>PollForThirdPartyJobs</a>, which determines whether there are any
//! jobs to act on. </p>
//! </li>
//! <li>
//! <p>
//! <a>PutThirdPartyJobFailureResult</a>, which provides details of a job
//! failure.</p>
//! </li>
//! <li>
//! <p>
//! <a>PutThirdPartyJobSuccessResult</a>, which provides details of a job
//! success.</p>
//! </li>
//! </ul>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
mod idempotency_token;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
mod lens;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Paginators for the service
pub mod paginator;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::DateTime;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("codepipeline", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
