// N shoud be a power of 2 and at least 8
pub(crate) struct BitArray<const N: usize>
where
    [u8; N / 8]: Sized,
{
    data: [u8; N / 8],
}

impl<const N: usize> BitArray<N>
where
    [u8; N / 8]: Sized,
{
    // Unfortunately, this is the best compile-time assertion I could come up with.
    // const_guards crate seems promising, but I didn't get from docs how to use it here.
    const TEST_N: () = assert!((N >= 8) && N.is_power_of_two());
    pub(crate) fn new() -> Self {
        Self::TEST_N;
        Self { data: [0; N / 8] }
    }

    pub(crate) fn set(&mut self, index: usize) {
        self.data[index >> 3] |= 1 << (index & 7);
    }

    pub(crate) fn get(&self, index: usize) -> bool {
        (self.data[index >> 3] & (1 << (index & 7))) != 0
    }
}
