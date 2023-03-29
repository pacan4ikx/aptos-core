// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use cfg_block::cfg_block;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct PeerMonitoringServiceConfig {
    pub enable_peer_monitoring_client: bool, // Whether or not to spawn the monitoring client
    pub latency_monitoring: LatencyMonitoringConfig,
    pub max_concurrent_requests: u64, // Max num of concurrent server tasks
    pub max_network_channel_size: u64, // Max num of pending network messages
    pub max_request_jitter_ms: u64, // Max amount of jitter (ms) that a request will be delayed for
    pub metadata_update_interval_ms: u64, // The interval (ms) between metadata updates
    pub network_monitoring: NetworkMonitoringConfig,
    pub node_monitoring: NodeMonitoringConfig,
    pub peer_monitor_interval_ms: u64, // The interval (ms) between peer monitor executions

    // By default, network performance monitoring is disabled
    #[cfg(feature = "network-perf-test")]
    pub performance_monitoring: PerformanceMonitoringConfig,
}

impl Default for PeerMonitoringServiceConfig {
    fn default() -> Self {
        Self {
            enable_peer_monitoring_client: false,
            latency_monitoring: LatencyMonitoringConfig::default(),
            max_concurrent_requests: 1000,
            max_network_channel_size: 1000,
            max_request_jitter_ms: 1000, // Monitoring requests are very infrequent
            metadata_update_interval_ms: 5000,
            network_monitoring: NetworkMonitoringConfig::default(),
            node_monitoring: NodeMonitoringConfig::default(),
            peer_monitor_interval_ms: 1000,

            // By default, network performance monitoring is disabled
            #[cfg(feature = "network-perf-test")]
            performance_monitoring: PerformanceMonitoringConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct LatencyMonitoringConfig {
    pub latency_ping_interval_ms: u64, // The interval (ms) between latency pings for each peer
    pub latency_ping_timeout_ms: u64,  // The timeout (ms) for each latency ping
    pub max_latency_ping_failures: u64, // Max ping failures before the peer connection fails
    pub max_num_latency_pings_to_retain: usize, // The max latency pings to retain per peer
}

impl Default for LatencyMonitoringConfig {
    fn default() -> Self {
        Self {
            latency_ping_interval_ms: 30_000, // 30 seconds
            latency_ping_timeout_ms: 20_000,  // 20 seconds
            max_latency_ping_failures: 3,
            max_num_latency_pings_to_retain: 10,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct NetworkMonitoringConfig {
    pub network_info_request_interval_ms: u64, // The interval (ms) between network info requests
    pub network_info_request_timeout_ms: u64,  // The timeout (ms) for each network info request
}

impl Default for NetworkMonitoringConfig {
    fn default() -> Self {
        Self {
            network_info_request_interval_ms: 60_000, // 1 minute
            network_info_request_timeout_ms: 10_000,  // 10 seconds
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct NodeMonitoringConfig {
    pub node_info_request_interval_ms: u64, // The interval (ms) between node info requests
    pub node_info_request_timeout_ms: u64,  // The timeout (ms) for each node info request
}

impl Default for NodeMonitoringConfig {
    fn default() -> Self {
        Self {
            node_info_request_interval_ms: 20_000, // 20 seconds
            node_info_request_timeout_ms: 10_000,  // 10 seconds
        }
    }
}

// By default, network performance monitoring is disabled
cfg_block! {
    #[cfg(feature = "network-perf-test")] {
        #[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
        #[serde(default, deny_unknown_fields)]
        pub struct PerformanceMonitoringConfig {
            pub enable_direct_send_testing: bool, // Whether or not to enable direct send test mode
            pub direct_send_data_size: u64, // The size of the data to send in each direct send request
            pub direct_send_interval_usec: u64, // The interval (microseconds) between direct send requests
            pub enable_rpc_testing: bool,   // Whether or not to enable RPC test mode
            pub rpc_data_size: u64,         // The size of the data to send in each RPC request
            pub rpc_interval_usec: u64,       // The interval (microseconds) between RPC requests
            pub rpc_timeout_ms: u64,        // The timeout (ms) for each RPC request
        }

        impl Default for PerformanceMonitoringConfig {
            fn default() -> Self {
                Self {
                    enable_direct_send_test: false,    // Disable direct send test mode
                    direct_send_data_size: 512 * 1024, // 512 KB
                    direct_send_interval_ms: 1000,      // 1000 microseconds
                    enable_rpc_test: true,             // Enable RPC test mode
                    rpc_data_size: 512 * 1024,         // 512 KB
                    rpc_interval_usec: 1000,              // 1000 microseconds
                    rpc_timeout_ms: 10_000,            // 10 seconds
                }
            }
        }
    }
}
