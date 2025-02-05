// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>An analyzed segment for a real-time analysis session.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RealtimeContactAnalysisSegment {
    /// <p>The analyzed transcript.</p>
    pub transcript: std::option::Option<crate::model::Transcript>,
    /// <p>The matched category rules.</p>
    pub categories: std::option::Option<crate::model::Categories>,
}
impl std::fmt::Debug for RealtimeContactAnalysisSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RealtimeContactAnalysisSegment");
        formatter.field("transcript", &self.transcript);
        formatter.field("categories", &self.categories);
        formatter.finish()
    }
}
/// See [`RealtimeContactAnalysisSegment`](crate::model::RealtimeContactAnalysisSegment)
pub mod realtime_contact_analysis_segment {
    /// A builder for [`RealtimeContactAnalysisSegment`](crate::model::RealtimeContactAnalysisSegment)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) transcript: std::option::Option<crate::model::Transcript>,
        pub(crate) categories: std::option::Option<crate::model::Categories>,
    }
    impl Builder {
        /// <p>The analyzed transcript.</p>
        pub fn transcript(mut self, input: crate::model::Transcript) -> Self {
            self.transcript = Some(input);
            self
        }
        pub fn set_transcript(
            mut self,
            input: std::option::Option<crate::model::Transcript>,
        ) -> Self {
            self.transcript = input;
            self
        }
        /// <p>The matched category rules.</p>
        pub fn categories(mut self, input: crate::model::Categories) -> Self {
            self.categories = Some(input);
            self
        }
        pub fn set_categories(
            mut self,
            input: std::option::Option<crate::model::Categories>,
        ) -> Self {
            self.categories = input;
            self
        }
        /// Consumes the builder and constructs a [`RealtimeContactAnalysisSegment`](crate::model::RealtimeContactAnalysisSegment)
        pub fn build(self) -> crate::model::RealtimeContactAnalysisSegment {
            crate::model::RealtimeContactAnalysisSegment {
                transcript: self.transcript,
                categories: self.categories,
            }
        }
    }
}
impl RealtimeContactAnalysisSegment {
    /// Creates a new builder-style object to manufacture [`RealtimeContactAnalysisSegment`](crate::model::RealtimeContactAnalysisSegment)
    pub fn builder() -> crate::model::realtime_contact_analysis_segment::Builder {
        crate::model::realtime_contact_analysis_segment::Builder::default()
    }
}

/// <p>Provides the category rules that are used to automatically categorize contacts based on
/// uttered keywords and phrases.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Categories {
    /// <p>The category rules that have been matched in the analyzed segment.</p>
    pub matched_categories: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The category rule that was matched and when it occurred in the transcript.</p>
    pub matched_details: std::option::Option<
        std::collections::HashMap<std::string::String, crate::model::CategoryDetails>,
    >,
}
impl std::fmt::Debug for Categories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Categories");
        formatter.field("matched_categories", &self.matched_categories);
        formatter.field("matched_details", &self.matched_details);
        formatter.finish()
    }
}
/// See [`Categories`](crate::model::Categories)
pub mod categories {
    /// A builder for [`Categories`](crate::model::Categories)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) matched_categories: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) matched_details: std::option::Option<
            std::collections::HashMap<std::string::String, crate::model::CategoryDetails>,
        >,
    }
    impl Builder {
        pub fn matched_categories(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.matched_categories.unwrap_or_default();
            v.push(input.into());
            self.matched_categories = Some(v);
            self
        }
        pub fn set_matched_categories(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.matched_categories = input;
            self
        }
        pub fn matched_details(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<crate::model::CategoryDetails>,
        ) -> Self {
            let mut hash_map = self.matched_details.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.matched_details = Some(hash_map);
            self
        }
        pub fn set_matched_details(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, crate::model::CategoryDetails>,
            >,
        ) -> Self {
            self.matched_details = input;
            self
        }
        /// Consumes the builder and constructs a [`Categories`](crate::model::Categories)
        pub fn build(self) -> crate::model::Categories {
            crate::model::Categories {
                matched_categories: self.matched_categories,
                matched_details: self.matched_details,
            }
        }
    }
}
impl Categories {
    /// Creates a new builder-style object to manufacture [`Categories`](crate::model::Categories)
    pub fn builder() -> crate::model::categories::Builder {
        crate::model::categories::Builder::default()
    }
}

/// <p>Provides information about the category rule that was matched.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CategoryDetails {
    /// <p>The section of audio where the category rule was detected.</p>
    pub points_of_interest: std::option::Option<std::vec::Vec<crate::model::PointOfInterest>>,
}
impl std::fmt::Debug for CategoryDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CategoryDetails");
        formatter.field("points_of_interest", &self.points_of_interest);
        formatter.finish()
    }
}
/// See [`CategoryDetails`](crate::model::CategoryDetails)
pub mod category_details {
    /// A builder for [`CategoryDetails`](crate::model::CategoryDetails)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) points_of_interest:
            std::option::Option<std::vec::Vec<crate::model::PointOfInterest>>,
    }
    impl Builder {
        pub fn points_of_interest(
            mut self,
            input: impl Into<crate::model::PointOfInterest>,
        ) -> Self {
            let mut v = self.points_of_interest.unwrap_or_default();
            v.push(input.into());
            self.points_of_interest = Some(v);
            self
        }
        pub fn set_points_of_interest(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::PointOfInterest>>,
        ) -> Self {
            self.points_of_interest = input;
            self
        }
        /// Consumes the builder and constructs a [`CategoryDetails`](crate::model::CategoryDetails)
        pub fn build(self) -> crate::model::CategoryDetails {
            crate::model::CategoryDetails {
                points_of_interest: self.points_of_interest,
            }
        }
    }
}
impl CategoryDetails {
    /// Creates a new builder-style object to manufacture [`CategoryDetails`](crate::model::CategoryDetails)
    pub fn builder() -> crate::model::category_details::Builder {
        crate::model::category_details::Builder::default()
    }
}

/// <p>The section of the contact audio where that category rule was detected.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PointOfInterest {
    /// <p>The beginning offset in milliseconds where the category rule was detected.</p>
    pub begin_offset_millis: i32,
    /// <p>The ending offset in milliseconds where the category rule was detected.</p>
    pub end_offset_millis: i32,
}
impl std::fmt::Debug for PointOfInterest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PointOfInterest");
        formatter.field("begin_offset_millis", &self.begin_offset_millis);
        formatter.field("end_offset_millis", &self.end_offset_millis);
        formatter.finish()
    }
}
/// See [`PointOfInterest`](crate::model::PointOfInterest)
pub mod point_of_interest {
    /// A builder for [`PointOfInterest`](crate::model::PointOfInterest)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) begin_offset_millis: std::option::Option<i32>,
        pub(crate) end_offset_millis: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The beginning offset in milliseconds where the category rule was detected.</p>
        pub fn begin_offset_millis(mut self, input: i32) -> Self {
            self.begin_offset_millis = Some(input);
            self
        }
        pub fn set_begin_offset_millis(mut self, input: std::option::Option<i32>) -> Self {
            self.begin_offset_millis = input;
            self
        }
        /// <p>The ending offset in milliseconds where the category rule was detected.</p>
        pub fn end_offset_millis(mut self, input: i32) -> Self {
            self.end_offset_millis = Some(input);
            self
        }
        pub fn set_end_offset_millis(mut self, input: std::option::Option<i32>) -> Self {
            self.end_offset_millis = input;
            self
        }
        /// Consumes the builder and constructs a [`PointOfInterest`](crate::model::PointOfInterest)
        pub fn build(self) -> crate::model::PointOfInterest {
            crate::model::PointOfInterest {
                begin_offset_millis: self.begin_offset_millis.unwrap_or_default(),
                end_offset_millis: self.end_offset_millis.unwrap_or_default(),
            }
        }
    }
}
impl PointOfInterest {
    /// Creates a new builder-style object to manufacture [`PointOfInterest`](crate::model::PointOfInterest)
    pub fn builder() -> crate::model::point_of_interest::Builder {
        crate::model::point_of_interest::Builder::default()
    }
}

/// <p>A list of messages in the session.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Transcript {
    /// <p>The identifier of the transcript.</p>
    pub id: std::option::Option<std::string::String>,
    /// <p>The identifier of the participant.</p>
    pub participant_id: std::option::Option<std::string::String>,
    /// <p>The role of participant. For example, is it a customer, agent, or system.</p>
    pub participant_role: std::option::Option<std::string::String>,
    /// <p>The content of the transcript.</p>
    pub content: std::option::Option<std::string::String>,
    /// <p>The beginning offset in the contact for this transcript.</p>
    pub begin_offset_millis: i32,
    /// <p>The end offset in the contact for this transcript.</p>
    pub end_offset_millis: i32,
    /// <p>The sentiment of the detected for this piece of transcript.</p>
    pub sentiment: std::option::Option<crate::model::SentimentValue>,
    /// <p>List of positions where issues were detected on the transcript.</p>
    pub issues_detected: std::option::Option<std::vec::Vec<crate::model::IssueDetected>>,
}
impl std::fmt::Debug for Transcript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Transcript");
        formatter.field("id", &self.id);
        formatter.field("participant_id", &self.participant_id);
        formatter.field("participant_role", &self.participant_role);
        formatter.field("content", &self.content);
        formatter.field("begin_offset_millis", &self.begin_offset_millis);
        formatter.field("end_offset_millis", &self.end_offset_millis);
        formatter.field("sentiment", &self.sentiment);
        formatter.field("issues_detected", &self.issues_detected);
        formatter.finish()
    }
}
/// See [`Transcript`](crate::model::Transcript)
pub mod transcript {
    /// A builder for [`Transcript`](crate::model::Transcript)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) id: std::option::Option<std::string::String>,
        pub(crate) participant_id: std::option::Option<std::string::String>,
        pub(crate) participant_role: std::option::Option<std::string::String>,
        pub(crate) content: std::option::Option<std::string::String>,
        pub(crate) begin_offset_millis: std::option::Option<i32>,
        pub(crate) end_offset_millis: std::option::Option<i32>,
        pub(crate) sentiment: std::option::Option<crate::model::SentimentValue>,
        pub(crate) issues_detected: std::option::Option<std::vec::Vec<crate::model::IssueDetected>>,
    }
    impl Builder {
        /// <p>The identifier of the transcript.</p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input;
            self
        }
        /// <p>The identifier of the participant.</p>
        pub fn participant_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.participant_id = Some(input.into());
            self
        }
        pub fn set_participant_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.participant_id = input;
            self
        }
        /// <p>The role of participant. For example, is it a customer, agent, or system.</p>
        pub fn participant_role(mut self, input: impl Into<std::string::String>) -> Self {
            self.participant_role = Some(input.into());
            self
        }
        pub fn set_participant_role(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.participant_role = input;
            self
        }
        /// <p>The content of the transcript.</p>
        pub fn content(mut self, input: impl Into<std::string::String>) -> Self {
            self.content = Some(input.into());
            self
        }
        pub fn set_content(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content = input;
            self
        }
        /// <p>The beginning offset in the contact for this transcript.</p>
        pub fn begin_offset_millis(mut self, input: i32) -> Self {
            self.begin_offset_millis = Some(input);
            self
        }
        pub fn set_begin_offset_millis(mut self, input: std::option::Option<i32>) -> Self {
            self.begin_offset_millis = input;
            self
        }
        /// <p>The end offset in the contact for this transcript.</p>
        pub fn end_offset_millis(mut self, input: i32) -> Self {
            self.end_offset_millis = Some(input);
            self
        }
        pub fn set_end_offset_millis(mut self, input: std::option::Option<i32>) -> Self {
            self.end_offset_millis = input;
            self
        }
        /// <p>The sentiment of the detected for this piece of transcript.</p>
        pub fn sentiment(mut self, input: crate::model::SentimentValue) -> Self {
            self.sentiment = Some(input);
            self
        }
        pub fn set_sentiment(
            mut self,
            input: std::option::Option<crate::model::SentimentValue>,
        ) -> Self {
            self.sentiment = input;
            self
        }
        pub fn issues_detected(mut self, input: impl Into<crate::model::IssueDetected>) -> Self {
            let mut v = self.issues_detected.unwrap_or_default();
            v.push(input.into());
            self.issues_detected = Some(v);
            self
        }
        pub fn set_issues_detected(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::IssueDetected>>,
        ) -> Self {
            self.issues_detected = input;
            self
        }
        /// Consumes the builder and constructs a [`Transcript`](crate::model::Transcript)
        pub fn build(self) -> crate::model::Transcript {
            crate::model::Transcript {
                id: self.id,
                participant_id: self.participant_id,
                participant_role: self.participant_role,
                content: self.content,
                begin_offset_millis: self.begin_offset_millis.unwrap_or_default(),
                end_offset_millis: self.end_offset_millis.unwrap_or_default(),
                sentiment: self.sentiment,
                issues_detected: self.issues_detected,
            }
        }
    }
}
impl Transcript {
    /// Creates a new builder-style object to manufacture [`Transcript`](crate::model::Transcript)
    pub fn builder() -> crate::model::transcript::Builder {
        crate::model::transcript::Builder::default()
    }
}

/// <p>Potential issues that are detected based on an artificial intelligence analysis of each
/// turn in the conversation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct IssueDetected {
    /// <p>The offset for when the issue was detected in the segment.</p>
    pub character_offsets: std::option::Option<crate::model::CharacterOffsets>,
}
impl std::fmt::Debug for IssueDetected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("IssueDetected");
        formatter.field("character_offsets", &self.character_offsets);
        formatter.finish()
    }
}
/// See [`IssueDetected`](crate::model::IssueDetected)
pub mod issue_detected {
    /// A builder for [`IssueDetected`](crate::model::IssueDetected)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) character_offsets: std::option::Option<crate::model::CharacterOffsets>,
    }
    impl Builder {
        /// <p>The offset for when the issue was detected in the segment.</p>
        pub fn character_offsets(mut self, input: crate::model::CharacterOffsets) -> Self {
            self.character_offsets = Some(input);
            self
        }
        pub fn set_character_offsets(
            mut self,
            input: std::option::Option<crate::model::CharacterOffsets>,
        ) -> Self {
            self.character_offsets = input;
            self
        }
        /// Consumes the builder and constructs a [`IssueDetected`](crate::model::IssueDetected)
        pub fn build(self) -> crate::model::IssueDetected {
            crate::model::IssueDetected {
                character_offsets: self.character_offsets,
            }
        }
    }
}
impl IssueDetected {
    /// Creates a new builder-style object to manufacture [`IssueDetected`](crate::model::IssueDetected)
    pub fn builder() -> crate::model::issue_detected::Builder {
        crate::model::issue_detected::Builder::default()
    }
}

/// <p>For characters that were detected as issues, where they occur in the transcript.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CharacterOffsets {
    /// <p>The beginning of the issue.</p>
    pub begin_offset_char: i32,
    /// <p>The end of the issue.</p>
    pub end_offset_char: i32,
}
impl std::fmt::Debug for CharacterOffsets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CharacterOffsets");
        formatter.field("begin_offset_char", &self.begin_offset_char);
        formatter.field("end_offset_char", &self.end_offset_char);
        formatter.finish()
    }
}
/// See [`CharacterOffsets`](crate::model::CharacterOffsets)
pub mod character_offsets {
    /// A builder for [`CharacterOffsets`](crate::model::CharacterOffsets)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) begin_offset_char: std::option::Option<i32>,
        pub(crate) end_offset_char: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The beginning of the issue.</p>
        pub fn begin_offset_char(mut self, input: i32) -> Self {
            self.begin_offset_char = Some(input);
            self
        }
        pub fn set_begin_offset_char(mut self, input: std::option::Option<i32>) -> Self {
            self.begin_offset_char = input;
            self
        }
        /// <p>The end of the issue.</p>
        pub fn end_offset_char(mut self, input: i32) -> Self {
            self.end_offset_char = Some(input);
            self
        }
        pub fn set_end_offset_char(mut self, input: std::option::Option<i32>) -> Self {
            self.end_offset_char = input;
            self
        }
        /// Consumes the builder and constructs a [`CharacterOffsets`](crate::model::CharacterOffsets)
        pub fn build(self) -> crate::model::CharacterOffsets {
            crate::model::CharacterOffsets {
                begin_offset_char: self.begin_offset_char.unwrap_or_default(),
                end_offset_char: self.end_offset_char.unwrap_or_default(),
            }
        }
    }
}
impl CharacterOffsets {
    /// Creates a new builder-style object to manufacture [`CharacterOffsets`](crate::model::CharacterOffsets)
    pub fn builder() -> crate::model::character_offsets::Builder {
        crate::model::character_offsets::Builder::default()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum SentimentValue {
    Negative,
    Neutral,
    Positive,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for SentimentValue {
    fn from(s: &str) -> Self {
        match s {
            "NEGATIVE" => SentimentValue::Negative,
            "NEUTRAL" => SentimentValue::Neutral,
            "POSITIVE" => SentimentValue::Positive,
            other => SentimentValue::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for SentimentValue {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(SentimentValue::from(s))
    }
}
impl SentimentValue {
    pub fn as_str(&self) -> &str {
        match self {
            SentimentValue::Negative => "NEGATIVE",
            SentimentValue::Neutral => "NEUTRAL",
            SentimentValue::Positive => "POSITIVE",
            SentimentValue::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["NEGATIVE", "NEUTRAL", "POSITIVE"]
    }
}
impl AsRef<str> for SentimentValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
