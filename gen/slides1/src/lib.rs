// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Slides* crate version *1.0.4+20161213*, where *20161213* is the exact revision of the *slides:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.4*.
//! 
//! Everything else about the *Slides* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/slides/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/slides1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Slides.html) ... 
//! 
//! * [presentations](struct.Presentation.html)
//!  * [*batch update*](struct.PresentationBatchUpdateCall.html), [*create*](struct.PresentationCreateCall.html), [*get*](struct.PresentationGetCall.html) and [*pages get*](struct.PresentationPageGetCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Slides.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.presentations().create(...).doit()
//! let r = hub.presentations().get(...).doit()
//! let r = hub.presentations().batch_update(...).doit()
//! let r = hub.presentations().pages_get(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-slides1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_slides1 as slides1;
//! use slides1::BatchUpdatePresentationRequest;
//! use slides1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use slides1::Slides;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Slides::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = BatchUpdatePresentationRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.presentations().batch_update(req, "presentationId")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage the files in your Google Drive
    Drive,

    /// View your Google Slides presentations
    PresentationReadonly,

    /// View your Google Spreadsheets
    SpreadsheetReadonly,

    /// View the files in your Google Drive
    DriveReadonly,

    /// View and manage your spreadsheets in Google Drive
    Spreadsheet,

    /// View and manage your Google Slides presentations
    Presentation,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::PresentationReadonly => "https://www.googleapis.com/auth/presentations.readonly",
            Scope::SpreadsheetReadonly => "https://www.googleapis.com/auth/spreadsheets.readonly",
            Scope::DriveReadonly => "https://www.googleapis.com/auth/drive.readonly",
            Scope::Spreadsheet => "https://www.googleapis.com/auth/spreadsheets",
            Scope::Presentation => "https://www.googleapis.com/auth/presentations",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::PresentationReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Slides related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_slides1 as slides1;
/// use slides1::BatchUpdatePresentationRequest;
/// use slides1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use slides1::Slides;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Slides::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchUpdatePresentationRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.presentations().batch_update(req, "presentationId")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct Slides<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Slides<C, A> {}

impl<'a, C, A> Slides<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Slides<C, A> {
        Slides {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.4".to_string(),
            _base_url: "https://slides.googleapis.com/".to_string(),
            _root_url: "https://slides.googleapis.com/".to_string(),
        }
    }

    pub fn presentations(&'a self) -> PresentationMethods<'a, C, A> {
        PresentationMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.4`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://slides.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        let prev = self._base_url.clone();
        self._base_url = new_base_url;
        prev
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://slides.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        let prev = self._root_url.clone();
        self._root_url = new_root_url;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// Replaces all shapes that match the given criteria with the provided image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceAllShapesWithImageRequest {
    /// If set, this request will replace all of the shapes that contain the
    /// given text.
    #[serde(rename="containsText")]
    pub contains_text: Option<SubstringMatchCriteria>,
    /// The image URL.
    /// 
    /// The image is fetched once at insertion time and a copy is stored for
    /// display inside the presentation. Images must be less than 50MB in size,
    /// cannot exceed 25 megapixels, and must be in either in PNG, JPEG, or GIF
    /// format.
    #[serde(rename="imageUrl")]
    pub image_url: Option<String>,
    /// The replace method.
    #[serde(rename="replaceMethod")]
    pub replace_method: Option<String>,
}

impl Part for ReplaceAllShapesWithImageRequest {}


/// A PageElement kind representing a
/// joined collection of PageElements.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    /// The collection of elements in the group. The minimum size of a group is 2.
    pub children: Option<Vec<PageElement>>,
}

impl Part for Group {}


/// The general text content. The text must reside in a compatible shape (e.g.
/// text box or rectangle) or a table cell in a page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextContent {
    /// The bulleted lists contained in this text, keyed by list ID.
    pub lists: Option<HashMap<String, List>>,
    /// The text contents broken down into its component parts, including styling
    /// information. This property is read-only.
    #[serde(rename="textElements")]
    pub text_elements: Option<Vec<TextElement>>,
}

impl Part for TextContent {}


/// The stretched picture fill. The page or page element is filled entirely with
/// the specified picture. The picture is stretched to fit its container.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StretchedPictureFill {
    /// Reading the content_url:
    /// 
    /// An URL to a picture with a default lifetime of 30 minutes.
    /// This URL is tagged with the account of the requester. Anyone with the URL
    /// effectively accesses the picture as the original requester. Access to the
    /// picture may be lost if the presentation's sharing settings change.
    /// 
    /// Writing the content_url:
    /// 
    /// The picture is fetched once at insertion time and a copy is stored for
    /// display inside the presentation. Pictures must be less than 50MB in size,
    /// cannot exceed 25 megapixels, and must be in either in PNG, JPEG, or GIF
    /// format.
    #[serde(rename="contentUrl")]
    pub content_url: Option<String>,
    /// The original size of the picture fill. This field is read-only.
    pub size: Option<Size>,
}

impl Part for StretchedPictureFill {}


/// Creates an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateImageRequest {
    /// The image URL.
    /// 
    /// The image is fetched once at insertion time and a copy is stored for
    /// display inside the presentation. Images must be less than 50MB in size,
    /// cannot exceed 25 megapixels, and must be in either in PNG, JPEG, or GIF
    /// format.
    pub url: Option<String>,
    /// A user-supplied object ID.
    /// 
    /// If you specify an ID, it must be unique among all pages and page elements
    /// in the presentation. The ID must start with an alphanumeric character or an
    /// underscore (matches regex `[a-zA-Z0-9_]`); remaining characters
    /// may include those as well as a hyphen or colon (matches regex
    /// `[a-zA-Z0-9_-:]`).
    /// The length of the ID must not be less than 5 or greater than 50.
    /// 
    /// If you don't specify an ID, a unique one is generated.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The element properties for the image.
    /// 
    /// When the aspect ratio of the provided size does not match the image aspect
    /// ratio, the image is scaled and centered with respect to the size in order
    /// to maintain aspect ratio. The provided transform is applied after this
    /// operation.
    #[serde(rename="elementProperties")]
    pub element_properties: Option<PageElementProperties>,
}

impl Part for CreateImageRequest {}


/// Replaces all instances of text matching a criteria with replace text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceAllTextRequest {
    /// Finds text in a shape matching this substring.
    #[serde(rename="containsText")]
    pub contains_text: Option<SubstringMatchCriteria>,
    /// The text that will replace the matched text.
    #[serde(rename="replaceText")]
    pub replace_text: Option<String>,
}

impl Part for ReplaceAllTextRequest {}


/// Response message from a batch update.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update presentations](struct.PresentationBatchUpdateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdatePresentationResponse {
    /// The presentation the updates were applied to.
    #[serde(rename="presentationId")]
    pub presentation_id: Option<String>,
    /// The reply of the updates.  This maps 1:1 with the updates, although
    /// replies to some requests may be empty.
    pub replies: Option<Vec<Response>>,
}

impl ResponseResult for BatchUpdatePresentationResponse {}


/// The crop properties of an object enclosed in a container. For example, an
/// Image.
/// 
/// The crop properties is represented by the offsets of four edges which define
/// a crop rectangle. The offsets are measured in percentage from the
/// corresponding edges of the object's original bounding rectangle towards
/// inside, relative to the object's original dimensions.
/// 
/// - If the offset is in the interval (0, 1), the corresponding edge of crop
/// rectangle is positioned inside of the object's original bounding rectangle.
/// - If the offset is negative or greater than 1, the corresponding edge of crop
/// rectangle is positioned outside of the object's original bounding rectangle.
/// - If the left edge of the crop rectangle is on the right side of its right
/// edge, the object will be flipped horizontally.
/// - If the top edge of the crop rectangle is below its bottom edge, the object
/// will be flipped vertically.
/// - If all offsets and rotation angle is 0, the object is not cropped.
/// 
/// After cropping, the content in the crop rectangle will be stretched to fit
/// its container.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CropProperties {
    /// The offset specifies the left edge of the crop rectangle that is located to
    /// the right of the original bounding rectangle left edge, relative to the
    /// object's original width.
    #[serde(rename="leftOffset")]
    pub left_offset: Option<f32>,
    /// The rotation angle of the crop window around its center, in radians.
    /// Rotation angle is applied after the offset.
    pub angle: Option<f32>,
    /// The offset specifies the bottom edge of the crop rectangle that is located
    /// above the original bounding rectangle bottom edge, relative to the object's
    /// original height.
    #[serde(rename="bottomOffset")]
    pub bottom_offset: Option<f32>,
    /// The offset specifies the right edge of the crop rectangle that is located
    /// to the left of the original bounding rectangle right edge, relative to the
    /// object's original width.
    #[serde(rename="rightOffset")]
    pub right_offset: Option<f32>,
    /// The offset specifies the top edge of the crop rectangle that is located
    /// below the original bounding rectangle top edge, relative to the object's
    /// original height.
    #[serde(rename="topOffset")]
    pub top_offset: Option<f32>,
}

impl Part for CropProperties {}


/// The properties of the SheetsChart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SheetsChartProperties {
    /// The properties of the embedded chart image.
    #[serde(rename="chartImageProperties")]
    pub chart_image_properties: Option<ImageProperties>,
}

impl Part for SheetsChartProperties {}


/// A criteria that matches a specific string of text in a shape or table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubstringMatchCriteria {
    /// The text to search for in the shape or table.
    pub text: Option<String>,
    /// Indicates whether the search should respect case:
    /// 
    /// - `True`: the search is case sensitive.
    /// - `False`: the search is case insensitive.
    #[serde(rename="matchCase")]
    pub match_case: Option<bool>,
}

impl Part for SubstringMatchCriteria {}


/// Contains properties describing the look and feel of a list bullet at a given
/// level of nesting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NestingLevel {
    /// The style of a bullet at this level of nesting.
    #[serde(rename="bulletStyle")]
    pub bullet_style: Option<TextStyle>,
}

impl Part for NestingLevel {}


/// The properties of the Video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoProperties {
    /// The outline of the video. The default outline matches the defaults for new
    /// videos created in the Slides editor.
    pub outline: Option<Outline>,
}

impl Part for VideoProperties {}


/// The shape background fill.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShapeBackgroundFill {
    /// Solid color fill.
    #[serde(rename="solidFill")]
    pub solid_fill: Option<SolidFill>,
    /// The background fill property state.
    /// 
    /// Updating the the fill on a shape will implicitly update this field to
    /// `RENDERED`, unless another value is specified in the same request. To
    /// have no fill on a shape, set this field to `NOT_RENDERED`. In this case,
    /// any other fill fields set in the same request will be ignored.
    #[serde(rename="propertyState")]
    pub property_state: Option<String>,
}

impl Part for ShapeBackgroundFill {}


/// The palette of predefined colors for a page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColorScheme {
    /// The ThemeColorType and corresponding concrete color pairs.
    pub colors: Option<Vec<ThemeColorPair>>,
}

impl Part for ColorScheme {}


/// The page background fill.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageBackgroundFill {
    /// Solid color fill.
    #[serde(rename="solidFill")]
    pub solid_fill: Option<SolidFill>,
    /// The background fill property state.
    /// 
    /// Updating the the fill on a page will implicitly update this field to
    /// `RENDERED`, unless another value is specified in the same request. To
    /// have no fill on a page, set this field to `NOT_RENDERED`. In this case,
    /// any other fill fields set in the same request will be ignored.
    #[serde(rename="propertyState")]
    pub property_state: Option<String>,
    /// Stretched picture fill.
    #[serde(rename="stretchedPictureFill")]
    pub stretched_picture_fill: Option<StretchedPictureFill>,
}

impl Part for PageBackgroundFill {}


/// Refreshes an embedded Google Sheets chart by replacing it with the latest
/// version of the chart from Google Sheets.
/// 
/// NOTE: Refreshing charts requires  at least one of the spreadsheets.readonly,
/// spreadsheets, drive.readonly, or drive OAuth scopes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RefreshSheetsChartRequest {
    /// The object ID of the chart to refresh.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for RefreshSheetsChartRequest {}


/// A magnitude in a single direction in the specified units.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    /// The magnitude.
    pub magnitude: Option<f64>,
    /// The units for magnitude.
    pub unit: Option<String>,
}

impl Part for Dimension {}


/// A solid color fill. The page or page element is filled entirely with the
/// specified color value.
/// 
/// If any field is unset, its value may be inherited from a parent placeholder
/// if it exists.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SolidFill {
    /// The color value of the solid fill.
    pub color: Option<OpaqueColor>,
    /// The fraction of this `color` that should be applied to the pixel.
    /// That is, the final pixel color is defined by the equation:
    /// 
    ///   pixel color = alpha * (color) + (1.0 - alpha) * (background color)
    /// 
    /// This means that a value of 1.0 corresponds to a solid color, whereas
    /// a value of 0.0 corresponds to a completely transparent color.
    pub alpha: Option<f32>,
}

impl Part for SolidFill {}


/// Inserts columns into a table.
/// 
/// Other columns in the table will be resized to fit the new column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTableColumnsRequest {
    /// The table to insert columns into.
    #[serde(rename="tableObjectId")]
    pub table_object_id: Option<String>,
    /// Whether to insert new columns to the right of the reference cell location.
    /// 
    /// - `True`: insert to the right.
    /// - `False`: insert to the left.
    #[serde(rename="insertRight")]
    pub insert_right: Option<bool>,
    /// The number of columns to be inserted. Maximum 20 per request.
    pub number: Option<i32>,
    /// The reference table cell location from which columns will be inserted.
    /// 
    /// A new column will be inserted to the left (or right) of the column where
    /// the reference cell is. If the reference cell is a merged cell, a new
    /// column will be inserted to the left (or right) of the merged cell.
    #[serde(rename="cellLocation")]
    pub cell_location: Option<TableCellLocation>,
}

impl Part for InsertTableColumnsRequest {}


/// Deletes a column from a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteTableColumnRequest {
    /// The reference table cell location from which a column will be deleted.
    /// 
    /// The column this cell spans will be deleted. If this is a merged cell,
    /// multiple columns will be deleted. If no columns remain in the table after
    /// this deletion, the whole table is deleted.
    #[serde(rename="cellLocation")]
    pub cell_location: Option<TableCellLocation>,
    /// The table to delete columns from.
    #[serde(rename="tableObjectId")]
    pub table_object_id: Option<String>,
}

impl Part for DeleteTableColumnRequest {}


/// A visual element rendered on a page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageElement {
    /// A word art page element.
    #[serde(rename="wordArt")]
    pub word_art: Option<WordArt>,
    /// The description of the page element. Combined with title to display alt
    /// text.
    pub description: Option<String>,
    /// The object ID for this page element. Object IDs used by
    /// google.apps.slides.v1.Page and
    /// google.apps.slides.v1.PageElement share the same namespace.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The title of the page element. Combined with description to display alt
    /// text.
    pub title: Option<String>,
    /// An image page element.
    pub image: Option<Image>,
    /// The transform of the page element.
    pub transform: Option<AffineTransform>,
    /// A generic shape.
    pub shape: Option<Shape>,
    /// A linked chart embedded from Google Sheets. Unlinked charts are
    /// represented as images.
    #[serde(rename="sheetsChart")]
    pub sheets_chart: Option<SheetsChart>,
    /// A video page element.
    pub video: Option<Video>,
    /// A collection of page elements joined as a single unit.
    #[serde(rename="elementGroup")]
    pub element_group: Option<Group>,
    /// A table page element.
    pub table: Option<Table>,
    /// A line page element.
    pub line: Option<Line>,
    /// The size of the page element.
    pub size: Option<Size>,
}

impl Part for PageElement {}


/// The properties of a Shape.
/// 
/// If the shape is a placeholder shape as determined by the
/// placeholder field, then these
/// properties may be inherited from a parent placeholder shape.
/// Determining the rendered value of the property depends on the corresponding
/// property_state field value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShapeProperties {
    /// The shadow properties of the shape. If unset, the shadow is inherited from
    /// a parent placeholder if it exists. If the shape has no parent, then the
    /// default shadow matches the defaults for new shapes created in the Slides
    /// editor. This property is read-only.
    pub shadow: Option<Shadow>,
    /// The background fill of the shape. If unset, the background fill is
    /// inherited from a parent placeholder if it exists. If the shape has no
    /// parent, then the default background fill depends on the shape type,
    /// matching the defaults for new shapes created in the Slides editor.
    #[serde(rename="shapeBackgroundFill")]
    pub shape_background_fill: Option<ShapeBackgroundFill>,
    /// The hyperlink destination of the shape. If unset, there is no link. Links
    /// are not inherited from parent placeholders.
    pub link: Option<Link>,
    /// The outline of the shape. If unset, the outline is inherited from a
    /// parent placeholder if it exists. If the shape has no parent, then the
    /// default outline depends on the shape type, matching the defaults for
    /// new shapes created in the Slides editor.
    pub outline: Option<Outline>,
}

impl Part for ShapeProperties {}


/// A single response from an update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    /// The result of creating an image.
    #[serde(rename="createImage")]
    pub create_image: Option<CreateImageResponse>,
    /// The result of replacing all shapes containing the specified text with
    /// an image.
    #[serde(rename="replaceAllShapesWithImage")]
    pub replace_all_shapes_with_image: Option<ReplaceAllShapesWithImageResponse>,
    /// The result of duplicating an object.
    #[serde(rename="duplicateObject")]
    pub duplicate_object: Option<DuplicateObjectResponse>,
    /// The result of creating a shape.
    #[serde(rename="createShape")]
    pub create_shape: Option<CreateShapeResponse>,
    /// The result of replacing text.
    #[serde(rename="replaceAllText")]
    pub replace_all_text: Option<ReplaceAllTextResponse>,
    /// The result of creating a Google Sheets chart.
    #[serde(rename="createSheetsChart")]
    pub create_sheets_chart: Option<CreateSheetsChartResponse>,
    /// The result of creating a video.
    #[serde(rename="createVideo")]
    pub create_video: Option<CreateVideoResponse>,
    /// The result of creating a line.
    #[serde(rename="createLine")]
    pub create_line: Option<CreateLineResponse>,
    /// The result of creating a table.
    #[serde(rename="createTable")]
    pub create_table: Option<CreateTableResponse>,
    /// The result of creating a slide.
    #[serde(rename="createSlide")]
    pub create_slide: Option<CreateSlideResponse>,
}

impl Part for Response {}


/// The result of creating a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateTableResponse {
    /// The object ID of the created table.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for CreateTableResponse {}


/// Styles that apply to a whole paragraph.
/// 
/// If this text is contained in a shape with a parent placeholder, then these paragraph styles may be
/// inherited from the parent. Which paragraph styles are inherited depend on the
/// nesting level of lists:
/// 
/// * A paragraph not in a list will inherit its paragraph style from the
///   paragraph at the 0 nesting level of the list inside the parent placeholder.
/// * A paragraph in a list will inherit its paragraph style from the paragraph
///   at its corresponding nesting level of the list inside the parent
///   placeholder.
/// 
/// Inherited paragraph styles are represented as unset fields in this message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParagraphStyle {
    /// The spacing mode for the paragraph. This property is read-only.
    #[serde(rename="spacingMode")]
    pub spacing_mode: Option<String>,
    /// The text direction of this paragraph. This property is read-only.
    pub direction: Option<String>,
    /// The amount of extra space above the paragraph. If unset, the value is
    /// inherited from the parent. This property is read-only.
    #[serde(rename="spaceBelow")]
    pub space_below: Option<Dimension>,
    /// The amount of space between lines, as a percentage of normal, where normal
    /// is represented as 100.0. If unset, the value is inherited from the parent.
    /// This property is read-only.
    #[serde(rename="lineSpacing")]
    pub line_spacing: Option<f32>,
    /// The amount indentation for the paragraph on the side that corresponds to
    /// the start of the text, based on the current text direction. If unset, the
    /// value is inherited from the parent. This property is read-only.
    #[serde(rename="indentStart")]
    pub indent_start: Option<Dimension>,
    /// The amount of extra space above the paragraph. If unset, the value is
    /// inherited from the parent. This property is read-only.
    #[serde(rename="spaceAbove")]
    pub space_above: Option<Dimension>,
    /// The amount indentation for the paragraph on the side that corresponds to
    /// the end of the text, based on the current text direction. If unset, the
    /// value is inherited from the parent. This property is read-only.
    #[serde(rename="indentEnd")]
    pub indent_end: Option<Dimension>,
    /// The amount of indentation for the start of the first line of the paragraph.
    /// If unset, the value is inherited from the parent. This property is
    /// read-only.
    #[serde(rename="indentFirstLine")]
    pub indent_first_line: Option<Dimension>,
    /// The text alignment for this paragraph. This property is read-only.
    pub alignment: Option<String>,
}

impl Part for ParagraphStyle {}


/// Describes the bullet of a paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bullet {
    /// The nesting level of this paragraph in the list.
    #[serde(rename="nestingLevel")]
    pub nesting_level: Option<i32>,
    /// The ID of the list this paragraph belongs to.
    #[serde(rename="listId")]
    pub list_id: Option<String>,
    /// The paragraph specific text style applied to this bullet.
    #[serde(rename="bulletStyle")]
    pub bullet_style: Option<TextStyle>,
    /// The rendered bullet glyph for this paragraph.
    pub glyph: Option<String>,
}

impl Part for Bullet {}


/// Duplicates a slide or page element.
/// 
/// When duplicating a slide, the duplicate slide will be created immediately
/// following the specified slide. When duplicating a page element, the duplicate
/// will be placed on the same page at the same position as the original.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DuplicateObjectRequest {
    /// The object being duplicated may contain other objects, for example when
    /// duplicating a slide or a group page element. This map defines how the IDs
    /// of duplicated objects are generated: the keys are the IDs of the original
    /// objects and its values are the IDs that will be assigned to the
    /// corresponding duplicate object. The ID of the source object's duplicate
    /// may be specified in this map as well, using the same value of the
    /// `object_id` field as a key and the newly desired ID as the value.
    /// 
    /// All keys must correspond to existing IDs in the presentation. All values
    /// must be unique in the presentation and must start with an alphanumeric
    /// character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining
    /// characters may include those as well as a hyphen or colon (matches regex
    /// `[a-zA-Z0-9_-:]`). The length of the new ID must not be less than 5 or
    /// greater than 50.
    /// 
    /// If any IDs of source objects are omitted from the map, a new random ID will
    /// be assigned. If the map is empty or unset, all duplicate objects will
    /// receive a new random ID.
    #[serde(rename="objectIds")]
    pub object_ids: Option<HashMap<String, String>>,
    /// The ID of the object to duplicate.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for DuplicateObjectRequest {}


/// A table range represents a reference to a subset of a table.
/// 
/// It's important to note that the cells specified by a table range do not
/// necessarily form a rectangle. For example, let's say we have a 3 x 3 table
/// where all the cells of the last row are merged together. The table looks
/// like this:
/// 
///            
///   [             ]
/// 
/// A table range with location = (0, 0), row span = 3 and column span = 2
/// specifies the following cells:
/// 
///    x     x 
///   [      x      ]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableRange {
    /// The row span of the table range.
    #[serde(rename="rowSpan")]
    pub row_span: Option<i32>,
    /// The column span of the table range.
    #[serde(rename="columnSpan")]
    pub column_span: Option<i32>,
    /// The starting location of the table range.
    pub location: Option<TableCellLocation>,
}

impl Part for TableRange {}


/// Updates the properties of a Line.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateLinePropertiesRequest {
    /// The fields that should be updated.
    /// 
    /// At least one field must be specified. The root `lineProperties` is
    /// implied and should not be specified. A single `"*"` can be used as
    /// short-hand for listing every field.
    /// 
    /// For example to update the line solid fill color, set `fields` to
    /// `"lineFill.solidFill.color"`.
    /// 
    /// To reset a property to its default value, include its field name in the
    /// field mask but leave the field itself unset.
    pub fields: Option<String>,
    /// The line properties to update.
    #[serde(rename="lineProperties")]
    pub line_properties: Option<LineProperties>,
    /// The object ID of the line the update is applied to.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for UpdateLinePropertiesRequest {}


/// The table cell background fill.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCellBackgroundFill {
    /// Solid color fill.
    #[serde(rename="solidFill")]
    pub solid_fill: Option<SolidFill>,
    /// The background fill property state.
    /// 
    /// Updating the the fill on a table cell will implicitly update this field
    /// to `RENDERED`, unless another value is specified in the same request. To
    /// have no fill on a table cell, set this field to `NOT_RENDERED`. In this
    /// case, any other fill fields set in the same request will be ignored.
    #[serde(rename="propertyState")]
    pub property_state: Option<String>,
}

impl Part for TableCellBackgroundFill {}


/// Creates a video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateVideoRequest {
    /// The video source.
    pub source: Option<String>,
    /// A user-supplied object ID.
    /// 
    /// If you specify an ID, it must be unique among all pages and page elements
    /// in the presentation. The ID must start with an alphanumeric character or an
    /// underscore (matches regex `[a-zA-Z0-9_]`); remaining characters
    /// may include those as well as a hyphen or colon (matches regex
    /// `[a-zA-Z0-9_-:]`).
    /// The length of the ID must not be less than 5 or greater than 50.
    /// 
    /// If you don't specify an ID, a unique one is generated.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The element properties for the video.
    #[serde(rename="elementProperties")]
    pub element_properties: Option<PageElementProperties>,
    /// The video source's unique identifier for this video.
    /// 
    /// e.g. For YouTube video https://www.youtube.com/watch?v=7U3axjORYZ0,
    /// the ID is 7U3axjORYZ0.
    pub id: Option<String>,
}

impl Part for CreateVideoRequest {}


/// A color that can either be fully opaque or fully transparent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OptionalColor {
    /// If set, this will be used as an opaque color. If unset, this represents
    /// a transparent color.
    #[serde(rename="opaqueColor")]
    pub opaque_color: Option<OpaqueColor>,
}

impl Part for OptionalColor {}


/// The fill of the line.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LineFill {
    /// Solid color fill.
    #[serde(rename="solidFill")]
    pub solid_fill: Option<SolidFill>,
}

impl Part for LineFill {}


/// Creates an embedded Google Sheets chart.
/// 
/// NOTE: Chart creation requires  at least one of the spreadsheets.readonly,
/// spreadsheets, drive.readonly, or drive OAuth scopes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateSheetsChartRequest {
    /// The ID of the specific chart in the Google Sheets spreadsheet.
    #[serde(rename="chartId")]
    pub chart_id: Option<i32>,
    /// The ID of the Google Sheets spreadsheet that contains the chart.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The mode with which the chart is linked to the source spreadsheet. When
    /// not specified, the chart will be an image that is not linked.
    #[serde(rename="linkingMode")]
    pub linking_mode: Option<String>,
    /// A user-supplied object ID.
    /// 
    /// If specified, the ID must be unique among all pages and page elements in
    /// the presentation. The ID should start with a word character [a-zA-Z0-9_]
    /// and then followed by any number of the following characters [a-zA-Z0-9_-:].
    /// The length of the ID should not be less than 5 or greater than 50.
    /// If empty, a unique identifier will be generated.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The element properties for the chart.
    /// 
    /// When the aspect ratio of the provided size does not match the chart aspect
    /// ratio, the chart is scaled and centered with respect to the size in order
    /// to maintain aspect ratio. The provided transform is applied after this
    /// operation.
    #[serde(rename="elementProperties")]
    pub element_properties: Option<PageElementProperties>,
}

impl Part for CreateSheetsChartRequest {}


/// The result of replacing shapes with an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceAllShapesWithImageResponse {
    /// The number of shapes replaced with images.
    #[serde(rename="occurrencesChanged")]
    pub occurrences_changed: Option<i32>,
}

impl Part for ReplaceAllShapesWithImageResponse {}


/// The properties of Page that are only
/// relevant for pages with page_type SLIDE.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SlideProperties {
    /// The object ID of the layout that this slide is based on.
    #[serde(rename="layoutObjectId")]
    pub layout_object_id: Option<String>,
    /// The object ID of the master that this slide is based on.
    #[serde(rename="masterObjectId")]
    pub master_object_id: Option<String>,
}

impl Part for SlideProperties {}


/// A page in a presentation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pages get presentations](struct.PresentationPageGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Page {
    /// Layout specific properties. Only set if page_type = LAYOUT.
    #[serde(rename="layoutProperties")]
    pub layout_properties: Option<LayoutProperties>,
    /// The page elements rendered on the page.
    #[serde(rename="pageElements")]
    pub page_elements: Option<Vec<PageElement>>,
    /// The object ID for this page. Object IDs used by
    /// Page and
    /// PageElement share the same namespace.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The properties of the page.
    #[serde(rename="pageProperties")]
    pub page_properties: Option<PageProperties>,
    /// The type of the page.
    #[serde(rename="pageType")]
    pub page_type: Option<String>,
    /// Slide specific properties. Only set if page_type = SLIDE.
    #[serde(rename="slideProperties")]
    pub slide_properties: Option<SlideProperties>,
}

impl ResponseResult for Page {}


/// A themeable solid color value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OpaqueColor {
    /// An opaque theme color.
    #[serde(rename="themeColor")]
    pub theme_color: Option<String>,
    /// An opaque RGB color.
    #[serde(rename="rgbColor")]
    pub rgb_color: Option<RgbColor>,
}

impl Part for OpaqueColor {}


/// The shadow properties of a page element.
/// 
/// If these fields are unset, they may be inherited from a parent placeholder
/// if it exists. If there is no parent, the fields will default to the value
/// used for new page elements created in the Slides editor, which may depend on
/// the page element kind.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Shadow {
    /// The shadow color value.
    pub color: Option<OpaqueColor>,
    /// Transform that encodes the translate, scale, and skew of the shadow,
    /// relative to the alignment position.
    pub transform: Option<AffineTransform>,
    /// The shadow property state.
    /// 
    /// Updating the the shadow on a page element will implicitly update this field
    /// to `RENDERED`, unless another value is specified in the same request. To
    /// have no shadow on a page element, set this field to `NOT_RENDERED`. In this
    /// case, any other shadow fields set in the same request will be ignored.
    #[serde(rename="propertyState")]
    pub property_state: Option<String>,
    /// The radius of the shadow blur. The larger the radius, the more diffuse the
    /// shadow becomes.
    #[serde(rename="blurRadius")]
    pub blur_radius: Option<Dimension>,
    /// The alpha of the shadow's color, from 0.0 to 1.0.
    pub alpha: Option<f32>,
    /// The type of the shadow.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The alignment point of the shadow, that sets the origin for translate,
    /// scale and skew of the shadow.
    pub alignment: Option<String>,
    /// Whether the shadow should rotate with the shape.
    #[serde(rename="rotateWithShape")]
    pub rotate_with_shape: Option<bool>,
}

impl Part for Shadow {}


/// A location of a single table cell within a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCellLocation {
    /// The 0-based row index.
    #[serde(rename="rowIndex")]
    pub row_index: Option<i32>,
    /// The 0-based column index.
    #[serde(rename="columnIndex")]
    pub column_index: Option<i32>,
}

impl Part for TableCellLocation {}


/// Common properties for a page element.
/// 
/// Note: When you initially create a
/// PageElement, the API may modify
/// the values of both `size` and `transform`, but the
/// visual size will be unchanged.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageElementProperties {
    /// The object ID of the page where the element is located.
    #[serde(rename="pageObjectId")]
    pub page_object_id: Option<String>,
    /// The transform for the element.
    pub transform: Option<AffineTransform>,
    /// The size of the element.
    pub size: Option<Size>,
}

impl Part for PageElementProperties {}


/// Update the properties of a Video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateVideoPropertiesRequest {
    /// The video properties to update.
    #[serde(rename="videoProperties")]
    pub video_properties: Option<VideoProperties>,
    /// The object ID of the video the updates are applied to.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The fields that should be updated.
    /// 
    /// At least one field must be specified. The root `videoProperties` is
    /// implied and should not be specified. A single `"*"` can be used as
    /// short-hand for listing every field.
    /// 
    /// For example to update the video outline color, set `fields` to
    /// `"outline.outlineFill.solidFill.color"`.
    /// 
    /// To reset a property to its default value, include its field name in the
    /// field mask but leave the field itself unset.
    pub fields: Option<String>,
}

impl Part for UpdateVideoPropertiesRequest {}


/// The response of duplicating an object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DuplicateObjectResponse {
    /// The ID of the new duplicate object.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for DuplicateObjectResponse {}


/// The result of replacing text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceAllTextResponse {
    /// The number of occurrences changed by replacing all text.
    #[serde(rename="occurrencesChanged")]
    pub occurrences_changed: Option<i32>,
}

impl Part for ReplaceAllTextResponse {}


/// Properties and contents of each row in a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableRow {
    /// Properties and contents of each cell.
    /// 
    /// Cells that span multiple columns are represented only once with a
    /// column_span greater
    /// than 1. As a result, the length of this collection does not always match
    /// the number of columns of the entire table.
    #[serde(rename="tableCells")]
    pub table_cells: Option<Vec<TableCell>>,
    /// Height of a row.
    #[serde(rename="rowHeight")]
    pub row_height: Option<Dimension>,
}

impl Part for TableRow {}


/// A PageElement kind representing
/// a linked chart embedded from Google Sheets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SheetsChart {
    /// The ID of the specific chart in the Google Sheets spreadsheet that is
    /// embedded.
    #[serde(rename="chartId")]
    pub chart_id: Option<i32>,
    /// The ID of the Google Sheets spreadsheet that contains the source chart.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The properties of the Sheets chart.
    #[serde(rename="sheetsChartProperties")]
    pub sheets_chart_properties: Option<SheetsChartProperties>,
    /// The URL of an image of the embedded chart, with a default lifetime of 30
    /// minutes. This URL is tagged with the account of the requester. Anyone with
    /// the URL effectively accesses the image as the original requester. Access to
    /// the image may be lost if the presentation's sharing settings change.
    #[serde(rename="contentUrl")]
    pub content_url: Option<String>,
}

impl Part for SheetsChart {}


/// A PageElement kind representing a
/// video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Video {
    /// An URL to a video. The URL is valid as long as the source video
    /// exists and sharing settings do not change.
    pub url: Option<String>,
    /// The properties of the video.
    #[serde(rename="videoProperties")]
    pub video_properties: Option<VideoProperties>,
    /// The video source's unique identifier for this video.
    pub id: Option<String>,
    /// The video source.
    pub source: Option<String>,
}

impl Part for Video {}


/// Creates a line.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateLineRequest {
    /// The category of line to be created.
    #[serde(rename="lineCategory")]
    pub line_category: Option<String>,
    /// A user-supplied object ID.
    /// 
    /// If you specify an ID, it must be unique among all pages and page elements
    /// in the presentation. The ID must start with an alphanumeric character or an
    /// underscore (matches regex `[a-zA-Z0-9_]`); remaining characters
    /// may include those as well as a hyphen or colon (matches regex
    /// `[a-zA-Z0-9_-:]`).
    /// The length of the ID must not be less than 5 or greater than 50.
    /// 
    /// If you don't specify an ID, a unique one is generated.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The element properties for the line.
    #[serde(rename="elementProperties")]
    pub element_properties: Option<PageElementProperties>,
}

impl Part for CreateLineRequest {}


/// A TextElement kind that represents auto text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoText {
    /// The rendered content of this auto text, if available.
    pub content: Option<String>,
    /// The styling applied to this auto text.
    pub style: Option<TextStyle>,
    /// The type of this auto text.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for AutoText {}


/// AffineTransform uses a 3x3 matrix with an implied last row of [ 0 0 1 ]
/// to transform source coordinates (x,y) into destination coordinates (x', y')
/// according to:
/// 
///       x'  x  =   shear_y  scale_y  translate_y  
///       1  [ 1 ]
/// 
/// After transformation,
/// 
///      x' = scale_x * x + shear_x * y + translate_x;
///      y' = scale_y * y + shear_y * x + translate_y;
/// 
/// This message is therefore composed of these six matrix elements.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AffineTransform {
    /// The units for translate elements.
    pub unit: Option<String>,
    /// The X coordinate translation element.
    #[serde(rename="translateX")]
    pub translate_x: Option<f64>,
    /// The Y coordinate translation element.
    #[serde(rename="translateY")]
    pub translate_y: Option<f64>,
    /// The X coordinate scaling element.
    #[serde(rename="scaleX")]
    pub scale_x: Option<f64>,
    /// The Y coordinate scaling element.
    #[serde(rename="scaleY")]
    pub scale_y: Option<f64>,
    /// The Y coordinate shearing element.
    #[serde(rename="shearY")]
    pub shear_y: Option<f64>,
    /// The X coordinate shearing element.
    #[serde(rename="shearX")]
    pub shear_x: Option<f64>,
}

impl Part for AffineTransform {}


/// A width and height.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Size {
    /// The width of the object.
    pub width: Option<Dimension>,
    /// The height of the object.
    pub height: Option<Dimension>,
}

impl Part for Size {}


/// Inserts text into a shape or a table cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTextRequest {
    /// The text to be inserted.
    /// 
    /// Inserting a newline character will implicitly create a new
    /// ParagraphMarker at that index.
    /// The paragraph style of the new paragraph will be copied from the paragraph
    /// at the current insertion index, including lists and bullets.
    /// 
    /// Text styles for inserted text will be determined automatically, generally
    /// preserving the styling of neighboring text. In most cases, the text will be
    /// added to the TextRun that exists at the
    /// insertion index.
    /// 
    /// Some control characters (U+0000-U+0008, U+000C-U+001F) and characters
    /// from the Unicode Basic Multilingual Plane Private Use Area (U+E000-U+F8FF)
    /// will be stripped out of the inserted text.
    pub text: Option<String>,
    /// The optional table cell location if the text is to be inserted into a table
    /// cell. If present, the object_id must refer to a table.
    #[serde(rename="cellLocation")]
    pub cell_location: Option<TableCellLocation>,
    /// The object ID of the shape or table where the text will be inserted.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The index where the text will be inserted, in Unicode code units, based
    /// on TextElement indexes.
    /// 
    /// The index is zero-based and is computed from the start of the string.
    /// The index may be adjusted to prevent insertions inside Unicode grapheme
    /// clusters. In these cases, the text will be inserted immediately after the
    /// grapheme cluster.
    #[serde(rename="insertionIndex")]
    pub insertion_index: Option<i32>,
}

impl Part for InsertTextRequest {}


/// Represents the styling that can be applied to a TextRun.
/// 
/// If this text is contained in a shape with a parent placeholder, then these text styles may be
/// inherited from the parent. Which text styles are inherited depend on the
/// nesting level of lists:
/// 
/// * A text run in a paragraph that is not in a list will inherit its text style
///   from the the newline character in the paragraph at the 0 nesting level of
///   the list inside the parent placeholder.
/// * A text run in a paragraph that is in a list will inherit its text style
///   from the newline character in the paragraph at its corresponding nesting
///   level of the list inside the parent placeholder.
/// 
/// Inherited text styles are represented as unset fields in this message. If
/// text is contained in a shape without a parent placeholder, unsetting these
/// fields will revert the style to a value matching the defaults in the Slides
/// editor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextStyle {
    /// The color of the text itself. If set, the color is either opaque or
    /// transparent, depending on if the `opaque_color` field in it is set.
    #[serde(rename="foregroundColor")]
    pub foreground_color: Option<OptionalColor>,
    /// Whether or not the text is bold.
    pub bold: Option<bool>,
    /// The text's vertical offset from its normal position.
    /// 
    /// Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically
    /// rendered in a smaller font size, computed based on the `font_size` field.
    /// The `font_size` itself is not affected by changes in this field.
    #[serde(rename="baselineOffset")]
    pub baseline_offset: Option<String>,
    /// Whether or not the text is struck through.
    pub strikethrough: Option<bool>,
    /// Whether or not the text is in small capital letters.
    #[serde(rename="smallCaps")]
    pub small_caps: Option<bool>,
    /// The font family of the text.
    /// 
    /// The font family can be any font from the Font menu in Slides or from
    /// [Google Fonts] (https://fonts.google.com/). If the font name is
    /// unrecognized, the text is rendered in `Arial`.
    /// 
    /// Some fonts can affect the weight of the text. If an update request
    /// specifies values for both `font_family` and `bold`, the explicitly-set
    /// `bold` value is used.
    #[serde(rename="fontFamily")]
    pub font_family: Option<String>,
    /// The hyperlink destination of the text. If unset, there is no link. Links
    /// are not inherited from parent text.
    /// 
    /// Changing the link in an update request causes some other changes to the
    /// text style of the range:
    /// 
    /// * When setting a link, the text foreground color will be set to
    ///   ThemeColorType.HYPERLINK and the text will
    ///   be underlined. If these fields are modified in the same
    ///   request, those values will be used instead of the link defaults.
    /// * Setting a link on a text range that overlaps with an existing link will
    ///   also update the existing link to point to the new URL.
    /// * Links are not settable on newline characters. As a result, setting a link
    ///   on a text range that crosses a paragraph boundary, such as `"ABC\n123"`,
    ///   will separate the newline character(s) into their own text runs. The
    ///   link will be applied separately to the runs before and after the newline.
    /// * Removing a link will update the text style of the range to match the
    ///   style of the preceding text (or the default text styles if the preceding
    ///   text is another link) unless different styles are being set in the same
    ///   request.
    pub link: Option<Link>,
    /// Whether or not the text is italicized.
    pub italic: Option<bool>,
    /// The size of the text's font. When read, the `font_size` will specified in
    /// points.
    #[serde(rename="fontSize")]
    pub font_size: Option<Dimension>,
    /// Whether or not the text is underlined.
    pub underline: Option<bool>,
    /// The background color of the text. If set, the color is either opaque or
    /// transparent, depending on if the `opaque_color` field in it is set.
    #[serde(rename="backgroundColor")]
    pub background_color: Option<OptionalColor>,
}

impl Part for TextStyle {}


/// Properties of each column in a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableColumnProperties {
    /// Width of a column.
    #[serde(rename="columnWidth")]
    pub column_width: Option<Dimension>,
}

impl Part for TableColumnProperties {}


/// The result of creating a slide.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateSlideResponse {
    /// The object ID of the created slide.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for CreateSlideResponse {}


/// Properties and contents of each table cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCell {
    /// The text content of the cell.
    pub text: Option<TextContent>,
    /// Row span of the cell.
    #[serde(rename="rowSpan")]
    pub row_span: Option<i32>,
    /// The properties of the table cell.
    #[serde(rename="tableCellProperties")]
    pub table_cell_properties: Option<TableCellProperties>,
    /// The location of the cell within the table.
    pub location: Option<TableCellLocation>,
    /// Column span of the cell.
    #[serde(rename="columnSpan")]
    pub column_span: Option<i32>,
}

impl Part for TableCell {}


/// An RGB color.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RgbColor {
    /// The blue component of the color, from 0.0 to 1.0.
    pub blue: Option<f32>,
    /// The green component of the color, from 0.0 to 1.0.
    pub green: Option<f32>,
    /// The red component of the color, from 0.0 to 1.0.
    pub red: Option<f32>,
}

impl Part for RgbColor {}


/// Inserts rows into a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTableRowsRequest {
    /// The table to insert rows into.
    #[serde(rename="tableObjectId")]
    pub table_object_id: Option<String>,
    /// Whether to insert new rows below the reference cell location.
    /// 
    /// - `True`: insert below the cell.
    /// - `False`: insert above the cell.
    #[serde(rename="insertBelow")]
    pub insert_below: Option<bool>,
    /// The reference table cell location from which rows will be inserted.
    /// 
    /// A new row will be inserted above (or below) the row where the reference
    /// cell is. If the reference cell is a merged cell, a new row will be
    /// inserted above (or below) the merged cell.
    #[serde(rename="cellLocation")]
    pub cell_location: Option<TableCellLocation>,
    /// The number of rows to be inserted. Maximum 20 per request.
    pub number: Option<i32>,
}

impl Part for InsertTableRowsRequest {}


/// A Google Slides presentation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create presentations](struct.PresentationCreateCall.html) (request|response)
/// * [get presentations](struct.PresentationGetCall.html) (response)
/// * [batch update presentations](struct.PresentationBatchUpdateCall.html) (none)
/// * [pages get presentations](struct.PresentationPageGetCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Presentation {
    /// The slides in the presentation.
    /// A slide inherits properties from a slide layout.
    pub slides: Option<Vec<Page>>,
    /// The ID of the presentation.
    #[serde(rename="presentationId")]
    pub presentation_id: Option<String>,
    /// The slide masters in the presentation. A slide master contains all common
    /// page elements and the common properties for a set of layouts. They serve
    /// three purposes:
    /// 
    /// - Placeholder shapes on a master contain the default text styles and shape
    ///   properties of all placeholder shapes on pages that use that master.
    /// - The master page properties define the common page properties inherited by
    ///   its layouts.
    /// - Any other shapes on the master slide will appear on all slides using that
    ///   master, regardless of their layout.
    pub masters: Option<Vec<Page>>,
    /// The size of pages in the presentation.
    #[serde(rename="pageSize")]
    pub page_size: Option<Size>,
    /// The title of the presentation.
    pub title: Option<String>,
    /// The locale of the presentation, as an IETF BCP 47 language tag.
    pub locale: Option<String>,
    /// The layouts in the presentation. A layout is a template that determines
    /// how content is arranged and styled on the slides that inherit from that
    /// layout.
    pub layouts: Option<Vec<Page>>,
}

impl RequestValue for Presentation {}
impl Resource for Presentation {}
impl ResponseResult for Presentation {}


/// A PageElement kind representing
/// word art.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WordArt {
    /// The text rendered as word art.
    #[serde(rename="renderedText")]
    pub rendered_text: Option<String>,
}

impl Part for WordArt {}


/// The properties of Page are only
/// relevant for pages with page_type LAYOUT.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LayoutProperties {
    /// The human readable name of the layout in the presentation's locale.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The name of the layout.
    pub name: Option<String>,
    /// The object ID of the master that this layout is based on.
    #[serde(rename="masterObjectId")]
    pub master_object_id: Option<String>,
}

impl Part for LayoutProperties {}


/// The properties of the Image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageProperties {
    /// The outline of the image. If not set, the the image has no outline.
    pub outline: Option<Outline>,
    /// The brightness effect of the image. The value should be in the interval
    /// [-1.0, 1.0], where 0 means no effect. This property is read-only.
    pub brightness: Option<f32>,
    /// The recolor effect of the image. If not set, the image is not recolored.
    /// This property is read-only.
    pub recolor: Option<Recolor>,
    /// The hyperlink destination of the image. If unset, there is no link.
    pub link: Option<Link>,
    /// The transparency effect of the image. The value should be in the interval
    /// [0.0, 1.0], where 0 means no effect and 1 means completely transparent.
    /// This property is read-only.
    pub transparency: Option<f32>,
    /// The crop properties of the image. If not set, the image is not cropped.
    /// This property is read-only.
    #[serde(rename="cropProperties")]
    pub crop_properties: Option<CropProperties>,
    /// The shadow of the image. If not set, the image has no shadow. This property
    /// is read-only.
    pub shadow: Option<Shadow>,
    /// The contrast effect of the image. The value should be in the interval
    /// [-1.0, 1.0], where 0 means no effect. This property is read-only.
    pub contrast: Option<f32>,
}

impl Part for ImageProperties {}


/// A TextElement describes the content of a range of indices in the text content
/// of a Shape or TableCell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextElement {
    /// A TextElement representing a spot in the text that is dynamically
    /// replaced with content that can change over time.
    #[serde(rename="autoText")]
    pub auto_text: Option<AutoText>,
    /// The zero-based end index of this text element, exclusive, in Unicode code
    /// units.
    #[serde(rename="endIndex")]
    pub end_index: Option<i32>,
    /// The zero-based start index of this text element, in Unicode code units.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// A marker representing the beginning of a new paragraph.
    /// 
    /// The `start_index` and `end_index` of this TextElement represent the
    /// range of the paragraph. Other TextElements with an index range contained
    /// inside this paragraph's range are considered to be part of this
    /// paragraph. The range of indices of two separate paragraphs will never
    /// overlap.
    #[serde(rename="paragraphMarker")]
    pub paragraph_marker: Option<ParagraphMarker>,
    /// A TextElement representing a run of text where all of the characters
    /// in the run have the same TextStyle.
    /// 
    /// The `start_index` and `end_index` of TextRuns will always be fully
    /// contained in the index range of a single `paragraph_marker` TextElement.
    /// In other words, a TextRun will never span multiple paragraphs.
    #[serde(rename="textRun")]
    pub text_run: Option<TextRun>,
}

impl Part for TextElement {}


/// Deletes a row from a table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteTableRowRequest {
    /// The reference table cell location from which a row will be deleted.
    /// 
    /// The row this cell spans will be deleted. If this is a merged cell, multiple
    /// rows will be deleted. If no rows remain in the table after this deletion,
    /// the whole table is deleted.
    #[serde(rename="cellLocation")]
    pub cell_location: Option<TableCellLocation>,
    /// The table to delete rows from.
    #[serde(rename="tableObjectId")]
    pub table_object_id: Option<String>,
}

impl Part for DeleteTableRowRequest {}


/// The result of creating a video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateVideoResponse {
    /// The object ID of the created video.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for CreateVideoResponse {}


/// Update the properties of a Shape.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateShapePropertiesRequest {
    /// The fields that should be updated.
    /// 
    /// At least one field must be specified. The root `shapeProperties` is
    /// implied and should not be specified. A single `"*"` can be used as
    /// short-hand for listing every field.
    /// 
    /// For example to update the shape background solid fill color, set `fields`
    /// to `"shapeBackgroundFill.solidFill.color"`.
    /// 
    /// To reset a property to its default value, include its field name in the
    /// field mask but leave the field itself unset.
    pub fields: Option<String>,
    /// The shape properties to update.
    #[serde(rename="shapeProperties")]
    pub shape_properties: Option<ShapeProperties>,
    /// The object ID of the shape the updates are applied to.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for UpdateShapePropertiesRequest {}


/// A hypertext link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// If set, indicates this is a link to the external web page at this URL.
    pub url: Option<String>,
    /// If set, indicates this is a link to the specific page in this
    /// presentation with this ID. A page with this ID may not exist.
    #[serde(rename="pageObjectId")]
    pub page_object_id: Option<String>,
    /// If set, indicates this is a link to the slide at this zero-based index
    /// in the presentation. There may not be a slide at this index.
    #[serde(rename="slideIndex")]
    pub slide_index: Option<i32>,
    /// If set, indicates this is a link to a slide in this presentation,
    /// addressed by its position.
    #[serde(rename="relativeLink")]
    pub relative_link: Option<String>,
}

impl Part for Link {}


/// The result of creating a shape.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateShapeResponse {
    /// The object ID of the created shape.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for CreateShapeResponse {}


/// A TextElement kind that represents a run of text that all has the same
/// styling.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextRun {
    /// The text of this run.
    pub content: Option<String>,
    /// The styling applied to this run.
    pub style: Option<TextStyle>,
}

impl Part for TextRun {}


/// The properties of the Page.
/// 
/// The page will inherit properties from the parent page. Depending on the page
/// type the hierarchy is defined in either
/// SlideProperties or
/// LayoutProperties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageProperties {
    /// The background fill of the page. If unset, the background fill is inherited
    /// from a parent page if it exists. If the page has no parent, then the
    /// background fill defaults to the corresponding fill in the Slides editor.
    #[serde(rename="pageBackgroundFill")]
    pub page_background_fill: Option<PageBackgroundFill>,
    /// The color scheme of the page. If unset, the color scheme is inherited from
    /// a parent page. If the page has no parent, the color scheme uses a default
    /// Slides color scheme. This field is read-only.
    #[serde(rename="colorScheme")]
    pub color_scheme: Option<ColorScheme>,
}

impl Part for PageProperties {}


/// A PageElement kind representing a
/// line, curved connector, or bent connector.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Line {
    /// The properties of the line.
    #[serde(rename="lineProperties")]
    pub line_properties: Option<LineProperties>,
    /// The type of the line.
    #[serde(rename="lineType")]
    pub line_type: Option<String>,
}

impl Part for Line {}


/// The placeholder information that uniquely identifies a placeholder shape.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Placeholder {
    /// The object ID of this shape's parent placeholder.
    /// If unset, the parent placeholder shape does not exist, so the shape does
    /// not inherit properties from any other shape.
    #[serde(rename="parentObjectId")]
    pub parent_object_id: Option<String>,
    /// The index of the placeholder. If the same placeholder types are the present
    /// in the same page, they would have different index values.
    pub index: Option<i32>,
    /// The type of the placeholder.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for Placeholder {}


/// Update the styling of text in a Shape or
/// Table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTextStyleRequest {
    /// The fields that should be updated.
    /// 
    /// At least one field must be specified. The root `style` is implied and
    /// should not be specified. A single `"*"` can be used as short-hand for
    /// listing every field.
    /// 
    /// For example to update the text style to bold, set `fields` to `"bold"`.
    /// 
    /// To reset a property to its default value,
    /// include its field name in the field mask but leave the field itself unset.
    pub fields: Option<String>,
    /// The style(s) to set on the text.
    /// 
    /// If the value for a particular style matches that of the parent, that style
    /// will be set to inherit.
    /// 
    /// Certain text style changes may cause other changes meant to mirror the
    /// behavior of the Slides editor. See the documentation of
    /// TextStyle for more information.
    pub style: Option<TextStyle>,
    /// The range of text to style.
    /// 
    /// The range may be extended to include adjacent newlines.
    /// 
    /// If the range fully contains a paragraph belonging to a list, the
    /// paragraph's bullet is also updated with the matching text style.
    #[serde(rename="textRange")]
    pub text_range: Option<Range>,
    /// The object ID of the shape or table with the text to be styled.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The optional table cell location if the text to be styled is in a table
    /// cell. If present, the object_id must refer to a table.
    #[serde(rename="cellLocation")]
    pub cell_location: Option<TableCellLocation>,
}

impl Part for UpdateTextStyleRequest {}


/// Updates the position of slides in the presentation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSlidesPositionRequest {
    /// The IDs of the slides in the presentation that should be moved.
    /// The slides in this list must be in existing presentation order, without
    /// duplicates.
    #[serde(rename="slideObjectIds")]
    pub slide_object_ids: Option<Vec<String>>,
    /// The index where the slides should be inserted, based on the slide
    /// arrangement before the move takes place. Must be between zero and the
    /// number of slides in the presentation, inclusive.
    #[serde(rename="insertionIndex")]
    pub insertion_index: Option<i32>,
}

impl Part for UpdateSlidesPositionRequest {}


/// The properties of the Line.
/// 
/// When unset, these fields default to values that match the appearance of
/// new lines created in the Slides editor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LineProperties {
    /// The dash style of the line.
    #[serde(rename="dashStyle")]
    pub dash_style: Option<String>,
    /// The hyperlink destination of the line. If unset, there is no link.
    pub link: Option<Link>,
    /// The style of the arrow at the beginning of the line.
    #[serde(rename="startArrow")]
    pub start_arrow: Option<String>,
    /// The thickness of the line.
    pub weight: Option<Dimension>,
    /// The style of the arrow at the end of the line.
    #[serde(rename="endArrow")]
    pub end_arrow: Option<String>,
    /// The fill of the line. The default line fill matches the defaults for new
    /// lines created in the Slides editor.
    #[serde(rename="lineFill")]
    pub line_fill: Option<LineFill>,
}

impl Part for LineProperties {}


/// The outline of a PageElement.
/// 
/// If these fields are unset, they may be inherited from a parent placeholder
/// if it exists. If there is no parent, the fields will default to the value
/// used for new page elements created in the Slides editor, which may depend on
/// the page element kind.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Outline {
    /// The dash style of the outline.
    #[serde(rename="dashStyle")]
    pub dash_style: Option<String>,
    /// The outline property state.
    /// 
    /// Updating the the outline on a page element will implicitly update this
    /// field to`RENDERED`, unless another value is specified in the same request.
    /// To have no outline on a page element, set this field to `NOT_RENDERED`. In
    /// this case, any other outline fields set in the same request will be
    /// ignored.
    #[serde(rename="propertyState")]
    pub property_state: Option<String>,
    /// The fill of the outline.
    #[serde(rename="outlineFill")]
    pub outline_fill: Option<OutlineFill>,
    /// The thickness of the outline.
    pub weight: Option<Dimension>,
}

impl Part for Outline {}


/// The result of creating a line.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateLineResponse {
    /// The object ID of the created line.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for CreateLineResponse {}


/// A single kind of update to apply to a presentation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    /// Inserts text into a shape or table cell.
    #[serde(rename="insertText")]
    pub insert_text: Option<InsertTextRequest>,
    /// Inserts rows into a table.
    #[serde(rename="insertTableRows")]
    pub insert_table_rows: Option<InsertTableRowsRequest>,
    /// Replaces all shapes matching some criteria with an image.
    #[serde(rename="replaceAllShapesWithImage")]
    pub replace_all_shapes_with_image: Option<ReplaceAllShapesWithImageRequest>,
    /// Updates the position of a set of slides in the presentation.
    #[serde(rename="updateSlidesPosition")]
    pub update_slides_position: Option<UpdateSlidesPositionRequest>,
    /// Duplicates a slide or page element.
    #[serde(rename="duplicateObject")]
    pub duplicate_object: Option<DuplicateObjectRequest>,
    /// Refreshes a Google Sheets chart.
    #[serde(rename="refreshSheetsChart")]
    pub refresh_sheets_chart: Option<RefreshSheetsChartRequest>,
    /// Replaces all instances of specified text.
    #[serde(rename="replaceAllText")]
    pub replace_all_text: Option<ReplaceAllTextRequest>,
    /// Creates an embedded Google Sheets chart.
    #[serde(rename="createSheetsChart")]
    pub create_sheets_chart: Option<CreateSheetsChartRequest>,
    /// Creates a video.
    #[serde(rename="createVideo")]
    pub create_video: Option<CreateVideoRequest>,
    /// Deletes a row from a table.
    #[serde(rename="deleteTableRow")]
    pub delete_table_row: Option<DeleteTableRowRequest>,
    /// Creates a new slide.
    #[serde(rename="createSlide")]
    pub create_slide: Option<CreateSlideRequest>,
    /// Creates a new table.
    #[serde(rename="createTable")]
    pub create_table: Option<CreateTableRequest>,
    /// Updates the properties of an Image.
    #[serde(rename="updateImageProperties")]
    pub update_image_properties: Option<UpdateImagePropertiesRequest>,
    /// Creates a line.
    #[serde(rename="createLine")]
    pub create_line: Option<CreateLineRequest>,
    /// Deletes a page or page element from the presentation.
    #[serde(rename="deleteObject")]
    pub delete_object: Option<DeleteObjectRequest>,
    /// Creates an image.
    #[serde(rename="createImage")]
    pub create_image: Option<CreateImageRequest>,
    /// Updates the properties of a Video.
    #[serde(rename="updateVideoProperties")]
    pub update_video_properties: Option<UpdateVideoPropertiesRequest>,
    /// Updates the properties of a Shape.
    #[serde(rename="updateShapeProperties")]
    pub update_shape_properties: Option<UpdateShapePropertiesRequest>,
    /// Creates a new shape.
    #[serde(rename="createShape")]
    pub create_shape: Option<CreateShapeRequest>,
    /// Updates the transform of a page element.
    #[serde(rename="updatePageElementTransform")]
    pub update_page_element_transform: Option<UpdatePageElementTransformRequest>,
    /// Creates bullets for paragraphs.
    #[serde(rename="createParagraphBullets")]
    pub create_paragraph_bullets: Option<CreateParagraphBulletsRequest>,
    /// Updates the styling of text within a Shape or Table.
    #[serde(rename="updateTextStyle")]
    pub update_text_style: Option<UpdateTextStyleRequest>,
    /// Deletes text from a shape or a table cell.
    #[serde(rename="deleteText")]
    pub delete_text: Option<DeleteTextRequest>,
    /// Updates the properties of a Page.
    #[serde(rename="updatePageProperties")]
    pub update_page_properties: Option<UpdatePagePropertiesRequest>,
    /// Updates the properties of a Line.
    #[serde(rename="updateLineProperties")]
    pub update_line_properties: Option<UpdateLinePropertiesRequest>,
    /// Updates the properties of a TableCell.
    #[serde(rename="updateTableCellProperties")]
    pub update_table_cell_properties: Option<UpdateTableCellPropertiesRequest>,
    /// Deletes a column from a table.
    #[serde(rename="deleteTableColumn")]
    pub delete_table_column: Option<DeleteTableColumnRequest>,
    /// Inserts columns into a table.
    #[serde(rename="insertTableColumns")]
    pub insert_table_columns: Option<InsertTableColumnsRequest>,
}

impl Part for Request {}


/// A List describes the look and feel of bullets belonging to paragraphs
/// associated with a list. A paragraph that is part of a list has an implicit
/// reference to that list's ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct List {
    /// A map of nesting levels to the properties of bullets at the associated
    /// level. A list has at most nine levels of nesting, so the possible values
    /// for the keys of this map are 0 through 8, inclusive.
    #[serde(rename="nestingLevel")]
    pub nesting_level: Option<HashMap<String, NestingLevel>>,
    /// The ID of the list.
    #[serde(rename="listId")]
    pub list_id: Option<String>,
}

impl Part for List {}


/// Update the properties of a TableCell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTableCellPropertiesRequest {
    /// The table range representing the subset of the table to which the updates
    /// are applied. If a table range is not specified, the updates will apply to
    /// the entire table.
    #[serde(rename="tableRange")]
    pub table_range: Option<TableRange>,
    /// The table cell properties to update.
    #[serde(rename="tableCellProperties")]
    pub table_cell_properties: Option<TableCellProperties>,
    /// The object ID of the table.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The fields that should be updated.
    /// 
    /// At least one field must be specified. The root `tableCellProperties` is
    /// implied and should not be specified. A single `"*"` can be used as
    /// short-hand for listing every field.
    /// 
    /// For example to update the table cell background solid fill color, set
    /// `fields` to `"tableCellBackgroundFill.solidFill.color"`.
    /// 
    /// To reset a property to its default value, include its field name in the
    /// field mask but leave the field itself unset.
    pub fields: Option<String>,
}

impl Part for UpdateTableCellPropertiesRequest {}


/// Specifies a contiguous range of an indexed collection, such as characters in
/// text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Range {
    /// The optional zero-based index of the end of the collection.
    /// Required for `SPECIFIC_RANGE` delete mode.
    #[serde(rename="endIndex")]
    pub end_index: Option<i32>,
    /// The optional zero-based index of the beginning of the collection.
    /// Required for `SPECIFIC_RANGE` and `FROM_START_INDEX` ranges.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// The type of range.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for Range {}


/// A TextElement kind that represents the beginning of a new paragraph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParagraphMarker {
    /// The paragraph's style
    pub style: Option<ParagraphStyle>,
    /// The bullet for this paragraph. If not present, the paragraph does not
    /// belong to a list.
    pub bullet: Option<Bullet>,
}

impl Part for ParagraphMarker {}


/// Slide layout reference. This may reference either:
/// 
/// - A predefined layout
/// - One of the layouts in the presentation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LayoutReference {
    /// Predefined layout.
    #[serde(rename="predefinedLayout")]
    pub predefined_layout: Option<String>,
    /// Layout ID: the object ID of one of the layouts in the presentation.
    #[serde(rename="layoutId")]
    pub layout_id: Option<String>,
}

impl Part for LayoutReference {}


/// Creates a new shape.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateShapeRequest {
    /// A user-supplied object ID.
    /// 
    /// If you specify an ID, it must be unique among all pages and page elements
    /// in the presentation. The ID must start with an alphanumeric character or an
    /// underscore (matches regex `[a-zA-Z0-9_]`); remaining characters
    /// may include those as well as a hyphen or colon (matches regex
    /// `[a-zA-Z0-9_-:]`).
    /// The length of the ID must not be less than 5 or greater than 50.
    /// If empty, a unique identifier will be generated.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The shape type.
    #[serde(rename="shapeType")]
    pub shape_type: Option<String>,
    /// The element properties for the shape.
    #[serde(rename="elementProperties")]
    pub element_properties: Option<PageElementProperties>,
}

impl Part for CreateShapeRequest {}


/// A color and position in a gradient band.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColorStop {
    /// The color of the gradient stop.
    pub color: Option<OpaqueColor>,
    /// The relative position of the color stop in the gradient band measured
    /// in percentage. The value should be in the interval [0.0, 1.0].
    pub position: Option<f32>,
    /// The alpha value of this color in the gradient band. Defaults to 1.0,
    /// fully opaque.
    pub alpha: Option<f32>,
}

impl Part for ColorStop {}


/// The result of creating an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateImageResponse {
    /// The object ID of the created image.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for CreateImageResponse {}


/// Creates a new slide.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateSlideRequest {
    /// Layout reference of the slide to be inserted, based on the *current
    /// master*, which is one of the following:
    /// 
    /// - The master of the previous slide index.
    /// - The master of the first slide, if the insertion_index is zero.
    /// - The first master in the presentation, if there are no slides.
    /// 
    /// If the LayoutReference is not found in the current master, a 400 bad
    /// request error is returned.
    /// 
    /// If you don't specify a layout reference, then the new slide will use the
    /// predefined layout `BLANK`.
    #[serde(rename="slideLayoutReference")]
    pub slide_layout_reference: Option<LayoutReference>,
    /// A user-supplied object ID.
    /// 
    /// If you specify an ID, it must be unique among all pages and page elements
    /// in the presentation. The ID must start with an alphanumeric character or an
    /// underscore (matches regex `[a-zA-Z0-9_]`); remaining characters
    /// may include those as well as a hyphen or colon (matches regex
    /// `[a-zA-Z0-9_-:]`).
    /// The length of the ID must not be less than 5 or greater than 50.
    /// 
    /// If you don't specify an ID, a unique one is generated.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The optional zero-based index indicating where to insert the slides.
    /// 
    /// If you don't specify an index, the new slide is created at the end.
    #[serde(rename="insertionIndex")]
    pub insertion_index: Option<i32>,
}

impl Part for CreateSlideRequest {}


/// Creates a new table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateTableRequest {
    /// Number of columns in the table.
    pub columns: Option<i32>,
    /// Number of rows in the table.
    pub rows: Option<i32>,
    /// A user-supplied object ID.
    /// 
    /// If you specify an ID, it must be unique among all pages and page elements
    /// in the presentation. The ID must start with an alphanumeric character or an
    /// underscore (matches regex `[a-zA-Z0-9_]`); remaining characters
    /// may include those as well as a hyphen or colon (matches regex
    /// `[a-zA-Z0-9_-:]`).
    /// The length of the ID must not be less than 5 or greater than 50.
    /// 
    /// If you don't specify an ID, a unique one is generated.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The element properties for the table.
    /// 
    /// The table will be created at the provided size, subject to a minimum size.
    /// If no size is provided, the table will be automatically sized.
    /// 
    /// Table transforms must have a scale of 1 and no shear components. If no
    /// transform is provided, the table will be centered on the page.
    #[serde(rename="elementProperties")]
    pub element_properties: Option<PageElementProperties>,
}

impl Part for CreateTableRequest {}


/// The fill of the outline.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OutlineFill {
    /// Solid color fill.
    #[serde(rename="solidFill")]
    pub solid_fill: Option<SolidFill>,
}

impl Part for OutlineFill {}


/// Deletes an object, either pages or
/// page elements, from the
/// presentation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteObjectRequest {
    /// The object ID of the page or page element to delete.
    /// 
    /// If after a delete operation a group contains
    /// only 1 or no page elements, the group is also deleted.
    /// 
    /// If a placeholder is deleted on a layout, any empty inheriting shapes are
    /// also deleted.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for DeleteObjectRequest {}


/// A PageElement kind representing an
/// image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// An URL to an image with a default lifetime of 30 minutes.
    /// This URL is tagged with the account of the requester. Anyone with the URL
    /// effectively accesses the image as the original requester. Access to the
    /// image may be lost if the presentation's sharing settings change.
    #[serde(rename="contentUrl")]
    pub content_url: Option<String>,
    /// The properties of the image.
    #[serde(rename="imageProperties")]
    pub image_properties: Option<ImageProperties>,
}

impl Part for Image {}


/// The properties of the TableCell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCellProperties {
    /// The background fill of the table cell. The default fill matches the fill
    /// for newly created table cells in the Slides editor.
    #[serde(rename="tableCellBackgroundFill")]
    pub table_cell_background_fill: Option<TableCellBackgroundFill>,
}

impl Part for TableCellProperties {}


/// A recolor effect applied on an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Recolor {
    /// The recolor effect is represented by a gradient, which is a list of color
    /// stops. This property is read-only.
    #[serde(rename="recolorStops")]
    pub recolor_stops: Option<Vec<ColorStop>>,
}

impl Part for Recolor {}


/// Request message for PresentationsService.BatchUpdatePresentation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update presentations](struct.PresentationBatchUpdateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdatePresentationRequest {
    /// A list of updates to apply to the presentation.
    pub requests: Option<Vec<Request>>,
}

impl RequestValue for BatchUpdatePresentationRequest {}


/// The result of creating an embedded Google Sheets chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateSheetsChartResponse {
    /// The object ID of the created chart.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for CreateSheetsChartResponse {}


/// A PageElement kind representing a
/// generic shape that does not have a more specific classification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Shape {
    /// The text content of the shape.
    pub text: Option<TextContent>,
    /// The properties of the shape.
    #[serde(rename="shapeProperties")]
    pub shape_properties: Option<ShapeProperties>,
    /// Placeholders are shapes that are inherit from corresponding placeholders on
    /// layouts and masters.
    /// 
    /// If set, the shape is a placeholder shape and any inherited properties
    /// can be resolved by looking at the parent placeholder identified by the
    /// Placeholder.parent_object_id field.
    pub placeholder: Option<Placeholder>,
    /// The type of the shape.
    #[serde(rename="shapeType")]
    pub shape_type: Option<String>,
}

impl Part for Shape {}


/// Updates the transform of a page element.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePageElementTransformRequest {
    /// The apply mode of the transform update.
    #[serde(rename="applyMode")]
    pub apply_mode: Option<String>,
    /// The input transform matrix used to update the page element.
    pub transform: Option<AffineTransform>,
    /// The object ID of the page element to update.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for UpdatePageElementTransformRequest {}


/// Creates bullets for all of the paragraphs that overlap with the given
/// text index range.
/// 
/// The nesting level of each paragraph will be determined by counting leading
/// tabs in front of each paragraph. To avoid excess space between the bullet and
/// the corresponding paragraph, these leading tabs are removed by this request.
/// This may change the indices of parts of the text.
/// 
/// If the paragraph immediately before paragraphs being updated is in a list
/// with a matching preset, the paragraphs being updated are added to that
/// preceding list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateParagraphBulletsRequest {
    /// The kinds of bullet glyphs to be used. Defaults to the
    /// `BULLET_DISC_CIRCLE_SQUARE` preset.
    #[serde(rename="bulletPreset")]
    pub bullet_preset: Option<String>,
    /// The range of text to apply the bullet presets to, based on TextElement indexes.
    #[serde(rename="textRange")]
    pub text_range: Option<Range>,
    /// The object ID of the shape or table containing the text to add bullets to.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The optional table cell location if the text to be modified is in a table
    /// cell. If present, the object_id must refer to a table.
    #[serde(rename="cellLocation")]
    pub cell_location: Option<TableCellLocation>,
}

impl Part for CreateParagraphBulletsRequest {}


/// Deletes text from a shape or a table cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteTextRequest {
    /// The range of text to delete, based on TextElement indexes.
    /// 
    /// There is always an implicit newline character at the end of a shape's or
    /// table cell's text that cannot be deleted. `Range.Type.ALL` will use the
    /// correct bounds, but care must be taken when specifying explicit bounds for
    /// range types `FROM_START_INDEX` and `FIXED_RANGE`. For example, if the text
    /// is "ABC", followed by an implicit newline, then the maximum value is 2 for
    /// `text_range.start_index` and 3 for `text_range.end_index`.
    /// 
    /// Deleting text that crosses a paragraph boundary may result in changes
    /// to paragraph styles and lists as the two paragraphs are merged.
    /// 
    /// Ranges that include only one code unit of a surrogate pair are expanded to
    /// include both code units.
    #[serde(rename="textRange")]
    pub text_range: Option<Range>,
    /// The object ID of the shape or table from which the text will be deleted.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
    /// The optional table cell location if the text is to be deleted from a table
    /// cell. If present, the object_id must refer to a table.
    #[serde(rename="cellLocation")]
    pub cell_location: Option<TableCellLocation>,
}

impl Part for DeleteTextRequest {}


/// Update the properties of an Image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateImagePropertiesRequest {
    /// The fields that should be updated.
    /// 
    /// At least one field must be specified. The root `imageProperties` is
    /// implied and should not be specified. A single `"*"` can be used as
    /// short-hand for listing every field.
    /// 
    /// For example to update the image outline color, set `fields` to
    /// `"outline.outlineFill.solidFill.color"`.
    /// 
    /// To reset a property to its default value, include its field name in the
    /// field mask but leave the field itself unset.
    pub fields: Option<String>,
    /// The image properties to update.
    #[serde(rename="imageProperties")]
    pub image_properties: Option<ImageProperties>,
    /// The object ID of the image the updates are applied to.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for UpdateImagePropertiesRequest {}


/// A PageElement kind representing a
/// table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    /// Properties of each column.
    #[serde(rename="tableColumns")]
    pub table_columns: Option<Vec<TableColumnProperties>>,
    /// Properties and contents of each row.
    /// 
    /// Cells that span multiple rows are contained in only one of these rows and
    /// have a row_span greater
    /// than 1.
    #[serde(rename="tableRows")]
    pub table_rows: Option<Vec<TableRow>>,
    /// Number of rows in the table.
    pub rows: Option<i32>,
    /// Number of columns in the table.
    pub columns: Option<i32>,
}

impl Part for Table {}


/// A pair mapping a theme color type to the concrete color it represents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThemeColorPair {
    /// The concrete color corresponding to the theme color type above.
    pub color: Option<RgbColor>,
    /// The type of the theme color.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for ThemeColorPair {}


/// Updates the properties of a Page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePagePropertiesRequest {
    /// The page properties to update.
    #[serde(rename="pageProperties")]
    pub page_properties: Option<PageProperties>,
    /// The fields that should be updated.
    /// 
    /// At least one field must be specified. The root `pageProperties` is
    /// implied and should not be specified. A single `"*"` can be used as
    /// short-hand for listing every field.
    /// 
    /// For example to update the page background solid fill color, set `fields`
    /// to `"pageBackgroundFill.solidFill.color"`.
    /// 
    /// To reset a property to its default value, include its field name in the
    /// field mask but leave the field itself unset.
    pub fields: Option<String>,
    /// The object ID of the page the update is applied to.
    #[serde(rename="objectId")]
    pub object_id: Option<String>,
}

impl Part for UpdatePagePropertiesRequest {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *presentation* resources.
/// It is not used directly, but through the `Slides` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_slides1 as slides1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use slides1::Slides;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Slides::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_update(...)`, `create(...)`, `get(...)` and `pages_get(...)`
/// // to build up your call.
/// let rb = hub.presentations();
/// # }
/// ```
pub struct PresentationMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Slides<C, A>,
}

impl<'a, C, A> MethodsBuilder for PresentationMethods<'a, C, A> {}

impl<'a, C, A> PresentationMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new presentation using the title given in the request. Other
    /// fields in the request are ignored.
    /// Returns the created presentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Presentation) -> PresentationCreateCall<'a, C, A> {
        PresentationCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest version of the specified presentation.
    /// 
    /// # Arguments
    ///
    /// * `presentationId` - The ID of the presentation to retrieve.
    pub fn get(&self, presentation_id: &str) -> PresentationGetCall<'a, C, A> {
        PresentationGetCall {
            hub: self.hub,
            _presentation_id: presentation_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies one or more updates to the presentation.
    /// 
    /// Each request is validated before
    /// being applied. If any request is not valid, then the entire request will
    /// fail and nothing will be applied.
    /// 
    /// Some requests have replies to
    /// give you some information about how they are applied. Other requests do
    /// not need to return information; these each return an empty reply.
    /// The order of replies matches that of the requests.
    /// 
    /// For example, suppose you call batchUpdate with four updates, and only the
    /// third one returns information. The response would have two empty replies:
    /// the reply to the third request, and another empty reply, in that order.
    /// 
    /// Because other users may be editing the presentation, the presentation
    /// might not exactly reflect your changes: your changes may
    /// be altered with respect to collaborator changes. If there are no
    /// collaborators, the presentation should reflect your changes. In any case,
    /// the updates in your request are guaranteed to be applied together
    /// atomically.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `presentationId` - The presentation to apply the updates to.
    pub fn batch_update(&self, request: BatchUpdatePresentationRequest, presentation_id: &str) -> PresentationBatchUpdateCall<'a, C, A> {
        PresentationBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _presentation_id: presentation_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest version of the specified page in the presentation.
    /// 
    /// # Arguments
    ///
    /// * `presentationId` - The ID of the presentation to retrieve.
    /// * `pageObjectId` - The object ID of the page to retrieve.
    pub fn pages_get(&self, presentation_id: &str, page_object_id: &str) -> PresentationPageGetCall<'a, C, A> {
        PresentationPageGetCall {
            hub: self.hub,
            _presentation_id: presentation_id.to_string(),
            _page_object_id: page_object_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Creates a new presentation using the title given in the request. Other
/// fields in the request are ignored.
/// Returns the created presentation.
///
/// A builder for the *create* method supported by a *presentation* resource.
/// It is not used directly, but through a `PresentationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_slides1 as slides1;
/// use slides1::Presentation;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use slides1::Slides;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Slides::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Presentation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.presentations().create(req)
///              .doit();
/// # }
/// ```
pub struct PresentationCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Slides<C, A>,
    _request: Presentation,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PresentationCreateCall<'a, C, A> {}

impl<'a, C, A> PresentationCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Presentation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "slides.presentations.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/presentations";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }


        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Presentation) -> PresentationCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PresentationCreateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PresentationCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PresentationCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Gets the latest version of the specified presentation.
///
/// A builder for the *get* method supported by a *presentation* resource.
/// It is not used directly, but through a `PresentationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_slides1 as slides1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use slides1::Slides;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Slides::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.presentations().get("presentationId")
///              .doit();
/// # }
/// ```
pub struct PresentationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Slides<C, A>,
    _presentation_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PresentationGetCall<'a, C, A> {}

impl<'a, C, A> PresentationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Presentation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "slides.presentations.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("presentationId", self._presentation_id.to_string()));
        for &field in ["alt", "presentationId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/presentations/{+presentationId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+presentationId}", "presentationId")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["presentationId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the presentation to retrieve.
    ///
    /// Sets the *presentation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn presentation_id(mut self, new_value: &str) -> PresentationGetCall<'a, C, A> {
        self._presentation_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PresentationGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PresentationGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::DriveReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PresentationGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Applies one or more updates to the presentation.
/// 
/// Each request is validated before
/// being applied. If any request is not valid, then the entire request will
/// fail and nothing will be applied.
/// 
/// Some requests have replies to
/// give you some information about how they are applied. Other requests do
/// not need to return information; these each return an empty reply.
/// The order of replies matches that of the requests.
/// 
/// For example, suppose you call batchUpdate with four updates, and only the
/// third one returns information. The response would have two empty replies:
/// the reply to the third request, and another empty reply, in that order.
/// 
/// Because other users may be editing the presentation, the presentation
/// might not exactly reflect your changes: your changes may
/// be altered with respect to collaborator changes. If there are no
/// collaborators, the presentation should reflect your changes. In any case,
/// the updates in your request are guaranteed to be applied together
/// atomically.
///
/// A builder for the *batchUpdate* method supported by a *presentation* resource.
/// It is not used directly, but through a `PresentationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_slides1 as slides1;
/// use slides1::BatchUpdatePresentationRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use slides1::Slides;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Slides::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchUpdatePresentationRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.presentations().batch_update(req, "presentationId")
///              .doit();
/// # }
/// ```
pub struct PresentationBatchUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Slides<C, A>,
    _request: BatchUpdatePresentationRequest,
    _presentation_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PresentationBatchUpdateCall<'a, C, A> {}

impl<'a, C, A> PresentationBatchUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchUpdatePresentationResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "slides.presentations.batchUpdate",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("presentationId", self._presentation_id.to_string()));
        for &field in ["alt", "presentationId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/presentations/{presentationId}:batchUpdate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{presentationId}", "presentationId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["presentationId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BatchUpdatePresentationRequest) -> PresentationBatchUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The presentation to apply the updates to.
    ///
    /// Sets the *presentation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn presentation_id(mut self, new_value: &str) -> PresentationBatchUpdateCall<'a, C, A> {
        self._presentation_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PresentationBatchUpdateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PresentationBatchUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PresentationBatchUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Gets the latest version of the specified page in the presentation.
///
/// A builder for the *pages.get* method supported by a *presentation* resource.
/// It is not used directly, but through a `PresentationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_slides1 as slides1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use slides1::Slides;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Slides::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.presentations().pages_get("presentationId", "pageObjectId")
///              .doit();
/// # }
/// ```
pub struct PresentationPageGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Slides<C, A>,
    _presentation_id: String,
    _page_object_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PresentationPageGetCall<'a, C, A> {}

impl<'a, C, A> PresentationPageGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Page)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "slides.presentations.pages.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("presentationId", self._presentation_id.to_string()));
        params.push(("pageObjectId", self._page_object_id.to_string()));
        for &field in ["alt", "presentationId", "pageObjectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/presentations/{presentationId}/pages/{pageObjectId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{presentationId}", "presentationId"), ("{pageObjectId}", "pageObjectId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["pageObjectId", "presentationId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the presentation to retrieve.
    ///
    /// Sets the *presentation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn presentation_id(mut self, new_value: &str) -> PresentationPageGetCall<'a, C, A> {
        self._presentation_id = new_value.to_string();
        self
    }
    /// The object ID of the page to retrieve.
    ///
    /// Sets the *page object id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn page_object_id(mut self, new_value: &str) -> PresentationPageGetCall<'a, C, A> {
        self._page_object_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PresentationPageGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PresentationPageGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::DriveReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PresentationPageGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


