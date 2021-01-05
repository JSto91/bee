// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default, Debug)]
pub struct ProtocolMetrics {
    invalid_packets: AtomicU64,

    milestone_requests_received: AtomicU64,
    messages_received: AtomicU64,
    message_requests_received: AtomicU64,
    heartbeats_received: AtomicU64,

    milestone_requests_sent: AtomicU64,
    messages_sent: AtomicU64,
    message_requests_sent: AtomicU64,
    heartbeats_sent: AtomicU64,

    invalid_messages: AtomicU64,
    new_messages: AtomicU64,
    known_messages: AtomicU64,
    messages_average_latency: AtomicU64,

    referenced_messages: AtomicU64,
    excluded_no_transaction_messages: AtomicU64,
    excluded_conflicting_messages: AtomicU64,
    included_messages: AtomicU64,
}

impl ProtocolMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ProtocolMetrics {
    pub fn invalid_packets(&self) -> u64 {
        self.invalid_packets.load(Ordering::Relaxed)
    }

    pub(crate) fn invalid_packets_inc(&self) -> u64 {
        self.invalid_packets.fetch_add(1, Ordering::SeqCst)
    }

    pub fn milestone_requests_received(&self) -> u64 {
        self.milestone_requests_received.load(Ordering::Relaxed)
    }

    pub(crate) fn milestone_requests_received_inc(&self) -> u64 {
        self.milestone_requests_received.fetch_add(1, Ordering::SeqCst)
    }

    pub fn messages_received(&self) -> u64 {
        self.messages_received.load(Ordering::Relaxed)
    }

    pub(crate) fn messages_received_inc(&self) -> u64 {
        self.messages_received.fetch_add(1, Ordering::SeqCst)
    }

    pub fn message_requests_received(&self) -> u64 {
        self.message_requests_received.load(Ordering::Relaxed)
    }

    pub(crate) fn message_requests_received_inc(&self) -> u64 {
        self.message_requests_received.fetch_add(1, Ordering::SeqCst)
    }

    pub fn heartbeats_received(&self) -> u64 {
        self.heartbeats_received.load(Ordering::Relaxed)
    }

    pub(crate) fn heartbeats_received_inc(&self) -> u64 {
        self.heartbeats_received.fetch_add(1, Ordering::SeqCst)
    }

    pub fn milestone_requests_sent(&self) -> u64 {
        self.milestone_requests_sent.load(Ordering::Relaxed)
    }

    pub(crate) fn milestone_requests_sent_inc(&self) -> u64 {
        self.milestone_requests_sent.fetch_add(1, Ordering::SeqCst)
    }

    pub fn messages_sent(&self) -> u64 {
        self.messages_sent.load(Ordering::Relaxed)
    }

    pub(crate) fn messages_sent_inc(&self) -> u64 {
        self.messages_sent.fetch_add(1, Ordering::SeqCst)
    }

    pub fn message_requests_sent(&self) -> u64 {
        self.message_requests_sent.load(Ordering::Relaxed)
    }

    pub(crate) fn message_requests_sent_inc(&self) -> u64 {
        self.message_requests_sent.fetch_add(1, Ordering::SeqCst)
    }

    pub fn heartbeats_sent(&self) -> u64 {
        self.heartbeats_sent.load(Ordering::Relaxed)
    }

    pub(crate) fn heartbeats_sent_inc(&self) -> u64 {
        self.heartbeats_sent.fetch_add(1, Ordering::SeqCst)
    }

    pub fn invalid_messages(&self) -> u64 {
        self.invalid_messages.load(Ordering::Relaxed)
    }

    pub(crate) fn invalid_messages_inc(&self) -> u64 {
        self.invalid_messages.fetch_add(1, Ordering::SeqCst)
    }

    pub fn new_messages(&self) -> u64 {
        self.new_messages.load(Ordering::Relaxed)
    }

    pub(crate) fn new_messages_inc(&self) -> u64 {
        self.new_messages.fetch_add(1, Ordering::SeqCst)
    }

    pub fn known_messages(&self) -> u64 {
        self.known_messages.load(Ordering::Relaxed)
    }

    pub(crate) fn known_messages_inc(&self) -> u64 {
        self.known_messages.fetch_add(1, Ordering::SeqCst)
    }

    pub fn messages_average_latency(&self) -> u64 {
        self.messages_average_latency.load(Ordering::Relaxed)
    }

    pub(crate) fn messages_average_latency_set(&self, val: u64) {
        self.messages_average_latency.store(val, Ordering::Relaxed)
    }

    pub fn referenced_messages(&self) -> u64 {
        self.referenced_messages.load(Ordering::Relaxed)
    }

    pub fn referenced_messages_inc(&self, value: u64) -> u64 {
        self.referenced_messages.fetch_add(value, Ordering::SeqCst)
    }

    pub fn excluded_no_transaction_messages(&self) -> u64 {
        self.excluded_no_transaction_messages.load(Ordering::Relaxed)
    }

    pub fn excluded_no_transaction_messages_inc(&self, value: u64) -> u64 {
        self.excluded_no_transaction_messages.fetch_add(value, Ordering::SeqCst)
    }

    pub fn excluded_conflicting_messages(&self) -> u64 {
        self.excluded_conflicting_messages.load(Ordering::Relaxed)
    }

    pub fn excluded_conflicting_messages_inc(&self, value: u64) -> u64 {
        self.excluded_conflicting_messages.fetch_add(value, Ordering::SeqCst)
    }

    pub fn included_messages(&self) -> u64 {
        self.included_messages.load(Ordering::Relaxed)
    }

    pub fn included_messages_inc(&self, value: u64) -> u64 {
        self.included_messages.fetch_add(value, Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn protocol_metrics() {
        let metrics = ProtocolMetrics::default();

        assert_eq!(metrics.invalid_packets(), 0);
        assert_eq!(metrics.milestone_requests_received(), 0);
        assert_eq!(metrics.messages_received(), 0);
        assert_eq!(metrics.message_requests_received(), 0);
        assert_eq!(metrics.heartbeats_received(), 0);
        assert_eq!(metrics.milestone_requests_sent(), 0);
        assert_eq!(metrics.messages_sent(), 0);
        assert_eq!(metrics.message_requests_sent(), 0);
        assert_eq!(metrics.heartbeats_sent(), 0);
        assert_eq!(metrics.invalid_messages(), 0);
        assert_eq!(metrics.new_messages(), 0);
        assert_eq!(metrics.known_messages(), 0);
        assert_eq!(metrics.messages_average_latency(), 0);
        assert_eq!(metrics.referenced_messages(), 0);
        assert_eq!(metrics.excluded_no_transaction_messages(), 0);
        assert_eq!(metrics.excluded_conflicting_messages(), 0);
        assert_eq!(metrics.included_messages(), 0);

        metrics.invalid_packets_inc();
        metrics.milestone_requests_received_inc();
        metrics.messages_received_inc();
        metrics.message_requests_received_inc();
        metrics.heartbeats_received_inc();
        metrics.milestone_requests_sent_inc();
        metrics.messages_sent_inc();
        metrics.message_requests_sent_inc();
        metrics.heartbeats_sent_inc();
        metrics.invalid_messages_inc();
        metrics.new_messages_inc();
        metrics.known_messages_inc();
        metrics.messages_average_latency_set(42);
        metrics.referenced_messages_inc(1);
        metrics.excluded_no_transaction_messages_inc(1);
        metrics.excluded_conflicting_messages_inc(1);
        metrics.included_messages_inc(1);

        assert_eq!(metrics.invalid_packets(), 1);
        assert_eq!(metrics.milestone_requests_received(), 1);
        assert_eq!(metrics.messages_received(), 1);
        assert_eq!(metrics.message_requests_received(), 1);
        assert_eq!(metrics.heartbeats_received(), 1);
        assert_eq!(metrics.milestone_requests_sent(), 1);
        assert_eq!(metrics.messages_sent(), 1);
        assert_eq!(metrics.message_requests_sent(), 1);
        assert_eq!(metrics.heartbeats_sent(), 1);
        assert_eq!(metrics.invalid_messages(), 1);
        assert_eq!(metrics.new_messages(), 1);
        assert_eq!(metrics.known_messages(), 1);
        assert_eq!(metrics.messages_average_latency(), 42);
        assert_eq!(metrics.referenced_messages(), 1);
        assert_eq!(metrics.excluded_no_transaction_messages(), 1);
        assert_eq!(metrics.excluded_conflicting_messages(), 1);
        assert_eq!(metrics.included_messages(), 1);
    }
}
