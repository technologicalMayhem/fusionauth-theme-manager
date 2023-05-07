use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub theme: Theme,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    #[serde(skip_serializing)]
    pub data: Data,
    pub default_messages: String,
    #[serde(skip_serializing)]
    pub id: String,
    #[serde(skip_serializing)]
    pub insert_instant: u64,
    #[serde(skip_serializing)]
    pub last_update_instant: u64,
    #[serde(skip_serializing)]
    pub localized_messages: LocalizedMessages,
    pub name: String,
    pub templates: Templates,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedMessages {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Templates {
    pub account_edit: String,
    pub account_index: String,
    pub account_two_factor_disable: String,
    pub account_two_factor_enable: String,
    pub account_two_factor_index: String,
    pub account_web_authn_add: String,
    pub account_web_authn_delete: String,
    pub account_web_authn_index: String,
    pub email_complete: String,
    pub email_send: String,
    pub email_sent: String,
    pub email_verification_required: String,
    pub email_verify: String,
    pub helpers: String,
    pub index: String,
    #[serde(rename = "oauth2Authorize")]
    pub oauth2_authorize: String,
    #[serde(rename = "oauth2AuthorizedNotRegistered")]
    pub oauth2_authorized_not_registered: String,
    #[serde(rename = "oauth2ChildRegistrationNotAllowed")]
    pub oauth2_child_registration_not_allowed: String,
    #[serde(rename = "oauth2ChildRegistrationNotAllowedComplete")]
    pub oauth2_child_registration_not_allowed_complete: String,
    #[serde(rename = "oauth2CompleteRegistration")]
    pub oauth2_complete_registration: String,
    #[serde(rename = "oauth2Device")]
    pub oauth2_device: String,
    #[serde(rename = "oauth2DeviceComplete")]
    pub oauth2_device_complete: String,
    #[serde(rename = "oauth2Error")]
    pub oauth2_error: String,
    #[serde(rename = "oauth2Logout")]
    pub oauth2_logout: String,
    #[serde(rename = "oauth2Passwordless")]
    pub oauth2_passwordless: String,
    #[serde(rename = "oauth2Register")]
    pub oauth2_register: String,
    #[serde(rename = "oauth2StartIdPLink")]
    pub oauth2_start_id_plink: String,
    #[serde(rename = "oauth2TwoFactor")]
    pub oauth2_two_factor: String,
    #[serde(rename = "oauth2TwoFactorEnable")]
    pub oauth2_two_factor_enable: String,
    #[serde(rename = "oauth2TwoFactorEnableComplete")]
    pub oauth2_two_factor_enable_complete: String,
    #[serde(rename = "oauth2TwoFactorMethods")]
    pub oauth2_two_factor_methods: String,
    #[serde(rename = "oauth2Wait")]
    pub oauth2_wait: String,
    #[serde(rename = "oauth2WebAuthn")]
    pub oauth2_web_authn: String,
    #[serde(rename = "oauth2WebAuthnReauth")]
    pub oauth2_web_authn_reauth: String,
    #[serde(rename = "oauth2WebAuthnReauthEnable")]
    pub oauth2_web_authn_reauth_enable: String,
    pub password_change: String,
    pub password_complete: String,
    pub password_forgot: String,
    pub password_sent: String,
    pub registration_complete: String,
    pub registration_send: String,
    pub registration_sent: String,
    pub registration_verification_required: String,
    pub registration_verify: String,
    #[serde(rename = "samlv2Logout")]
    pub samlv2_logout: String,
    pub unauthorized: String,
}
