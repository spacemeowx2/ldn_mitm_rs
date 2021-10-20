#![allow(dead_code)]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct VirtualAmiiboUuidInfo {
    use_random_uuid: bool,
    uuid: [u8; 10],
}

const SSID_LENGTH_MAX: usize = 32;
const ADVERTISE_DATA_SIZE_MAX: usize = 384;
const USERNAME_BYTES_MAX: usize = 32;
const NODE_COUNT_MAX: usize = 8;
const STATION_COUNT_MAX: usize = NODE_COUNT_MAX - 1;
const PASSPHRASE_LENGTH_MAX: usize = 64;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum NodeStateChange {
    None = 0,
    Connect = 1,
    Disconnect = 2,
    DisconnectAndConnect = 3,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum ScanFilterFlag {
    LocalCommunicationId = 1 << 0,
    SessionId = 1 << 1,
    NetworkType = 1 << 2,
    Ssid = 1 << 4,
    SceneId = 1 << 5,
    // IntentId = LocalCommunicationId | SceneId,
    IntentId = (1 << 0) | (1 << 5),
    // NetworkId = IntentId | SessionId,
    NetworkId = ((1 << 0) | (1 << 5)) | (1 << 1),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum CommState {
    None,
    Initialized,
    AccessPoint,
    AccessPointCreated,
    Station,
    StationConnected,
    Error,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct MacAddress([u8; 6]);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Ipv4Address([u8; 4]);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Ssid {
    length: u8,
    raw: [u8; SSID_LENGTH_MAX + 1],
}

impl Default for Ssid {
    fn default() -> Self {
        Self {
            length: Default::default(),
            raw: [0; SSID_LENGTH_MAX + 1],
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct CommonNetworkInfo {
    bssid: MacAddress,
    ssid: Ssid,
    channel: u16,
    link_level: u8,
    network_type: u8,
    _unk: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct NodeInfo {
    ipv4_address: u32,
    mac_address: MacAddress,
    node_id: i8,
    is_connected: i8,
    username: [u8; USERNAME_BYTES_MAX + 1],
    _unk1: u8,
    local_communication_version: u16,
    _unk2: [u8; 16],
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct LdnNetworkInfo {
    unk_random: [u8; 16],
    security_mode: u16,
    station_accept_policy: u8,
    _unk1: [u8; 3],
    node_count_max: u8,
    node_count: u8,
    nodes: [NodeInfo; NODE_COUNT_MAX],
    _unk2: u16,
    advertise_data_size: u16,
    advertise_data: [u8; ADVERTISE_DATA_SIZE_MAX],
    _unk3: [u8; 148],
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct IntentId {
    local_communication_id: u64,
    _unk1: u16,
    scene_id: u16,
    _unk2: u16,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct SessionId(u128);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct NetworkId {
    intent_id: IntentId,   // 16bytes
    session_id: SessionId, // 16bytes
} // 32bytes

// sf::LargeData
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct NetworkInfo {
    network_id: NetworkId,
    common: CommonNetworkInfo,
    ldn: LdnNetworkInfo,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct SecurityConfig {
    security_mode: u16,
    passphrase_size: u16,
    passphrase: [u8; PASSPHRASE_LENGTH_MAX],
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct UserConfig {
    username: [u8; USERNAME_BYTES_MAX + 1],
    _unk: [u8; 15],
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct NetworkConfig {
    intent_id: IntentId, // 16byte
    channel: u16,
    node_count_max: u8,
    _unk1: u8,
    local_communication_version: u16,
    _unk2: [u8; 10],
} // 32bytes

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct CreateNetworkConfig {
    security_config: SecurityConfig,
    user_config: UserConfig,
    _unk: [u8; 4],
    network_config: NetworkConfig,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct ConnectNetworkData {
    security_config: SecurityConfig,
    user_config: UserConfig,
    local_communication_version: u32,
    option: u32,
}

// sf::PrefersPointerTransferMode
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct NodeLatestUpdate {
    state_change: u8,
    _unk: [u8; 7],
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct SecurityParameter {
    unk_random: [u8; 16],
    session_id: SessionId,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct ScanFilter {
    network_id: NetworkId,
    network_type: u32,
    bssid: MacAddress,
    ssid: Ssid,
    unk: [u8; 16],
    flag: u32,
}
