use crate::result::IResult;

pub trait AltList<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O>;
    fn len(&self) -> usize;
}

impl<I, O, F0, F1> AltList<I, O> for (F0, F1) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 2 }
}

impl<I, O, F0, F1, F2> AltList<I, O> for (F0, F1, F2) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 3 }
}

impl<I, O, F0, F1, F2, F3> AltList<I, O> for (F0, F1, F2, F3) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 4 }
}

impl<I, O, F0, F1, F2, F3, F4> AltList<I, O> for (F0, F1, F2, F3, F4) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 5 }
}

impl<I, O, F0, F1, F2, F3, F4, F5> AltList<I, O> for (F0, F1, F2, F3, F4, F5) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 6 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 7 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 8 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 9 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 10 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O>,
    F10: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            10 => f10(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 11 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O>,
    F10: Fn(I) -> IResult<I, O>,
    F11: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            10 => f10(input),
            11 => f11(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 12 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O>,
    F10: Fn(I) -> IResult<I, O>,
    F11: Fn(I) -> IResult<I, O>,
    F12: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            10 => f10(input),
            11 => f11(input),
            12 => f12(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 13 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O>,
    F10: Fn(I) -> IResult<I, O>,
    F11: Fn(I) -> IResult<I, O>,
    F12: Fn(I) -> IResult<I, O>,
    F13: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            10 => f10(input),
            11 => f11(input),
            12 => f12(input),
            13 => f13(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 14 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O>,
    F10: Fn(I) -> IResult<I, O>,
    F11: Fn(I) -> IResult<I, O>,
    F12: Fn(I) -> IResult<I, O>,
    F13: Fn(I) -> IResult<I, O>,
    F14: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            10 => f10(input),
            11 => f11(input),
            12 => f12(input),
            13 => f13(input),
            14 => f14(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 15 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O>,
    F10: Fn(I) -> IResult<I, O>,
    F11: Fn(I) -> IResult<I, O>,
    F12: Fn(I) -> IResult<I, O>,
    F13: Fn(I) -> IResult<I, O>,
    F14: Fn(I) -> IResult<I, O>,
    F15: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            10 => f10(input),
            11 => f11(input),
            12 => f12(input),
            13 => f13(input),
            14 => f14(input),
            15 => f15(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 16 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O>,
    F10: Fn(I) -> IResult<I, O>,
    F11: Fn(I) -> IResult<I, O>,
    F12: Fn(I) -> IResult<I, O>,
    F13: Fn(I) -> IResult<I, O>,
    F14: Fn(I) -> IResult<I, O>,
    F15: Fn(I) -> IResult<I, O>,
    F16: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15, f16) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            10 => f10(input),
            11 => f11(input),
            12 => f12(input),
            13 => f13(input),
            14 => f14(input),
            15 => f15(input),
            16 => f16(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 17 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O>,
    F10: Fn(I) -> IResult<I, O>,
    F11: Fn(I) -> IResult<I, O>,
    F12: Fn(I) -> IResult<I, O>,
    F13: Fn(I) -> IResult<I, O>,
    F14: Fn(I) -> IResult<I, O>,
    F15: Fn(I) -> IResult<I, O>,
    F16: Fn(I) -> IResult<I, O>,
    F17: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15, f16, f17) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            10 => f10(input),
            11 => f11(input),
            12 => f12(input),
            13 => f13(input),
            14 => f14(input),
            15 => f15(input),
            16 => f16(input),
            17 => f17(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 18 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O>,
    F10: Fn(I) -> IResult<I, O>,
    F11: Fn(I) -> IResult<I, O>,
    F12: Fn(I) -> IResult<I, O>,
    F13: Fn(I) -> IResult<I, O>,
    F14: Fn(I) -> IResult<I, O>,
    F15: Fn(I) -> IResult<I, O>,
    F16: Fn(I) -> IResult<I, O>,
    F17: Fn(I) -> IResult<I, O>,
    F18: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15, f16, f17, f18) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            10 => f10(input),
            11 => f11(input),
            12 => f12(input),
            13 => f13(input),
            14 => f14(input),
            15 => f15(input),
            16 => f16(input),
            17 => f17(input),
            18 => f18(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 19 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O>,
    F10: Fn(I) -> IResult<I, O>,
    F11: Fn(I) -> IResult<I, O>,
    F12: Fn(I) -> IResult<I, O>,
    F13: Fn(I) -> IResult<I, O>,
    F14: Fn(I) -> IResult<I, O>,
    F15: Fn(I) -> IResult<I, O>,
    F16: Fn(I) -> IResult<I, O>,
    F17: Fn(I) -> IResult<I, O>,
    F18: Fn(I) -> IResult<I, O>,
    F19: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15, f16, f17, f18, f19) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            10 => f10(input),
            11 => f11(input),
            12 => f12(input),
            13 => f13(input),
            14 => f14(input),
            15 => f15(input),
            16 => f16(input),
            17 => f17(input),
            18 => f18(input),
            19 => f19(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 20 }
}

impl<I, O, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20> AltList<I, O> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20) where
    F0: Fn(I) -> IResult<I, O>,
    F1: Fn(I) -> IResult<I, O>,
    F2: Fn(I) -> IResult<I, O>,
    F3: Fn(I) -> IResult<I, O>,
    F4: Fn(I) -> IResult<I, O>,
    F5: Fn(I) -> IResult<I, O>,
    F6: Fn(I) -> IResult<I, O>,
    F7: Fn(I) -> IResult<I, O>,
    F8: Fn(I) -> IResult<I, O>,
    F9: Fn(I) -> IResult<I, O>,
    F10: Fn(I) -> IResult<I, O>,
    F11: Fn(I) -> IResult<I, O>,
    F12: Fn(I) -> IResult<I, O>,
    F13: Fn(I) -> IResult<I, O>,
    F14: Fn(I) -> IResult<I, O>,
    F15: Fn(I) -> IResult<I, O>,
    F16: Fn(I) -> IResult<I, O>,
    F17: Fn(I) -> IResult<I, O>,
    F18: Fn(I) -> IResult<I, O>,
    F19: Fn(I) -> IResult<I, O>,
    F20: Fn(I) -> IResult<I, O> {
    fn apply_nth(&self, n: usize, input: I) -> IResult<I, O> {
        let (f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15, f16, f17, f18, f19, f20) = self;
        match n {
            0 => f0(input),
            1 => f1(input),
            2 => f2(input),
            3 => f3(input),
            4 => f4(input),
            5 => f5(input),
            6 => f6(input),
            7 => f7(input),
            8 => f8(input),
            9 => f9(input),
            10 => f10(input),
            11 => f11(input),
            12 => f12(input),
            13 => f13(input),
            14 => f14(input),
            15 => f15(input),
            16 => f16(input),
            17 => f17(input),
            18 => f18(input),
            19 => f19(input),
            20 => f20(input),
            _ => unreachable!()
        }
    }
    fn len(&self) -> usize { 21 }
}
