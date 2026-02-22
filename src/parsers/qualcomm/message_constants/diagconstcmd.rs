// Diag command constants
// Origin: http://cgit.osmocom.org/osmo-qcdiag/tree/src/protocol/diagcmd.h

const DIAG_VERNO_F: u16 = 0x00;
const DIAG_STATUS_F: u16 = 0x0c;
const DIAG_LOG_F: u16 = 0x10; // Log packet Request/Reponse
const DIAG_BAD_CMD_F: u16 = 0x13;
const DIAG_DIAG_VER_F: u16 = 0x1c; // Version response
const DIAG_TS_F: u16 = 0x1d;
const DIAG_SUBSYS_CMD_F: u16 = 0x4b;
const DIAG_EVENT_REPORT_F: u16 = 0x60; // Static Event reporting
const DIAG_STATUS_SNAPSHOT_F: u16 = 0x63;
const DIAG_LOG_CONFIG_F: u16 = 0x73; // Logging configuration packet
const DIAG_EXT_MSG_F: u16 = 0x79; // Request for extended message report
const DIAG_EXT_BUILD_ID_F: u16 = 0x7c;
const DIAG_EXT_MSG_CONFIG_F: u16 = 0x7d; // Request for Extended message report
const DIAG_EXT_MSG_TERSE_F: u16 = 0x7e;
const DIAG_SUBSYS_CMD_VER_2_F: u16 = 0x80;
const DIAG_EVENT_MASK_GET_F: u16 = 0x81;
const DIAG_EVENT_MASK_SET_F: u16 = 0x82;
const DIAG_QSR_EXT_MSG_TERSE_F: u16 = 0x92; // QSR extended messages
const DIAG_MULTI_RADIO_CMD_F: u16 = 0x98; // Found on newer dual SIMs
const DIAG_QSR4_EXT_MSG_TERSE_F: u16 = 0x99; // QSR4 extended messages
const DIAG_MSG_SMALL_F: u16 = 0x9c;
const DIAG_QSH_TRACE_PAYLOAD_F: u16 = 0x9d;
const DIAG_SECURE_LOG_F: u16 = 0x9e;

const DIAG_SUBSYS_ID_1X: u16 = 0x01;
const DIAG_SUBSYS_ID_WCDMA: u16 = 0x04;
const DIAG_SUBSYS_ID_GSM: u16 = 0x05;
const DIAG_SUBSYS_ID_UMTS: u16 = 0x07;
const DIAG_SUBSYS_ID_DTV: u16 = 0x0A;
const DIAG_SUBSYS_ID_APPS: u16 = 0x0B;
const DIAG_SUBSYS_ID_LTE: u16 = 0x0B; // Also shared by NR
const DIAG_SUBSYS_ID_TDSCDMA: u16 = 0x0D;

// Log configuration operations
// Origin: http://cgit.osmocom.org/osmo-qcdiag/tree/src/protocol/diag_log.c
const LOG_CONFIG_DISABLE_OP: u16 = 0;
const LOG_CONFIG_RETRIEVE_ID_RANGES_OP: u16 = 1;
const LOG_CONFIG_RETRIEVE_VALID_MASK_OP: u16 = 2;
const LOG_CONFIG_SET_MASK_OP: u16 = 3;
const LOG_CONFIG_GET_LOGMASK_OP: u16 = 4;