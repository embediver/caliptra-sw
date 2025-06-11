// Licensed under the Apache-2.0 license

#![allow(non_camel_case_types)]

use caliptra_api_c_binding_proc_literal::mailbox_command_id_literal;

#[repr(C)]
pub enum mailbox_command {
    FIRMWARE_LOAD = mailbox_command_id_literal!(CommandId::FIRMWARE_LOAD), // "FWLD"
    GET_IDEV_CERT = mailbox_command_id_literal!(CommandId::GET_IDEV_CERT), // "IDEC"
    GET_IDEV_INFO = mailbox_command_id_literal!(CommandId::GET_IDEV_INFO), // "IDEI"
    POPULATE_IDEV_CERT = mailbox_command_id_literal!(CommandId::POPULATE_IDEV_CERT), // "IDEP"
    GET_LDEV_CERT = mailbox_command_id_literal!(CommandId::GET_LDEV_CERT), // "LDEV"
    GET_RT_ALIAS_CERT = mailbox_command_id_literal!(CommandId::GET_RT_ALIAS_CERT), // "CERR"
    ECDSA384_VERIFY = mailbox_command_id_literal!(CommandId::ECDSA384_VERIFY), // "SIGV"
    LMS_VERIFY = mailbox_command_id_literal!(CommandId::LMS_VERIFY),       // "LMSV"
    STASH_MEASUREMENT = mailbox_command_id_literal!(CommandId::STASH_MEASUREMENT), // "MEAS"
    INVOKE_DPE = mailbox_command_id_literal!(CommandId::INVOKE_DPE),       // "DPEC"
    DISABLE_ATTESTATION = mailbox_command_id_literal!(CommandId::DISABLE_ATTESTATION), // "DSBL"
    FW_INFO = mailbox_command_id_literal!(CommandId::FW_INFO),             // "INFO"
    DPE_TAG_TCI = mailbox_command_id_literal!(CommandId::DPE_TAG_TCI),     // "TAGT"
    DPE_GET_TAGGED_TCI = mailbox_command_id_literal!(CommandId::DPE_GET_TAGGED_TCI), // "GTGD"
    INCREMENT_PCR_RESET_COUNTER =
        mailbox_command_id_literal!(CommandId::INCREMENT_PCR_RESET_COUNTER), // "PCRR"
    QUOTE_PCRS = mailbox_command_id_literal!(CommandId::QUOTE_PCRS),       // "PCRQ"
    EXTEND_PCR = mailbox_command_id_literal!(CommandId::EXTEND_PCR),       // "PCRE"
    ADD_SUBJECT_ALT_NAME = mailbox_command_id_literal!(CommandId::ADD_SUBJECT_ALT_NAME), // "ALTN"
    CERTIFY_KEY_EXTENDED = mailbox_command_id_literal!(CommandId::CERTIFY_KEY_EXTENDED), // "CKEX"

    // FIPS module commands.
    /// The status command.
    VERSION = mailbox_command_id_literal!(CommandId::VERSION), // "FPVR"
    /// The self-test command.
    SELF_TEST_START = mailbox_command_id_literal!(CommandId::SELF_TEST_START), // "FPST"
    /// The self-test get results.
    SELF_TEST_GET_RESULTS = mailbox_command_id_literal!(CommandId::SELF_TEST_GET_RESULTS), // "FPGR"
    /// The shutdown command.
    SHUTDOWN = mailbox_command_id_literal!(CommandId::SHUTDOWN), // "FPSD"
    /// The capabilities command.
    CAPABILITIES = mailbox_command_id_literal!(CommandId::CAPABILITIES), // "CAPS"
    /// The authorization manifest set command.
    SET_AUTH_MANIFEST = mailbox_command_id_literal!(CommandId::SET_AUTH_MANIFEST), // "ATMN"
    /// The authorize and stash command.
    AUTHORIZE_AND_STASH = mailbox_command_id_literal!(CommandId::AUTHORIZE_AND_STASH), // "ATSH"
    /// The get IDevID CSR command.
    GET_IDEV_CSR = mailbox_command_id_literal!(CommandId::GET_IDEV_CSR), // "IDCR"
    /// The get FMC Alias CSR command.
    GET_FMC_ALIAS_CSR = mailbox_command_id_literal!(CommandId::GET_FMC_ALIAS_CSR), // "FMCR"
    /// The sign with exported ecdsa command.
    SIGN_WITH_EXPORTED_ECDSA = mailbox_command_id_literal!(CommandId::SIGN_WITH_EXPORTED_ECDSA), // "SWEE"
    /// The revoke exported CDI handle command.
    REVOKE_EXPORTED_CDI_HANDLE = mailbox_command_id_literal!(CommandId::REVOKE_EXPORTED_CDI_HANDLE), // "RVCH"
}
