// Diag command constants
// Origin: http://cgit.osmocom.org/osmo-qcdiag/tree/src/protocol/diagcmd.h

pub const DIAG_VERNO_F: [u8;1] = [0x00];
pub const DIAG_STATUS_F: [u8;1] = [0x0c];
pub const DIAG_LOG_F: [u8;1] = [0x10]; // Log packet Request/Reponse
pub const DIAG_BAD_CMD_F: [u8;1] = [0x13];
pub const DIAG_DIAG_VER_F: [u8;1] = [0x1c]; // Version response
pub const DIAG_TS_F: [u8;1] = [0x1d];
pub const DIAG_SUBSYS_CMD_F: [u8;1] = [0x4b];
pub const DIAG_EVENT_REPORT_F: [u8;1] = [0x60]; // Static Event reporting
pub const DIAG_STATUS_SNAPSHOT_F: [u8;1] = [0x63];
pub const DIAG_LOG_CONFIG_F: [u8;1] = [0x73]; // Logging configuration packet
pub const DIAG_EXT_MSG_F: [u8;1] = [0x79]; // Request for extended message report
pub const DIAG_EXT_BUILD_ID_F: [u8;1] = [0x7c];
pub const DIAG_EXT_MSG_CONFIG_F: [u8;1] = [0x7d]; // Request for Extended message report
pub const DIAG_EXT_MSG_TERSE_F: [u8;1] = [0x7e];
pub const DIAG_SUBSYS_CMD_VER_2_F: [u8;1] = [0x80];
pub const DIAG_EVENT_MASK_GET_F: [u8;1] = [0x81];
pub const DIAG_EVENT_MASK_SET_F: [u8;1] = [0x82];
pub const DIAG_QSR_EXT_MSG_TERSE_F: [u8;1] = [0x92]; // QSR extended messages
pub const DIAG_MULTI_RADIO_CMD_F: [u8;1] = [0x98]; // Found on newer dual SIMs
pub const DIAG_QSR4_EXT_MSG_TERSE_F: [u8;1] = [0x99]; // QSR4 extended messages
pub const DIAG_MSG_SMALL_F: [u8;1] = [0x9c];
pub const DIAG_QSH_TRACE_PAYLOAD_F: [u8;1] = [0x9d];
pub const DIAG_SECURE_LOG_F: [u8;1] = [0x9e];

pub const DIAG_SUBSYS_ID_1X: [u8;1] = [0x01];
pub const DIAG_SUBSYS_ID_WCDMA: [u8;1] = [0x04];
pub const DIAG_SUBSYS_ID_GSM: [u8;1] = [0x05];
pub const DIAG_SUBSYS_ID_UMTS: [u8;1] = [0x07];
pub const DIAG_SUBSYS_ID_DTV: [u8;1] = [0x0A];
pub const DIAG_SUBSYS_ID_APPS: [u8;1] = [0x0B];
pub const DIAG_SUBSYS_ID_LTE: [u8;1] = [0x0B]; // Also shared by NR
pub const DIAG_SUBSYS_ID_TDSCDMA: [u8;1] = [0x0D];

// Log configuration operations
// Origin: http://cgit.osmocom.org/osmo-qcdiag/tree/src/protocol/diag_log.c
pub const LOG_CONFIG_DISABLE_OP: [u8;1] = [0];
pub const LOG_CONFIG_RETRIEVE_ID_RANGES_OP: [u8;1] = [1];
pub const LOG_CONFIG_RETRIEVE_VALID_MASK_OP: [u8;1] = [2];
pub const LOG_CONFIG_SET_MASK_OP: [u8;1] = [3];
pub const LOG_CONFIG_GET_LOGMASK_OP: [u8;1] = [4];