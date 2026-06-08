// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct MyContainer {
    pub data: [i32; 3],
}

pub struct MyContainerIntoIter {
    data: [i32; 3],
    index: usize,
}

impl Iterator for MyContainerIntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl IntoIterator for MyContainer {
    type Item = i32;
    type IntoIter = MyContainerIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        MyContainerIntoIter { data: self.data, index: 0 }
    }
}

pub struct MyContainerIter<'a> {
    data: &'a [i32],
}

impl<'a> Iterator for MyContainerIter<'a> {
    type Item = &'a i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.data.split_off_first()
    }
}

impl<'a> IntoIterator for &'a MyContainer {
    type Item = &'a i32;
    type IntoIter = MyContainerIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MyContainerIter { data: &self.data }
    }
}

pub struct MyContainerIterMut<'a> {
    data: &'a mut [i32],
}

impl<'a> Iterator for MyContainerIterMut<'a> {
    type Item = &'a mut i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.data.split_off_first_mut()
    }
}

impl<'a> IntoIterator for &'a mut MyContainer {
    type Item = &'a mut i32;
    type IntoIter = MyContainerIterMut<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MyContainerIterMut { data: &mut self.data }
    }
}

pub fn make_container(a: i32, b: i32, c: i32) -> MyContainer {
    MyContainer { data: [a, b, c] }
}

pub struct MyIterator {
    pub value: i32,
}

impl Iterator for MyIterator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value)
    }
}

pub struct ContainerWithRefIntoIter<'a> {
    pub iter: &'a mut MyIterator,
}

impl<'a> IntoIterator for ContainerWithRefIntoIter<'a> {
    type Item = i32;
    type IntoIter = &'a mut MyIterator;
    fn into_iter(self) -> Self::IntoIter {
        self.iter
    }
}

pub struct ContainerWithInherentBegin {
    pub data: [i32; 3],
}

pub struct SimpleIntoIter {
    pub val: i32,
}
impl Iterator for SimpleIntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl IntoIterator for ContainerWithInherentBegin {
    type Item = i32;
    type IntoIter = SimpleIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        SimpleIntoIter { val: 0 }
    }
}

impl ContainerWithInherentBegin {
    pub fn begin(&self) -> i32 {
        42
    }
}

pub fn make_inherent_container() -> ContainerWithInherentBegin {
    ContainerWithInherentBegin { data: [1, 2, 3] }
}

pub fn make_iterator(value: i32) -> MyIterator {
    MyIterator { value }
}

pub fn make_ref_container<'a>(iter: &'a mut MyIterator) -> ContainerWithRefIntoIter<'a> {
    ContainerWithRefIntoIter { iter }
}
