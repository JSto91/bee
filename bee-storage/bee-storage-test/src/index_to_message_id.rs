// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use bee_message::{payload::indexation::PaddedIndex, MessageId};
use bee_storage::{
    access::{AsIterator, Batch, BatchBuilder, Delete, Exist, Fetch, Insert, Truncate},
    backend,
};
use bee_test::rand::{message::rand_message_id, payload::rand_indexation_payload};

pub trait StorageBackend:
    backend::StorageBackend
    + Exist<(PaddedIndex, MessageId), ()>
    + Fetch<PaddedIndex, Vec<MessageId>>
    + Insert<(PaddedIndex, MessageId), ()>
    + Delete<(PaddedIndex, MessageId), ()>
    + BatchBuilder
    + Batch<(PaddedIndex, MessageId), ()>
    + for<'a> AsIterator<'a, (PaddedIndex, MessageId), ()>
    + Truncate<(PaddedIndex, MessageId), ()>
{
}

impl<T> StorageBackend for T where
    T: backend::StorageBackend
        + Exist<(PaddedIndex, MessageId), ()>
        + Fetch<PaddedIndex, Vec<MessageId>>
        + Insert<(PaddedIndex, MessageId), ()>
        + Delete<(PaddedIndex, MessageId), ()>
        + BatchBuilder
        + Batch<(PaddedIndex, MessageId), ()>
        + for<'a> AsIterator<'a, (PaddedIndex, MessageId), ()>
        + Truncate<(PaddedIndex, MessageId), ()>
{
}

pub fn index_to_message_id_access<B: StorageBackend>(storage: &B) {
    let (index, message_id) = (rand_indexation_payload().padded_index(), rand_message_id());

    assert!(!Exist::<(PaddedIndex, MessageId), ()>::exist(storage, &(index, message_id)).unwrap());
    assert!(
        Fetch::<PaddedIndex, Vec<MessageId>>::fetch(storage, &index)
            .unwrap()
            .unwrap()
            .is_empty()
    );

    Insert::<(PaddedIndex, MessageId), ()>::insert(storage, &(index, message_id), &()).unwrap();

    assert!(Exist::<(PaddedIndex, MessageId), ()>::exist(storage, &(index, message_id)).unwrap());
    assert_eq!(
        Fetch::<PaddedIndex, Vec<MessageId>>::fetch(storage, &index)
            .unwrap()
            .unwrap(),
        vec![message_id]
    );

    Delete::<(PaddedIndex, MessageId), ()>::delete(storage, &(index, message_id)).unwrap();

    assert!(!Exist::<(PaddedIndex, MessageId), ()>::exist(storage, &(index, message_id)).unwrap());
    assert!(
        Fetch::<PaddedIndex, Vec<MessageId>>::fetch(storage, &index)
            .unwrap()
            .unwrap()
            .is_empty()
    );

    let mut batch = B::batch_begin();

    for _ in 0..10 {
        let (index, message_id) = (rand_indexation_payload().padded_index(), rand_message_id());
        Insert::<(PaddedIndex, MessageId), ()>::insert(storage, &(index, message_id), &()).unwrap();
        Batch::<(PaddedIndex, MessageId), ()>::batch_delete(storage, &mut batch, &(index, message_id)).unwrap();
    }

    let mut message_ids = HashMap::<PaddedIndex, Vec<MessageId>>::new();

    for _ in 0..5 {
        let index = rand_indexation_payload().padded_index();
        for _ in 0..5 {
            let message_id = rand_message_id();
            Batch::<(PaddedIndex, MessageId), ()>::batch_insert(storage, &mut batch, &(index, message_id), &())
                .unwrap();
            message_ids.entry(index).or_default().push(message_id);
        }
    }

    storage.batch_commit(batch, true).unwrap();

    let iter = AsIterator::<(PaddedIndex, MessageId), ()>::iter(storage).unwrap();
    let mut count = 0;

    for result in iter {
        let ((index, message_id), _) = result.unwrap();
        assert!(message_ids.get(&index).unwrap().contains(&message_id));
        count += 1;
    }

    assert_eq!(count, message_ids.iter().fold(0, |acc, v| acc + v.1.len()));

    Truncate::<(PaddedIndex, MessageId), ()>::truncate(storage).unwrap();

    let mut iter = AsIterator::<(PaddedIndex, MessageId), ()>::iter(storage).unwrap();

    assert!(iter.next().is_none());
}
